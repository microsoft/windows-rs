pub trait IActivateAudioInterfaceAsyncOperation_Impl: Sized {
    fn GetActivateResult(&mut self, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivateAudioInterfaceAsyncOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivateAudioInterfaceAsyncOperation_Vtbl {
        unsafe extern "system" fn GetActivateResult<Impl: IActivateAudioInterfaceAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetActivateResult(::core::mem::transmute_copy(&activateresult), ::core::mem::transmute_copy(&activatedinterface)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetActivateResult: GetActivateResult::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceAsyncOperation as ::windows::core::Interface>::IID
    }
}
pub trait IActivateAudioInterfaceCompletionHandler_Impl: Sized {
    fn ActivateCompleted(&mut self, activateoperation: &::core::option::Option<IActivateAudioInterfaceAsyncOperation>) -> ::windows::core::Result<()>;
}
impl IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivateAudioInterfaceCompletionHandler_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivateAudioInterfaceCompletionHandler_Vtbl {
        unsafe extern "system" fn ActivateCompleted<Impl: IActivateAudioInterfaceCompletionHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activateoperation: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateCompleted(::core::mem::transmute(&activateoperation)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ActivateCompleted: ActivateCompleted::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceCompletionHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioAmbisonicsControl_Impl: Sized {
    fn SetData(&mut self, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::core::Result<()>;
    fn SetHeadTracking(&mut self, benableheadtracking: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetHeadTracking(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetRotation(&mut self, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioAmbisonicsControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioAmbisonicsControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioAmbisonicsControl_Vtbl {
        unsafe extern "system" fn SetData<Impl: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute_copy(&pambisonicsparams), ::core::mem::transmute_copy(&cbambisonicsparams)).into()
        }
        unsafe extern "system" fn SetHeadTracking<Impl: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benableheadtracking: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeadTracking(::core::mem::transmute_copy(&benableheadtracking)).into()
        }
        unsafe extern "system" fn GetHeadTracking<Impl: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenableheadtracking: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHeadTracking() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenableheadtracking = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&w)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetData: SetData::<Impl, IMPL_OFFSET>,
            SetHeadTracking: SetHeadTracking::<Impl, IMPL_OFFSET>,
            GetHeadTracking: GetHeadTracking::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioAmbisonicsControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioAutoGainControl_Impl: Sized {
    fn GetEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&mut self, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioAutoGainControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioAutoGainControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioAutoGainControl_Vtbl {
        unsafe extern "system" fn GetEnabled<Impl: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&benable), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEnabled: GetEnabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioAutoGainControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioBass_Impl: Sized + IPerChannelDbLevel_Impl {}
impl IAudioBass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioBass_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioBass_Vtbl {
        Self { base: IPerChannelDbLevel_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioBass as ::windows::core::Interface>::IID
    }
}
pub trait IAudioCaptureClient_Impl: Sized {
    fn GetBuffer(&mut self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::Result<()>;
    fn ReleaseBuffer(&mut self, numframesread: u32) -> ::windows::core::Result<()>;
    fn GetNextPacketSize(&mut self) -> ::windows::core::Result<u32>;
}
impl IAudioCaptureClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioCaptureClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioCaptureClient_Vtbl {
        unsafe extern "system" fn GetBuffer<Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pnumframestoread), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pu64deviceposition), ::core::mem::transmute_copy(&pu64qpcposition)).into()
        }
        unsafe extern "system" fn ReleaseBuffer<Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframesread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseBuffer(::core::mem::transmute_copy(&numframesread)).into()
        }
        unsafe extern "system" fn GetNextPacketSize<Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumframesinnextpacket = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Impl, IMPL_OFFSET>,
            GetNextPacketSize: GetNextPacketSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioCaptureClient as ::windows::core::Interface>::IID
    }
}
pub trait IAudioChannelConfig_Impl: Sized {
    fn SetChannelConfig(&mut self, dwconfig: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetChannelConfig(&mut self) -> ::windows::core::Result<u32>;
}
impl IAudioChannelConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioChannelConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioChannelConfig_Vtbl {
        unsafe extern "system" fn SetChannelConfig<Impl: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconfig: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelConfig(::core::mem::transmute_copy(&dwconfig), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetChannelConfig<Impl: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconfig: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelConfig() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetChannelConfig: SetChannelConfig::<Impl, IMPL_OFFSET>,
            GetChannelConfig: GetChannelConfig::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioChannelConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioClient_Impl: Sized {
    fn Initialize(&mut self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetBufferSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetStreamLatency(&mut self) -> ::windows::core::Result<i64>;
    fn GetCurrentPadding(&mut self) -> ::windows::core::Result<u32>;
    fn IsFormatSupported(&mut self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX) -> ::windows::core::Result<*mut WAVEFORMATEX>;
    fn GetMixFormat(&mut self) -> ::windows::core::Result<*mut WAVEFORMATEX>;
    fn GetDevicePeriod(&mut self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn SetEventHandle(&mut self, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetService(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioClient_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&sharemode), ::core::mem::transmute_copy(&streamflags), ::core::mem::transmute_copy(&hnsbufferduration), ::core::mem::transmute_copy(&hnsperiodicity), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&audiosessionguid)).into()
        }
        unsafe extern "system" fn GetBufferSize<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumbufferframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumbufferframes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamLatency<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnslatency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStreamLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *phnslatency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPadding<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumpaddingframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentPadding() {
                ::core::result::Result::Ok(ok__) => {
                    *pnumpaddingframes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFormatSupported<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFormatSupported(::core::mem::transmute_copy(&sharemode), ::core::mem::transmute_copy(&pformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclosestmatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMixFormat<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMixFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdeviceformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePeriod<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevicePeriod(::core::mem::transmute_copy(&phnsdefaultdeviceperiod), ::core::mem::transmute_copy(&phnsminimumdeviceperiod)).into()
        }
        unsafe extern "system" fn Start<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Reset<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn SetEventHandle<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventHandle(::core::mem::transmute_copy(&eventhandle)).into()
        }
        unsafe extern "system" fn GetService<Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetService(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetBufferSize: GetBufferSize::<Impl, IMPL_OFFSET>,
            GetStreamLatency: GetStreamLatency::<Impl, IMPL_OFFSET>,
            GetCurrentPadding: GetCurrentPadding::<Impl, IMPL_OFFSET>,
            IsFormatSupported: IsFormatSupported::<Impl, IMPL_OFFSET>,
            GetMixFormat: GetMixFormat::<Impl, IMPL_OFFSET>,
            GetDevicePeriod: GetDevicePeriod::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            SetEventHandle: SetEventHandle::<Impl, IMPL_OFFSET>,
            GetService: GetService::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioClient2_Impl: Sized + IAudioClient_Impl {
    fn IsOffloadCapable(&mut self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetClientProperties(&mut self, pproperties: *const AudioClientProperties) -> ::windows::core::Result<()>;
    fn GetBufferSizeLimits(&mut self, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioClient2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClient2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioClient2_Vtbl {
        unsafe extern "system" fn IsOffloadCapable<Impl: IAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOffloadCapable(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    *pboffloadcapable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientProperties<Impl: IAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: *const AudioClientProperties) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientProperties(::core::mem::transmute_copy(&pproperties)).into()
        }
        unsafe extern "system" fn GetBufferSizeLimits<Impl: IAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBufferSizeLimits(::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&beventdriven), ::core::mem::transmute_copy(&phnsminbufferduration), ::core::mem::transmute_copy(&phnsmaxbufferduration)).into()
        }
        Self {
            base: IAudioClient_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsOffloadCapable: IsOffloadCapable::<Impl, IMPL_OFFSET>,
            SetClientProperties: SetClientProperties::<Impl, IMPL_OFFSET>,
            GetBufferSizeLimits: GetBufferSizeLimits::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClient2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioClient3_Impl: Sized + IAudioClient_Impl + IAudioClient2_Impl {
    fn GetSharedModeEnginePeriod(&mut self, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::Result<()>;
    fn GetCurrentSharedModeEnginePeriod(&mut self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::Result<()>;
    fn InitializeSharedAudioStream(&mut self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioClient3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClient3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioClient3_Vtbl {
        unsafe extern "system" fn GetSharedModeEnginePeriod<Impl: IAudioClient3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSharedModeEnginePeriod(::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pdefaultperiodinframes), ::core::mem::transmute_copy(&pfundamentalperiodinframes), ::core::mem::transmute_copy(&pminperiodinframes), ::core::mem::transmute_copy(&pmaxperiodinframes)).into()
        }
        unsafe extern "system" fn GetCurrentSharedModeEnginePeriod<Impl: IAudioClient3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentSharedModeEnginePeriod(::core::mem::transmute_copy(&ppformat), ::core::mem::transmute_copy(&pcurrentperiodinframes)).into()
        }
        unsafe extern "system" fn InitializeSharedAudioStream<Impl: IAudioClient3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeSharedAudioStream(::core::mem::transmute_copy(&streamflags), ::core::mem::transmute_copy(&periodinframes), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&audiosessionguid)).into()
        }
        Self {
            base: IAudioClient2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSharedModeEnginePeriod: GetSharedModeEnginePeriod::<Impl, IMPL_OFFSET>,
            GetCurrentSharedModeEnginePeriod: GetCurrentSharedModeEnginePeriod::<Impl, IMPL_OFFSET>,
            InitializeSharedAudioStream: InitializeSharedAudioStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClient3 as ::windows::core::Interface>::IID
    }
}
pub trait IAudioClientDuckingControl_Impl: Sized {
    fn SetDuckingOptionsForCurrentStream(&mut self, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::Result<()>;
}
impl IAudioClientDuckingControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClientDuckingControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioClientDuckingControl_Vtbl {
        unsafe extern "system" fn SetDuckingOptionsForCurrentStream<Impl: IAudioClientDuckingControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuckingOptionsForCurrentStream(::core::mem::transmute_copy(&options)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDuckingOptionsForCurrentStream: SetDuckingOptionsForCurrentStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClientDuckingControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioClock_Impl: Sized {
    fn GetFrequency(&mut self) -> ::windows::core::Result<u64>;
    fn GetPosition(&mut self, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::Result<()>;
    fn GetCharacteristics(&mut self) -> ::windows::core::Result<u32>;
}
impl IAudioClock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClock_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioClock_Vtbl {
        unsafe extern "system" fn GetFrequency<Impl: IAudioClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu64frequency: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *pu64frequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Impl: IAudioClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPosition(::core::mem::transmute_copy(&pu64position), ::core::mem::transmute_copy(&pu64qpcposition)).into()
        }
        unsafe extern "system" fn GetCharacteristics<Impl: IAudioClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcharacteristics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFrequency: GetFrequency::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
            GetCharacteristics: GetCharacteristics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClock as ::windows::core::Interface>::IID
    }
}
pub trait IAudioClock2_Impl: Sized {
    fn GetDevicePosition(&mut self, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::Result<()>;
}
impl IAudioClock2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClock2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioClock2_Vtbl {
        unsafe extern "system" fn GetDevicePosition<Impl: IAudioClock2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevicePosition(::core::mem::transmute_copy(&deviceposition), ::core::mem::transmute_copy(&qpcposition)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDevicePosition: GetDevicePosition::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClock2 as ::windows::core::Interface>::IID
    }
}
pub trait IAudioClockAdjustment_Impl: Sized {
    fn SetSampleRate(&mut self, flsamplerate: f32) -> ::windows::core::Result<()>;
}
impl IAudioClockAdjustment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioClockAdjustment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioClockAdjustment_Vtbl {
        unsafe extern "system" fn SetSampleRate<Impl: IAudioClockAdjustment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flsamplerate: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSampleRate(::core::mem::transmute_copy(&flsamplerate)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetSampleRate: SetSampleRate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClockAdjustment as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEffectsChangedNotificationClient_Impl: Sized {
    fn OnAudioEffectsChanged(&mut self) -> ::windows::core::Result<()>;
}
impl IAudioEffectsChangedNotificationClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectsChangedNotificationClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEffectsChangedNotificationClient_Vtbl {
        unsafe extern "system" fn OnAudioEffectsChanged<Impl: IAudioEffectsChangedNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAudioEffectsChanged().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnAudioEffectsChanged: OnAudioEffectsChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffectsChangedNotificationClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEffectsManager_Impl: Sized {
    fn RegisterAudioEffectsChangedNotificationCallback(&mut self, client: &::core::option::Option<IAudioEffectsChangedNotificationClient>) -> ::windows::core::Result<()>;
    fn UnregisterAudioEffectsChangedNotificationCallback(&mut self, client: &::core::option::Option<IAudioEffectsChangedNotificationClient>) -> ::windows::core::Result<()>;
    fn GetAudioEffects(&mut self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::Result<()>;
    fn SetAudioEffectState(&mut self, effectid: &::windows::core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEffectsManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEffectsManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEffectsManager_Vtbl {
        unsafe extern "system" fn RegisterAudioEffectsChangedNotificationCallback<Impl: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, client: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterAudioEffectsChangedNotificationCallback(::core::mem::transmute(&client)).into()
        }
        unsafe extern "system" fn UnregisterAudioEffectsChangedNotificationCallback<Impl: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, client: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterAudioEffectsChangedNotificationCallback(::core::mem::transmute(&client)).into()
        }
        unsafe extern "system" fn GetAudioEffects<Impl: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAudioEffects(::core::mem::transmute_copy(&effects), ::core::mem::transmute_copy(&numeffects)).into()
        }
        unsafe extern "system" fn SetAudioEffectState<Impl: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioEffectState(::core::mem::transmute_copy(&effectid), ::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterAudioEffectsChangedNotificationCallback: RegisterAudioEffectsChangedNotificationCallback::<Impl, IMPL_OFFSET>,
            UnregisterAudioEffectsChangedNotificationCallback: UnregisterAudioEffectsChangedNotificationCallback::<Impl, IMPL_OFFSET>,
            GetAudioEffects: GetAudioEffects::<Impl, IMPL_OFFSET>,
            SetAudioEffectState: SetAudioEffectState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffectsManager as ::windows::core::Interface>::IID
    }
}
pub trait IAudioFormatEnumerator_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetFormat(&mut self, index: u32) -> ::windows::core::Result<*mut WAVEFORMATEX>;
}
impl IAudioFormatEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioFormatEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioFormatEnumerator_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IAudioFormatEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IAudioFormatEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, format: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *format = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetFormat: GetFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFormatEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IAudioInputSelector_Impl: Sized {
    fn GetSelection(&mut self) -> ::windows::core::Result<u32>;
    fn SetSelection(&mut self, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IAudioInputSelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioInputSelector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioInputSelector_Vtbl {
        unsafe extern "system" fn GetSelection<Impl: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pnidselected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&nidselect), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            SetSelection: SetSelection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioInputSelector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioLoudness_Impl: Sized {
    fn GetEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&mut self, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioLoudness_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioLoudness_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioLoudness_Vtbl {
        unsafe extern "system" fn GetEnabled<Impl: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&benable), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEnabled: GetEnabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioLoudness as ::windows::core::Interface>::IID
    }
}
pub trait IAudioMidrange_Impl: Sized + IPerChannelDbLevel_Impl {}
impl IAudioMidrange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMidrange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioMidrange_Vtbl {
        Self { base: IPerChannelDbLevel_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMidrange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioMute_Impl: Sized {
    fn SetMute(&mut self, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMute(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioMute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMute_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioMute_Vtbl {
        unsafe extern "system" fn SetMute<Impl: IAudioMute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMute(::core::mem::transmute_copy(&bmuted), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetMute<Impl: IAudioMute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmuted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMute() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmuted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMute: SetMute::<Impl, IMPL_OFFSET>,
            GetMute: GetMute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMute as ::windows::core::Interface>::IID
    }
}
pub trait IAudioOutputSelector_Impl: Sized {
    fn GetSelection(&mut self) -> ::windows::core::Result<u32>;
    fn SetSelection(&mut self, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IAudioOutputSelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioOutputSelector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioOutputSelector_Vtbl {
        unsafe extern "system" fn GetSelection<Impl: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    *pnidselected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Impl: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSelection(::core::mem::transmute_copy(&nidselect), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSelection: GetSelection::<Impl, IMPL_OFFSET>,
            SetSelection: SetSelection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioOutputSelector as ::windows::core::Interface>::IID
    }
}
pub trait IAudioPeakMeter_Impl: Sized {
    fn GetChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetLevel(&mut self, nchannel: u32) -> ::windows::core::Result<f32>;
}
impl IAudioPeakMeter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioPeakMeter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioPeakMeter_Vtbl {
        unsafe extern "system" fn GetChannelCount<Impl: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcchannels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevel<Impl: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevel(::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetChannelCount: GetChannelCount::<Impl, IMPL_OFFSET>,
            GetLevel: GetLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioPeakMeter as ::windows::core::Interface>::IID
    }
}
pub trait IAudioRenderClient_Impl: Sized {
    fn GetBuffer(&mut self, numframesrequested: u32) -> ::windows::core::Result<*mut u8>;
    fn ReleaseBuffer(&mut self, numframeswritten: u32, dwflags: u32) -> ::windows::core::Result<()>;
}
impl IAudioRenderClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioRenderClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioRenderClient_Vtbl {
        unsafe extern "system" fn GetBuffer<Impl: IAudioRenderClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(::core::mem::transmute_copy(&numframesrequested)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Impl: IAudioRenderClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframeswritten: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseBuffer(::core::mem::transmute_copy(&numframeswritten), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioRenderClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSessionControl_Impl: Sized {
    fn GetState(&mut self) -> ::windows::core::Result<AudioSessionState>;
    fn GetDisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetDisplayName(&mut self, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetIconPath(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetIconPath(&mut self, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetGroupingParam(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetGroupingParam(&mut self, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterAudioSessionNotification(&mut self, newnotifications: &::core::option::Option<IAudioSessionEvents>) -> ::windows::core::Result<()>;
    fn UnregisterAudioSessionNotification(&mut self, newnotifications: &::core::option::Option<IAudioSessionEvents>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSessionControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSessionControl_Vtbl {
        unsafe extern "system" fn GetState<Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut AudioSessionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetIconPath<Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIconPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIconPath<Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIconPath(::core::mem::transmute_copy(&value), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetGroupingParam<Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroupingParam() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupingParam<Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGroupingParam(::core::mem::transmute_copy(&r#override), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn RegisterAudioSessionNotification<Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterAudioSessionNotification(::core::mem::transmute(&newnotifications)).into()
        }
        unsafe extern "system" fn UnregisterAudioSessionNotification<Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newnotifications: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterAudioSessionNotification(::core::mem::transmute(&newnotifications)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetState: GetState::<Impl, IMPL_OFFSET>,
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            GetIconPath: GetIconPath::<Impl, IMPL_OFFSET>,
            SetIconPath: SetIconPath::<Impl, IMPL_OFFSET>,
            GetGroupingParam: GetGroupingParam::<Impl, IMPL_OFFSET>,
            SetGroupingParam: SetGroupingParam::<Impl, IMPL_OFFSET>,
            RegisterAudioSessionNotification: RegisterAudioSessionNotification::<Impl, IMPL_OFFSET>,
            UnregisterAudioSessionNotification: UnregisterAudioSessionNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSessionControl2_Impl: Sized + IAudioSessionControl_Impl {
    fn GetSessionIdentifier(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetSessionInstanceIdentifier(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetProcessId(&mut self) -> ::windows::core::Result<u32>;
    fn IsSystemSoundsSession(&mut self) -> ::windows::core::Result<()>;
    fn SetDuckingPreference(&mut self, optout: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSessionControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionControl2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSessionControl2_Vtbl {
        unsafe extern "system" fn GetSessionIdentifier<Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionInstanceIdentifier<Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionInstanceIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessId<Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    *pretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSystemSoundsSession<Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSystemSoundsSession().into()
        }
        unsafe extern "system" fn SetDuckingPreference<Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optout: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDuckingPreference(::core::mem::transmute_copy(&optout)).into()
        }
        Self {
            base: IAudioSessionControl_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSessionIdentifier: GetSessionIdentifier::<Impl, IMPL_OFFSET>,
            GetSessionInstanceIdentifier: GetSessionInstanceIdentifier::<Impl, IMPL_OFFSET>,
            GetProcessId: GetProcessId::<Impl, IMPL_OFFSET>,
            IsSystemSoundsSession: IsSystemSoundsSession::<Impl, IMPL_OFFSET>,
            SetDuckingPreference: SetDuckingPreference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionControl2 as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSessionEnumerator_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetSession(&mut self, sessioncount: i32) -> ::windows::core::Result<IAudioSessionControl>;
}
impl IAudioSessionEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSessionEnumerator_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IAudioSessionEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessioncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *sessioncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSession<Impl: IAudioSessionEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessioncount: i32, session: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSession(::core::mem::transmute_copy(&sessioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *session = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetSession: GetSession::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSessionEvents_Impl: Sized {
    fn OnDisplayNameChanged(&mut self, newdisplayname: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnIconPathChanged(&mut self, newiconpath: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnSimpleVolumeChanged(&mut self, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnChannelVolumeChanged(&mut self, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnGroupingParamChanged(&mut self, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnStateChanged(&mut self, newstate: AudioSessionState) -> ::windows::core::Result<()>;
    fn OnSessionDisconnected(&mut self, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSessionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSessionEvents_Vtbl {
        unsafe extern "system" fn OnDisplayNameChanged<Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newdisplayname: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisplayNameChanged(::core::mem::transmute_copy(&newdisplayname), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnIconPathChanged<Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newiconpath: super::super::Foundation::PWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIconPathChanged(::core::mem::transmute_copy(&newiconpath), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnSimpleVolumeChanged<Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSimpleVolumeChanged(::core::mem::transmute_copy(&newvolume), ::core::mem::transmute_copy(&newmute), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnChannelVolumeChanged<Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnChannelVolumeChanged(::core::mem::transmute_copy(&channelcount), ::core::mem::transmute_copy(&newchannelvolumearray), ::core::mem::transmute_copy(&changedchannel), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnGroupingParamChanged<Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnGroupingParamChanged(::core::mem::transmute_copy(&newgroupingparam), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnStateChanged<Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: AudioSessionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStateChanged(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn OnSessionDisconnected<Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSessionDisconnected(::core::mem::transmute_copy(&disconnectreason)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnDisplayNameChanged: OnDisplayNameChanged::<Impl, IMPL_OFFSET>,
            OnIconPathChanged: OnIconPathChanged::<Impl, IMPL_OFFSET>,
            OnSimpleVolumeChanged: OnSimpleVolumeChanged::<Impl, IMPL_OFFSET>,
            OnChannelVolumeChanged: OnChannelVolumeChanged::<Impl, IMPL_OFFSET>,
            OnGroupingParamChanged: OnGroupingParamChanged::<Impl, IMPL_OFFSET>,
            OnStateChanged: OnStateChanged::<Impl, IMPL_OFFSET>,
            OnSessionDisconnected: OnSessionDisconnected::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionEvents as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSessionManager_Impl: Sized {
    fn GetAudioSessionControl(&mut self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<IAudioSessionControl>;
    fn GetSimpleAudioVolume(&mut self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<ISimpleAudioVolume>;
}
impl IAudioSessionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSessionManager_Vtbl {
        unsafe extern "system" fn GetAudioSessionControl<Impl: IAudioSessionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, sessioncontrol: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioSessionControl(::core::mem::transmute_copy(&audiosessionguid), ::core::mem::transmute_copy(&streamflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *sessioncontrol = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSimpleAudioVolume<Impl: IAudioSessionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, audiovolume: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSimpleAudioVolume(::core::mem::transmute_copy(&audiosessionguid), ::core::mem::transmute_copy(&streamflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *audiovolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAudioSessionControl: GetAudioSessionControl::<Impl, IMPL_OFFSET>,
            GetSimpleAudioVolume: GetSimpleAudioVolume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSessionManager2_Impl: Sized + IAudioSessionManager_Impl {
    fn GetSessionEnumerator(&mut self) -> ::windows::core::Result<IAudioSessionEnumerator>;
    fn RegisterSessionNotification(&mut self, sessionnotification: &::core::option::Option<IAudioSessionNotification>) -> ::windows::core::Result<()>;
    fn UnregisterSessionNotification(&mut self, sessionnotification: &::core::option::Option<IAudioSessionNotification>) -> ::windows::core::Result<()>;
    fn RegisterDuckNotification(&mut self, sessionid: super::super::Foundation::PWSTR, ducknotification: &::core::option::Option<IAudioVolumeDuckNotification>) -> ::windows::core::Result<()>;
    fn UnregisterDuckNotification(&mut self, ducknotification: &::core::option::Option<IAudioVolumeDuckNotification>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSessionManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionManager2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSessionManager2_Vtbl {
        unsafe extern "system" fn GetSessionEnumerator<Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSessionEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *sessionenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterSessionNotification<Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterSessionNotification(::core::mem::transmute(&sessionnotification)).into()
        }
        unsafe extern "system" fn UnregisterSessionNotification<Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionnotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterSessionNotification(::core::mem::transmute(&sessionnotification)).into()
        }
        unsafe extern "system" fn RegisterDuckNotification<Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: super::super::Foundation::PWSTR, ducknotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterDuckNotification(::core::mem::transmute_copy(&sessionid), ::core::mem::transmute(&ducknotification)).into()
        }
        unsafe extern "system" fn UnregisterDuckNotification<Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ducknotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterDuckNotification(::core::mem::transmute(&ducknotification)).into()
        }
        Self {
            base: IAudioSessionManager_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSessionEnumerator: GetSessionEnumerator::<Impl, IMPL_OFFSET>,
            RegisterSessionNotification: RegisterSessionNotification::<Impl, IMPL_OFFSET>,
            UnregisterSessionNotification: UnregisterSessionNotification::<Impl, IMPL_OFFSET>,
            RegisterDuckNotification: RegisterDuckNotification::<Impl, IMPL_OFFSET>,
            UnregisterDuckNotification: UnregisterDuckNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionManager2 as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSessionNotification_Impl: Sized {
    fn OnSessionCreated(&mut self, newsession: &::core::option::Option<IAudioSessionControl>) -> ::windows::core::Result<()>;
}
impl IAudioSessionNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSessionNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSessionNotification_Vtbl {
        unsafe extern "system" fn OnSessionCreated<Impl: IAudioSessionNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsession: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSessionCreated(::core::mem::transmute(&newsession)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnSessionCreated: OnSessionCreated::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionNotification as ::windows::core::Interface>::IID
    }
}
pub trait IAudioStateMonitor_Impl: Sized {
    fn RegisterCallback(&mut self, callback: &PAudioStateMonitorCallback, context: *const ::core::ffi::c_void) -> ::windows::core::Result<i64>;
    fn UnregisterCallback(&mut self, registration: i64);
    fn GetSoundLevel(&mut self) -> AudioStateMonitorSoundLevel;
}
impl IAudioStateMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioStateMonitor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioStateMonitor_Vtbl {
        unsafe extern "system" fn RegisterCallback<Impl: IAudioStateMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, registration: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterCallback(::core::mem::transmute_copy(&callback), ::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    *registration = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterCallback<Impl: IAudioStateMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registration: i64) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterCallback(::core::mem::transmute_copy(&registration))
        }
        unsafe extern "system" fn GetSoundLevel<Impl: IAudioStateMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> AudioStateMonitorSoundLevel {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSoundLevel()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterCallback: RegisterCallback::<Impl, IMPL_OFFSET>,
            UnregisterCallback: UnregisterCallback::<Impl, IMPL_OFFSET>,
            GetSoundLevel: GetSoundLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioStateMonitor as ::windows::core::Interface>::IID
    }
}
pub trait IAudioStreamVolume_Impl: Sized {
    fn GetChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetChannelVolume(&mut self, dwindex: u32, flevel: f32) -> ::windows::core::Result<()>;
    fn GetChannelVolume(&mut self, dwindex: u32) -> ::windows::core::Result<f32>;
    fn SetAllVolumes(&mut self, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::Result<()>;
    fn GetAllVolumes(&mut self, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::Result<()>;
}
impl IAudioStreamVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioStreamVolume_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioStreamVolume_Vtbl {
        unsafe extern "system" fn GetChannelCount<Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolume<Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelVolume(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&flevel)).into()
        }
        unsafe extern "system" fn GetChannelVolume<Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolume(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllVolumes<Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllVolumes(::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes)).into()
        }
        unsafe extern "system" fn GetAllVolumes<Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAllVolumes(::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetChannelCount: GetChannelCount::<Impl, IMPL_OFFSET>,
            SetChannelVolume: SetChannelVolume::<Impl, IMPL_OFFSET>,
            GetChannelVolume: GetChannelVolume::<Impl, IMPL_OFFSET>,
            SetAllVolumes: SetAllVolumes::<Impl, IMPL_OFFSET>,
            GetAllVolumes: GetAllVolumes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioStreamVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioSystemEffectsPropertyChangeNotificationClient_Impl: Sized {
    fn OnPropertyChanged(&mut self, r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002, key: &super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
        unsafe extern "system" fn OnPropertyChanged<Impl: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: __MIDL___MIDL_itf_mmdeviceapi_0000_0008_0002, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPropertyChanged(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&key)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnPropertyChanged: OnPropertyChanged::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffectsPropertyChangeNotificationClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioSystemEffectsPropertyStore_Impl: Sized {
    fn OpenDefaultPropertyStore(&mut self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenUserPropertyStore(&mut self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenVolatilePropertyStore(&mut self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn ResetUserPropertyStore(&mut self) -> ::windows::core::Result<()>;
    fn ResetVolatilePropertyStore(&mut self) -> ::windows::core::Result<()>;
    fn RegisterPropertyChangeNotification(&mut self, callback: &::core::option::Option<IAudioSystemEffectsPropertyChangeNotificationClient>) -> ::windows::core::Result<()>;
    fn UnregisterPropertyChangeNotification(&mut self, callback: &::core::option::Option<IAudioSystemEffectsPropertyChangeNotificationClient>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAudioSystemEffectsPropertyStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioSystemEffectsPropertyStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioSystemEffectsPropertyStore_Vtbl {
        unsafe extern "system" fn OpenDefaultPropertyStore<Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenDefaultPropertyStore(::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *propstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenUserPropertyStore<Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenUserPropertyStore(::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *propstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenVolatilePropertyStore<Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenVolatilePropertyStore(::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *propstore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetUserPropertyStore<Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetUserPropertyStore().into()
        }
        unsafe extern "system" fn ResetVolatilePropertyStore<Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetVolatilePropertyStore().into()
        }
        unsafe extern "system" fn RegisterPropertyChangeNotification<Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterPropertyChangeNotification(::core::mem::transmute(&callback)).into()
        }
        unsafe extern "system" fn UnregisterPropertyChangeNotification<Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterPropertyChangeNotification(::core::mem::transmute(&callback)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OpenDefaultPropertyStore: OpenDefaultPropertyStore::<Impl, IMPL_OFFSET>,
            OpenUserPropertyStore: OpenUserPropertyStore::<Impl, IMPL_OFFSET>,
            OpenVolatilePropertyStore: OpenVolatilePropertyStore::<Impl, IMPL_OFFSET>,
            ResetUserPropertyStore: ResetUserPropertyStore::<Impl, IMPL_OFFSET>,
            ResetVolatilePropertyStore: ResetVolatilePropertyStore::<Impl, IMPL_OFFSET>,
            RegisterPropertyChangeNotification: RegisterPropertyChangeNotification::<Impl, IMPL_OFFSET>,
            UnregisterPropertyChangeNotification: UnregisterPropertyChangeNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffectsPropertyStore as ::windows::core::Interface>::IID
    }
}
pub trait IAudioTreble_Impl: Sized + IPerChannelDbLevel_Impl {}
impl IAudioTreble_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioTreble_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioTreble_Vtbl {
        Self { base: IPerChannelDbLevel_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioTreble as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioVolumeDuckNotification_Impl: Sized {
    fn OnVolumeDuckNotification(&mut self, sessionid: super::super::Foundation::PWSTR, countcommunicationsessions: u32) -> ::windows::core::Result<()>;
    fn OnVolumeUnduckNotification(&mut self, sessionid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioVolumeDuckNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioVolumeDuckNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioVolumeDuckNotification_Vtbl {
        unsafe extern "system" fn OnVolumeDuckNotification<Impl: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: super::super::Foundation::PWSTR, countcommunicationsessions: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnVolumeDuckNotification(::core::mem::transmute_copy(&sessionid), ::core::mem::transmute_copy(&countcommunicationsessions)).into()
        }
        unsafe extern "system" fn OnVolumeUnduckNotification<Impl: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnVolumeUnduckNotification(::core::mem::transmute_copy(&sessionid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnVolumeDuckNotification: OnVolumeDuckNotification::<Impl, IMPL_OFFSET>,
            OnVolumeUnduckNotification: OnVolumeUnduckNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioVolumeDuckNotification as ::windows::core::Interface>::IID
    }
}
pub trait IAudioVolumeLevel_Impl: Sized + IPerChannelDbLevel_Impl {}
impl IAudioVolumeLevel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioVolumeLevel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioVolumeLevel_Vtbl {
        Self { base: IPerChannelDbLevel_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioVolumeLevel as ::windows::core::Interface>::IID
    }
}
pub trait IChannelAudioVolume_Impl: Sized {
    fn GetChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetChannelVolume(&mut self, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetChannelVolume(&mut self, dwindex: u32) -> ::windows::core::Result<f32>;
    fn SetAllVolumes(&mut self, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetAllVolumes(&mut self, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::Result<()>;
}
impl IChannelAudioVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChannelAudioVolume_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChannelAudioVolume_Vtbl {
        unsafe extern "system" fn GetChannelCount<Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolume<Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelVolume(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetChannelVolume<Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolume(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllVolumes<Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllVolumes(::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetAllVolumes<Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAllVolumes(::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetChannelCount: GetChannelCount::<Impl, IMPL_OFFSET>,
            SetChannelVolume: SetChannelVolume::<Impl, IMPL_OFFSET>,
            GetChannelVolume: GetChannelVolume::<Impl, IMPL_OFFSET>,
            SetAllVolumes: SetAllVolumes::<Impl, IMPL_OFFSET>,
            GetAllVolumes: GetAllVolumes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChannelAudioVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConnector_Impl: Sized {
    fn GetType(&mut self) -> ::windows::core::Result<ConnectorType>;
    fn GetDataFlow(&mut self) -> ::windows::core::Result<DataFlow>;
    fn ConnectTo(&mut self, pconnectto: &::core::option::Option<IConnector>) -> ::windows::core::Result<()>;
    fn Disconnect(&mut self) -> ::windows::core::Result<()>;
    fn IsConnected(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetConnectedTo(&mut self) -> ::windows::core::Result<IConnector>;
    fn GetConnectorIdConnectedTo(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetDeviceIdConnectedTo(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnector_Vtbl {
        unsafe extern "system" fn GetType<Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut ConnectorType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataFlow<Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflow: *mut DataFlow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataFlow() {
                ::core::result::Result::Ok(ok__) => {
                    *pflow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTo<Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectto: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConnectTo(::core::mem::transmute(&pconnectto)).into()
        }
        unsafe extern "system" fn Disconnect<Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disconnect().into()
        }
        unsafe extern "system" fn IsConnected<Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    *pbconnected = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectedTo<Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconto: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectedTo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppconto = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectorIdConnectedTo<Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrconnectorid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectorIdConnectedTo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwstrconnectorid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIdConnectedTo<Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceIdConnectedTo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwstrdeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetDataFlow: GetDataFlow::<Impl, IMPL_OFFSET>,
            ConnectTo: ConnectTo::<Impl, IMPL_OFFSET>,
            Disconnect: Disconnect::<Impl, IMPL_OFFSET>,
            IsConnected: IsConnected::<Impl, IMPL_OFFSET>,
            GetConnectedTo: GetConnectedTo::<Impl, IMPL_OFFSET>,
            GetConnectorIdConnectedTo: GetConnectorIdConnectedTo::<Impl, IMPL_OFFSET>,
            GetDeviceIdConnectedTo: GetDeviceIdConnectedTo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnector as ::windows::core::Interface>::IID
    }
}
pub trait IControlChangeNotify_Impl: Sized {
    fn OnNotify(&mut self, dwsenderprocessid: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IControlChangeNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlChangeNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlChangeNotify_Vtbl {
        unsafe extern "system" fn OnNotify<Impl: IControlChangeNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsenderprocessid: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNotify(::core::mem::transmute_copy(&dwsenderprocessid), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnNotify: OnNotify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChangeNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IControlInterface_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetIID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "Win32_Foundation")]
impl IControlInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IControlInterface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IControlInterface_Vtbl {
        unsafe extern "system" fn GetName<Impl: IControlInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIID<Impl: IControlInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIID() {
                ::core::result::Result::Ok(ok__) => {
                    *piid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetName: GetName::<Impl, IMPL_OFFSET>, GetIID: GetIID::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlInterface as ::windows::core::Interface>::IID
    }
}
pub trait IDeviceSpecificProperty_Impl: Sized {
    fn GetType(&mut self) -> ::windows::core::Result<u16>;
    fn GetValue(&mut self, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetValue(&mut self, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Get4BRange(&mut self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::Result<()>;
}
impl IDeviceSpecificProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceSpecificProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceSpecificProperty_Vtbl {
        unsafe extern "system" fn GetType<Impl: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *pvtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetValue(::core::mem::transmute_copy(&pvvalue), ::core::mem::transmute_copy(&pcbvalue)).into()
        }
        unsafe extern "system" fn SetValue<Impl: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&pvvalue), ::core::mem::transmute_copy(&cbvalue), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn Get4BRange<Impl: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Get4BRange(::core::mem::transmute_copy(&plmin), ::core::mem::transmute_copy(&plmax), ::core::mem::transmute_copy(&plstepping)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Get4BRange: Get4BRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceSpecificProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDeviceTopology_Impl: Sized {
    fn GetConnectorCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetConnector(&mut self, nindex: u32) -> ::windows::core::Result<IConnector>;
    fn GetSubunitCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetSubunit(&mut self, nindex: u32) -> ::windows::core::Result<ISubunit>;
    fn GetPartById(&mut self, nid: u32) -> ::windows::core::Result<IPart>;
    fn GetDeviceId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetSignalPath(&mut self, pipartfrom: &::core::option::Option<IPart>, pipartto: &::core::option::Option<IPart>, brejectmixedpaths: super::super::Foundation::BOOL) -> ::windows::core::Result<IPartsList>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDeviceTopology_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDeviceTopology_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDeviceTopology_Vtbl {
        unsafe extern "system" fn GetConnectorCount<Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnectorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnector<Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppconnector: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConnector(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppconnector = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubunitCount<Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubunitCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubunit<Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppsubunit: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubunit(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubunit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartById<Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nid: u32, pppart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartById(::core::mem::transmute_copy(&nid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceId<Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwstrdeviceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalPath<Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipartfrom: ::windows::core::RawPtr, pipartto: ::windows::core::RawPtr, brejectmixedpaths: super::super::Foundation::BOOL, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignalPath(::core::mem::transmute(&pipartfrom), ::core::mem::transmute(&pipartto), ::core::mem::transmute_copy(&brejectmixedpaths)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppparts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetConnectorCount: GetConnectorCount::<Impl, IMPL_OFFSET>,
            GetConnector: GetConnector::<Impl, IMPL_OFFSET>,
            GetSubunitCount: GetSubunitCount::<Impl, IMPL_OFFSET>,
            GetSubunit: GetSubunit::<Impl, IMPL_OFFSET>,
            GetPartById: GetPartById::<Impl, IMPL_OFFSET>,
            GetDeviceId: GetDeviceId::<Impl, IMPL_OFFSET>,
            GetSignalPath: GetSignalPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceTopology as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMMDevice_Impl: Sized {
    fn Activate(&mut self, iid: *const ::windows::core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OpenPropertyStore(&mut self, stgmaccess: super::super::System::Com::StructuredStorage::STGM) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetState(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMMDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMDevice_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMMDevice_Vtbl {
        unsafe extern "system" fn Activate<Impl: IMMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute_copy(&pactivationparams), ::core::mem::transmute_copy(&ppinterface)).into()
        }
        unsafe extern "system" fn OpenPropertyStore<Impl: IMMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: super::super::System::Com::StructuredStorage::STGM, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPropertyStore(::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Impl: IMMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppstrid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: IMMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Activate: Activate::<Impl, IMPL_OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Impl, IMPL_OFFSET>,
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetState: GetState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMMDeviceActivator_Impl: Sized {
    fn Activate(&mut self, iid: *const ::windows::core::GUID, pdevice: &::core::option::Option<IMMDevice>, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IMMDeviceActivator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMDeviceActivator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMMDeviceActivator_Vtbl {
        unsafe extern "system" fn Activate<Impl: IMMDeviceActivator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, pdevice: ::windows::core::RawPtr, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&iid), ::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pactivationparams), ::core::mem::transmute_copy(&ppinterface)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Activate: Activate::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMDeviceActivator as ::windows::core::Interface>::IID
    }
}
pub trait IMMDeviceCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn Item(&mut self, ndevice: u32) -> ::windows::core::Result<IMMDevice>;
}
impl IMMDeviceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMDeviceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMMDeviceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IMMDeviceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdevices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcdevices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IMMDeviceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ndevice: u32, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&ndevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetCount: GetCount::<Impl, IMPL_OFFSET>, Item: Item::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMDeviceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMMDeviceEnumerator_Impl: Sized {
    fn EnumAudioEndpoints(&mut self, dataflow: EDataFlow, dwstatemask: u32) -> ::windows::core::Result<IMMDeviceCollection>;
    fn GetDefaultAudioEndpoint(&mut self, dataflow: EDataFlow, role: ERole) -> ::windows::core::Result<IMMDevice>;
    fn GetDevice(&mut self, pwstrid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IMMDevice>;
    fn RegisterEndpointNotificationCallback(&mut self, pclient: &::core::option::Option<IMMNotificationClient>) -> ::windows::core::Result<()>;
    fn UnregisterEndpointNotificationCallback(&mut self, pclient: &::core::option::Option<IMMNotificationClient>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMMDeviceEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMDeviceEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMMDeviceEnumerator_Vtbl {
        unsafe extern "system" fn EnumAudioEndpoints<Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, dwstatemask: u32, ppdevices: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAudioEndpoints(::core::mem::transmute_copy(&dataflow), ::core::mem::transmute_copy(&dwstatemask)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevices = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAudioEndpoint<Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, role: ERole, ppendpoint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultAudioEndpoint(::core::mem::transmute_copy(&dataflow), ::core::mem::transmute_copy(&role)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppendpoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrid: super::super::Foundation::PWSTR, ppdevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(::core::mem::transmute_copy(&pwstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEndpointNotificationCallback<Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclient: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterEndpointNotificationCallback(::core::mem::transmute(&pclient)).into()
        }
        unsafe extern "system" fn UnregisterEndpointNotificationCallback<Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclient: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterEndpointNotificationCallback(::core::mem::transmute(&pclient)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            EnumAudioEndpoints: EnumAudioEndpoints::<Impl, IMPL_OFFSET>,
            GetDefaultAudioEndpoint: GetDefaultAudioEndpoint::<Impl, IMPL_OFFSET>,
            GetDevice: GetDevice::<Impl, IMPL_OFFSET>,
            RegisterEndpointNotificationCallback: RegisterEndpointNotificationCallback::<Impl, IMPL_OFFSET>,
            UnregisterEndpointNotificationCallback: UnregisterEndpointNotificationCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMDeviceEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IMMEndpoint_Impl: Sized {
    fn GetDataFlow(&mut self) -> ::windows::core::Result<EDataFlow>;
}
impl IMMEndpoint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMEndpoint_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMMEndpoint_Vtbl {
        unsafe extern "system" fn GetDataFlow<Impl: IMMEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataflow: *mut EDataFlow) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataFlow() {
                ::core::result::Result::Ok(ok__) => {
                    *pdataflow = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDataFlow: GetDataFlow::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMEndpoint as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMMNotificationClient_Impl: Sized {
    fn OnDeviceStateChanged(&mut self, pwstrdeviceid: super::super::Foundation::PWSTR, dwnewstate: u32) -> ::windows::core::Result<()>;
    fn OnDeviceAdded(&mut self, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnDeviceRemoved(&mut self, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnDefaultDeviceChanged(&mut self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnPropertyValueChanged(&mut self, pwstrdeviceid: super::super::Foundation::PWSTR, key: &super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMMNotificationClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMMNotificationClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMMNotificationClient_Vtbl {
        unsafe extern "system" fn OnDeviceStateChanged<Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: super::super::Foundation::PWSTR, dwnewstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDeviceStateChanged(::core::mem::transmute_copy(&pwstrdeviceid), ::core::mem::transmute_copy(&dwnewstate)).into()
        }
        unsafe extern "system" fn OnDeviceAdded<Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDeviceAdded(::core::mem::transmute_copy(&pwstrdeviceid)).into()
        }
        unsafe extern "system" fn OnDeviceRemoved<Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDeviceRemoved(::core::mem::transmute_copy(&pwstrdeviceid)).into()
        }
        unsafe extern "system" fn OnDefaultDeviceChanged<Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDefaultDeviceChanged(::core::mem::transmute_copy(&flow), ::core::mem::transmute_copy(&role), ::core::mem::transmute_copy(&pwstrdefaultdeviceid)).into()
        }
        unsafe extern "system" fn OnPropertyValueChanged<Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: super::super::Foundation::PWSTR, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnPropertyValueChanged(::core::mem::transmute_copy(&pwstrdeviceid), ::core::mem::transmute_copy(&key)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnDeviceStateChanged: OnDeviceStateChanged::<Impl, IMPL_OFFSET>,
            OnDeviceAdded: OnDeviceAdded::<Impl, IMPL_OFFSET>,
            OnDeviceRemoved: OnDeviceRemoved::<Impl, IMPL_OFFSET>,
            OnDefaultDeviceChanged: OnDefaultDeviceChanged::<Impl, IMPL_OFFSET>,
            OnPropertyValueChanged: OnPropertyValueChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMNotificationClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMessageFilter_Impl: Sized {
    fn HandleInComingCall(&mut self, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32;
    fn RetryRejectedCall(&mut self, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32;
    fn MessagePending(&mut self, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32;
}
#[cfg(feature = "Win32_System_Com")]
impl IMessageFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageFilter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageFilter_Vtbl {
        unsafe extern "system" fn HandleInComingCall<Impl: IMessageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HandleInComingCall(::core::mem::transmute_copy(&dwcalltype), ::core::mem::transmute_copy(&htaskcaller), ::core::mem::transmute_copy(&dwtickcount), ::core::mem::transmute_copy(&lpinterfaceinfo))
        }
        unsafe extern "system" fn RetryRejectedCall<Impl: IMessageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetryRejectedCall(::core::mem::transmute_copy(&htaskcallee), ::core::mem::transmute_copy(&dwtickcount), ::core::mem::transmute_copy(&dwrejecttype))
        }
        unsafe extern "system" fn MessagePending<Impl: IMessageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MessagePending(::core::mem::transmute_copy(&htaskcallee), ::core::mem::transmute_copy(&dwtickcount), ::core::mem::transmute_copy(&dwpendingtype))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            HandleInComingCall: HandleInComingCall::<Impl, IMPL_OFFSET>,
            RetryRejectedCall: RetryRejectedCall::<Impl, IMPL_OFFSET>,
            MessagePending: MessagePending::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPart_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetLocalId(&mut self) -> ::windows::core::Result<u32>;
    fn GetGlobalId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetPartType(&mut self) -> ::windows::core::Result<PartType>;
    fn GetSubType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetControlInterfaceCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetControlInterface(&mut self, nindex: u32) -> ::windows::core::Result<IControlInterface>;
    fn EnumPartsIncoming(&mut self) -> ::windows::core::Result<IPartsList>;
    fn EnumPartsOutgoing(&mut self) -> ::windows::core::Result<IPartsList>;
    fn GetTopologyObject(&mut self) -> ::windows::core::Result<IDeviceTopology>;
    fn Activate(&mut self, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RegisterControlChangeCallback(&mut self, riid: *const ::windows::core::GUID, pnotify: &::core::option::Option<IControlChangeNotify>) -> ::windows::core::Result<()>;
    fn UnregisterControlChangeCallback(&mut self, pnotify: &::core::option::Option<IControlChangeNotify>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPart_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPart_Vtbl {
        unsafe extern "system" fn GetName<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalId<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalId() {
                ::core::result::Result::Ok(ok__) => {
                    *pnid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalId<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrglobalid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwstrglobalid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartType<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparttype: *mut PartType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartType() {
                ::core::result::Result::Ok(ok__) => {
                    *pparttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubType<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubType() {
                ::core::result::Result::Ok(ok__) => {
                    *psubtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetControlInterfaceCount<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControlInterfaceCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetControlInterface<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppinterfacedesc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetControlInterface(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinterfacedesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPartsIncoming<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPartsIncoming() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPartsOutgoing<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparts: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumPartsOutgoing() {
                ::core::result::Result::Ok(ok__) => {
                    *ppparts = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTopologyObject<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptopology: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTopologyObject() {
                ::core::result::Result::Ok(ok__) => {
                    *pptopology = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&refiid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn RegisterControlChangeCallback<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterControlChangeCallback(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&pnotify)).into()
        }
        unsafe extern "system" fn UnregisterControlChangeCallback<Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterControlChangeCallback(::core::mem::transmute(&pnotify)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetLocalId: GetLocalId::<Impl, IMPL_OFFSET>,
            GetGlobalId: GetGlobalId::<Impl, IMPL_OFFSET>,
            GetPartType: GetPartType::<Impl, IMPL_OFFSET>,
            GetSubType: GetSubType::<Impl, IMPL_OFFSET>,
            GetControlInterfaceCount: GetControlInterfaceCount::<Impl, IMPL_OFFSET>,
            GetControlInterface: GetControlInterface::<Impl, IMPL_OFFSET>,
            EnumPartsIncoming: EnumPartsIncoming::<Impl, IMPL_OFFSET>,
            EnumPartsOutgoing: EnumPartsOutgoing::<Impl, IMPL_OFFSET>,
            GetTopologyObject: GetTopologyObject::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            RegisterControlChangeCallback: RegisterControlChangeCallback::<Impl, IMPL_OFFSET>,
            UnregisterControlChangeCallback: UnregisterControlChangeCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPart as ::windows::core::Interface>::IID
    }
}
pub trait IPartsList_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetPart(&mut self, nindex: u32) -> ::windows::core::Result<IPart>;
}
impl IPartsList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPartsList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPartsList_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPartsList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPart<Impl: IPartsList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pppart: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPart(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pppart = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetPart: GetPart::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPartsList as ::windows::core::Interface>::IID
    }
}
pub trait IPerChannelDbLevel_Impl: Sized {
    fn GetChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetLevelRange(&mut self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()>;
    fn GetLevel(&mut self, nchannel: u32) -> ::windows::core::Result<f32>;
    fn SetLevel(&mut self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetLevelUniform(&mut self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetLevelAllChannels(&mut self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IPerChannelDbLevel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPerChannelDbLevel_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPerChannelDbLevel_Vtbl {
        unsafe extern "system" fn GetChannelCount<Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcchannels = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevelRange<Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLevelRange(::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&pfminleveldb), ::core::mem::transmute_copy(&pfmaxleveldb), ::core::mem::transmute_copy(&pfstepping)).into()
        }
        unsafe extern "system" fn GetLevel<Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLevel(::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfleveldb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevel<Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLevel(::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetLevelUniform<Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLevelUniform(::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetLevelAllChannels<Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLevelAllChannels(::core::mem::transmute_copy(&alevelsdb), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetChannelCount: GetChannelCount::<Impl, IMPL_OFFSET>,
            GetLevelRange: GetLevelRange::<Impl, IMPL_OFFSET>,
            GetLevel: GetLevel::<Impl, IMPL_OFFSET>,
            SetLevel: SetLevel::<Impl, IMPL_OFFSET>,
            SetLevelUniform: SetLevelUniform::<Impl, IMPL_OFFSET>,
            SetLevelAllChannels: SetLevelAllChannels::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerChannelDbLevel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimpleAudioVolume_Impl: Sized {
    fn SetMasterVolume(&mut self, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMasterVolume(&mut self) -> ::windows::core::Result<f32>;
    fn SetMute(&mut self, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMute(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISimpleAudioVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISimpleAudioVolume_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISimpleAudioVolume_Vtbl {
        unsafe extern "system" fn SetMasterVolume<Impl: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMasterVolume(::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetMasterVolume<Impl: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMasterVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *pflevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMute<Impl: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMute(::core::mem::transmute_copy(&bmute), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetMute<Impl: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMute() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmute = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMasterVolume: SetMasterVolume::<Impl, IMPL_OFFSET>,
            GetMasterVolume: GetMasterVolume::<Impl, IMPL_OFFSET>,
            SetMute: SetMute::<Impl, IMPL_OFFSET>,
            GetMute: GetMute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleAudioVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpatialAudioClient_Impl: Sized {
    fn GetStaticObjectPosition(&mut self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::Result<()>;
    fn GetNativeStaticObjectTypeMask(&mut self) -> ::windows::core::Result<AudioObjectType>;
    fn GetMaxDynamicObjectCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetSupportedAudioObjectFormatEnumerator(&mut self) -> ::windows::core::Result<IAudioFormatEnumerator>;
    fn GetMaxFrameCount(&mut self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32>;
    fn IsAudioObjectFormatSupported(&mut self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn IsSpatialAudioStreamAvailable(&mut self, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn ActivateSpatialAudioStream(&mut self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows::core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpatialAudioClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioClient_Vtbl {
        unsafe extern "system" fn GetStaticObjectPosition<Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStaticObjectPosition(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)).into()
        }
        unsafe extern "system" fn GetNativeStaticObjectTypeMask<Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: *mut AudioObjectType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNativeStaticObjectTypeMask() {
                ::core::result::Result::Ok(ok__) => {
                    *mask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxDynamicObjectCount<Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxDynamicObjectCount() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedAudioObjectFormatEnumerator<Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedAudioObjectFormatEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    *enumerator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxFrameCount<Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxFrameCount(::core::mem::transmute_copy(&objectformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *framecountperbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAudioObjectFormatSupported<Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsAudioObjectFormatSupported(::core::mem::transmute_copy(&objectformat)).into()
        }
        unsafe extern "system" fn IsSpatialAudioStreamAvailable<Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSpatialAudioStreamAvailable(::core::mem::transmute_copy(&streamuuid), ::core::mem::transmute_copy(&auxiliaryinfo)).into()
        }
        unsafe extern "system" fn ActivateSpatialAudioStream<Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows::core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateSpatialAudioStream(::core::mem::transmute_copy(&activationparams), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&stream)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetStaticObjectPosition: GetStaticObjectPosition::<Impl, IMPL_OFFSET>,
            GetNativeStaticObjectTypeMask: GetNativeStaticObjectTypeMask::<Impl, IMPL_OFFSET>,
            GetMaxDynamicObjectCount: GetMaxDynamicObjectCount::<Impl, IMPL_OFFSET>,
            GetSupportedAudioObjectFormatEnumerator: GetSupportedAudioObjectFormatEnumerator::<Impl, IMPL_OFFSET>,
            GetMaxFrameCount: GetMaxFrameCount::<Impl, IMPL_OFFSET>,
            IsAudioObjectFormatSupported: IsAudioObjectFormatSupported::<Impl, IMPL_OFFSET>,
            IsSpatialAudioStreamAvailable: IsSpatialAudioStreamAvailable::<Impl, IMPL_OFFSET>,
            ActivateSpatialAudioStream: ActivateSpatialAudioStream::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpatialAudioClient2_Impl: Sized + ISpatialAudioClient_Impl {
    fn IsOffloadCapable(&mut self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetMaxFrameCountForCategory(&mut self, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpatialAudioClient2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioClient2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioClient2_Vtbl {
        unsafe extern "system" fn IsOffloadCapable<Impl: ISpatialAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsOffloadCapable(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    *isoffloadcapable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxFrameCountForCategory<Impl: ISpatialAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxFrameCountForCategory(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&offloadenabled), ::core::mem::transmute_copy(&objectformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *framecountperbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpatialAudioClient_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsOffloadCapable: IsOffloadCapable::<Impl, IMPL_OFFSET>,
            GetMaxFrameCountForCategory: GetMaxFrameCountForCategory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioClient2 as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataClient_Impl: Sized {
    fn ActivateSpatialAudioMetadataItems(&mut self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::core::option::Option<ISpatialAudioMetadataItemsBuffer>, metadataitems: *mut ::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()>;
    fn GetSpatialAudioMetadataItemsBufferLength(&mut self, maxitemcount: u16) -> ::windows::core::Result<u32>;
    fn ActivateSpatialAudioMetadataWriter(&mut self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> ::windows::core::Result<ISpatialAudioMetadataWriter>;
    fn ActivateSpatialAudioMetadataCopier(&mut self) -> ::windows::core::Result<ISpatialAudioMetadataCopier>;
    fn ActivateSpatialAudioMetadataReader(&mut self) -> ::windows::core::Result<ISpatialAudioMetadataReader>;
}
impl ISpatialAudioMetadataClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataClient_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioMetadataClient_Vtbl {
        unsafe extern "system" fn ActivateSpatialAudioMetadataItems<Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::windows::core::RawPtr, metadataitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ActivateSpatialAudioMetadataItems(::core::mem::transmute_copy(&maxitemcount), ::core::mem::transmute_copy(&framecount), ::core::mem::transmute_copy(&metadataitemsbuffer), ::core::mem::transmute_copy(&metadataitems)).into()
        }
        unsafe extern "system" fn GetSpatialAudioMetadataItemsBufferLength<Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxitemcount: u16, bufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpatialAudioMetadataItemsBufferLength(::core::mem::transmute_copy(&maxitemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *bufferlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataWriter<Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioMetadataWriter(::core::mem::transmute_copy(&overflowmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *metadatawriter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataCopier<Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadatacopier: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioMetadataCopier() {
                ::core::result::Result::Ok(ok__) => {
                    *metadatacopier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataReader<Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadatareader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioMetadataReader() {
                ::core::result::Result::Ok(ok__) => {
                    *metadatareader = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ActivateSpatialAudioMetadataItems: ActivateSpatialAudioMetadataItems::<Impl, IMPL_OFFSET>,
            GetSpatialAudioMetadataItemsBufferLength: GetSpatialAudioMetadataItemsBufferLength::<Impl, IMPL_OFFSET>,
            ActivateSpatialAudioMetadataWriter: ActivateSpatialAudioMetadataWriter::<Impl, IMPL_OFFSET>,
            ActivateSpatialAudioMetadataCopier: ActivateSpatialAudioMetadataCopier::<Impl, IMPL_OFFSET>,
            ActivateSpatialAudioMetadataReader: ActivateSpatialAudioMetadataReader::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataClient as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataCopier_Impl: Sized {
    fn Open(&mut self, metadataitems: &::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()>;
    fn CopyMetadataForFrames(&mut self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: &::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<u16>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl ISpatialAudioMetadataCopier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataCopier_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioMetadataCopier_Vtbl {
        unsafe extern "system" fn Open<Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute(&metadataitems)).into()
        }
        unsafe extern "system" fn CopyMetadataForFrames<Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: ::windows::core::RawPtr, itemscopied: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyMetadataForFrames(::core::mem::transmute_copy(&copyframecount), ::core::mem::transmute_copy(&copymode), ::core::mem::transmute(&dstmetadataitems)) {
                ::core::result::Result::Ok(ok__) => {
                    *itemscopied = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            CopyMetadataForFrames: CopyMetadataForFrames::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataCopier as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataItems_Impl: Sized {
    fn GetFrameCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetItemCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetMaxItemCount(&mut self) -> ::windows::core::Result<u16>;
    fn GetMaxValueBufferLength(&mut self) -> ::windows::core::Result<u32>;
    fn GetInfo(&mut self) -> ::windows::core::Result<SpatialAudioMetadataItemsInfo>;
}
impl ISpatialAudioMetadataItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataItems_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioMetadataItems_Vtbl {
        unsafe extern "system" fn GetFrameCount<Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    *framecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCount<Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *itemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxItemCount<Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxitemcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    *maxitemcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueBufferLength<Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxvaluebufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxValueBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    *maxvaluebufferlength = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, info: *mut SpatialAudioMetadataItemsInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *info = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFrameCount: GetFrameCount::<Impl, IMPL_OFFSET>,
            GetItemCount: GetItemCount::<Impl, IMPL_OFFSET>,
            GetMaxItemCount: GetMaxItemCount::<Impl, IMPL_OFFSET>,
            GetMaxValueBufferLength: GetMaxValueBufferLength::<Impl, IMPL_OFFSET>,
            GetInfo: GetInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataItems as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataItemsBuffer_Impl: Sized {
    fn AttachToBuffer(&mut self, buffer: *mut u8, bufferlength: u32) -> ::windows::core::Result<()>;
    fn AttachToPopulatedBuffer(&mut self, buffer: *mut u8, bufferlength: u32) -> ::windows::core::Result<()>;
    fn DetachBuffer(&mut self) -> ::windows::core::Result<()>;
}
impl ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataItemsBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioMetadataItemsBuffer_Vtbl {
        unsafe extern "system" fn AttachToBuffer<Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachToBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)).into()
        }
        unsafe extern "system" fn AttachToPopulatedBuffer<Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AttachToPopulatedBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)).into()
        }
        unsafe extern "system" fn DetachBuffer<Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DetachBuffer().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AttachToBuffer: AttachToBuffer::<Impl, IMPL_OFFSET>,
            AttachToPopulatedBuffer: AttachToPopulatedBuffer::<Impl, IMPL_OFFSET>,
            DetachBuffer: DetachBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataItemsBuffer as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataReader_Impl: Sized {
    fn Open(&mut self, metadataitems: &::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()>;
    fn ReadNextItem(&mut self, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::Result<()>;
    fn ReadNextItemCommand(&mut self, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl ISpatialAudioMetadataReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioMetadataReader_Vtbl {
        unsafe extern "system" fn Open<Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute(&metadataitems)).into()
        }
        unsafe extern "system" fn ReadNextItem<Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadNextItem(::core::mem::transmute_copy(&commandcount), ::core::mem::transmute_copy(&frameoffset)).into()
        }
        unsafe extern "system" fn ReadNextItemCommand<Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadNextItemCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), ::core::mem::transmute_copy(&maxvaluebufferlength), ::core::mem::transmute_copy(&valuebufferlength)).into()
        }
        unsafe extern "system" fn Close<Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            ReadNextItem: ReadNextItem::<Impl, IMPL_OFFSET>,
            ReadNextItemCommand: ReadNextItemCommand::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataReader as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataWriter_Impl: Sized {
    fn Open(&mut self, metadataitems: &::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()>;
    fn WriteNextItem(&mut self, frameoffset: u16) -> ::windows::core::Result<()>;
    fn WriteNextItemCommand(&mut self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl ISpatialAudioMetadataWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioMetadataWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioMetadataWriter_Vtbl {
        unsafe extern "system" fn Open<Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open(::core::mem::transmute(&metadataitems)).into()
        }
        unsafe extern "system" fn WriteNextItem<Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameoffset: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteNextItem(::core::mem::transmute_copy(&frameoffset)).into()
        }
        unsafe extern "system" fn WriteNextItemCommand<Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteNextItemCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), ::core::mem::transmute_copy(&valuebufferlength)).into()
        }
        unsafe extern "system" fn Close<Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            WriteNextItem: WriteNextItem::<Impl, IMPL_OFFSET>,
            WriteNextItemCommand: WriteNextItemCommand::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObject_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn SetPosition(&mut self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()>;
    fn SetVolume(&mut self, volume: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObject_Vtbl {
        unsafe extern "system" fn SetPosition<Impl: ISpatialAudioObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)).into()
        }
        unsafe extern "system" fn SetVolume<Impl: ISpatialAudioObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&volume)).into()
        }
        Self {
            base: ISpatialAudioObjectBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectBase_Impl: Sized {
    fn GetBuffer(&mut self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()>;
    fn SetEndOfStream(&mut self, framecount: u32) -> ::windows::core::Result<()>;
    fn IsActive(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetAudioObjectType(&mut self) -> ::windows::core::Result<AudioObjectType>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObjectBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObjectBase_Vtbl {
        unsafe extern "system" fn GetBuffer<Impl: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)).into()
        }
        unsafe extern "system" fn SetEndOfStream<Impl: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndOfStream(::core::mem::transmute_copy(&framecount)).into()
        }
        unsafe extern "system" fn IsActive<Impl: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    *isactive = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioObjectType<Impl: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    *audioobjecttype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            SetEndOfStream: SetEndOfStream::<Impl, IMPL_OFFSET>,
            IsActive: IsActive::<Impl, IMPL_OFFSET>,
            GetAudioObjectType: GetAudioObjectType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectForHrtf_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn SetPosition(&mut self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()>;
    fn SetGain(&mut self, gain: f32) -> ::windows::core::Result<()>;
    fn SetOrientation(&mut self, orientation: *const *const f32) -> ::windows::core::Result<()>;
    fn SetEnvironment(&mut self, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::Result<()>;
    fn SetDistanceDecay(&mut self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::Result<()>;
    fn SetDirectivity(&mut self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObjectForHrtf_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectForHrtf_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObjectForHrtf_Vtbl {
        unsafe extern "system" fn SetPosition<Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)).into()
        }
        unsafe extern "system" fn SetGain<Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGain(::core::mem::transmute_copy(&gain)).into()
        }
        unsafe extern "system" fn SetOrientation<Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: *const *const f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(::core::mem::transmute_copy(&orientation)).into()
        }
        unsafe extern "system" fn SetEnvironment<Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnvironment(::core::mem::transmute_copy(&environment)).into()
        }
        unsafe extern "system" fn SetDistanceDecay<Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDistanceDecay(::core::mem::transmute_copy(&distancedecay)).into()
        }
        unsafe extern "system" fn SetDirectivity<Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDirectivity(::core::mem::transmute_copy(&directivity)).into()
        }
        Self {
            base: ISpatialAudioObjectBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            SetGain: SetGain::<Impl, IMPL_OFFSET>,
            SetOrientation: SetOrientation::<Impl, IMPL_OFFSET>,
            SetEnvironment: SetEnvironment::<Impl, IMPL_OFFSET>,
            SetDistanceDecay: SetDistanceDecay::<Impl, IMPL_OFFSET>,
            SetDirectivity: SetDirectivity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForHrtf as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectForMetadataCommands_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn WriteNextMetadataCommand(&mut self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectForMetadataCommands_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObjectForMetadataCommands_Vtbl {
        unsafe extern "system" fn WriteNextMetadataCommand<Impl: ISpatialAudioObjectForMetadataCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteNextMetadataCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), ::core::mem::transmute_copy(&valuebufferlength)).into()
        }
        Self {
            base: ISpatialAudioObjectBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WriteNextMetadataCommand: WriteNextMetadataCommand::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataCommands as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectForMetadataItems_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn GetSpatialAudioMetadataItems(&mut self) -> ::windows::core::Result<ISpatialAudioMetadataItems>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObjectForMetadataItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectForMetadataItems_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObjectForMetadataItems_Vtbl {
        unsafe extern "system" fn GetSpatialAudioMetadataItems<Impl: ISpatialAudioObjectForMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpatialAudioMetadataItems() {
                ::core::result::Result::Ok(ok__) => {
                    *metadataitems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpatialAudioObjectBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSpatialAudioMetadataItems: GetSpatialAudioMetadataItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataItems as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStream_Impl: Sized + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObject(&mut self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObject>;
}
impl ISpatialAudioObjectRenderStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStream_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObjectRenderStream_Vtbl {
        unsafe extern "system" fn ActivateSpatialAudioObject<Impl: ISpatialAudioObjectRenderStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioObject(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *audioobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActivateSpatialAudioObject: ActivateSpatialAudioObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStream as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamBase_Impl: Sized {
    fn GetAvailableDynamicObjectCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetService(&mut self, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Start(&mut self) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn BeginUpdatingAudioObjects(&mut self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()>;
    fn EndUpdatingAudioObjects(&mut self) -> ::windows::core::Result<()>;
}
impl ISpatialAudioObjectRenderStreamBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObjectRenderStreamBase_Vtbl {
        unsafe extern "system" fn GetAvailableDynamicObjectCount<Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableDynamicObjectCount() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetService(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&service)).into()
        }
        unsafe extern "system" fn Start<Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start().into()
        }
        unsafe extern "system" fn Stop<Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Reset<Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn BeginUpdatingAudioObjects<Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginUpdatingAudioObjects(::core::mem::transmute_copy(&availabledynamicobjectcount), ::core::mem::transmute_copy(&framecountperbuffer)).into()
        }
        unsafe extern "system" fn EndUpdatingAudioObjects<Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndUpdatingAudioObjects().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAvailableDynamicObjectCount: GetAvailableDynamicObjectCount::<Impl, IMPL_OFFSET>,
            GetService: GetService::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            BeginUpdatingAudioObjects: BeginUpdatingAudioObjects::<Impl, IMPL_OFFSET>,
            EndUpdatingAudioObjects: EndUpdatingAudioObjects::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamBase as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamForHrtf_Impl: Sized + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForHrtf(&mut self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForHrtf>;
}
impl ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStreamForHrtf_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
        unsafe extern "system" fn ActivateSpatialAudioObjectForHrtf<Impl: ISpatialAudioObjectRenderStreamForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioObjectForHrtf(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *audioobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActivateSpatialAudioObjectForHrtf: ActivateSpatialAudioObjectForHrtf::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamForHrtf as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamForMetadata_Impl: Sized + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForMetadataCommands(&mut self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataCommands>;
    fn ActivateSpatialAudioObjectForMetadataItems(&mut self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataItems>;
}
impl ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStreamForMetadata_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataCommands<Impl: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioObjectForMetadataCommands(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *audioobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataItems<Impl: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateSpatialAudioObjectForMetadataItems(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *audioobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ActivateSpatialAudioObjectForMetadataCommands: ActivateSpatialAudioObjectForMetadataCommands::<Impl, IMPL_OFFSET>,
            ActivateSpatialAudioObjectForMetadataItems: ActivateSpatialAudioObjectForMetadataItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamForMetadata as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamNotify_Impl: Sized {
    fn OnAvailableDynamicObjectCountChange(&mut self, sender: &::core::option::Option<ISpatialAudioObjectRenderStreamBase>, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::Result<()>;
}
impl ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISpatialAudioObjectRenderStreamNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISpatialAudioObjectRenderStreamNotify_Vtbl {
        unsafe extern "system" fn OnAvailableDynamicObjectCountChange<Impl: ISpatialAudioObjectRenderStreamNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: ::windows::core::RawPtr, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAvailableDynamicObjectCountChange(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&hnscompliancedeadlinetime), ::core::mem::transmute_copy(&availabledynamicobjectcountchange)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnAvailableDynamicObjectCountChange: OnAvailableDynamicObjectCountChange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamNotify as ::windows::core::Interface>::IID
    }
}
pub trait ISubunit_Impl: Sized {}
impl ISubunit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISubunit_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISubunit_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISubunit as ::windows::core::Interface>::IID
    }
}
