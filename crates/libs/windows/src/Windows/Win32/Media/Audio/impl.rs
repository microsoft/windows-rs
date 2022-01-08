pub trait IActivateAudioInterfaceAsyncOperationImpl: Sized {
    fn GetActivateResult();
}
impl ::windows::core::RuntimeName for IActivateAudioInterfaceAsyncOperation {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IActivateAudioInterfaceAsyncOperation";
}
impl IActivateAudioInterfaceAsyncOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivateAudioInterfaceAsyncOperationImpl, const OFFSET: isize>() -> IActivateAudioInterfaceAsyncOperationVtbl {
        unsafe extern "system" fn GetActivateResult<Impl: IActivateAudioInterfaceAsyncOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivateResult(::core::mem::transmute_copy(&activateresult), ::core::mem::transmute_copy(&activatedinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivateAudioInterfaceAsyncOperation>, ::windows::core::GetTrustLevel, GetActivateResult::<Impl, OFFSET>)
    }
}
pub trait IActivateAudioInterfaceCompletionHandlerImpl: Sized {
    fn ActivateCompleted();
}
impl ::windows::core::RuntimeName for IActivateAudioInterfaceCompletionHandler {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IActivateAudioInterfaceCompletionHandler";
}
impl IActivateAudioInterfaceCompletionHandlerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivateAudioInterfaceCompletionHandlerImpl, const OFFSET: isize>() -> IActivateAudioInterfaceCompletionHandlerVtbl {
        unsafe extern "system" fn ActivateCompleted<Impl: IActivateAudioInterfaceCompletionHandlerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activateoperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateCompleted(&*(&activateoperation as *const <IActivateAudioInterfaceAsyncOperation as ::windows::core::Abi>::Abi as *const <IActivateAudioInterfaceAsyncOperation as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IActivateAudioInterfaceCompletionHandler>, ::windows::core::GetTrustLevel, ActivateCompleted::<Impl, OFFSET>)
    }
}
pub trait IAudioAmbisonicsControlImpl: Sized {
    fn SetData();
    fn SetHeadTracking();
    fn GetHeadTracking();
    fn SetRotation();
}
impl ::windows::core::RuntimeName for IAudioAmbisonicsControl {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioAmbisonicsControl";
}
impl IAudioAmbisonicsControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioAmbisonicsControlImpl, const OFFSET: isize>() -> IAudioAmbisonicsControlVtbl {
        unsafe extern "system" fn SetData<Impl: IAudioAmbisonicsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&pambisonicsparams as *const <AMBISONICS_PARAMS as ::windows::core::Abi>::Abi as *const <AMBISONICS_PARAMS as ::windows::core::DefaultType>::DefaultType), cbambisonicsparams) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetHeadTracking<Impl: IAudioAmbisonicsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benableheadtracking: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHeadTracking(&*(&benableheadtracking as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHeadTracking<Impl: IAudioAmbisonicsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenableheadtracking: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeadTracking(::core::mem::transmute_copy(&pbenableheadtracking)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IAudioAmbisonicsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRotation(x, y, z, w) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioAmbisonicsControl>, ::windows::core::GetTrustLevel, SetData::<Impl, OFFSET>, SetHeadTracking::<Impl, OFFSET>, GetHeadTracking::<Impl, OFFSET>, SetRotation::<Impl, OFFSET>)
    }
}
pub trait IAudioAutoGainControlImpl: Sized {
    fn GetEnabled();
    fn SetEnabled();
}
impl ::windows::core::RuntimeName for IAudioAutoGainControl {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioAutoGainControl";
}
impl IAudioAutoGainControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioAutoGainControlImpl, const OFFSET: isize>() -> IAudioAutoGainControlVtbl {
        unsafe extern "system" fn GetEnabled<Impl: IAudioAutoGainControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnabled(::core::mem::transmute_copy(&pbenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IAudioAutoGainControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(&*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioAutoGainControl>, ::windows::core::GetTrustLevel, GetEnabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>)
    }
}
pub trait IAudioBassImpl: Sized + IPerChannelDbLevelImpl {}
impl ::windows::core::RuntimeName for IAudioBass {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioBass";
}
impl IAudioBassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioBassImpl, const OFFSET: isize>() -> IAudioBassVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioBass>, ::windows::core::GetTrustLevel)
    }
}
pub trait IAudioCaptureClientImpl: Sized {
    fn GetBuffer();
    fn ReleaseBuffer();
    fn GetNextPacketSize();
}
impl ::windows::core::RuntimeName for IAudioCaptureClient {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioCaptureClient";
}
impl IAudioCaptureClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioCaptureClientImpl, const OFFSET: isize>() -> IAudioCaptureClientVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IAudioCaptureClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pnumframestoread), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pu64deviceposition), ::core::mem::transmute_copy(&pu64qpcposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Impl: IAudioCaptureClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframesread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseBuffer(numframesread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextPacketSize<Impl: IAudioCaptureClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextPacketSize(::core::mem::transmute_copy(&pnumframesinnextpacket)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioCaptureClient>, ::windows::core::GetTrustLevel, GetBuffer::<Impl, OFFSET>, ReleaseBuffer::<Impl, OFFSET>, GetNextPacketSize::<Impl, OFFSET>)
    }
}
pub trait IAudioChannelConfigImpl: Sized {
    fn SetChannelConfig();
    fn GetChannelConfig();
}
impl ::windows::core::RuntimeName for IAudioChannelConfig {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioChannelConfig";
}
impl IAudioChannelConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioChannelConfigImpl, const OFFSET: isize>() -> IAudioChannelConfigVtbl {
        unsafe extern "system" fn SetChannelConfig<Impl: IAudioChannelConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconfig: u32, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannelConfig(dwconfig, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelConfig<Impl: IAudioChannelConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconfig: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelConfig(::core::mem::transmute_copy(&pdwconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioChannelConfig>, ::windows::core::GetTrustLevel, SetChannelConfig::<Impl, OFFSET>, GetChannelConfig::<Impl, OFFSET>)
    }
}
pub trait IAudioClientImpl: Sized {
    fn Initialize();
    fn GetBufferSize();
    fn GetStreamLatency();
    fn GetCurrentPadding();
    fn IsFormatSupported();
    fn GetMixFormat();
    fn GetDevicePeriod();
    fn Start();
    fn Stop();
    fn Reset();
    fn SetEventHandle();
    fn GetService();
}
impl ::windows::core::RuntimeName for IAudioClient {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioClient";
}
impl IAudioClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClientImpl, const OFFSET: isize>() -> IAudioClientVtbl {
        unsafe extern "system" fn Initialize<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(sharemode, streamflags, hnsbufferduration, hnsperiodicity, &*(&pformat as *const <WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&audiosessionguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferSize<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumbufferframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferSize(::core::mem::transmute_copy(&pnumbufferframes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamLatency<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnslatency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamLatency(::core::mem::transmute_copy(&phnslatency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPadding<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumpaddingframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPadding(::core::mem::transmute_copy(&pnumpaddingframes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFormatSupported<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFormatSupported(sharemode, &*(&pformat as *const <WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclosestmatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMixFormat<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMixFormat(::core::mem::transmute_copy(&ppdeviceformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePeriod<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevicePeriod(::core::mem::transmute_copy(&phnsdefaultdeviceperiod), ::core::mem::transmute_copy(&phnsminimumdeviceperiod)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventHandle<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventHandle(&*(&eventhandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Impl: IAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetService(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
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
            ::windows::core::GetRuntimeClassName::<IAudioClient>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            GetBufferSize::<Impl, OFFSET>,
            GetStreamLatency::<Impl, OFFSET>,
            GetCurrentPadding::<Impl, OFFSET>,
            IsFormatSupported::<Impl, OFFSET>,
            GetMixFormat::<Impl, OFFSET>,
            GetDevicePeriod::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            SetEventHandle::<Impl, OFFSET>,
            GetService::<Impl, OFFSET>,
        )
    }
}
pub trait IAudioClient2Impl: Sized + IAudioClientImpl {
    fn IsOffloadCapable();
    fn SetClientProperties();
    fn GetBufferSizeLimits();
}
impl ::windows::core::RuntimeName for IAudioClient2 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioClient2";
}
impl IAudioClient2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClient2Impl, const OFFSET: isize>() -> IAudioClient2Vtbl {
        unsafe extern "system" fn IsOffloadCapable<Impl: IAudioClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOffloadCapable(category, ::core::mem::transmute_copy(&pboffloadcapable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientProperties<Impl: IAudioClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: *const AudioClientProperties) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClientProperties(&*(&pproperties as *const <AudioClientProperties as ::windows::core::Abi>::Abi as *const <AudioClientProperties as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBufferSizeLimits<Impl: IAudioClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferSizeLimits(&*(&pformat as *const <WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&beventdriven as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phnsminbufferduration), ::core::mem::transmute_copy(&phnsmaxbufferduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioClient2>, ::windows::core::GetTrustLevel, IsOffloadCapable::<Impl, OFFSET>, SetClientProperties::<Impl, OFFSET>, GetBufferSizeLimits::<Impl, OFFSET>)
    }
}
pub trait IAudioClient3Impl: Sized + IAudioClient2Impl + IAudioClientImpl {
    fn GetSharedModeEnginePeriod();
    fn GetCurrentSharedModeEnginePeriod();
    fn InitializeSharedAudioStream();
}
impl ::windows::core::RuntimeName for IAudioClient3 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioClient3";
}
impl IAudioClient3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClient3Impl, const OFFSET: isize>() -> IAudioClient3Vtbl {
        unsafe extern "system" fn GetSharedModeEnginePeriod<Impl: IAudioClient3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSharedModeEnginePeriod(&*(&pformat as *const <WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdefaultperiodinframes), ::core::mem::transmute_copy(&pfundamentalperiodinframes), ::core::mem::transmute_copy(&pminperiodinframes), ::core::mem::transmute_copy(&pmaxperiodinframes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentSharedModeEnginePeriod<Impl: IAudioClient3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentSharedModeEnginePeriod(::core::mem::transmute_copy(&ppformat), ::core::mem::transmute_copy(&pcurrentperiodinframes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeSharedAudioStream<Impl: IAudioClient3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeSharedAudioStream(streamflags, periodinframes, &*(&pformat as *const <WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), &*(&audiosessionguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioClient3>, ::windows::core::GetTrustLevel, GetSharedModeEnginePeriod::<Impl, OFFSET>, GetCurrentSharedModeEnginePeriod::<Impl, OFFSET>, InitializeSharedAudioStream::<Impl, OFFSET>)
    }
}
pub trait IAudioClientDuckingControlImpl: Sized {
    fn SetDuckingOptionsForCurrentStream();
}
impl ::windows::core::RuntimeName for IAudioClientDuckingControl {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioClientDuckingControl";
}
impl IAudioClientDuckingControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClientDuckingControlImpl, const OFFSET: isize>() -> IAudioClientDuckingControlVtbl {
        unsafe extern "system" fn SetDuckingOptionsForCurrentStream<Impl: IAudioClientDuckingControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDuckingOptionsForCurrentStream(options) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioClientDuckingControl>, ::windows::core::GetTrustLevel, SetDuckingOptionsForCurrentStream::<Impl, OFFSET>)
    }
}
pub trait IAudioClockImpl: Sized {
    fn GetFrequency();
    fn GetPosition();
    fn GetCharacteristics();
}
impl ::windows::core::RuntimeName for IAudioClock {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioClock";
}
impl IAudioClockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClockImpl, const OFFSET: isize>() -> IAudioClockVtbl {
        unsafe extern "system" fn GetFrequency<Impl: IAudioClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu64frequency: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrequency(::core::mem::transmute_copy(&pu64frequency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Impl: IAudioClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition(::core::mem::transmute_copy(&pu64position), ::core::mem::transmute_copy(&pu64qpcposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacteristics<Impl: IAudioClockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristics(::core::mem::transmute_copy(&pdwcharacteristics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioClock>, ::windows::core::GetTrustLevel, GetFrequency::<Impl, OFFSET>, GetPosition::<Impl, OFFSET>, GetCharacteristics::<Impl, OFFSET>)
    }
}
pub trait IAudioClock2Impl: Sized {
    fn GetDevicePosition();
}
impl ::windows::core::RuntimeName for IAudioClock2 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioClock2";
}
impl IAudioClock2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClock2Impl, const OFFSET: isize>() -> IAudioClock2Vtbl {
        unsafe extern "system" fn GetDevicePosition<Impl: IAudioClock2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevicePosition(::core::mem::transmute_copy(&deviceposition), ::core::mem::transmute_copy(&qpcposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioClock2>, ::windows::core::GetTrustLevel, GetDevicePosition::<Impl, OFFSET>)
    }
}
pub trait IAudioClockAdjustmentImpl: Sized {
    fn SetSampleRate();
}
impl ::windows::core::RuntimeName for IAudioClockAdjustment {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioClockAdjustment";
}
impl IAudioClockAdjustmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClockAdjustmentImpl, const OFFSET: isize>() -> IAudioClockAdjustmentVtbl {
        unsafe extern "system" fn SetSampleRate<Impl: IAudioClockAdjustmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flsamplerate: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSampleRate(flsamplerate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioClockAdjustment>, ::windows::core::GetTrustLevel, SetSampleRate::<Impl, OFFSET>)
    }
}
pub trait IAudioEffectsChangedNotificationClientImpl: Sized {
    fn OnAudioEffectsChanged();
}
impl ::windows::core::RuntimeName for IAudioEffectsChangedNotificationClient {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioEffectsChangedNotificationClient";
}
impl IAudioEffectsChangedNotificationClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectsChangedNotificationClientImpl, const OFFSET: isize>() -> IAudioEffectsChangedNotificationClientVtbl {
        unsafe extern "system" fn OnAudioEffectsChanged<Impl: IAudioEffectsChangedNotificationClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAudioEffectsChanged() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEffectsChangedNotificationClient>, ::windows::core::GetTrustLevel, OnAudioEffectsChanged::<Impl, OFFSET>)
    }
}
pub trait IAudioEffectsManagerImpl: Sized {
    fn RegisterAudioEffectsChangedNotificationCallback();
    fn UnregisterAudioEffectsChangedNotificationCallback();
    fn GetAudioEffects();
    fn SetAudioEffectState();
}
impl ::windows::core::RuntimeName for IAudioEffectsManager {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioEffectsManager";
}
impl IAudioEffectsManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectsManagerImpl, const OFFSET: isize>() -> IAudioEffectsManagerVtbl {
        unsafe extern "system" fn RegisterAudioEffectsChangedNotificationCallback<Impl: IAudioEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, client: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterAudioEffectsChangedNotificationCallback(&*(&client as *const <IAudioEffectsChangedNotificationClient as ::windows::core::Abi>::Abi as *const <IAudioEffectsChangedNotificationClient as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAudioEffectsChangedNotificationCallback<Impl: IAudioEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, client: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterAudioEffectsChangedNotificationCallback(&*(&client as *const <IAudioEffectsChangedNotificationClient as ::windows::core::Abi>::Abi as *const <IAudioEffectsChangedNotificationClient as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioEffects<Impl: IAudioEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioEffects(::core::mem::transmute_copy(&effects), numeffects) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioEffectState<Impl: IAudioEffectsManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAudioEffectState(&*(&effectid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEffectsManager>, ::windows::core::GetTrustLevel, RegisterAudioEffectsChangedNotificationCallback::<Impl, OFFSET>, UnregisterAudioEffectsChangedNotificationCallback::<Impl, OFFSET>, GetAudioEffects::<Impl, OFFSET>, SetAudioEffectState::<Impl, OFFSET>)
    }
}
pub trait IAudioFormatEnumeratorImpl: Sized {
    fn GetCount();
    fn GetFormat();
}
impl ::windows::core::RuntimeName for IAudioFormatEnumerator {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioFormatEnumerator";
}
impl IAudioFormatEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFormatEnumeratorImpl, const OFFSET: isize>() -> IAudioFormatEnumeratorVtbl {
        unsafe extern "system" fn GetCount<Impl: IAudioFormatEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IAudioFormatEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, format: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(index, ::core::mem::transmute_copy(&format)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioFormatEnumerator>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetFormat::<Impl, OFFSET>)
    }
}
pub trait IAudioInputSelectorImpl: Sized {
    fn GetSelection();
    fn SetSelection();
}
impl ::windows::core::RuntimeName for IAudioInputSelector {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioInputSelector";
}
impl IAudioInputSelectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioInputSelectorImpl, const OFFSET: isize>() -> IAudioInputSelectorVtbl {
        unsafe extern "system" fn GetSelection<Impl: IAudioInputSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(::core::mem::transmute_copy(&pnidselected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: IAudioInputSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelection(nidselect, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioInputSelector>, ::windows::core::GetTrustLevel, GetSelection::<Impl, OFFSET>, SetSelection::<Impl, OFFSET>)
    }
}
pub trait IAudioLoudnessImpl: Sized {
    fn GetEnabled();
    fn SetEnabled();
}
impl ::windows::core::RuntimeName for IAudioLoudness {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioLoudness";
}
impl IAudioLoudnessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioLoudnessImpl, const OFFSET: isize>() -> IAudioLoudnessVtbl {
        unsafe extern "system" fn GetEnabled<Impl: IAudioLoudnessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnabled(::core::mem::transmute_copy(&pbenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IAudioLoudnessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEnabled(&*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioLoudness>, ::windows::core::GetTrustLevel, GetEnabled::<Impl, OFFSET>, SetEnabled::<Impl, OFFSET>)
    }
}
pub trait IAudioMidrangeImpl: Sized + IPerChannelDbLevelImpl {}
impl ::windows::core::RuntimeName for IAudioMidrange {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioMidrange";
}
impl IAudioMidrangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMidrangeImpl, const OFFSET: isize>() -> IAudioMidrangeVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioMidrange>, ::windows::core::GetTrustLevel)
    }
}
pub trait IAudioMuteImpl: Sized {
    fn SetMute();
    fn GetMute();
}
impl ::windows::core::RuntimeName for IAudioMute {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioMute";
}
impl IAudioMuteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMuteImpl, const OFFSET: isize>() -> IAudioMuteVtbl {
        unsafe extern "system" fn SetMute<Impl: IAudioMuteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmuted: super::super::Foundation::BOOL, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMute(&*(&bmuted as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMute<Impl: IAudioMuteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmuted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMute(::core::mem::transmute_copy(&pbmuted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioMute>, ::windows::core::GetTrustLevel, SetMute::<Impl, OFFSET>, GetMute::<Impl, OFFSET>)
    }
}
pub trait IAudioOutputSelectorImpl: Sized {
    fn GetSelection();
    fn SetSelection();
}
impl ::windows::core::RuntimeName for IAudioOutputSelector {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioOutputSelector";
}
impl IAudioOutputSelectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioOutputSelectorImpl, const OFFSET: isize>() -> IAudioOutputSelectorVtbl {
        unsafe extern "system" fn GetSelection<Impl: IAudioOutputSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection(::core::mem::transmute_copy(&pnidselected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: IAudioOutputSelectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSelection(nidselect, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioOutputSelector>, ::windows::core::GetTrustLevel, GetSelection::<Impl, OFFSET>, SetSelection::<Impl, OFFSET>)
    }
}
pub trait IAudioPeakMeterImpl: Sized {
    fn GetChannelCount();
    fn GetLevel();
}
impl ::windows::core::RuntimeName for IAudioPeakMeter {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioPeakMeter";
}
impl IAudioPeakMeterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioPeakMeterImpl, const OFFSET: isize>() -> IAudioPeakMeterVtbl {
        unsafe extern "system" fn GetChannelCount<Impl: IAudioPeakMeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount(::core::mem::transmute_copy(&pcchannels)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevel<Impl: IAudioPeakMeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevel(nchannel, ::core::mem::transmute_copy(&pflevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioPeakMeter>, ::windows::core::GetTrustLevel, GetChannelCount::<Impl, OFFSET>, GetLevel::<Impl, OFFSET>)
    }
}
pub trait IAudioRenderClientImpl: Sized {
    fn GetBuffer();
    fn ReleaseBuffer();
}
impl ::windows::core::RuntimeName for IAudioRenderClient {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioRenderClient";
}
impl IAudioRenderClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioRenderClientImpl, const OFFSET: isize>() -> IAudioRenderClientVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IAudioRenderClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(numframesrequested, ::core::mem::transmute_copy(&ppdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Impl: IAudioRenderClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframeswritten: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseBuffer(numframeswritten, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioRenderClient>, ::windows::core::GetTrustLevel, GetBuffer::<Impl, OFFSET>, ReleaseBuffer::<Impl, OFFSET>)
    }
}
pub trait IAudioSessionControlImpl: Sized {
    fn GetState();
    fn GetDisplayName();
    fn SetDisplayName();
    fn GetIconPath();
    fn SetIconPath();
    fn GetGroupingParam();
    fn SetGroupingParam();
    fn RegisterAudioSessionNotification();
    fn UnregisterAudioSessionNotification();
}
impl ::windows::core::RuntimeName for IAudioSessionControl {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioSessionControl";
}
impl IAudioSessionControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionControlImpl, const OFFSET: isize>() -> IAudioSessionControlVtbl {
        unsafe extern "system" fn GetState<Impl: IAudioSessionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut AudioSessionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Impl: IAudioSessionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IAudioSessionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::PWSTR, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisplayName(&*(&value as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIconPath<Impl: IAudioSessionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIconPath(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIconPath<Impl: IAudioSessionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::PWSTR, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIconPath(&*(&value as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroupingParam<Impl: IAudioSessionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroupingParam(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupingParam<Impl: IAudioSessionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#override: &::windows::core::GUID, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGroupingParam(&*(&r#override as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAudioSessionNotification<Impl: IAudioSessionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterAudioSessionNotification(&*(&newnotifications as *const <IAudioSessionEvents as ::windows::core::Abi>::Abi as *const <IAudioSessionEvents as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAudioSessionNotification<Impl: IAudioSessionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterAudioSessionNotification(&*(&newnotifications as *const <IAudioSessionEvents as ::windows::core::Abi>::Abi as *const <IAudioSessionEvents as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IAudioSessionControl>,
            ::windows::core::GetTrustLevel,
            GetState::<Impl, OFFSET>,
            GetDisplayName::<Impl, OFFSET>,
            SetDisplayName::<Impl, OFFSET>,
            GetIconPath::<Impl, OFFSET>,
            SetIconPath::<Impl, OFFSET>,
            GetGroupingParam::<Impl, OFFSET>,
            SetGroupingParam::<Impl, OFFSET>,
            RegisterAudioSessionNotification::<Impl, OFFSET>,
            UnregisterAudioSessionNotification::<Impl, OFFSET>,
        )
    }
}
pub trait IAudioSessionControl2Impl: Sized + IAudioSessionControlImpl {
    fn GetSessionIdentifier();
    fn GetSessionInstanceIdentifier();
    fn GetProcessId();
    fn IsSystemSoundsSession();
    fn SetDuckingPreference();
}
impl ::windows::core::RuntimeName for IAudioSessionControl2 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioSessionControl2";
}
impl IAudioSessionControl2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionControl2Impl, const OFFSET: isize>() -> IAudioSessionControl2Vtbl {
        unsafe extern "system" fn GetSessionIdentifier<Impl: IAudioSessionControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionIdentifier(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionInstanceIdentifier<Impl: IAudioSessionControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionInstanceIdentifier(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessId<Impl: IAudioSessionControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProcessId(::core::mem::transmute_copy(&pretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSystemSoundsSession<Impl: IAudioSessionControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSystemSoundsSession() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDuckingPreference<Impl: IAudioSessionControl2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optout: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDuckingPreference(&*(&optout as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSessionControl2>, ::windows::core::GetTrustLevel, GetSessionIdentifier::<Impl, OFFSET>, GetSessionInstanceIdentifier::<Impl, OFFSET>, GetProcessId::<Impl, OFFSET>, IsSystemSoundsSession::<Impl, OFFSET>, SetDuckingPreference::<Impl, OFFSET>)
    }
}
pub trait IAudioSessionEnumeratorImpl: Sized {
    fn GetCount();
    fn GetSession();
}
impl ::windows::core::RuntimeName for IAudioSessionEnumerator {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioSessionEnumerator";
}
impl IAudioSessionEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionEnumeratorImpl, const OFFSET: isize>() -> IAudioSessionEnumeratorVtbl {
        unsafe extern "system" fn GetCount<Impl: IAudioSessionEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessioncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&sessioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSession<Impl: IAudioSessionEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessioncount: i32, session: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSession(sessioncount, ::core::mem::transmute_copy(&session)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSessionEnumerator>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetSession::<Impl, OFFSET>)
    }
}
pub trait IAudioSessionEventsImpl: Sized {
    fn OnDisplayNameChanged();
    fn OnIconPathChanged();
    fn OnSimpleVolumeChanged();
    fn OnChannelVolumeChanged();
    fn OnGroupingParamChanged();
    fn OnStateChanged();
    fn OnSessionDisconnected();
}
impl ::windows::core::RuntimeName for IAudioSessionEvents {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioSessionEvents";
}
impl IAudioSessionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionEventsImpl, const OFFSET: isize>() -> IAudioSessionEventsVtbl {
        unsafe extern "system" fn OnDisplayNameChanged<Impl: IAudioSessionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newdisplayname: super::super::Foundation::PWSTR, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDisplayNameChanged(&*(&newdisplayname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnIconPathChanged<Impl: IAudioSessionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newiconpath: super::super::Foundation::PWSTR, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIconPathChanged(&*(&newiconpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSimpleVolumeChanged<Impl: IAudioSessionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSimpleVolumeChanged(newvolume, &*(&newmute as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnChannelVolumeChanged<Impl: IAudioSessionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnChannelVolumeChanged(channelcount, newchannelvolumearray, changedchannel, &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnGroupingParamChanged<Impl: IAudioSessionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newgroupingparam: &::windows::core::GUID, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnGroupingParamChanged(&*(&newgroupingparam as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnStateChanged<Impl: IAudioSessionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: AudioSessionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStateChanged(newstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSessionDisconnected<Impl: IAudioSessionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSessionDisconnected(disconnectreason) {
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
            ::windows::core::GetRuntimeClassName::<IAudioSessionEvents>,
            ::windows::core::GetTrustLevel,
            OnDisplayNameChanged::<Impl, OFFSET>,
            OnIconPathChanged::<Impl, OFFSET>,
            OnSimpleVolumeChanged::<Impl, OFFSET>,
            OnChannelVolumeChanged::<Impl, OFFSET>,
            OnGroupingParamChanged::<Impl, OFFSET>,
            OnStateChanged::<Impl, OFFSET>,
            OnSessionDisconnected::<Impl, OFFSET>,
        )
    }
}
pub trait IAudioSessionManagerImpl: Sized {
    fn GetAudioSessionControl();
    fn GetSimpleAudioVolume();
}
impl ::windows::core::RuntimeName for IAudioSessionManager {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioSessionManager";
}
impl IAudioSessionManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionManagerImpl, const OFFSET: isize>() -> IAudioSessionManagerVtbl {
        unsafe extern "system" fn GetAudioSessionControl<Impl: IAudioSessionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosessionguid: &::windows::core::GUID, streamflags: u32, sessioncontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioSessionControl(&*(&audiosessionguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), streamflags, ::core::mem::transmute_copy(&sessioncontrol)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSimpleAudioVolume<Impl: IAudioSessionManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosessionguid: &::windows::core::GUID, streamflags: u32, audiovolume: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSimpleAudioVolume(&*(&audiosessionguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), streamflags, ::core::mem::transmute_copy(&audiovolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSessionManager>, ::windows::core::GetTrustLevel, GetAudioSessionControl::<Impl, OFFSET>, GetSimpleAudioVolume::<Impl, OFFSET>)
    }
}
pub trait IAudioSessionManager2Impl: Sized + IAudioSessionManagerImpl {
    fn GetSessionEnumerator();
    fn RegisterSessionNotification();
    fn UnregisterSessionNotification();
    fn RegisterDuckNotification();
    fn UnregisterDuckNotification();
}
impl ::windows::core::RuntimeName for IAudioSessionManager2 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioSessionManager2";
}
impl IAudioSessionManager2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionManager2Impl, const OFFSET: isize>() -> IAudioSessionManager2Vtbl {
        unsafe extern "system" fn GetSessionEnumerator<Impl: IAudioSessionManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionEnumerator(::core::mem::transmute_copy(&sessionenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterSessionNotification<Impl: IAudioSessionManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterSessionNotification(&*(&sessionnotification as *const <IAudioSessionNotification as ::windows::core::Abi>::Abi as *const <IAudioSessionNotification as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterSessionNotification<Impl: IAudioSessionManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterSessionNotification(&*(&sessionnotification as *const <IAudioSessionNotification as ::windows::core::Abi>::Abi as *const <IAudioSessionNotification as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterDuckNotification<Impl: IAudioSessionManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: super::super::Foundation::PWSTR, ducknotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDuckNotification(&*(&sessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&ducknotification as *const <IAudioVolumeDuckNotification as ::windows::core::Abi>::Abi as *const <IAudioVolumeDuckNotification as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterDuckNotification<Impl: IAudioSessionManager2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ducknotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterDuckNotification(&*(&ducknotification as *const <IAudioVolumeDuckNotification as ::windows::core::Abi>::Abi as *const <IAudioVolumeDuckNotification as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSessionManager2>, ::windows::core::GetTrustLevel, GetSessionEnumerator::<Impl, OFFSET>, RegisterSessionNotification::<Impl, OFFSET>, UnregisterSessionNotification::<Impl, OFFSET>, RegisterDuckNotification::<Impl, OFFSET>, UnregisterDuckNotification::<Impl, OFFSET>)
    }
}
pub trait IAudioSessionNotificationImpl: Sized {
    fn OnSessionCreated();
}
impl ::windows::core::RuntimeName for IAudioSessionNotification {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioSessionNotification";
}
impl IAudioSessionNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionNotificationImpl, const OFFSET: isize>() -> IAudioSessionNotificationVtbl {
        unsafe extern "system" fn OnSessionCreated<Impl: IAudioSessionNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSessionCreated(&*(&newsession as *const <IAudioSessionControl as ::windows::core::Abi>::Abi as *const <IAudioSessionControl as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSessionNotification>, ::windows::core::GetTrustLevel, OnSessionCreated::<Impl, OFFSET>)
    }
}
pub trait IAudioStateMonitorImpl: Sized {
    fn RegisterCallback();
    fn UnregisterCallback();
    fn GetSoundLevel();
}
impl ::windows::core::RuntimeName for IAudioStateMonitor {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioStateMonitor";
}
impl IAudioStateMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioStateMonitorImpl, const OFFSET: isize>() -> IAudioStateMonitorVtbl {
        unsafe extern "system" fn RegisterCallback<Impl: IAudioStateMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, registration: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterCallback(&*(&callback as *const <PAudioStateMonitorCallback as ::windows::core::Abi>::Abi as *const <PAudioStateMonitorCallback as ::windows::core::DefaultType>::DefaultType), &*(&context as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&registration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterCallback<Impl: IAudioStateMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registration: i64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterCallback(registration).into()
        }
        unsafe extern "system" fn GetSoundLevel<Impl: IAudioStateMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> AudioStateMonitorSoundLevel {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSoundLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioStateMonitor>, ::windows::core::GetTrustLevel, RegisterCallback::<Impl, OFFSET>, UnregisterCallback::<Impl, OFFSET>, GetSoundLevel::<Impl, OFFSET>)
    }
}
pub trait IAudioStreamVolumeImpl: Sized {
    fn GetChannelCount();
    fn SetChannelVolume();
    fn GetChannelVolume();
    fn SetAllVolumes();
    fn GetAllVolumes();
}
impl ::windows::core::RuntimeName for IAudioStreamVolume {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioStreamVolume";
}
impl IAudioStreamVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioStreamVolumeImpl, const OFFSET: isize>() -> IAudioStreamVolumeVtbl {
        unsafe extern "system" fn GetChannelCount<Impl: IAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount(::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolume<Impl: IAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannelVolume(dwindex, flevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelVolume<Impl: IAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolume(dwindex, ::core::mem::transmute_copy(&pflevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllVolumes<Impl: IAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllVolumes(dwcount, pfvolumes) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllVolumes<Impl: IAudioStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllVolumes(dwcount, ::core::mem::transmute_copy(&pfvolumes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioStreamVolume>, ::windows::core::GetTrustLevel, GetChannelCount::<Impl, OFFSET>, SetChannelVolume::<Impl, OFFSET>, GetChannelVolume::<Impl, OFFSET>, SetAllVolumes::<Impl, OFFSET>, GetAllVolumes::<Impl, OFFSET>)
    }
}
pub trait IAudioSystemEffectsPropertyChangeNotificationClientImpl: Sized {
    fn OnPropertyChanged();
}
impl ::windows::core::RuntimeName for IAudioSystemEffectsPropertyChangeNotificationClient {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioSystemEffectsPropertyChangeNotificationClient";
}
impl IAudioSystemEffectsPropertyChangeNotificationClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsPropertyChangeNotificationClientImpl, const OFFSET: isize>() -> IAudioSystemEffectsPropertyChangeNotificationClientVtbl {
        unsafe extern "system" fn OnPropertyChanged<Impl: IAudioSystemEffectsPropertyChangeNotificationClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPropertyChanged(r#type, &*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioSystemEffectsPropertyChangeNotificationClient>, ::windows::core::GetTrustLevel, OnPropertyChanged::<Impl, OFFSET>)
    }
}
pub trait IAudioSystemEffectsPropertyStoreImpl: Sized {
    fn OpenDefaultPropertyStore();
    fn OpenUserPropertyStore();
    fn OpenVolatilePropertyStore();
    fn ResetUserPropertyStore();
    fn ResetVolatilePropertyStore();
    fn RegisterPropertyChangeNotification();
    fn UnregisterPropertyChangeNotification();
}
impl ::windows::core::RuntimeName for IAudioSystemEffectsPropertyStore {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioSystemEffectsPropertyStore";
}
impl IAudioSystemEffectsPropertyStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsPropertyStoreImpl, const OFFSET: isize>() -> IAudioSystemEffectsPropertyStoreVtbl {
        unsafe extern "system" fn OpenDefaultPropertyStore<Impl: IAudioSystemEffectsPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenDefaultPropertyStore(stgmaccess, ::core::mem::transmute_copy(&propstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenUserPropertyStore<Impl: IAudioSystemEffectsPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenUserPropertyStore(stgmaccess, ::core::mem::transmute_copy(&propstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenVolatilePropertyStore<Impl: IAudioSystemEffectsPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenVolatilePropertyStore(stgmaccess, ::core::mem::transmute_copy(&propstore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetUserPropertyStore<Impl: IAudioSystemEffectsPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetUserPropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetVolatilePropertyStore<Impl: IAudioSystemEffectsPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetVolatilePropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPropertyChangeNotification<Impl: IAudioSystemEffectsPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPropertyChangeNotification(&*(&callback as *const <IAudioSystemEffectsPropertyChangeNotificationClient as ::windows::core::Abi>::Abi as *const <IAudioSystemEffectsPropertyChangeNotificationClient as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterPropertyChangeNotification<Impl: IAudioSystemEffectsPropertyStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterPropertyChangeNotification(&*(&callback as *const <IAudioSystemEffectsPropertyChangeNotificationClient as ::windows::core::Abi>::Abi as *const <IAudioSystemEffectsPropertyChangeNotificationClient as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IAudioSystemEffectsPropertyStore>,
            ::windows::core::GetTrustLevel,
            OpenDefaultPropertyStore::<Impl, OFFSET>,
            OpenUserPropertyStore::<Impl, OFFSET>,
            OpenVolatilePropertyStore::<Impl, OFFSET>,
            ResetUserPropertyStore::<Impl, OFFSET>,
            ResetVolatilePropertyStore::<Impl, OFFSET>,
            RegisterPropertyChangeNotification::<Impl, OFFSET>,
            UnregisterPropertyChangeNotification::<Impl, OFFSET>,
        )
    }
}
pub trait IAudioTrebleImpl: Sized + IPerChannelDbLevelImpl {}
impl ::windows::core::RuntimeName for IAudioTreble {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioTreble";
}
impl IAudioTrebleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioTrebleImpl, const OFFSET: isize>() -> IAudioTrebleVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioTreble>, ::windows::core::GetTrustLevel)
    }
}
pub trait IAudioVolumeDuckNotificationImpl: Sized {
    fn OnVolumeDuckNotification();
    fn OnVolumeUnduckNotification();
}
impl ::windows::core::RuntimeName for IAudioVolumeDuckNotification {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioVolumeDuckNotification";
}
impl IAudioVolumeDuckNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioVolumeDuckNotificationImpl, const OFFSET: isize>() -> IAudioVolumeDuckNotificationVtbl {
        unsafe extern "system" fn OnVolumeDuckNotification<Impl: IAudioVolumeDuckNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: super::super::Foundation::PWSTR, countcommunicationsessions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnVolumeDuckNotification(&*(&sessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), countcommunicationsessions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnVolumeUnduckNotification<Impl: IAudioVolumeDuckNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnVolumeUnduckNotification(&*(&sessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioVolumeDuckNotification>, ::windows::core::GetTrustLevel, OnVolumeDuckNotification::<Impl, OFFSET>, OnVolumeUnduckNotification::<Impl, OFFSET>)
    }
}
pub trait IAudioVolumeLevelImpl: Sized + IPerChannelDbLevelImpl {}
impl ::windows::core::RuntimeName for IAudioVolumeLevel {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IAudioVolumeLevel";
}
impl IAudioVolumeLevelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioVolumeLevelImpl, const OFFSET: isize>() -> IAudioVolumeLevelVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioVolumeLevel>, ::windows::core::GetTrustLevel)
    }
}
pub trait IChannelAudioVolumeImpl: Sized {
    fn GetChannelCount();
    fn SetChannelVolume();
    fn GetChannelVolume();
    fn SetAllVolumes();
    fn GetAllVolumes();
}
impl ::windows::core::RuntimeName for IChannelAudioVolume {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IChannelAudioVolume";
}
impl IChannelAudioVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChannelAudioVolumeImpl, const OFFSET: isize>() -> IChannelAudioVolumeVtbl {
        unsafe extern "system" fn GetChannelCount<Impl: IChannelAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount(::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolume<Impl: IChannelAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannelVolume(dwindex, flevel, &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelVolume<Impl: IChannelAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolume(dwindex, ::core::mem::transmute_copy(&pflevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllVolumes<Impl: IChannelAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAllVolumes(dwcount, pfvolumes, &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllVolumes<Impl: IChannelAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllVolumes(dwcount, ::core::mem::transmute_copy(&pfvolumes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IChannelAudioVolume>, ::windows::core::GetTrustLevel, GetChannelCount::<Impl, OFFSET>, SetChannelVolume::<Impl, OFFSET>, GetChannelVolume::<Impl, OFFSET>, SetAllVolumes::<Impl, OFFSET>, GetAllVolumes::<Impl, OFFSET>)
    }
}
pub trait IConnectorImpl: Sized {
    fn GetType();
    fn GetDataFlow();
    fn ConnectTo();
    fn Disconnect();
    fn IsConnected();
    fn GetConnectedTo();
    fn GetConnectorIdConnectedTo();
    fn GetDeviceIdConnectedTo();
}
impl ::windows::core::RuntimeName for IConnector {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IConnector";
}
impl IConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectorImpl, const OFFSET: isize>() -> IConnectorVtbl {
        unsafe extern "system" fn GetType<Impl: IConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut ConnectorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataFlow<Impl: IConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflow: *mut DataFlow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataFlow(::core::mem::transmute_copy(&pflow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTo<Impl: IConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectto: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConnectTo(&*(&pconnectto as *const <IConnector as ::windows::core::Abi>::Abi as *const <IConnector as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Disconnect<Impl: IConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Disconnect() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Impl: IConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected(::core::mem::transmute_copy(&pbconnected)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectedTo<Impl: IConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconto: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectedTo(::core::mem::transmute_copy(&ppconto)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectorIdConnectedTo<Impl: IConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrconnectorid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectorIdConnectedTo(::core::mem::transmute_copy(&ppwstrconnectorid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIdConnectedTo<Impl: IConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceIdConnectedTo(::core::mem::transmute_copy(&ppwstrdeviceid)) {
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
            ::windows::core::GetRuntimeClassName::<IConnector>,
            ::windows::core::GetTrustLevel,
            GetType::<Impl, OFFSET>,
            GetDataFlow::<Impl, OFFSET>,
            ConnectTo::<Impl, OFFSET>,
            Disconnect::<Impl, OFFSET>,
            IsConnected::<Impl, OFFSET>,
            GetConnectedTo::<Impl, OFFSET>,
            GetConnectorIdConnectedTo::<Impl, OFFSET>,
            GetDeviceIdConnectedTo::<Impl, OFFSET>,
        )
    }
}
pub trait IControlChangeNotifyImpl: Sized {
    fn OnNotify();
}
impl ::windows::core::RuntimeName for IControlChangeNotify {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IControlChangeNotify";
}
impl IControlChangeNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChangeNotifyImpl, const OFFSET: isize>() -> IControlChangeNotifyVtbl {
        unsafe extern "system" fn OnNotify<Impl: IControlChangeNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsenderprocessid: u32, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNotify(dwsenderprocessid, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IControlChangeNotify>, ::windows::core::GetTrustLevel, OnNotify::<Impl, OFFSET>)
    }
}
pub trait IControlInterfaceImpl: Sized {
    fn GetName();
    fn GetIID();
}
impl ::windows::core::RuntimeName for IControlInterface {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IControlInterface";
}
impl IControlInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlInterfaceImpl, const OFFSET: isize>() -> IControlInterfaceVtbl {
        unsafe extern "system" fn GetName<Impl: IControlInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&ppwstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIID<Impl: IControlInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIID(::core::mem::transmute_copy(&piid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IControlInterface>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetIID::<Impl, OFFSET>)
    }
}
pub trait IDeviceSpecificPropertyImpl: Sized {
    fn GetType();
    fn GetValue();
    fn SetValue();
    fn Get4BRange();
}
impl ::windows::core::RuntimeName for IDeviceSpecificProperty {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IDeviceSpecificProperty";
}
impl IDeviceSpecificPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceSpecificPropertyImpl, const OFFSET: isize>() -> IDeviceSpecificPropertyVtbl {
        unsafe extern "system" fn GetType<Impl: IDeviceSpecificPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&pvtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IDeviceSpecificPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&pvvalue), pcbvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IDeviceSpecificPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&pvvalue as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), cbvalue, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get4BRange<Impl: IDeviceSpecificPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get4BRange(::core::mem::transmute_copy(&plmin), ::core::mem::transmute_copy(&plmax), ::core::mem::transmute_copy(&plstepping)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceSpecificProperty>, ::windows::core::GetTrustLevel, GetType::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Get4BRange::<Impl, OFFSET>)
    }
}
pub trait IDeviceTopologyImpl: Sized {
    fn GetConnectorCount();
    fn GetConnector();
    fn GetSubunitCount();
    fn GetSubunit();
    fn GetPartById();
    fn GetDeviceId();
    fn GetSignalPath();
}
impl ::windows::core::RuntimeName for IDeviceTopology {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IDeviceTopology";
}
impl IDeviceTopologyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceTopologyImpl, const OFFSET: isize>() -> IDeviceTopologyVtbl {
        unsafe extern "system" fn GetConnectorCount<Impl: IDeviceTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectorCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnector<Impl: IDeviceTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnector(nindex, ::core::mem::transmute_copy(&ppconnector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubunitCount<Impl: IDeviceTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubunitCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubunit<Impl: IDeviceTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppsubunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubunit(nindex, ::core::mem::transmute_copy(&ppsubunit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartById<Impl: IDeviceTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nid: u32, pppart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartById(nid, ::core::mem::transmute_copy(&pppart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceId<Impl: IDeviceTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceId(::core::mem::transmute_copy(&ppwstrdeviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalPath<Impl: IDeviceTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipartfrom: ::windows::core::RawPtr, pipartto: ::windows::core::RawPtr, brejectmixedpaths: super::super::Foundation::BOOL, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignalPath(&*(&pipartfrom as *const <IPart as ::windows::core::Abi>::Abi as *const <IPart as ::windows::core::DefaultType>::DefaultType), &*(&pipartto as *const <IPart as ::windows::core::Abi>::Abi as *const <IPart as ::windows::core::DefaultType>::DefaultType), &*(&brejectmixedpaths as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppparts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDeviceTopology>, ::windows::core::GetTrustLevel, GetConnectorCount::<Impl, OFFSET>, GetConnector::<Impl, OFFSET>, GetSubunitCount::<Impl, OFFSET>, GetSubunit::<Impl, OFFSET>, GetPartById::<Impl, OFFSET>, GetDeviceId::<Impl, OFFSET>, GetSignalPath::<Impl, OFFSET>)
    }
}
pub trait IMMDeviceImpl: Sized {
    fn Activate();
    fn OpenPropertyStore();
    fn GetId();
    fn GetState();
}
impl ::windows::core::RuntimeName for IMMDevice {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IMMDevice";
}
impl IMMDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMDeviceImpl, const OFFSET: isize>() -> IMMDeviceVtbl {
        unsafe extern "system" fn Activate<Impl: IMMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(&*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwclsctx, &*(&pactivationparams as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Impl: IMMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: super::super::System::Com::StructuredStorage::STGM, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPropertyStore(stgmaccess, ::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Impl: IMMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&ppstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: IMMDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState(::core::mem::transmute_copy(&pdwstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMMDevice>, ::windows::core::GetTrustLevel, Activate::<Impl, OFFSET>, OpenPropertyStore::<Impl, OFFSET>, GetId::<Impl, OFFSET>, GetState::<Impl, OFFSET>)
    }
}
pub trait IMMDeviceActivatorImpl: Sized {
    fn Activate();
}
impl ::windows::core::RuntimeName for IMMDeviceActivator {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IMMDeviceActivator";
}
impl IMMDeviceActivatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMDeviceActivatorImpl, const OFFSET: isize>() -> IMMDeviceActivatorVtbl {
        unsafe extern "system" fn Activate<Impl: IMMDeviceActivatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, pdevice: ::windows::core::RawPtr, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(
                &*(&iid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pdevice as *const <IMMDevice as ::windows::core::Abi>::Abi as *const <IMMDevice as ::windows::core::DefaultType>::DefaultType),
                &*(&pactivationparams as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppinterface),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMMDeviceActivator>, ::windows::core::GetTrustLevel, Activate::<Impl, OFFSET>)
    }
}
pub trait IMMDeviceCollectionImpl: Sized {
    fn GetCount();
    fn Item();
}
impl ::windows::core::RuntimeName for IMMDeviceCollection {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IMMDeviceCollection";
}
impl IMMDeviceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMDeviceCollectionImpl, const OFFSET: isize>() -> IMMDeviceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IMMDeviceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdevices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcdevices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IMMDeviceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ndevice: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(ndevice, ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMMDeviceCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
pub trait IMMDeviceEnumeratorImpl: Sized {
    fn EnumAudioEndpoints();
    fn GetDefaultAudioEndpoint();
    fn GetDevice();
    fn RegisterEndpointNotificationCallback();
    fn UnregisterEndpointNotificationCallback();
}
impl ::windows::core::RuntimeName for IMMDeviceEnumerator {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IMMDeviceEnumerator";
}
impl IMMDeviceEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMDeviceEnumeratorImpl, const OFFSET: isize>() -> IMMDeviceEnumeratorVtbl {
        unsafe extern "system" fn EnumAudioEndpoints<Impl: IMMDeviceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, dwstatemask: u32, ppdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAudioEndpoints(dataflow, dwstatemask, ::core::mem::transmute_copy(&ppdevices)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAudioEndpoint<Impl: IMMDeviceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, role: ERole, ppendpoint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAudioEndpoint(dataflow, role, ::core::mem::transmute_copy(&ppendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: IMMDeviceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrid: super::super::Foundation::PWSTR, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(&*(&pwstrid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEndpointNotificationCallback<Impl: IMMDeviceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclient: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterEndpointNotificationCallback(&*(&pclient as *const <IMMNotificationClient as ::windows::core::Abi>::Abi as *const <IMMNotificationClient as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterEndpointNotificationCallback<Impl: IMMDeviceEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclient: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterEndpointNotificationCallback(&*(&pclient as *const <IMMNotificationClient as ::windows::core::Abi>::Abi as *const <IMMNotificationClient as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMMDeviceEnumerator>, ::windows::core::GetTrustLevel, EnumAudioEndpoints::<Impl, OFFSET>, GetDefaultAudioEndpoint::<Impl, OFFSET>, GetDevice::<Impl, OFFSET>, RegisterEndpointNotificationCallback::<Impl, OFFSET>, UnregisterEndpointNotificationCallback::<Impl, OFFSET>)
    }
}
pub trait IMMEndpointImpl: Sized {
    fn GetDataFlow();
}
impl ::windows::core::RuntimeName for IMMEndpoint {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IMMEndpoint";
}
impl IMMEndpointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMEndpointImpl, const OFFSET: isize>() -> IMMEndpointVtbl {
        unsafe extern "system" fn GetDataFlow<Impl: IMMEndpointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataflow: *mut EDataFlow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataFlow(::core::mem::transmute_copy(&pdataflow)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMMEndpoint>, ::windows::core::GetTrustLevel, GetDataFlow::<Impl, OFFSET>)
    }
}
pub trait IMMNotificationClientImpl: Sized {
    fn OnDeviceStateChanged();
    fn OnDeviceAdded();
    fn OnDeviceRemoved();
    fn OnDefaultDeviceChanged();
    fn OnPropertyValueChanged();
}
impl ::windows::core::RuntimeName for IMMNotificationClient {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IMMNotificationClient";
}
impl IMMNotificationClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMNotificationClientImpl, const OFFSET: isize>() -> IMMNotificationClientVtbl {
        unsafe extern "system" fn OnDeviceStateChanged<Impl: IMMNotificationClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: super::super::Foundation::PWSTR, dwnewstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDeviceStateChanged(&*(&pwstrdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwnewstate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDeviceAdded<Impl: IMMNotificationClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDeviceAdded(&*(&pwstrdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDeviceRemoved<Impl: IMMNotificationClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDeviceRemoved(&*(&pwstrdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDefaultDeviceChanged<Impl: IMMNotificationClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDefaultDeviceChanged(flow, role, &*(&pwstrdefaultdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnPropertyValueChanged<Impl: IMMNotificationClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: super::super::Foundation::PWSTR, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnPropertyValueChanged(&*(&pwstrdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMMNotificationClient>, ::windows::core::GetTrustLevel, OnDeviceStateChanged::<Impl, OFFSET>, OnDeviceAdded::<Impl, OFFSET>, OnDeviceRemoved::<Impl, OFFSET>, OnDefaultDeviceChanged::<Impl, OFFSET>, OnPropertyValueChanged::<Impl, OFFSET>)
    }
}
pub trait IMessageFilterImpl: Sized {
    fn HandleInComingCall();
    fn RetryRejectedCall();
    fn MessagePending();
}
impl ::windows::core::RuntimeName for IMessageFilter {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IMessageFilter";
}
impl IMessageFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageFilterImpl, const OFFSET: isize>() -> IMessageFilterVtbl {
        unsafe extern "system" fn HandleInComingCall<Impl: IMessageFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HandleInComingCall(dwcalltype, &*(&htaskcaller as *const <super::HTASK as ::windows::core::Abi>::Abi as *const <super::HTASK as ::windows::core::DefaultType>::DefaultType), dwtickcount, &*(&lpinterfaceinfo as *const <super::super::System::Com::INTERFACEINFO as ::windows::core::Abi>::Abi as *const <super::super::System::Com::INTERFACEINFO as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetryRejectedCall<Impl: IMessageFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetryRejectedCall(&*(&htaskcallee as *const <super::HTASK as ::windows::core::Abi>::Abi as *const <super::HTASK as ::windows::core::DefaultType>::DefaultType), dwtickcount, dwrejecttype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MessagePending<Impl: IMessageFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MessagePending(&*(&htaskcallee as *const <super::HTASK as ::windows::core::Abi>::Abi as *const <super::HTASK as ::windows::core::DefaultType>::DefaultType), dwtickcount, dwpendingtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMessageFilter>, ::windows::core::GetTrustLevel, HandleInComingCall::<Impl, OFFSET>, RetryRejectedCall::<Impl, OFFSET>, MessagePending::<Impl, OFFSET>)
    }
}
pub trait IPartImpl: Sized {
    fn GetName();
    fn GetLocalId();
    fn GetGlobalId();
    fn GetPartType();
    fn GetSubType();
    fn GetControlInterfaceCount();
    fn GetControlInterface();
    fn EnumPartsIncoming();
    fn EnumPartsOutgoing();
    fn GetTopologyObject();
    fn Activate();
    fn RegisterControlChangeCallback();
    fn UnregisterControlChangeCallback();
}
impl ::windows::core::RuntimeName for IPart {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IPart";
}
impl IPartVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPartImpl, const OFFSET: isize>() -> IPartVtbl {
        unsafe extern "system" fn GetName<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&ppwstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalId<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalId(::core::mem::transmute_copy(&pnid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalId<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrglobalid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalId(::core::mem::transmute_copy(&ppwstrglobalid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartType<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparttype: *mut PartType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartType(::core::mem::transmute_copy(&pparttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubType<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubType(::core::mem::transmute_copy(&psubtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetControlInterfaceCount<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControlInterfaceCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetControlInterface<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppinterfacedesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControlInterface(nindex, ::core::mem::transmute_copy(&ppinterfacedesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPartsIncoming<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPartsIncoming(::core::mem::transmute_copy(&ppparts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPartsOutgoing<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPartsOutgoing(::core::mem::transmute_copy(&ppparts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTopologyObject<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptopology: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTopologyObject(::core::mem::transmute_copy(&pptopology)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, refiid: &::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate(dwclscontext, &*(&refiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterControlChangeCallback<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterControlChangeCallback(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pnotify as *const <IControlChangeNotify as ::windows::core::Abi>::Abi as *const <IControlChangeNotify as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterControlChangeCallback<Impl: IPartImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterControlChangeCallback(&*(&pnotify as *const <IControlChangeNotify as ::windows::core::Abi>::Abi as *const <IControlChangeNotify as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IPart>,
            ::windows::core::GetTrustLevel,
            GetName::<Impl, OFFSET>,
            GetLocalId::<Impl, OFFSET>,
            GetGlobalId::<Impl, OFFSET>,
            GetPartType::<Impl, OFFSET>,
            GetSubType::<Impl, OFFSET>,
            GetControlInterfaceCount::<Impl, OFFSET>,
            GetControlInterface::<Impl, OFFSET>,
            EnumPartsIncoming::<Impl, OFFSET>,
            EnumPartsOutgoing::<Impl, OFFSET>,
            GetTopologyObject::<Impl, OFFSET>,
            Activate::<Impl, OFFSET>,
            RegisterControlChangeCallback::<Impl, OFFSET>,
            UnregisterControlChangeCallback::<Impl, OFFSET>,
        )
    }
}
pub trait IPartsListImpl: Sized {
    fn GetCount();
    fn GetPart();
}
impl ::windows::core::RuntimeName for IPartsList {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IPartsList";
}
impl IPartsListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPartsListImpl, const OFFSET: isize>() -> IPartsListVtbl {
        unsafe extern "system" fn GetCount<Impl: IPartsListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPart<Impl: IPartsListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pppart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPart(nindex, ::core::mem::transmute_copy(&pppart)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPartsList>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetPart::<Impl, OFFSET>)
    }
}
pub trait IPerChannelDbLevelImpl: Sized {
    fn GetChannelCount();
    fn GetLevelRange();
    fn GetLevel();
    fn SetLevel();
    fn SetLevelUniform();
    fn SetLevelAllChannels();
}
impl ::windows::core::RuntimeName for IPerChannelDbLevel {
    const NAME: &'static str = "Windows.Win32.Media.Audio.IPerChannelDbLevel";
}
impl IPerChannelDbLevelVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerChannelDbLevelImpl, const OFFSET: isize>() -> IPerChannelDbLevelVtbl {
        unsafe extern "system" fn GetChannelCount<Impl: IPerChannelDbLevelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount(::core::mem::transmute_copy(&pcchannels)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevelRange<Impl: IPerChannelDbLevelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevelRange(nchannel, ::core::mem::transmute_copy(&pfminleveldb), ::core::mem::transmute_copy(&pfmaxleveldb), ::core::mem::transmute_copy(&pfstepping)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevel<Impl: IPerChannelDbLevelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevel(nchannel, ::core::mem::transmute_copy(&pfleveldb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevel<Impl: IPerChannelDbLevelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLevel(nchannel, fleveldb, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevelUniform<Impl: IPerChannelDbLevelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLevelUniform(fleveldb, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevelAllChannels<Impl: IPerChannelDbLevelImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alevelsdb: *const f32, cchannels: u32, pguideventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLevelAllChannels(alevelsdb, cchannels, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPerChannelDbLevel>, ::windows::core::GetTrustLevel, GetChannelCount::<Impl, OFFSET>, GetLevelRange::<Impl, OFFSET>, GetLevel::<Impl, OFFSET>, SetLevel::<Impl, OFFSET>, SetLevelUniform::<Impl, OFFSET>, SetLevelAllChannels::<Impl, OFFSET>)
    }
}
pub trait ISimpleAudioVolumeImpl: Sized {
    fn SetMasterVolume();
    fn GetMasterVolume();
    fn SetMute();
    fn GetMute();
}
impl ::windows::core::RuntimeName for ISimpleAudioVolume {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISimpleAudioVolume";
}
impl ISimpleAudioVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleAudioVolumeImpl, const OFFSET: isize>() -> ISimpleAudioVolumeVtbl {
        unsafe extern "system" fn SetMasterVolume<Impl: ISimpleAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flevel: f32, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMasterVolume(flevel, &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMasterVolume<Impl: ISimpleAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMasterVolume(::core::mem::transmute_copy(&pflevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMute<Impl: ISimpleAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL, eventcontext: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMute(&*(&bmute as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&eventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMute<Impl: ISimpleAudioVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMute(::core::mem::transmute_copy(&pbmute)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISimpleAudioVolume>, ::windows::core::GetTrustLevel, SetMasterVolume::<Impl, OFFSET>, GetMasterVolume::<Impl, OFFSET>, SetMute::<Impl, OFFSET>, GetMute::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioClientImpl: Sized {
    fn GetStaticObjectPosition();
    fn GetNativeStaticObjectTypeMask();
    fn GetMaxDynamicObjectCount();
    fn GetSupportedAudioObjectFormatEnumerator();
    fn GetMaxFrameCount();
    fn IsAudioObjectFormatSupported();
    fn IsSpatialAudioStreamAvailable();
    fn ActivateSpatialAudioStream();
}
impl ::windows::core::RuntimeName for ISpatialAudioClient {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioClient";
}
impl ISpatialAudioClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioClientImpl, const OFFSET: isize>() -> ISpatialAudioClientVtbl {
        unsafe extern "system" fn GetStaticObjectPosition<Impl: ISpatialAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStaticObjectPosition(r#type, ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNativeStaticObjectTypeMask<Impl: ISpatialAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: *mut AudioObjectType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNativeStaticObjectTypeMask(::core::mem::transmute_copy(&mask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxDynamicObjectCount<Impl: ISpatialAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxDynamicObjectCount(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedAudioObjectFormatEnumerator<Impl: ISpatialAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedAudioObjectFormatEnumerator(::core::mem::transmute_copy(&enumerator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxFrameCount<Impl: ISpatialAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxFrameCount(&*(&objectformat as *const <WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&framecountperbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAudioObjectFormatSupported<Impl: ISpatialAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAudioObjectFormatSupported(&*(&objectformat as *const <WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSpatialAudioStreamAvailable<Impl: ISpatialAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamuuid: &::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSpatialAudioStreamAvailable(&*(&streamuuid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&auxiliaryinfo as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioStream<Impl: ISpatialAudioClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: &::windows::core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioStream(&*(&activationparams as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&stream)) {
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
            ::windows::core::GetRuntimeClassName::<ISpatialAudioClient>,
            ::windows::core::GetTrustLevel,
            GetStaticObjectPosition::<Impl, OFFSET>,
            GetNativeStaticObjectTypeMask::<Impl, OFFSET>,
            GetMaxDynamicObjectCount::<Impl, OFFSET>,
            GetSupportedAudioObjectFormatEnumerator::<Impl, OFFSET>,
            GetMaxFrameCount::<Impl, OFFSET>,
            IsAudioObjectFormatSupported::<Impl, OFFSET>,
            IsSpatialAudioStreamAvailable::<Impl, OFFSET>,
            ActivateSpatialAudioStream::<Impl, OFFSET>,
        )
    }
}
pub trait ISpatialAudioClient2Impl: Sized + ISpatialAudioClientImpl {
    fn IsOffloadCapable();
    fn GetMaxFrameCountForCategory();
}
impl ::windows::core::RuntimeName for ISpatialAudioClient2 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioClient2";
}
impl ISpatialAudioClient2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioClient2Impl, const OFFSET: isize>() -> ISpatialAudioClient2Vtbl {
        unsafe extern "system" fn IsOffloadCapable<Impl: ISpatialAudioClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOffloadCapable(category, ::core::mem::transmute_copy(&isoffloadcapable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxFrameCountForCategory<Impl: ISpatialAudioClient2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxFrameCountForCategory(category, &*(&offloadenabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&objectformat as *const <WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&framecountperbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioClient2>, ::windows::core::GetTrustLevel, IsOffloadCapable::<Impl, OFFSET>, GetMaxFrameCountForCategory::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioMetadataClientImpl: Sized {
    fn ActivateSpatialAudioMetadataItems();
    fn GetSpatialAudioMetadataItemsBufferLength();
    fn ActivateSpatialAudioMetadataWriter();
    fn ActivateSpatialAudioMetadataCopier();
    fn ActivateSpatialAudioMetadataReader();
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataClient {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioMetadataClient";
}
impl ISpatialAudioMetadataClientVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataClientImpl, const OFFSET: isize>() -> ISpatialAudioMetadataClientVtbl {
        unsafe extern "system" fn ActivateSpatialAudioMetadataItems<Impl: ISpatialAudioMetadataClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::windows::core::RawPtr, metadataitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioMetadataItems(maxitemcount, framecount, ::core::mem::transmute_copy(&metadataitemsbuffer), ::core::mem::transmute_copy(&metadataitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpatialAudioMetadataItemsBufferLength<Impl: ISpatialAudioMetadataClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxitemcount: u16, bufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpatialAudioMetadataItemsBufferLength(maxitemcount, ::core::mem::transmute_copy(&bufferlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataWriter<Impl: ISpatialAudioMetadataClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioMetadataWriter(overflowmode, ::core::mem::transmute_copy(&metadatawriter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataCopier<Impl: ISpatialAudioMetadataClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadatacopier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioMetadataCopier(::core::mem::transmute_copy(&metadatacopier)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataReader<Impl: ISpatialAudioMetadataClientImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioMetadataReader(::core::mem::transmute_copy(&metadatareader)) {
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
            ::windows::core::GetRuntimeClassName::<ISpatialAudioMetadataClient>,
            ::windows::core::GetTrustLevel,
            ActivateSpatialAudioMetadataItems::<Impl, OFFSET>,
            GetSpatialAudioMetadataItemsBufferLength::<Impl, OFFSET>,
            ActivateSpatialAudioMetadataWriter::<Impl, OFFSET>,
            ActivateSpatialAudioMetadataCopier::<Impl, OFFSET>,
            ActivateSpatialAudioMetadataReader::<Impl, OFFSET>,
        )
    }
}
pub trait ISpatialAudioMetadataCopierImpl: Sized {
    fn Open();
    fn CopyMetadataForFrames();
    fn Close();
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataCopier {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioMetadataCopier";
}
impl ISpatialAudioMetadataCopierVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataCopierImpl, const OFFSET: isize>() -> ISpatialAudioMetadataCopierVtbl {
        unsafe extern "system" fn Open<Impl: ISpatialAudioMetadataCopierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&metadataitems as *const <ISpatialAudioMetadataItems as ::windows::core::Abi>::Abi as *const <ISpatialAudioMetadataItems as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyMetadataForFrames<Impl: ISpatialAudioMetadataCopierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: ::windows::core::RawPtr, itemscopied: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyMetadataForFrames(copyframecount, copymode, &*(&dstmetadataitems as *const <ISpatialAudioMetadataItems as ::windows::core::Abi>::Abi as *const <ISpatialAudioMetadataItems as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&itemscopied)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ISpatialAudioMetadataCopierImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioMetadataCopier>, ::windows::core::GetTrustLevel, Open::<Impl, OFFSET>, CopyMetadataForFrames::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioMetadataItemsImpl: Sized {
    fn GetFrameCount();
    fn GetItemCount();
    fn GetMaxItemCount();
    fn GetMaxValueBufferLength();
    fn GetInfo();
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataItems {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioMetadataItems";
}
impl ISpatialAudioMetadataItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataItemsImpl, const OFFSET: isize>() -> ISpatialAudioMetadataItemsVtbl {
        unsafe extern "system" fn GetFrameCount<Impl: ISpatialAudioMetadataItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameCount(::core::mem::transmute_copy(&framecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCount<Impl: ISpatialAudioMetadataItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCount(::core::mem::transmute_copy(&itemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxItemCount<Impl: ISpatialAudioMetadataItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxitemcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxItemCount(::core::mem::transmute_copy(&maxitemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueBufferLength<Impl: ISpatialAudioMetadataItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxvaluebufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxValueBufferLength(::core::mem::transmute_copy(&maxvaluebufferlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Impl: ISpatialAudioMetadataItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, info: *mut SpatialAudioMetadataItemsInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo(::core::mem::transmute_copy(&info)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioMetadataItems>, ::windows::core::GetTrustLevel, GetFrameCount::<Impl, OFFSET>, GetItemCount::<Impl, OFFSET>, GetMaxItemCount::<Impl, OFFSET>, GetMaxValueBufferLength::<Impl, OFFSET>, GetInfo::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioMetadataItemsBufferImpl: Sized {
    fn AttachToBuffer();
    fn AttachToPopulatedBuffer();
    fn DetachBuffer();
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataItemsBuffer {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioMetadataItemsBuffer";
}
impl ISpatialAudioMetadataItemsBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataItemsBufferImpl, const OFFSET: isize>() -> ISpatialAudioMetadataItemsBufferVtbl {
        unsafe extern "system" fn AttachToBuffer<Impl: ISpatialAudioMetadataItemsBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachToBuffer(::core::mem::transmute_copy(&buffer), bufferlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AttachToPopulatedBuffer<Impl: ISpatialAudioMetadataItemsBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AttachToPopulatedBuffer(::core::mem::transmute_copy(&buffer), bufferlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Impl: ISpatialAudioMetadataItemsBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DetachBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioMetadataItemsBuffer>, ::windows::core::GetTrustLevel, AttachToBuffer::<Impl, OFFSET>, AttachToPopulatedBuffer::<Impl, OFFSET>, DetachBuffer::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioMetadataReaderImpl: Sized {
    fn Open();
    fn ReadNextItem();
    fn ReadNextItemCommand();
    fn Close();
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataReader {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioMetadataReader";
}
impl ISpatialAudioMetadataReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataReaderImpl, const OFFSET: isize>() -> ISpatialAudioMetadataReaderVtbl {
        unsafe extern "system" fn Open<Impl: ISpatialAudioMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&metadataitems as *const <ISpatialAudioMetadataItems as ::windows::core::Abi>::Abi as *const <ISpatialAudioMetadataItems as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadNextItem<Impl: ISpatialAudioMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadNextItem(::core::mem::transmute_copy(&commandcount), ::core::mem::transmute_copy(&frameoffset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadNextItemCommand<Impl: ISpatialAudioMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadNextItemCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), maxvaluebufferlength, ::core::mem::transmute_copy(&valuebufferlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ISpatialAudioMetadataReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioMetadataReader>, ::windows::core::GetTrustLevel, Open::<Impl, OFFSET>, ReadNextItem::<Impl, OFFSET>, ReadNextItemCommand::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioMetadataWriterImpl: Sized {
    fn Open();
    fn WriteNextItem();
    fn WriteNextItemCommand();
    fn Close();
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataWriter {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioMetadataWriter";
}
impl ISpatialAudioMetadataWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataWriterImpl, const OFFSET: isize>() -> ISpatialAudioMetadataWriterVtbl {
        unsafe extern "system" fn Open<Impl: ISpatialAudioMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open(&*(&metadataitems as *const <ISpatialAudioMetadataItems as ::windows::core::Abi>::Abi as *const <ISpatialAudioMetadataItems as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteNextItem<Impl: ISpatialAudioMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameoffset: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteNextItem(frameoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteNextItemCommand<Impl: ISpatialAudioMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteNextItemCommand(commandid, &*(&valuebuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), valuebufferlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ISpatialAudioMetadataWriterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioMetadataWriter>, ::windows::core::GetTrustLevel, Open::<Impl, OFFSET>, WriteNextItem::<Impl, OFFSET>, WriteNextItemCommand::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioObjectImpl: Sized + ISpatialAudioObjectBaseImpl {
    fn SetPosition();
    fn SetVolume();
}
impl ::windows::core::RuntimeName for ISpatialAudioObject {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObject";
}
impl ISpatialAudioObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectImpl, const OFFSET: isize>() -> ISpatialAudioObjectVtbl {
        unsafe extern "system" fn SetPosition<Impl: ISpatialAudioObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPosition(x, y, z) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: ISpatialAudioObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVolume(volume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioObject>, ::windows::core::GetTrustLevel, SetPosition::<Impl, OFFSET>, SetVolume::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioObjectBaseImpl: Sized {
    fn GetBuffer();
    fn SetEndOfStream();
    fn IsActive();
    fn GetAudioObjectType();
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectBase {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObjectBase";
}
impl ISpatialAudioObjectBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectBaseImpl, const OFFSET: isize>() -> ISpatialAudioObjectBaseVtbl {
        unsafe extern "system" fn GetBuffer<Impl: ISpatialAudioObjectBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndOfStream<Impl: ISpatialAudioObjectBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEndOfStream(framecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsActive<Impl: ISpatialAudioObjectBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive(::core::mem::transmute_copy(&isactive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioObjectType<Impl: ISpatialAudioObjectBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioObjectType(::core::mem::transmute_copy(&audioobjecttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioObjectBase>, ::windows::core::GetTrustLevel, GetBuffer::<Impl, OFFSET>, SetEndOfStream::<Impl, OFFSET>, IsActive::<Impl, OFFSET>, GetAudioObjectType::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioObjectForHrtfImpl: Sized + ISpatialAudioObjectBaseImpl {
    fn SetPosition();
    fn SetGain();
    fn SetOrientation();
    fn SetEnvironment();
    fn SetDistanceDecay();
    fn SetDirectivity();
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectForHrtf {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObjectForHrtf";
}
impl ISpatialAudioObjectForHrtfVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectForHrtfImpl, const OFFSET: isize>() -> ISpatialAudioObjectForHrtfVtbl {
        unsafe extern "system" fn SetPosition<Impl: ISpatialAudioObjectForHrtfImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPosition(x, y, z) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGain<Impl: ISpatialAudioObjectForHrtfImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGain(gain) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: ISpatialAudioObjectForHrtfImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: *const *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOrientation(orientation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnvironment<Impl: ISpatialAudioObjectForHrtfImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDistanceDecay<Impl: ISpatialAudioObjectForHrtfImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDistanceDecay(&*(&distancedecay as *const <SpatialAudioHrtfDistanceDecay as ::windows::core::Abi>::Abi as *const <SpatialAudioHrtfDistanceDecay as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDirectivity<Impl: ISpatialAudioObjectForHrtfImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDirectivity(&*(&directivity as *const <SpatialAudioHrtfDirectivityUnion as ::windows::core::Abi>::Abi as *const <SpatialAudioHrtfDirectivityUnion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioObjectForHrtf>, ::windows::core::GetTrustLevel, SetPosition::<Impl, OFFSET>, SetGain::<Impl, OFFSET>, SetOrientation::<Impl, OFFSET>, SetEnvironment::<Impl, OFFSET>, SetDistanceDecay::<Impl, OFFSET>, SetDirectivity::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioObjectForMetadataCommandsImpl: Sized + ISpatialAudioObjectBaseImpl {
    fn WriteNextMetadataCommand();
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectForMetadataCommands {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObjectForMetadataCommands";
}
impl ISpatialAudioObjectForMetadataCommandsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectForMetadataCommandsImpl, const OFFSET: isize>() -> ISpatialAudioObjectForMetadataCommandsVtbl {
        unsafe extern "system" fn WriteNextMetadataCommand<Impl: ISpatialAudioObjectForMetadataCommandsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteNextMetadataCommand(commandid, &*(&valuebuffer as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), valuebufferlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioObjectForMetadataCommands>, ::windows::core::GetTrustLevel, WriteNextMetadataCommand::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioObjectForMetadataItemsImpl: Sized + ISpatialAudioObjectBaseImpl {
    fn GetSpatialAudioMetadataItems();
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectForMetadataItems {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObjectForMetadataItems";
}
impl ISpatialAudioObjectForMetadataItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectForMetadataItemsImpl, const OFFSET: isize>() -> ISpatialAudioObjectForMetadataItemsVtbl {
        unsafe extern "system" fn GetSpatialAudioMetadataItems<Impl: ISpatialAudioObjectForMetadataItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpatialAudioMetadataItems(::core::mem::transmute_copy(&metadataitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioObjectForMetadataItems>, ::windows::core::GetTrustLevel, GetSpatialAudioMetadataItems::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioObjectRenderStreamImpl: Sized + ISpatialAudioObjectRenderStreamBaseImpl {
    fn ActivateSpatialAudioObject();
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStream {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObjectRenderStream";
}
impl ISpatialAudioObjectRenderStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStreamImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamVtbl {
        unsafe extern "system" fn ActivateSpatialAudioObject<Impl: ISpatialAudioObjectRenderStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioObject(r#type, ::core::mem::transmute_copy(&audioobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioObjectRenderStream>, ::windows::core::GetTrustLevel, ActivateSpatialAudioObject::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioObjectRenderStreamBaseImpl: Sized {
    fn GetAvailableDynamicObjectCount();
    fn GetService();
    fn Start();
    fn Stop();
    fn Reset();
    fn BeginUpdatingAudioObjects();
    fn EndUpdatingAudioObjects();
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStreamBase {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObjectRenderStreamBase";
}
impl ISpatialAudioObjectRenderStreamBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStreamBaseImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamBaseVtbl {
        unsafe extern "system" fn GetAvailableDynamicObjectCount<Impl: ISpatialAudioObjectRenderStreamBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableDynamicObjectCount(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Impl: ISpatialAudioObjectRenderStreamBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetService(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&service)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: ISpatialAudioObjectRenderStreamBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: ISpatialAudioObjectRenderStreamBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: ISpatialAudioObjectRenderStreamBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUpdatingAudioObjects<Impl: ISpatialAudioObjectRenderStreamBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginUpdatingAudioObjects(::core::mem::transmute_copy(&availabledynamicobjectcount), ::core::mem::transmute_copy(&framecountperbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUpdatingAudioObjects<Impl: ISpatialAudioObjectRenderStreamBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndUpdatingAudioObjects() {
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
            ::windows::core::GetRuntimeClassName::<ISpatialAudioObjectRenderStreamBase>,
            ::windows::core::GetTrustLevel,
            GetAvailableDynamicObjectCount::<Impl, OFFSET>,
            GetService::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Reset::<Impl, OFFSET>,
            BeginUpdatingAudioObjects::<Impl, OFFSET>,
            EndUpdatingAudioObjects::<Impl, OFFSET>,
        )
    }
}
pub trait ISpatialAudioObjectRenderStreamForHrtfImpl: Sized + ISpatialAudioObjectRenderStreamBaseImpl {
    fn ActivateSpatialAudioObjectForHrtf();
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStreamForHrtf {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObjectRenderStreamForHrtf";
}
impl ISpatialAudioObjectRenderStreamForHrtfVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStreamForHrtfImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamForHrtfVtbl {
        unsafe extern "system" fn ActivateSpatialAudioObjectForHrtf<Impl: ISpatialAudioObjectRenderStreamForHrtfImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioObjectForHrtf(r#type, ::core::mem::transmute_copy(&audioobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioObjectRenderStreamForHrtf>, ::windows::core::GetTrustLevel, ActivateSpatialAudioObjectForHrtf::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioObjectRenderStreamForMetadataImpl: Sized + ISpatialAudioObjectRenderStreamBaseImpl {
    fn ActivateSpatialAudioObjectForMetadataCommands();
    fn ActivateSpatialAudioObjectForMetadataItems();
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStreamForMetadata {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObjectRenderStreamForMetadata";
}
impl ISpatialAudioObjectRenderStreamForMetadataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStreamForMetadataImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamForMetadataVtbl {
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataCommands<Impl: ISpatialAudioObjectRenderStreamForMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioObjectForMetadataCommands(r#type, ::core::mem::transmute_copy(&audioobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataItems<Impl: ISpatialAudioObjectRenderStreamForMetadataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioObjectForMetadataItems(r#type, ::core::mem::transmute_copy(&audioobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioObjectRenderStreamForMetadata>, ::windows::core::GetTrustLevel, ActivateSpatialAudioObjectForMetadataCommands::<Impl, OFFSET>, ActivateSpatialAudioObjectForMetadataItems::<Impl, OFFSET>)
    }
}
pub trait ISpatialAudioObjectRenderStreamNotifyImpl: Sized {
    fn OnAvailableDynamicObjectCountChange();
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStreamNotify {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISpatialAudioObjectRenderStreamNotify";
}
impl ISpatialAudioObjectRenderStreamNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStreamNotifyImpl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamNotifyVtbl {
        unsafe extern "system" fn OnAvailableDynamicObjectCountChange<Impl: ISpatialAudioObjectRenderStreamNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAvailableDynamicObjectCountChange(&*(&sender as *const <ISpatialAudioObjectRenderStreamBase as ::windows::core::Abi>::Abi as *const <ISpatialAudioObjectRenderStreamBase as ::windows::core::DefaultType>::DefaultType), hnscompliancedeadlinetime, availabledynamicobjectcountchange) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISpatialAudioObjectRenderStreamNotify>, ::windows::core::GetTrustLevel, OnAvailableDynamicObjectCountChange::<Impl, OFFSET>)
    }
}
pub trait ISubunitImpl: Sized {}
impl ::windows::core::RuntimeName for ISubunit {
    const NAME: &'static str = "Windows.Win32.Media.Audio.ISubunit";
}
impl ISubunitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISubunitImpl, const OFFSET: isize>() -> ISubunitVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISubunit>, ::windows::core::GetTrustLevel)
    }
}
