pub trait IActivateAudioInterfaceAsyncOperation_Impl: Sized {
    fn GetActivateResult(&self, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IActivateAudioInterfaceAsyncOperation {}
impl IActivateAudioInterfaceAsyncOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivateAudioInterfaceAsyncOperation_Impl, const OFFSET: isize>() -> IActivateAudioInterfaceAsyncOperation_Vtbl {
        unsafe extern "system" fn GetActivateResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivateAudioInterfaceAsyncOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activateresult: *mut ::windows::core::HRESULT, activatedinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetActivateResult(::core::mem::transmute_copy(&activateresult), ::core::mem::transmute_copy(&activatedinterface)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetActivateResult: GetActivateResult::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceAsyncOperation as ::windows::core::Interface>::IID
    }
}
pub trait IActivateAudioInterfaceCompletionHandler_Impl: Sized {
    fn ActivateCompleted(&self, activateoperation: &::core::option::Option<IActivateAudioInterfaceAsyncOperation>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IActivateAudioInterfaceCompletionHandler {}
impl IActivateAudioInterfaceCompletionHandler_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivateAudioInterfaceCompletionHandler_Impl, const OFFSET: isize>() -> IActivateAudioInterfaceCompletionHandler_Vtbl {
        unsafe extern "system" fn ActivateCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivateAudioInterfaceCompletionHandler_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activateoperation: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateCompleted(::core::mem::transmute(&activateoperation)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ActivateCompleted: ActivateCompleted::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivateAudioInterfaceCompletionHandler as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioAmbisonicsControl_Impl: Sized {
    fn SetData(&self, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::core::Result<()>;
    fn SetHeadTracking(&self, benableheadtracking: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetHeadTracking(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioAmbisonicsControl {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioAmbisonicsControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: isize>() -> IAudioAmbisonicsControl_Vtbl {
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute_copy(&pambisonicsparams), ::core::mem::transmute_copy(&cbambisonicsparams)).into()
        }
        unsafe extern "system" fn SetHeadTracking<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benableheadtracking: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHeadTracking(::core::mem::transmute_copy(&benableheadtracking)).into()
        }
        unsafe extern "system" fn GetHeadTracking<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenableheadtracking: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHeadTracking() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenableheadtracking, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRotation(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&w)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetData: SetData::<Identity, Impl, OFFSET>,
            SetHeadTracking: SetHeadTracking::<Identity, Impl, OFFSET>,
            GetHeadTracking: GetHeadTracking::<Identity, Impl, OFFSET>,
            SetRotation: SetRotation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioAmbisonicsControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioAutoGainControl_Impl: Sized {
    fn GetEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioAutoGainControl {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioAutoGainControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioAutoGainControl_Impl, const OFFSET: isize>() -> IAudioAutoGainControl_Vtbl {
        unsafe extern "system" fn GetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioAutoGainControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&benable), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetEnabled: GetEnabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioAutoGainControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioBass_Impl: Sized + IPerChannelDbLevel_Impl {}
impl ::windows::core::RuntimeName for IAudioBass {}
impl IAudioBass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioBass_Impl, const OFFSET: isize>() -> IAudioBass_Vtbl {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioBass as ::windows::core::Interface>::IID || iid == &<IPerChannelDbLevel as ::windows::core::Interface>::IID
    }
}
pub trait IAudioCaptureClient_Impl: Sized {
    fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::Result<()>;
    fn ReleaseBuffer(&self, numframesread: u32) -> ::windows::core::Result<()>;
    fn GetNextPacketSize(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IAudioCaptureClient {}
impl IAudioCaptureClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>() -> IAudioCaptureClient_Vtbl {
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBuffer(::core::mem::transmute_copy(&ppdata), ::core::mem::transmute_copy(&pnumframestoread), ::core::mem::transmute_copy(&pdwflags), ::core::mem::transmute_copy(&pu64deviceposition), ::core::mem::transmute_copy(&pu64qpcposition)).into()
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframesread: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseBuffer(::core::mem::transmute_copy(&numframesread)).into()
        }
        unsafe extern "system" fn GetNextPacketSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextPacketSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumframesinnextpacket, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Identity, Impl, OFFSET>,
            GetNextPacketSize: GetNextPacketSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioCaptureClient as ::windows::core::Interface>::IID
    }
}
pub trait IAudioChannelConfig_Impl: Sized {
    fn SetChannelConfig(&self, dwconfig: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetChannelConfig(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IAudioChannelConfig {}
impl IAudioChannelConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioChannelConfig_Impl, const OFFSET: isize>() -> IAudioChannelConfig_Vtbl {
        unsafe extern "system" fn SetChannelConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconfig: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChannelConfig(::core::mem::transmute_copy(&dwconfig), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetChannelConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioChannelConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwconfig: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelConfig() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconfig, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetChannelConfig: SetChannelConfig::<Identity, Impl, OFFSET>,
            GetChannelConfig: GetChannelConfig::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioChannelConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioClient_Impl: Sized {
    fn Initialize(&self, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetBufferSize(&self) -> ::windows::core::Result<u32>;
    fn GetStreamLatency(&self) -> ::windows::core::Result<i64>;
    fn GetCurrentPadding(&self) -> ::windows::core::Result<u32>;
    fn IsFormatSupported(&self, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT;
    fn GetMixFormat(&self) -> ::windows::core::Result<*mut WAVEFORMATEX>;
    fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn SetEventHandle(&self, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn GetService(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioClient {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>() -> IAudioClient_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: i64, hnsperiodicity: i64, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&sharemode), ::core::mem::transmute_copy(&streamflags), ::core::mem::transmute_copy(&hnsbufferduration), ::core::mem::transmute_copy(&hnsperiodicity), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&audiosessionguid)).into()
        }
        unsafe extern "system" fn GetBufferSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumbufferframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBufferSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumbufferframes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStreamLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnslatency: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStreamLatency() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phnslatency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPadding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnumpaddingframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentPadding() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnumpaddingframes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFormatSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sharemode: AUDCLNT_SHAREMODE, pformat: *const WAVEFORMATEX, ppclosestmatch: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsFormatSupported(::core::mem::transmute_copy(&sharemode), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&ppclosestmatch))
        }
        unsafe extern "system" fn GetMixFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdeviceformat: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMixFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdeviceformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePeriod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phnsdefaultdeviceperiod: *mut i64, phnsminimumdeviceperiod: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDevicePeriod(::core::mem::transmute_copy(&phnsdefaultdeviceperiod), ::core::mem::transmute_copy(&phnsminimumdeviceperiod)).into()
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn SetEventHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventHandle(::core::mem::transmute_copy(&eventhandle)).into()
        }
        unsafe extern "system" fn GetService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetService(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, Impl, OFFSET>,
            GetStreamLatency: GetStreamLatency::<Identity, Impl, OFFSET>,
            GetCurrentPadding: GetCurrentPadding::<Identity, Impl, OFFSET>,
            IsFormatSupported: IsFormatSupported::<Identity, Impl, OFFSET>,
            GetMixFormat: GetMixFormat::<Identity, Impl, OFFSET>,
            GetDevicePeriod: GetDevicePeriod::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            SetEventHandle: SetEventHandle::<Identity, Impl, OFFSET>,
            GetService: GetService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioClient2_Impl: Sized + IAudioClient_Impl {
    fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> ::windows::core::Result<()>;
    fn GetBufferSizeLimits(&self, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioClient2 {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioClient2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient2_Impl, const OFFSET: isize>() -> IAudioClient2_Vtbl {
        unsafe extern "system" fn IsOffloadCapable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOffloadCapable(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboffloadcapable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: *const AudioClientProperties) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientProperties(::core::mem::transmute_copy(&pproperties)).into()
        }
        unsafe extern "system" fn GetBufferSizeLimits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, beventdriven: super::super::Foundation::BOOL, phnsminbufferduration: *mut i64, phnsmaxbufferduration: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBufferSizeLimits(::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&beventdriven), ::core::mem::transmute_copy(&phnsminbufferduration), ::core::mem::transmute_copy(&phnsmaxbufferduration)).into()
        }
        Self {
            base__: IAudioClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsOffloadCapable: IsOffloadCapable::<Identity, Impl, OFFSET>,
            SetClientProperties: SetClientProperties::<Identity, Impl, OFFSET>,
            GetBufferSizeLimits: GetBufferSizeLimits::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClient2 as ::windows::core::Interface>::IID || iid == &<IAudioClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioClient3_Impl: Sized + IAudioClient_Impl + IAudioClient2_Impl {
    fn GetSharedModeEnginePeriod(&self, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::Result<()>;
    fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::Result<()>;
    fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioClient3 {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioClient3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient3_Impl, const OFFSET: isize>() -> IAudioClient3_Vtbl {
        unsafe extern "system" fn GetSharedModeEnginePeriod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformat: *const WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSharedModeEnginePeriod(::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&pdefaultperiodinframes), ::core::mem::transmute_copy(&pfundamentalperiodinframes), ::core::mem::transmute_copy(&pminperiodinframes), ::core::mem::transmute_copy(&pmaxperiodinframes)).into()
        }
        unsafe extern "system" fn GetCurrentSharedModeEnginePeriod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppformat: *mut *mut WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentSharedModeEnginePeriod(::core::mem::transmute_copy(&ppformat), ::core::mem::transmute_copy(&pcurrentperiodinframes)).into()
        }
        unsafe extern "system" fn InitializeSharedAudioStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClient3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamflags: u32, periodinframes: u32, pformat: *const WAVEFORMATEX, audiosessionguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeSharedAudioStream(::core::mem::transmute_copy(&streamflags), ::core::mem::transmute_copy(&periodinframes), ::core::mem::transmute_copy(&pformat), ::core::mem::transmute_copy(&audiosessionguid)).into()
        }
        Self {
            base__: IAudioClient2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSharedModeEnginePeriod: GetSharedModeEnginePeriod::<Identity, Impl, OFFSET>,
            GetCurrentSharedModeEnginePeriod: GetCurrentSharedModeEnginePeriod::<Identity, Impl, OFFSET>,
            InitializeSharedAudioStream: InitializeSharedAudioStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClient3 as ::windows::core::Interface>::IID || iid == &<IAudioClient as ::windows::core::Interface>::IID || iid == &<IAudioClient2 as ::windows::core::Interface>::IID
    }
}
pub trait IAudioClientDuckingControl_Impl: Sized {
    fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioClientDuckingControl {}
impl IAudioClientDuckingControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClientDuckingControl_Impl, const OFFSET: isize>() -> IAudioClientDuckingControl_Vtbl {
        unsafe extern "system" fn SetDuckingOptionsForCurrentStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClientDuckingControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, options: AUDIO_DUCKING_OPTIONS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDuckingOptionsForCurrentStream(::core::mem::transmute_copy(&options)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDuckingOptionsForCurrentStream: SetDuckingOptionsForCurrentStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClientDuckingControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioClock_Impl: Sized {
    fn GetFrequency(&self) -> ::windows::core::Result<u64>;
    fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::Result<()>;
    fn GetCharacteristics(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IAudioClock {}
impl IAudioClock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClock_Impl, const OFFSET: isize>() -> IAudioClock_Vtbl {
        unsafe extern "system" fn GetFrequency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu64frequency: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pu64frequency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu64position: *mut u64, pu64qpcposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPosition(::core::mem::transmute_copy(&pu64position), ::core::mem::transmute_copy(&pu64qpcposition)).into()
        }
        unsafe extern "system" fn GetCharacteristics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCharacteristics() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcharacteristics, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFrequency: GetFrequency::<Identity, Impl, OFFSET>,
            GetPosition: GetPosition::<Identity, Impl, OFFSET>,
            GetCharacteristics: GetCharacteristics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClock as ::windows::core::Interface>::IID
    }
}
pub trait IAudioClock2_Impl: Sized {
    fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioClock2 {}
impl IAudioClock2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClock2_Impl, const OFFSET: isize>() -> IAudioClock2_Vtbl {
        unsafe extern "system" fn GetDevicePosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClock2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, deviceposition: *mut u64, qpcposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDevicePosition(::core::mem::transmute_copy(&deviceposition), ::core::mem::transmute_copy(&qpcposition)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDevicePosition: GetDevicePosition::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClock2 as ::windows::core::Interface>::IID
    }
}
pub trait IAudioClockAdjustment_Impl: Sized {
    fn SetSampleRate(&self, flsamplerate: f32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioClockAdjustment {}
impl IAudioClockAdjustment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClockAdjustment_Impl, const OFFSET: isize>() -> IAudioClockAdjustment_Vtbl {
        unsafe extern "system" fn SetSampleRate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioClockAdjustment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flsamplerate: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSampleRate(::core::mem::transmute_copy(&flsamplerate)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetSampleRate: SetSampleRate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioClockAdjustment as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEffectsChangedNotificationClient_Impl: Sized {
    fn OnAudioEffectsChanged(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioEffectsChangedNotificationClient {}
impl IAudioEffectsChangedNotificationClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEffectsChangedNotificationClient_Impl, const OFFSET: isize>() -> IAudioEffectsChangedNotificationClient_Vtbl {
        unsafe extern "system" fn OnAudioEffectsChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEffectsChangedNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAudioEffectsChanged().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnAudioEffectsChanged: OnAudioEffectsChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffectsChangedNotificationClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEffectsManager_Impl: Sized {
    fn RegisterAudioEffectsChangedNotificationCallback(&self, client: &::core::option::Option<IAudioEffectsChangedNotificationClient>) -> ::windows::core::Result<()>;
    fn UnregisterAudioEffectsChangedNotificationCallback(&self, client: &::core::option::Option<IAudioEffectsChangedNotificationClient>) -> ::windows::core::Result<()>;
    fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::Result<()>;
    fn SetAudioEffectState(&self, effectid: &::windows::core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioEffectsManager {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEffectsManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: isize>() -> IAudioEffectsManager_Vtbl {
        unsafe extern "system" fn RegisterAudioEffectsChangedNotificationCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, client: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterAudioEffectsChangedNotificationCallback(::core::mem::transmute(&client)).into()
        }
        unsafe extern "system" fn UnregisterAudioEffectsChangedNotificationCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, client: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterAudioEffectsChangedNotificationCallback(::core::mem::transmute(&client)).into()
        }
        unsafe extern "system" fn GetAudioEffects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAudioEffects(::core::mem::transmute_copy(&effects), ::core::mem::transmute_copy(&numeffects)).into()
        }
        unsafe extern "system" fn SetAudioEffectState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, effectid: ::windows::core::GUID, state: AUDIO_EFFECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAudioEffectState(::core::mem::transmute(&effectid), ::core::mem::transmute_copy(&state)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterAudioEffectsChangedNotificationCallback: RegisterAudioEffectsChangedNotificationCallback::<Identity, Impl, OFFSET>,
            UnregisterAudioEffectsChangedNotificationCallback: UnregisterAudioEffectsChangedNotificationCallback::<Identity, Impl, OFFSET>,
            GetAudioEffects: GetAudioEffects::<Identity, Impl, OFFSET>,
            SetAudioEffectState: SetAudioEffectState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEffectsManager as ::windows::core::Interface>::IID
    }
}
pub trait IAudioFormatEnumerator_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn GetFormat(&self, index: u32) -> ::windows::core::Result<*mut WAVEFORMATEX>;
}
impl ::windows::core::RuntimeName for IAudioFormatEnumerator {}
impl IAudioFormatEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioFormatEnumerator_Impl, const OFFSET: isize>() -> IAudioFormatEnumerator_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioFormatEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioFormatEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, format: *mut *mut WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFormat(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(format, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioFormatEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IAudioInputSelector_Impl: Sized {
    fn GetSelection(&self) -> ::windows::core::Result<u32>;
    fn SetSelection(&self, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioInputSelector {}
impl IAudioInputSelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputSelector_Impl, const OFFSET: isize>() -> IAudioInputSelector_Vtbl {
        unsafe extern "system" fn GetSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnidselected, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioInputSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelection(::core::mem::transmute_copy(&nidselect), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioInputSelector as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioLoudness_Impl: Sized {
    fn GetEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetEnabled(&self, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioLoudness {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioLoudness_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioLoudness_Impl, const OFFSET: isize>() -> IAudioLoudness_Vtbl {
        unsafe extern "system" fn GetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioLoudness_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benable: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&benable), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetEnabled: GetEnabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioLoudness as ::windows::core::Interface>::IID
    }
}
pub trait IAudioMidrange_Impl: Sized + IPerChannelDbLevel_Impl {}
impl ::windows::core::RuntimeName for IAudioMidrange {}
impl IAudioMidrange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioMidrange_Impl, const OFFSET: isize>() -> IAudioMidrange_Vtbl {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMidrange as ::windows::core::Interface>::IID || iid == &<IPerChannelDbLevel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioMute_Impl: Sized {
    fn SetMute(&self, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMute(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioMute {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioMute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioMute_Impl, const OFFSET: isize>() -> IAudioMute_Vtbl {
        unsafe extern "system" fn SetMute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioMute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmuted: super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMute(::core::mem::transmute_copy(&bmuted), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetMute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioMute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmuted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmuted, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetMute: SetMute::<Identity, Impl, OFFSET>,
            GetMute: GetMute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMute as ::windows::core::Interface>::IID
    }
}
pub trait IAudioOutputSelector_Impl: Sized {
    fn GetSelection(&self) -> ::windows::core::Result<u32>;
    fn SetSelection(&self, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioOutputSelector {}
impl IAudioOutputSelector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioOutputSelector_Impl, const OFFSET: isize>() -> IAudioOutputSelector_Vtbl {
        unsafe extern "system" fn GetSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnidselected: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSelection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnidselected, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSelection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioOutputSelector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nidselect: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSelection(::core::mem::transmute_copy(&nidselect), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, Impl, OFFSET>,
            SetSelection: SetSelection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioOutputSelector as ::windows::core::Interface>::IID
    }
}
pub trait IAudioPeakMeter_Impl: Sized {
    fn GetChannelCount(&self) -> ::windows::core::Result<u32>;
    fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32>;
}
impl ::windows::core::RuntimeName for IAudioPeakMeter {}
impl IAudioPeakMeter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioPeakMeter_Impl, const OFFSET: isize>() -> IAudioPeakMeter_Vtbl {
        unsafe extern "system" fn GetChannelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchannels, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioPeakMeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLevel(::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            GetLevel: GetLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioPeakMeter as ::windows::core::Interface>::IID
    }
}
pub trait IAudioRenderClient_Impl: Sized {
    fn GetBuffer(&self, numframesrequested: u32) -> ::windows::core::Result<*mut u8>;
    fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioRenderClient {}
impl IAudioRenderClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioRenderClient_Impl, const OFFSET: isize>() -> IAudioRenderClient_Vtbl {
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioRenderClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBuffer(::core::mem::transmute_copy(&numframesrequested)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioRenderClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numframeswritten: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseBuffer(::core::mem::transmute_copy(&numframeswritten), ::core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioRenderClient as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSessionControl_Impl: Sized {
    fn GetState(&self) -> ::windows::core::Result<AudioSessionState>;
    fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetDisplayName(&self, value: &::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetIconPath(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetIconPath(&self, value: &::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetGroupingParam(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetGroupingParam(&self, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterAudioSessionNotification(&self, newnotifications: &::core::option::Option<IAudioSessionEvents>) -> ::windows::core::Result<()>;
    fn UnregisterAudioSessionNotification(&self, newnotifications: &::core::option::Option<IAudioSessionEvents>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioSessionControl {}
impl IAudioSessionControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>() -> IAudioSessionControl_Vtbl {
        unsafe extern "system" fn GetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut AudioSessionState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&value), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetIconPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIconPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIconPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIconPath(::core::mem::transmute(&value), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetGroupingParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGroupingParam() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroupingParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#override: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGroupingParam(::core::mem::transmute_copy(&r#override), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn RegisterAudioSessionNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newnotifications: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterAudioSessionNotification(::core::mem::transmute(&newnotifications)).into()
        }
        unsafe extern "system" fn UnregisterAudioSessionNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newnotifications: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterAudioSessionNotification(::core::mem::transmute(&newnotifications)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetIconPath: GetIconPath::<Identity, Impl, OFFSET>,
            SetIconPath: SetIconPath::<Identity, Impl, OFFSET>,
            GetGroupingParam: GetGroupingParam::<Identity, Impl, OFFSET>,
            SetGroupingParam: SetGroupingParam::<Identity, Impl, OFFSET>,
            RegisterAudioSessionNotification: RegisterAudioSessionNotification::<Identity, Impl, OFFSET>,
            UnregisterAudioSessionNotification: UnregisterAudioSessionNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSessionControl2_Impl: Sized + IAudioSessionControl_Impl {
    fn GetSessionIdentifier(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetSessionInstanceIdentifier(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetProcessId(&self) -> ::windows::core::Result<u32>;
    fn IsSystemSoundsSession(&self) -> ::windows::core::Result<()>;
    fn SetDuckingPreference(&self, optout: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioSessionControl2 {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSessionControl2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: isize>() -> IAudioSessionControl2_Vtbl {
        unsafe extern "system" fn GetSessionIdentifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSessionIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSessionInstanceIdentifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSessionInstanceIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pretval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProcessId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSystemSoundsSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSystemSoundsSession().into()
        }
        unsafe extern "system" fn SetDuckingPreference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionControl2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optout: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDuckingPreference(::core::mem::transmute_copy(&optout)).into()
        }
        Self {
            base__: IAudioSessionControl_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSessionIdentifier: GetSessionIdentifier::<Identity, Impl, OFFSET>,
            GetSessionInstanceIdentifier: GetSessionInstanceIdentifier::<Identity, Impl, OFFSET>,
            GetProcessId: GetProcessId::<Identity, Impl, OFFSET>,
            IsSystemSoundsSession: IsSystemSoundsSession::<Identity, Impl, OFFSET>,
            SetDuckingPreference: SetDuckingPreference::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionControl2 as ::windows::core::Interface>::IID || iid == &<IAudioSessionControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSessionEnumerator_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<i32>;
    fn GetSession(&self, sessioncount: i32) -> ::windows::core::Result<IAudioSessionControl>;
}
impl ::windows::core::RuntimeName for IAudioSessionEnumerator {}
impl IAudioSessionEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEnumerator_Impl, const OFFSET: isize>() -> IAudioSessionEnumerator_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessioncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sessioncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessioncount: i32, session: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSession(::core::mem::transmute_copy(&sessioncount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(session, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetSession: GetSession::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioSessionEvents_Impl: Sized {
    fn OnDisplayNameChanged(&self, newdisplayname: &::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnIconPathChanged(&self, newiconpath: &::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnSimpleVolumeChanged(&self, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnChannelVolumeChanged(&self, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnGroupingParamChanged(&self, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnStateChanged(&self, newstate: AudioSessionState) -> ::windows::core::Result<()>;
    fn OnSessionDisconnected(&self, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioSessionEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioSessionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: isize>() -> IAudioSessionEvents_Vtbl {
        unsafe extern "system" fn OnDisplayNameChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newdisplayname: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisplayNameChanged(::core::mem::transmute(&newdisplayname), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnIconPathChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newiconpath: ::windows::core::PCWSTR, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIconPathChanged(::core::mem::transmute(&newiconpath), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnSimpleVolumeChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newvolume: f32, newmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSimpleVolumeChanged(::core::mem::transmute_copy(&newvolume), ::core::mem::transmute_copy(&newmute), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnChannelVolumeChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, channelcount: u32, newchannelvolumearray: *const f32, changedchannel: u32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnChannelVolumeChanged(::core::mem::transmute_copy(&channelcount), ::core::mem::transmute_copy(&newchannelvolumearray), ::core::mem::transmute_copy(&changedchannel), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnGroupingParamChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newgroupingparam: *const ::windows::core::GUID, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnGroupingParamChanged(::core::mem::transmute_copy(&newgroupingparam), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn OnStateChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newstate: AudioSessionState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStateChanged(::core::mem::transmute_copy(&newstate)).into()
        }
        unsafe extern "system" fn OnSessionDisconnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, disconnectreason: AudioSessionDisconnectReason) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSessionDisconnected(::core::mem::transmute_copy(&disconnectreason)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnDisplayNameChanged: OnDisplayNameChanged::<Identity, Impl, OFFSET>,
            OnIconPathChanged: OnIconPathChanged::<Identity, Impl, OFFSET>,
            OnSimpleVolumeChanged: OnSimpleVolumeChanged::<Identity, Impl, OFFSET>,
            OnChannelVolumeChanged: OnChannelVolumeChanged::<Identity, Impl, OFFSET>,
            OnGroupingParamChanged: OnGroupingParamChanged::<Identity, Impl, OFFSET>,
            OnStateChanged: OnStateChanged::<Identity, Impl, OFFSET>,
            OnSessionDisconnected: OnSessionDisconnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionEvents as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSessionManager_Impl: Sized {
    fn GetAudioSessionControl(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<IAudioSessionControl>;
    fn GetSimpleAudioVolume(&self, audiosessionguid: *const ::windows::core::GUID, streamflags: u32) -> ::windows::core::Result<ISimpleAudioVolume>;
}
impl ::windows::core::RuntimeName for IAudioSessionManager {}
impl IAudioSessionManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionManager_Impl, const OFFSET: isize>() -> IAudioSessionManager_Vtbl {
        unsafe extern "system" fn GetAudioSessionControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, sessioncontrol: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAudioSessionControl(::core::mem::transmute_copy(&audiosessionguid), ::core::mem::transmute_copy(&streamflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sessioncontrol, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSimpleAudioVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audiosessionguid: *const ::windows::core::GUID, streamflags: u32, audiovolume: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSimpleAudioVolume(::core::mem::transmute_copy(&audiosessionguid), ::core::mem::transmute_copy(&streamflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audiovolume, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAudioSessionControl: GetAudioSessionControl::<Identity, Impl, OFFSET>,
            GetSimpleAudioVolume: GetSimpleAudioVolume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionManager as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSessionManager2_Impl: Sized + IAudioSessionManager_Impl {
    fn GetSessionEnumerator(&self) -> ::windows::core::Result<IAudioSessionEnumerator>;
    fn RegisterSessionNotification(&self, sessionnotification: &::core::option::Option<IAudioSessionNotification>) -> ::windows::core::Result<()>;
    fn UnregisterSessionNotification(&self, sessionnotification: &::core::option::Option<IAudioSessionNotification>) -> ::windows::core::Result<()>;
    fn RegisterDuckNotification(&self, sessionid: &::windows::core::PCWSTR, ducknotification: &::core::option::Option<IAudioVolumeDuckNotification>) -> ::windows::core::Result<()>;
    fn UnregisterDuckNotification(&self, ducknotification: &::core::option::Option<IAudioVolumeDuckNotification>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioSessionManager2 {}
impl IAudioSessionManager2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: isize>() -> IAudioSessionManager2_Vtbl {
        unsafe extern "system" fn GetSessionEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSessionEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sessionenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterSessionNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionnotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterSessionNotification(::core::mem::transmute(&sessionnotification)).into()
        }
        unsafe extern "system" fn UnregisterSessionNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionnotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterSessionNotification(::core::mem::transmute(&sessionnotification)).into()
        }
        unsafe extern "system" fn RegisterDuckNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::PCWSTR, ducknotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterDuckNotification(::core::mem::transmute(&sessionid), ::core::mem::transmute(&ducknotification)).into()
        }
        unsafe extern "system" fn UnregisterDuckNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionManager2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ducknotification: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterDuckNotification(::core::mem::transmute(&ducknotification)).into()
        }
        Self {
            base__: IAudioSessionManager_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSessionEnumerator: GetSessionEnumerator::<Identity, Impl, OFFSET>,
            RegisterSessionNotification: RegisterSessionNotification::<Identity, Impl, OFFSET>,
            UnregisterSessionNotification: UnregisterSessionNotification::<Identity, Impl, OFFSET>,
            RegisterDuckNotification: RegisterDuckNotification::<Identity, Impl, OFFSET>,
            UnregisterDuckNotification: UnregisterDuckNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionManager2 as ::windows::core::Interface>::IID || iid == &<IAudioSessionManager as ::windows::core::Interface>::IID
    }
}
pub trait IAudioSessionNotification_Impl: Sized {
    fn OnSessionCreated(&self, newsession: &::core::option::Option<IAudioSessionControl>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioSessionNotification {}
impl IAudioSessionNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionNotification_Impl, const OFFSET: isize>() -> IAudioSessionNotification_Vtbl {
        unsafe extern "system" fn OnSessionCreated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSessionNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newsession: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSessionCreated(::core::mem::transmute(&newsession)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnSessionCreated: OnSessionCreated::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSessionNotification as ::windows::core::Interface>::IID
    }
}
pub trait IAudioStateMonitor_Impl: Sized {
    fn RegisterCallback(&self, callback: &PAudioStateMonitorCallback, context: *const ::core::ffi::c_void) -> ::windows::core::Result<i64>;
    fn UnregisterCallback(&self, registration: i64);
    fn GetSoundLevel(&self) -> AudioStateMonitorSoundLevel;
}
impl ::windows::core::RuntimeName for IAudioStateMonitor {}
impl IAudioStateMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStateMonitor_Impl, const OFFSET: isize>() -> IAudioStateMonitor_Vtbl {
        unsafe extern "system" fn RegisterCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStateMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void, context: *const ::core::ffi::c_void, registration: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterCallback(::core::mem::transmute(&callback), ::core::mem::transmute_copy(&context)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(registration, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStateMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, registration: i64) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterCallback(::core::mem::transmute_copy(&registration))
        }
        unsafe extern "system" fn GetSoundLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStateMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> AudioStateMonitorSoundLevel {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSoundLevel()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterCallback: RegisterCallback::<Identity, Impl, OFFSET>,
            UnregisterCallback: UnregisterCallback::<Identity, Impl, OFFSET>,
            GetSoundLevel: GetSoundLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioStateMonitor as ::windows::core::Interface>::IID
    }
}
pub trait IAudioStreamVolume_Impl: Sized {
    fn GetChannelCount(&self) -> ::windows::core::Result<u32>;
    fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> ::windows::core::Result<()>;
    fn GetChannelVolume(&self, dwindex: u32) -> ::windows::core::Result<f32>;
    fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::Result<()>;
    fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioStreamVolume {}
impl IAudioStreamVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: isize>() -> IAudioStreamVolume_Vtbl {
        unsafe extern "system" fn GetChannelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChannelVolume(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&flevel)).into()
        }
        unsafe extern "system" fn GetChannelVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelVolume(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllVolumes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllVolumes(::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes)).into()
        }
        unsafe extern "system" fn GetAllVolumes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllVolumes(::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            SetChannelVolume: SetChannelVolume::<Identity, Impl, OFFSET>,
            GetChannelVolume: GetChannelVolume::<Identity, Impl, OFFSET>,
            SetAllVolumes: SetAllVolumes::<Identity, Impl, OFFSET>,
            GetAllVolumes: GetAllVolumes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioStreamVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioSystemEffectsPropertyChangeNotificationClient_Impl: Sized {
    fn OnPropertyChanged(&self, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: &super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::RuntimeName for IAudioSystemEffectsPropertyChangeNotificationClient {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const OFFSET: isize>() -> IAudioSystemEffectsPropertyChangeNotificationClient_Vtbl {
        unsafe extern "system" fn OnPropertyChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyChangeNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AUDIO_SYSTEMEFFECTS_PROPERTYSTORE_TYPE, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPropertyChanged(::core::mem::transmute_copy(&r#type), ::core::mem::transmute(&key)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnPropertyChanged: OnPropertyChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffectsPropertyChangeNotificationClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IAudioSystemEffectsPropertyStore_Impl: Sized {
    fn OpenDefaultPropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenUserPropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn OpenVolatilePropertyStore(&self, stgmaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn ResetUserPropertyStore(&self) -> ::windows::core::Result<()>;
    fn ResetVolatilePropertyStore(&self) -> ::windows::core::Result<()>;
    fn RegisterPropertyChangeNotification(&self, callback: &::core::option::Option<IAudioSystemEffectsPropertyChangeNotificationClient>) -> ::windows::core::Result<()>;
    fn UnregisterPropertyChangeNotification(&self, callback: &::core::option::Option<IAudioSystemEffectsPropertyChangeNotificationClient>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::RuntimeName for IAudioSystemEffectsPropertyStore {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IAudioSystemEffectsPropertyStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>() -> IAudioSystemEffectsPropertyStore_Vtbl {
        unsafe extern "system" fn OpenDefaultPropertyStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenDefaultPropertyStore(::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propstore, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenUserPropertyStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenUserPropertyStore(::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propstore, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenVolatilePropertyStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: u32, propstore: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenVolatilePropertyStore(::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propstore, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetUserPropertyStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetUserPropertyStore().into()
        }
        unsafe extern "system" fn ResetVolatilePropertyStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetVolatilePropertyStore().into()
        }
        unsafe extern "system" fn RegisterPropertyChangeNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterPropertyChangeNotification(::core::mem::transmute(&callback)).into()
        }
        unsafe extern "system" fn UnregisterPropertyChangeNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioSystemEffectsPropertyStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterPropertyChangeNotification(::core::mem::transmute(&callback)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OpenDefaultPropertyStore: OpenDefaultPropertyStore::<Identity, Impl, OFFSET>,
            OpenUserPropertyStore: OpenUserPropertyStore::<Identity, Impl, OFFSET>,
            OpenVolatilePropertyStore: OpenVolatilePropertyStore::<Identity, Impl, OFFSET>,
            ResetUserPropertyStore: ResetUserPropertyStore::<Identity, Impl, OFFSET>,
            ResetVolatilePropertyStore: ResetVolatilePropertyStore::<Identity, Impl, OFFSET>,
            RegisterPropertyChangeNotification: RegisterPropertyChangeNotification::<Identity, Impl, OFFSET>,
            UnregisterPropertyChangeNotification: UnregisterPropertyChangeNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioSystemEffectsPropertyStore as ::windows::core::Interface>::IID
    }
}
pub trait IAudioTreble_Impl: Sized + IPerChannelDbLevel_Impl {}
impl ::windows::core::RuntimeName for IAudioTreble {}
impl IAudioTreble_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioTreble_Impl, const OFFSET: isize>() -> IAudioTreble_Vtbl {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioTreble as ::windows::core::Interface>::IID || iid == &<IPerChannelDbLevel as ::windows::core::Interface>::IID
    }
}
pub trait IAudioVolumeDuckNotification_Impl: Sized {
    fn OnVolumeDuckNotification(&self, sessionid: &::windows::core::PCWSTR, countcommunicationsessions: u32) -> ::windows::core::Result<()>;
    fn OnVolumeUnduckNotification(&self, sessionid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioVolumeDuckNotification {}
impl IAudioVolumeDuckNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>() -> IAudioVolumeDuckNotification_Vtbl {
        unsafe extern "system" fn OnVolumeDuckNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::PCWSTR, countcommunicationsessions: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnVolumeDuckNotification(::core::mem::transmute(&sessionid), ::core::mem::transmute_copy(&countcommunicationsessions)).into()
        }
        unsafe extern "system" fn OnVolumeUnduckNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioVolumeDuckNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sessionid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnVolumeUnduckNotification(::core::mem::transmute(&sessionid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnVolumeDuckNotification: OnVolumeDuckNotification::<Identity, Impl, OFFSET>,
            OnVolumeUnduckNotification: OnVolumeUnduckNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioVolumeDuckNotification as ::windows::core::Interface>::IID
    }
}
pub trait IAudioVolumeLevel_Impl: Sized + IPerChannelDbLevel_Impl {}
impl ::windows::core::RuntimeName for IAudioVolumeLevel {}
impl IAudioVolumeLevel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioVolumeLevel_Impl, const OFFSET: isize>() -> IAudioVolumeLevel_Vtbl {
        Self { base__: IPerChannelDbLevel_Vtbl::new::<Identity, Impl, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioVolumeLevel as ::windows::core::Interface>::IID || iid == &<IPerChannelDbLevel as ::windows::core::Interface>::IID
    }
}
pub trait IChannelAudioVolume_Impl: Sized {
    fn GetChannelCount(&self) -> ::windows::core::Result<u32>;
    fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetChannelVolume(&self, dwindex: u32) -> ::windows::core::Result<f32>;
    fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IChannelAudioVolume {}
impl IChannelAudioVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: isize>() -> IChannelAudioVolume_Vtbl {
        unsafe extern "system" fn GetChannelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChannelVolume(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetChannelVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelVolume(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllVolumes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *const f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllVolumes(::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetAllVolumes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllVolumes(::core::mem::transmute_copy(&dwcount), ::core::mem::transmute_copy(&pfvolumes)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            SetChannelVolume: SetChannelVolume::<Identity, Impl, OFFSET>,
            GetChannelVolume: GetChannelVolume::<Identity, Impl, OFFSET>,
            SetAllVolumes: SetAllVolumes::<Identity, Impl, OFFSET>,
            GetAllVolumes: GetAllVolumes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChannelAudioVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IConnector_Impl: Sized {
    fn GetType(&self) -> ::windows::core::Result<ConnectorType>;
    fn GetDataFlow(&self) -> ::windows::core::Result<DataFlow>;
    fn ConnectTo(&self, pconnectto: &::core::option::Option<IConnector>) -> ::windows::core::Result<()>;
    fn Disconnect(&self) -> ::windows::core::Result<()>;
    fn IsConnected(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetConnectedTo(&self) -> ::windows::core::Result<IConnector>;
    fn GetConnectorIdConnectedTo(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetDeviceIdConnectedTo(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IConnector {}
#[cfg(feature = "Win32_Foundation")]
impl IConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: isize>() -> IConnector_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut ConnectorType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataFlow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflow: *mut DataFlow) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataFlow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ConnectTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectto: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectTo(::core::mem::transmute(&pconnectto)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect().into()
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbconnected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbconnected, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectedTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppconto: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectedTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconto, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectorIdConnectedTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrconnectorid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectorIdConnectedTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrconnectorid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceIdConnectedTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceIdConnectedTo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrdeviceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetDataFlow: GetDataFlow::<Identity, Impl, OFFSET>,
            ConnectTo: ConnectTo::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectedTo: GetConnectedTo::<Identity, Impl, OFFSET>,
            GetConnectorIdConnectedTo: GetConnectorIdConnectedTo::<Identity, Impl, OFFSET>,
            GetDeviceIdConnectedTo: GetDeviceIdConnectedTo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnector as ::windows::core::Interface>::IID
    }
}
pub trait IControlChangeNotify_Impl: Sized {
    fn OnNotify(&self, dwsenderprocessid: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IControlChangeNotify {}
impl IControlChangeNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IControlChangeNotify_Impl, const OFFSET: isize>() -> IControlChangeNotify_Vtbl {
        unsafe extern "system" fn OnNotify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IControlChangeNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsenderprocessid: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNotify(::core::mem::transmute_copy(&dwsenderprocessid), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlChangeNotify as ::windows::core::Interface>::IID
    }
}
pub trait IControlInterface_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetIID(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IControlInterface {}
impl IControlInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IControlInterface_Impl, const OFFSET: isize>() -> IControlInterface_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IControlInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IControlInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetIID: GetIID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IControlInterface as ::windows::core::Interface>::IID
    }
}
pub trait IDeviceSpecificProperty_Impl: Sized {
    fn GetType(&self) -> ::windows::core::Result<u16>;
    fn GetValue(&self, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::Result<()>;
    fn SetValue(&self, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Get4BRange(&self, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDeviceSpecificProperty {}
impl IDeviceSpecificProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: isize>() -> IDeviceSpecificProperty_Vtbl {
        unsafe extern "system" fn GetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvtype: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvalue: *mut ::core::ffi::c_void, pcbvalue: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetValue(::core::mem::transmute_copy(&pvvalue), ::core::mem::transmute_copy(&pcbvalue)).into()
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvalue: *const ::core::ffi::c_void, cbvalue: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&pvvalue), ::core::mem::transmute_copy(&cbvalue), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn Get4BRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceSpecificProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmin: *mut i32, plmax: *mut i32, plstepping: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get4BRange(::core::mem::transmute_copy(&plmin), ::core::mem::transmute_copy(&plmax), ::core::mem::transmute_copy(&plstepping)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
            Get4BRange: Get4BRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceSpecificProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDeviceTopology_Impl: Sized {
    fn GetConnectorCount(&self) -> ::windows::core::Result<u32>;
    fn GetConnector(&self, nindex: u32) -> ::windows::core::Result<IConnector>;
    fn GetSubunitCount(&self) -> ::windows::core::Result<u32>;
    fn GetSubunit(&self, nindex: u32) -> ::windows::core::Result<ISubunit>;
    fn GetPartById(&self, nid: u32) -> ::windows::core::Result<IPart>;
    fn GetDeviceId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetSignalPath(&self, pipartfrom: &::core::option::Option<IPart>, pipartto: &::core::option::Option<IPart>, brejectmixedpaths: super::super::Foundation::BOOL) -> ::windows::core::Result<IPartsList>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDeviceTopology {}
#[cfg(feature = "Win32_Foundation")]
impl IDeviceTopology_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: isize>() -> IDeviceTopology_Vtbl {
        unsafe extern "system" fn GetConnectorCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectorCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppconnector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnector(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppconnector, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubunitCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubunitCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubunit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppsubunit: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubunit(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubunit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartById<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nid: u32, pppart: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartById(::core::mem::transmute_copy(&nid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrdeviceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrdeviceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignalPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDeviceTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipartfrom: *mut ::core::ffi::c_void, pipartto: *mut ::core::ffi::c_void, brejectmixedpaths: super::super::Foundation::BOOL, ppparts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSignalPath(::core::mem::transmute(&pipartfrom), ::core::mem::transmute(&pipartto), ::core::mem::transmute_copy(&brejectmixedpaths)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetConnectorCount: GetConnectorCount::<Identity, Impl, OFFSET>,
            GetConnector: GetConnector::<Identity, Impl, OFFSET>,
            GetSubunitCount: GetSubunitCount::<Identity, Impl, OFFSET>,
            GetSubunit: GetSubunit::<Identity, Impl, OFFSET>,
            GetPartById: GetPartById::<Identity, Impl, OFFSET>,
            GetDeviceId: GetDeviceId::<Identity, Impl, OFFSET>,
            GetSignalPath: GetSignalPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDeviceTopology as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IMMDevice_Impl: Sized {
    fn Activate(&self, iid: *const ::windows::core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OpenPropertyStore(&self, stgmaccess: super::super::System::Com::StructuredStorage::STGM) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetState(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows::core::RuntimeName for IMMDevice {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IMMDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: isize>() -> IMMDevice_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, dwclsctx: super::super::System::Com::CLSCTX, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&dwclsctx), ::core::mem::transmute_copy(&pactivationparams), ::core::mem::transmute_copy(&ppinterface)).into()
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stgmaccess: super::super::System::Com::StructuredStorage::STGM, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenPropertyStore(::core::mem::transmute_copy(&stgmaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstrid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstrid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IMMDeviceActivator_Impl: Sized {
    fn Activate(&self, iid: *const ::windows::core::GUID, pdevice: &::core::option::Option<IMMDevice>, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IMMDeviceActivator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IMMDeviceActivator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceActivator_Impl, const OFFSET: isize>() -> IMMDeviceActivator_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceActivator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, pdevice: *mut ::core::ffi::c_void, pactivationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::core::mem::transmute_copy(&iid), ::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pactivationparams), ::core::mem::transmute_copy(&ppinterface)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Activate: Activate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMDeviceActivator as ::windows::core::Interface>::IID
    }
}
pub trait IMMDeviceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn Item(&self, ndevice: u32) -> ::windows::core::Result<IMMDevice>;
}
impl ::windows::core::RuntimeName for IMMDeviceCollection {}
impl IMMDeviceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceCollection_Impl, const OFFSET: isize>() -> IMMDeviceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdevices: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcdevices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ndevice: u32, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&ndevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMDeviceCollection as ::windows::core::Interface>::IID
    }
}
pub trait IMMDeviceEnumerator_Impl: Sized {
    fn EnumAudioEndpoints(&self, dataflow: EDataFlow, dwstatemask: u32) -> ::windows::core::Result<IMMDeviceCollection>;
    fn GetDefaultAudioEndpoint(&self, dataflow: EDataFlow, role: ERole) -> ::windows::core::Result<IMMDevice>;
    fn GetDevice(&self, pwstrid: &::windows::core::PCWSTR) -> ::windows::core::Result<IMMDevice>;
    fn RegisterEndpointNotificationCallback(&self, pclient: &::core::option::Option<IMMNotificationClient>) -> ::windows::core::Result<()>;
    fn UnregisterEndpointNotificationCallback(&self, pclient: &::core::option::Option<IMMNotificationClient>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMMDeviceEnumerator {}
impl IMMDeviceEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>() -> IMMDeviceEnumerator_Vtbl {
        unsafe extern "system" fn EnumAudioEndpoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, dwstatemask: u32, ppdevices: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumAudioEndpoints(::core::mem::transmute_copy(&dataflow), ::core::mem::transmute_copy(&dwstatemask)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevices, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultAudioEndpoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dataflow: EDataFlow, role: ERole, ppendpoint: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultAudioEndpoint(::core::mem::transmute_copy(&dataflow), ::core::mem::transmute_copy(&role)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppendpoint, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrid: ::windows::core::PCWSTR, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevice(::core::mem::transmute(&pwstrid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdevice, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEndpointNotificationCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclient: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterEndpointNotificationCallback(::core::mem::transmute(&pclient)).into()
        }
        unsafe extern "system" fn UnregisterEndpointNotificationCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMDeviceEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclient: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterEndpointNotificationCallback(::core::mem::transmute(&pclient)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumAudioEndpoints: EnumAudioEndpoints::<Identity, Impl, OFFSET>,
            GetDefaultAudioEndpoint: GetDefaultAudioEndpoint::<Identity, Impl, OFFSET>,
            GetDevice: GetDevice::<Identity, Impl, OFFSET>,
            RegisterEndpointNotificationCallback: RegisterEndpointNotificationCallback::<Identity, Impl, OFFSET>,
            UnregisterEndpointNotificationCallback: UnregisterEndpointNotificationCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMDeviceEnumerator as ::windows::core::Interface>::IID
    }
}
pub trait IMMEndpoint_Impl: Sized {
    fn GetDataFlow(&self) -> ::windows::core::Result<EDataFlow>;
}
impl ::windows::core::RuntimeName for IMMEndpoint {}
impl IMMEndpoint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMEndpoint_Impl, const OFFSET: isize>() -> IMMEndpoint_Vtbl {
        unsafe extern "system" fn GetDataFlow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMEndpoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataflow: *mut EDataFlow) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataFlow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdataflow, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDataFlow: GetDataFlow::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMEndpoint as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IMMNotificationClient_Impl: Sized {
    fn OnDeviceStateChanged(&self, pwstrdeviceid: &::windows::core::PCWSTR, dwnewstate: u32) -> ::windows::core::Result<()>;
    fn OnDeviceAdded(&self, pwstrdeviceid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnDeviceRemoved(&self, pwstrdeviceid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnDefaultDeviceChanged(&self, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnPropertyValueChanged(&self, pwstrdeviceid: &::windows::core::PCWSTR, key: &super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::windows::core::RuntimeName for IMMNotificationClient {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IMMNotificationClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: isize>() -> IMMNotificationClient_Vtbl {
        unsafe extern "system" fn OnDeviceStateChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR, dwnewstate: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDeviceStateChanged(::core::mem::transmute(&pwstrdeviceid), ::core::mem::transmute_copy(&dwnewstate)).into()
        }
        unsafe extern "system" fn OnDeviceAdded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDeviceAdded(::core::mem::transmute(&pwstrdeviceid)).into()
        }
        unsafe extern "system" fn OnDeviceRemoved<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDeviceRemoved(::core::mem::transmute(&pwstrdeviceid)).into()
        }
        unsafe extern "system" fn OnDefaultDeviceChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flow: EDataFlow, role: ERole, pwstrdefaultdeviceid: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDefaultDeviceChanged(::core::mem::transmute_copy(&flow), ::core::mem::transmute_copy(&role), ::core::mem::transmute(&pwstrdefaultdeviceid)).into()
        }
        unsafe extern "system" fn OnPropertyValueChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMMNotificationClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwstrdeviceid: ::windows::core::PCWSTR, key: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnPropertyValueChanged(::core::mem::transmute(&pwstrdeviceid), ::core::mem::transmute(&key)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnDeviceStateChanged: OnDeviceStateChanged::<Identity, Impl, OFFSET>,
            OnDeviceAdded: OnDeviceAdded::<Identity, Impl, OFFSET>,
            OnDeviceRemoved: OnDeviceRemoved::<Identity, Impl, OFFSET>,
            OnDefaultDeviceChanged: OnDefaultDeviceChanged::<Identity, Impl, OFFSET>,
            OnPropertyValueChanged: OnPropertyValueChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMMNotificationClient as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMessageFilter_Impl: Sized {
    fn HandleInComingCall(&self, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32;
    fn RetryRejectedCall(&self, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32;
    fn MessagePending(&self, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMessageFilter {}
#[cfg(feature = "Win32_System_Com")]
impl IMessageFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageFilter_Impl, const OFFSET: isize>() -> IMessageFilter_Vtbl {
        unsafe extern "system" fn HandleInComingCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcalltype: u32, htaskcaller: super::HTASK, dwtickcount: u32, lpinterfaceinfo: *const super::super::System::Com::INTERFACEINFO) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HandleInComingCall(::core::mem::transmute_copy(&dwcalltype), ::core::mem::transmute_copy(&htaskcaller), ::core::mem::transmute_copy(&dwtickcount), ::core::mem::transmute_copy(&lpinterfaceinfo))
        }
        unsafe extern "system" fn RetryRejectedCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwrejecttype: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RetryRejectedCall(::core::mem::transmute_copy(&htaskcallee), ::core::mem::transmute_copy(&dwtickcount), ::core::mem::transmute_copy(&dwrejecttype))
        }
        unsafe extern "system" fn MessagePending<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, htaskcallee: super::HTASK, dwtickcount: u32, dwpendingtype: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MessagePending(::core::mem::transmute_copy(&htaskcallee), ::core::mem::transmute_copy(&dwtickcount), ::core::mem::transmute_copy(&dwpendingtype))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            HandleInComingCall: HandleInComingCall::<Identity, Impl, OFFSET>,
            RetryRejectedCall: RetryRejectedCall::<Identity, Impl, OFFSET>,
            MessagePending: MessagePending::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageFilter as ::windows::core::Interface>::IID
    }
}
pub trait IPart_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetLocalId(&self) -> ::windows::core::Result<u32>;
    fn GetGlobalId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetPartType(&self) -> ::windows::core::Result<PartType>;
    fn GetSubType(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetControlInterfaceCount(&self) -> ::windows::core::Result<u32>;
    fn GetControlInterface(&self, nindex: u32) -> ::windows::core::Result<IControlInterface>;
    fn EnumPartsIncoming(&self) -> ::windows::core::Result<IPartsList>;
    fn EnumPartsOutgoing(&self) -> ::windows::core::Result<IPartsList>;
    fn GetTopologyObject(&self) -> ::windows::core::Result<IDeviceTopology>;
    fn Activate(&self, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RegisterControlChangeCallback(&self, riid: *const ::windows::core::GUID, pnotify: &::core::option::Option<IControlChangeNotify>) -> ::windows::core::Result<()>;
    fn UnregisterControlChangeCallback(&self, pnotify: &::core::option::Option<IControlChangeNotify>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPart {}
impl IPart_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>() -> IPart_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwstrglobalid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlobalId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwstrglobalid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparttype: *mut PartType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pparttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubtype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psubtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetControlInterfaceCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetControlInterfaceCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetControlInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, ppinterfacedesc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetControlInterface(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinterfacedesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPartsIncoming<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumPartsIncoming() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumPartsOutgoing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppparts: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumPartsOutgoing() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppparts, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTopologyObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptopology: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTopologyObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptopology, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclscontext: u32, refiid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::core::mem::transmute_copy(&dwclscontext), ::core::mem::transmute_copy(&refiid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn RegisterControlChangeCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterControlChangeCallback(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&pnotify)).into()
        }
        unsafe extern "system" fn UnregisterControlChangeCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPart_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterControlChangeCallback(::core::mem::transmute(&pnotify)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetLocalId: GetLocalId::<Identity, Impl, OFFSET>,
            GetGlobalId: GetGlobalId::<Identity, Impl, OFFSET>,
            GetPartType: GetPartType::<Identity, Impl, OFFSET>,
            GetSubType: GetSubType::<Identity, Impl, OFFSET>,
            GetControlInterfaceCount: GetControlInterfaceCount::<Identity, Impl, OFFSET>,
            GetControlInterface: GetControlInterface::<Identity, Impl, OFFSET>,
            EnumPartsIncoming: EnumPartsIncoming::<Identity, Impl, OFFSET>,
            EnumPartsOutgoing: EnumPartsOutgoing::<Identity, Impl, OFFSET>,
            GetTopologyObject: GetTopologyObject::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            RegisterControlChangeCallback: RegisterControlChangeCallback::<Identity, Impl, OFFSET>,
            UnregisterControlChangeCallback: UnregisterControlChangeCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPart as ::windows::core::Interface>::IID
    }
}
pub trait IPartsList_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn GetPart(&self, nindex: u32) -> ::windows::core::Result<IPart>;
}
impl ::windows::core::RuntimeName for IPartsList {}
impl IPartsList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPartsList_Impl, const OFFSET: isize>() -> IPartsList_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPartsList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPartsList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nindex: u32, pppart: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPart(::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppart, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetPart: GetPart::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPartsList as ::windows::core::Interface>::IID
    }
}
pub trait IPerChannelDbLevel_Impl: Sized {
    fn GetChannelCount(&self) -> ::windows::core::Result<u32>;
    fn GetLevelRange(&self, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::Result<()>;
    fn GetLevel(&self, nchannel: u32) -> ::windows::core::Result<f32>;
    fn SetLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetLevelUniform(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetLevelAllChannels(&self, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPerChannelDbLevel {}
impl IPerChannelDbLevel_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>() -> IPerChannelDbLevel_Vtbl {
        unsafe extern "system" fn GetChannelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchannels: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchannels, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLevelRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfminleveldb: *mut f32, pfmaxleveldb: *mut f32, pfstepping: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLevelRange(::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&pfminleveldb), ::core::mem::transmute_copy(&pfmaxleveldb), ::core::mem::transmute_copy(&pfstepping)).into()
        }
        unsafe extern "system" fn GetLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLevel(::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfleveldb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLevel(::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetLevelUniform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLevelUniform(::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetLevelAllChannels<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPerChannelDbLevel_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alevelsdb: *const f32, cchannels: u32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLevelAllChannels(::core::mem::transmute_copy(&alevelsdb), ::core::mem::transmute_copy(&cchannels), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            GetLevelRange: GetLevelRange::<Identity, Impl, OFFSET>,
            GetLevel: GetLevel::<Identity, Impl, OFFSET>,
            SetLevel: SetLevel::<Identity, Impl, OFFSET>,
            SetLevelUniform: SetLevelUniform::<Identity, Impl, OFFSET>,
            SetLevelAllChannels: SetLevelAllChannels::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPerChannelDbLevel as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISimpleAudioVolume_Impl: Sized {
    fn SetMasterVolume(&self, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMasterVolume(&self) -> ::windows::core::Result<f32>;
    fn SetMute(&self, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMute(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISimpleAudioVolume {}
#[cfg(feature = "Win32_Foundation")]
impl ISimpleAudioVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: isize>() -> ISimpleAudioVolume_Vtbl {
        unsafe extern "system" fn SetMasterVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flevel: f32, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMasterVolume(::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetMasterVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMasterVolume() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::Foundation::BOOL, eventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMute(::core::mem::transmute_copy(&bmute), ::core::mem::transmute_copy(&eventcontext)).into()
        }
        unsafe extern "system" fn GetMute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMute() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmute, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetMasterVolume: SetMasterVolume::<Identity, Impl, OFFSET>,
            GetMasterVolume: GetMasterVolume::<Identity, Impl, OFFSET>,
            SetMute: SetMute::<Identity, Impl, OFFSET>,
            GetMute: GetMute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISimpleAudioVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpatialAudioClient_Impl: Sized {
    fn GetStaticObjectPosition(&self, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::Result<()>;
    fn GetNativeStaticObjectTypeMask(&self) -> ::windows::core::Result<AudioObjectType>;
    fn GetMaxDynamicObjectCount(&self) -> ::windows::core::Result<u32>;
    fn GetSupportedAudioObjectFormatEnumerator(&self) -> ::windows::core::Result<IAudioFormatEnumerator>;
    fn GetMaxFrameCount(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32>;
    fn IsAudioObjectFormatSupported(&self, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn IsSpatialAudioStreamAvailable(&self, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
    fn ActivateSpatialAudioStream(&self, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows::core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for ISpatialAudioClient {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpatialAudioClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: isize>() -> ISpatialAudioClient_Vtbl {
        unsafe extern "system" fn GetStaticObjectPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, x: *mut f32, y: *mut f32, z: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStaticObjectPosition(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)).into()
        }
        unsafe extern "system" fn GetNativeStaticObjectTypeMask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mask: *mut AudioObjectType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNativeStaticObjectTypeMask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxDynamicObjectCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxDynamicObjectCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedAudioObjectFormatEnumerator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSupportedAudioObjectFormatEnumerator() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(enumerator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxFrameCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxFrameCount(::core::mem::transmute_copy(&objectformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(framecountperbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAudioObjectFormatSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectformat: *const WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsAudioObjectFormatSupported(::core::mem::transmute_copy(&objectformat)).into()
        }
        unsafe extern "system" fn IsSpatialAudioStreamAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streamuuid: *const ::windows::core::GUID, auxiliaryinfo: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSpatialAudioStreamAvailable(::core::mem::transmute_copy(&streamuuid), ::core::mem::transmute_copy(&auxiliaryinfo)).into()
        }
        unsafe extern "system" fn ActivateSpatialAudioStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activationparams: *const super::super::System::Com::StructuredStorage::PROPVARIANT, riid: *const ::windows::core::GUID, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateSpatialAudioStream(::core::mem::transmute_copy(&activationparams), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&stream)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetStaticObjectPosition: GetStaticObjectPosition::<Identity, Impl, OFFSET>,
            GetNativeStaticObjectTypeMask: GetNativeStaticObjectTypeMask::<Identity, Impl, OFFSET>,
            GetMaxDynamicObjectCount: GetMaxDynamicObjectCount::<Identity, Impl, OFFSET>,
            GetSupportedAudioObjectFormatEnumerator: GetSupportedAudioObjectFormatEnumerator::<Identity, Impl, OFFSET>,
            GetMaxFrameCount: GetMaxFrameCount::<Identity, Impl, OFFSET>,
            IsAudioObjectFormatSupported: IsAudioObjectFormatSupported::<Identity, Impl, OFFSET>,
            IsSpatialAudioStreamAvailable: IsSpatialAudioStreamAvailable::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioStream: ActivateSpatialAudioStream::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioClient as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait ISpatialAudioClient2_Impl: Sized + ISpatialAudioClient_Impl {
    fn IsOffloadCapable(&self, category: AUDIO_STREAM_CATEGORY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetMaxFrameCountForCategory(&self, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for ISpatialAudioClient2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl ISpatialAudioClient2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient2_Impl, const OFFSET: isize>() -> ISpatialAudioClient2_Vtbl {
        unsafe extern "system" fn IsOffloadCapable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, isoffloadcapable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsOffloadCapable(::core::mem::transmute_copy(&category)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isoffloadcapable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxFrameCountForCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioClient2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, category: AUDIO_STREAM_CATEGORY, offloadenabled: super::super::Foundation::BOOL, objectformat: *const WAVEFORMATEX, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxFrameCountForCategory(::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&offloadenabled), ::core::mem::transmute_copy(&objectformat)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(framecountperbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpatialAudioClient_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsOffloadCapable: IsOffloadCapable::<Identity, Impl, OFFSET>,
            GetMaxFrameCountForCategory: GetMaxFrameCountForCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioClient2 as ::windows::core::Interface>::IID || iid == &<ISpatialAudioClient as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataClient_Impl: Sized {
    fn ActivateSpatialAudioMetadataItems(&self, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut ::core::option::Option<ISpatialAudioMetadataItemsBuffer>, metadataitems: *mut ::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()>;
    fn GetSpatialAudioMetadataItemsBufferLength(&self, maxitemcount: u16) -> ::windows::core::Result<u32>;
    fn ActivateSpatialAudioMetadataWriter(&self, overflowmode: SpatialAudioMetadataWriterOverflowMode) -> ::windows::core::Result<ISpatialAudioMetadataWriter>;
    fn ActivateSpatialAudioMetadataCopier(&self) -> ::windows::core::Result<ISpatialAudioMetadataCopier>;
    fn ActivateSpatialAudioMetadataReader(&self) -> ::windows::core::Result<ISpatialAudioMetadataReader>;
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataClient {}
impl ISpatialAudioMetadataClient_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>() -> ISpatialAudioMetadataClient_Vtbl {
        unsafe extern "system" fn ActivateSpatialAudioMetadataItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxitemcount: u16, framecount: u16, metadataitemsbuffer: *mut *mut ::core::ffi::c_void, metadataitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ActivateSpatialAudioMetadataItems(::core::mem::transmute_copy(&maxitemcount), ::core::mem::transmute_copy(&framecount), ::core::mem::transmute_copy(&metadataitemsbuffer), ::core::mem::transmute_copy(&metadataitems)).into()
        }
        unsafe extern "system" fn GetSpatialAudioMetadataItemsBufferLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxitemcount: u16, bufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSpatialAudioMetadataItemsBufferLength(::core::mem::transmute_copy(&maxitemcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bufferlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataWriter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, overflowmode: SpatialAudioMetadataWriterOverflowMode, metadatawriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivateSpatialAudioMetadataWriter(::core::mem::transmute_copy(&overflowmode)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadatawriter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataCopier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadatacopier: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivateSpatialAudioMetadataCopier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadatacopier, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioMetadataReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataClient_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadatareader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivateSpatialAudioMetadataReader() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadatareader, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ActivateSpatialAudioMetadataItems: ActivateSpatialAudioMetadataItems::<Identity, Impl, OFFSET>,
            GetSpatialAudioMetadataItemsBufferLength: GetSpatialAudioMetadataItemsBufferLength::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioMetadataWriter: ActivateSpatialAudioMetadataWriter::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioMetadataCopier: ActivateSpatialAudioMetadataCopier::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioMetadataReader: ActivateSpatialAudioMetadataReader::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataClient as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataCopier_Impl: Sized {
    fn Open(&self, metadataitems: &::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()>;
    fn CopyMetadataForFrames(&self, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: &::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<u16>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataCopier {}
impl ISpatialAudioMetadataCopier_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>() -> ISpatialAudioMetadataCopier_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&metadataitems)).into()
        }
        unsafe extern "system" fn CopyMetadataForFrames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, copyframecount: u16, copymode: SpatialAudioMetadataCopyMode, dstmetadataitems: *mut ::core::ffi::c_void, itemscopied: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyMetadataForFrames(::core::mem::transmute_copy(&copyframecount), ::core::mem::transmute_copy(&copymode), ::core::mem::transmute(&dstmetadataitems)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemscopied, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataCopier_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            CopyMetadataForFrames: CopyMetadataForFrames::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataCopier as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataItems_Impl: Sized {
    fn GetFrameCount(&self) -> ::windows::core::Result<u16>;
    fn GetItemCount(&self) -> ::windows::core::Result<u16>;
    fn GetMaxItemCount(&self) -> ::windows::core::Result<u16>;
    fn GetMaxValueBufferLength(&self) -> ::windows::core::Result<u32>;
    fn GetInfo(&self) -> ::windows::core::Result<SpatialAudioMetadataItemsInfo>;
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataItems {}
impl ISpatialAudioMetadataItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>() -> ISpatialAudioMetadataItems_Vtbl {
        unsafe extern "system" fn GetFrameCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framecount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrameCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(framecount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itemcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxItemCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxitemcount: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxItemCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxitemcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxValueBufferLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxvaluebufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxValueBufferLength() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxvaluebufferlength, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, info: *mut SpatialAudioMetadataItemsInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(info, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFrameCount: GetFrameCount::<Identity, Impl, OFFSET>,
            GetItemCount: GetItemCount::<Identity, Impl, OFFSET>,
            GetMaxItemCount: GetMaxItemCount::<Identity, Impl, OFFSET>,
            GetMaxValueBufferLength: GetMaxValueBufferLength::<Identity, Impl, OFFSET>,
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataItems as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataItemsBuffer_Impl: Sized {
    fn AttachToBuffer(&self, buffer: *mut u8, bufferlength: u32) -> ::windows::core::Result<()>;
    fn AttachToPopulatedBuffer(&self, buffer: *mut u8, bufferlength: u32) -> ::windows::core::Result<()>;
    fn DetachBuffer(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataItemsBuffer {}
impl ISpatialAudioMetadataItemsBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>() -> ISpatialAudioMetadataItemsBuffer_Vtbl {
        unsafe extern "system" fn AttachToBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachToBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)).into()
        }
        unsafe extern "system" fn AttachToPopulatedBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut u8, bufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AttachToPopulatedBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)).into()
        }
        unsafe extern "system" fn DetachBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataItemsBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DetachBuffer().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AttachToBuffer: AttachToBuffer::<Identity, Impl, OFFSET>,
            AttachToPopulatedBuffer: AttachToPopulatedBuffer::<Identity, Impl, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataItemsBuffer as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataReader_Impl: Sized {
    fn Open(&self, metadataitems: &::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()>;
    fn ReadNextItem(&self, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::Result<()>;
    fn ReadNextItemCommand(&self, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataReader {}
impl ISpatialAudioMetadataReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>() -> ISpatialAudioMetadataReader_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&metadataitems)).into()
        }
        unsafe extern "system" fn ReadNextItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandcount: *mut u8, frameoffset: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadNextItem(::core::mem::transmute_copy(&commandcount), ::core::mem::transmute_copy(&frameoffset)).into()
        }
        unsafe extern "system" fn ReadNextItemCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: *mut u8, valuebuffer: *mut ::core::ffi::c_void, maxvaluebufferlength: u32, valuebufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadNextItemCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), ::core::mem::transmute_copy(&maxvaluebufferlength), ::core::mem::transmute_copy(&valuebufferlength)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            ReadNextItem: ReadNextItem::<Identity, Impl, OFFSET>,
            ReadNextItemCommand: ReadNextItemCommand::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataReader as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioMetadataWriter_Impl: Sized {
    fn Open(&self, metadataitems: &::core::option::Option<ISpatialAudioMetadataItems>) -> ::windows::core::Result<()>;
    fn WriteNextItem(&self, frameoffset: u16) -> ::windows::core::Result<()>;
    fn WriteNextItemCommand(&self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISpatialAudioMetadataWriter {}
impl ISpatialAudioMetadataWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>() -> ISpatialAudioMetadataWriter_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open(::core::mem::transmute(&metadataitems)).into()
        }
        unsafe extern "system" fn WriteNextItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frameoffset: u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteNextItem(::core::mem::transmute_copy(&frameoffset)).into()
        }
        unsafe extern "system" fn WriteNextItemCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteNextItemCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), ::core::mem::transmute_copy(&valuebufferlength)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioMetadataWriter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            WriteNextItem: WriteNextItem::<Identity, Impl, OFFSET>,
            WriteNextItemCommand: WriteNextItemCommand::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioMetadataWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObject_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()>;
    fn SetVolume(&self, volume: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISpatialAudioObject {}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObject_Impl, const OFFSET: isize>() -> ISpatialAudioObject_Vtbl {
        unsafe extern "system" fn SetPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)).into()
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, volume: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVolume(::core::mem::transmute_copy(&volume)).into()
        }
        Self {
            base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObject as ::windows::core::Interface>::IID || iid == &<ISpatialAudioObjectBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectBase_Impl: Sized {
    fn GetBuffer(&self, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::Result<()>;
    fn SetEndOfStream(&self, framecount: u32) -> ::windows::core::Result<()>;
    fn IsActive(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetAudioObjectType(&self) -> ::windows::core::Result<AudioObjectType>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISpatialAudioObjectBase {}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObjectBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: isize>() -> ISpatialAudioObjectBase_Vtbl {
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: *mut *mut u8, bufferlength: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&bufferlength)).into()
        }
        unsafe extern "system" fn SetEndOfStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, framecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEndOfStream(::core::mem::transmute_copy(&framecount)).into()
        }
        unsafe extern "system" fn IsActive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isactive: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsActive() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isactive, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioObjectType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, audioobjecttype: *mut AudioObjectType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAudioObjectType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobjecttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            SetEndOfStream: SetEndOfStream::<Identity, Impl, OFFSET>,
            IsActive: IsActive::<Identity, Impl, OFFSET>,
            GetAudioObjectType: GetAudioObjectType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectForHrtf_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn SetPosition(&self, x: f32, y: f32, z: f32) -> ::windows::core::Result<()>;
    fn SetGain(&self, gain: f32) -> ::windows::core::Result<()>;
    fn SetOrientation(&self, orientation: *const *const f32) -> ::windows::core::Result<()>;
    fn SetEnvironment(&self, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::Result<()>;
    fn SetDistanceDecay(&self, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::Result<()>;
    fn SetDirectivity(&self, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISpatialAudioObjectForHrtf {}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObjectForHrtf_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>() -> ISpatialAudioObjectForHrtf_Vtbl {
        unsafe extern "system" fn SetPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z)).into()
        }
        unsafe extern "system" fn SetGain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gain: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGain(::core::mem::transmute_copy(&gain)).into()
        }
        unsafe extern "system" fn SetOrientation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, orientation: *const *const f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOrientation(::core::mem::transmute_copy(&orientation)).into()
        }
        unsafe extern "system" fn SetEnvironment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, environment: SpatialAudioHrtfEnvironmentType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnvironment(::core::mem::transmute_copy(&environment)).into()
        }
        unsafe extern "system" fn SetDistanceDecay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, distancedecay: *const SpatialAudioHrtfDistanceDecay) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDistanceDecay(::core::mem::transmute_copy(&distancedecay)).into()
        }
        unsafe extern "system" fn SetDirectivity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, directivity: *const SpatialAudioHrtfDirectivityUnion) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDirectivity(::core::mem::transmute_copy(&directivity)).into()
        }
        Self {
            base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            SetGain: SetGain::<Identity, Impl, OFFSET>,
            SetOrientation: SetOrientation::<Identity, Impl, OFFSET>,
            SetEnvironment: SetEnvironment::<Identity, Impl, OFFSET>,
            SetDistanceDecay: SetDistanceDecay::<Identity, Impl, OFFSET>,
            SetDirectivity: SetDirectivity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForHrtf as ::windows::core::Interface>::IID || iid == &<ISpatialAudioObjectBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectForMetadataCommands_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn WriteNextMetadataCommand(&self, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISpatialAudioObjectForMetadataCommands {}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObjectForMetadataCommands_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForMetadataCommands_Impl, const OFFSET: isize>() -> ISpatialAudioObjectForMetadataCommands_Vtbl {
        unsafe extern "system" fn WriteNextMetadataCommand<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForMetadataCommands_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, commandid: u8, valuebuffer: *const ::core::ffi::c_void, valuebufferlength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteNextMetadataCommand(::core::mem::transmute_copy(&commandid), ::core::mem::transmute_copy(&valuebuffer), ::core::mem::transmute_copy(&valuebufferlength)).into()
        }
        Self {
            base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteNextMetadataCommand: WriteNextMetadataCommand::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataCommands as ::windows::core::Interface>::IID || iid == &<ISpatialAudioObjectBase as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISpatialAudioObjectForMetadataItems_Impl: Sized + ISpatialAudioObjectBase_Impl {
    fn GetSpatialAudioMetadataItems(&self) -> ::windows::core::Result<ISpatialAudioMetadataItems>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISpatialAudioObjectForMetadataItems {}
#[cfg(feature = "Win32_Foundation")]
impl ISpatialAudioObjectForMetadataItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForMetadataItems_Impl, const OFFSET: isize>() -> ISpatialAudioObjectForMetadataItems_Vtbl {
        unsafe extern "system" fn GetSpatialAudioMetadataItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectForMetadataItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSpatialAudioMetadataItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadataitems, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpatialAudioObjectBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSpatialAudioMetadataItems: GetSpatialAudioMetadataItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectForMetadataItems as ::windows::core::Interface>::IID || iid == &<ISpatialAudioObjectBase as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStream_Impl: Sized + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObject(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObject>;
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStream {}
impl ISpatialAudioObjectRenderStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStream_Impl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStream_Vtbl {
        unsafe extern "system" fn ActivateSpatialAudioObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivateSpatialAudioObject(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            ActivateSpatialAudioObject: ActivateSpatialAudioObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStream as ::windows::core::Interface>::IID || iid == &<ISpatialAudioObjectRenderStreamBase as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamBase_Impl: Sized {
    fn GetAvailableDynamicObjectCount(&self) -> ::windows::core::Result<u32>;
    fn GetService(&self, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn BeginUpdatingAudioObjects(&self, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::Result<()>;
    fn EndUpdatingAudioObjects(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStreamBase {}
impl ISpatialAudioObjectRenderStreamBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamBase_Vtbl {
        unsafe extern "system" fn GetAvailableDynamicObjectCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAvailableDynamicObjectCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, service: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetService(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&service)).into()
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start().into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn BeginUpdatingAudioObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availabledynamicobjectcount: *mut u32, framecountperbuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginUpdatingAudioObjects(::core::mem::transmute_copy(&availabledynamicobjectcount), ::core::mem::transmute_copy(&framecountperbuffer)).into()
        }
        unsafe extern "system" fn EndUpdatingAudioObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUpdatingAudioObjects().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAvailableDynamicObjectCount: GetAvailableDynamicObjectCount::<Identity, Impl, OFFSET>,
            GetService: GetService::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            BeginUpdatingAudioObjects: BeginUpdatingAudioObjects::<Identity, Impl, OFFSET>,
            EndUpdatingAudioObjects: EndUpdatingAudioObjects::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamBase as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamForHrtf_Impl: Sized + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForHrtf(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForHrtf>;
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStreamForHrtf {}
impl ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForHrtf_Impl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamForHrtf_Vtbl {
        unsafe extern "system" fn ActivateSpatialAudioObjectForHrtf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForHrtf_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivateSpatialAudioObjectForHrtf(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            ActivateSpatialAudioObjectForHrtf: ActivateSpatialAudioObjectForHrtf::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamForHrtf as ::windows::core::Interface>::IID || iid == &<ISpatialAudioObjectRenderStreamBase as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamForMetadata_Impl: Sized + ISpatialAudioObjectRenderStreamBase_Impl {
    fn ActivateSpatialAudioObjectForMetadataCommands(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataCommands>;
    fn ActivateSpatialAudioObjectForMetadataItems(&self, r#type: AudioObjectType) -> ::windows::core::Result<ISpatialAudioObjectForMetadataItems>;
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStreamForMetadata {}
impl ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamForMetadata_Vtbl {
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataCommands<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivateSpatialAudioObjectForMetadataCommands(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateSpatialAudioObjectForMetadataItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamForMetadata_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: AudioObjectType, audioobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivateSpatialAudioObjectForMetadataItems(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(audioobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISpatialAudioObjectRenderStreamBase_Vtbl::new::<Identity, Impl, OFFSET>(),
            ActivateSpatialAudioObjectForMetadataCommands: ActivateSpatialAudioObjectForMetadataCommands::<Identity, Impl, OFFSET>,
            ActivateSpatialAudioObjectForMetadataItems: ActivateSpatialAudioObjectForMetadataItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamForMetadata as ::windows::core::Interface>::IID || iid == &<ISpatialAudioObjectRenderStreamBase as ::windows::core::Interface>::IID
    }
}
pub trait ISpatialAudioObjectRenderStreamNotify_Impl: Sized {
    fn OnAvailableDynamicObjectCountChange(&self, sender: &::core::option::Option<ISpatialAudioObjectRenderStreamBase>, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISpatialAudioObjectRenderStreamNotify {}
impl ISpatialAudioObjectRenderStreamNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamNotify_Impl, const OFFSET: isize>() -> ISpatialAudioObjectRenderStreamNotify_Vtbl {
        unsafe extern "system" fn OnAvailableDynamicObjectCountChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISpatialAudioObjectRenderStreamNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, hnscompliancedeadlinetime: i64, availabledynamicobjectcountchange: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAvailableDynamicObjectCountChange(::core::mem::transmute(&sender), ::core::mem::transmute_copy(&hnscompliancedeadlinetime), ::core::mem::transmute_copy(&availabledynamicobjectcountchange)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnAvailableDynamicObjectCountChange: OnAvailableDynamicObjectCountChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISpatialAudioObjectRenderStreamNotify as ::windows::core::Interface>::IID
    }
}
pub trait ISubunit_Impl: Sized {}
impl ::windows::core::RuntimeName for ISubunit {}
impl ISubunit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISubunit_Impl, const OFFSET: isize>() -> ISubunit_Vtbl {
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISubunit as ::windows::core::Interface>::IID
    }
}
