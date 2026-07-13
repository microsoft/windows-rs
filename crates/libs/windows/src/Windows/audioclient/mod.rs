pub type AMBISONICS_CHANNEL_ORDERING = i32;
pub const AMBISONICS_CHANNEL_ORDERING_ACN: AMBISONICS_CHANNEL_ORDERING = 0;
pub type AMBISONICS_NORMALIZATION = i32;
pub const AMBISONICS_NORMALIZATION_N3D: AMBISONICS_NORMALIZATION = 1;
pub const AMBISONICS_NORMALIZATION_SN3D: AMBISONICS_NORMALIZATION = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AMBISONICS_PARAMS {
    pub u32Size: u32,
    pub u32Version: u32,
    pub u32Type: AMBISONICS_TYPE,
    pub u32ChannelOrdering: AMBISONICS_CHANNEL_ORDERING,
    pub u32Normalization: AMBISONICS_NORMALIZATION,
    pub u32Order: u32,
    pub u32NumChannels: u32,
    pub pu32ChannelMap: *mut u32,
}
impl Default for AMBISONICS_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AMBISONICS_PARAM_VERSION_1: u32 = 1;
pub type AMBISONICS_TYPE = i32;
pub const AMBISONICS_TYPE_FULL3D: AMBISONICS_TYPE = 0;
pub const AUDCLNT_BUFFERFLAGS_DATA_DISCONTINUITY: _AUDCLNT_BUFFERFLAGS = 1;
pub const AUDCLNT_BUFFERFLAGS_SILENT: _AUDCLNT_BUFFERFLAGS = 2;
pub const AUDCLNT_BUFFERFLAGS_TIMESTAMP_ERROR: _AUDCLNT_BUFFERFLAGS = 4;
pub const AUDCLNT_E_ALREADY_INITIALIZED: i32 = -2004287486;
pub const AUDCLNT_E_BUFDURATION_PERIOD_NOT_EQUAL: i32 = -2004287469;
pub const AUDCLNT_E_BUFFER_ERROR: i32 = -2004287464;
pub const AUDCLNT_E_BUFFER_OPERATION_PENDING: i32 = -2004287477;
pub const AUDCLNT_E_BUFFER_SIZE_ERROR: i32 = -2004287466;
pub const AUDCLNT_E_BUFFER_SIZE_NOT_ALIGNED: i32 = -2004287463;
pub const AUDCLNT_E_BUFFER_TOO_LARGE: i32 = -2004287482;
pub const AUDCLNT_E_CPUUSAGE_EXCEEDED: i32 = -2004287465;
pub const AUDCLNT_E_DEVICE_INVALIDATED: i32 = -2004287484;
pub const AUDCLNT_E_DEVICE_IN_USE: i32 = -2004287478;
pub const AUDCLNT_E_EFFECT_NOT_AVAILABLE: i32 = -2004287423;
pub const AUDCLNT_E_EFFECT_STATE_READ_ONLY: i32 = -2004287422;
pub const AUDCLNT_E_ENDPOINT_CREATE_FAILED: i32 = -2004287473;
pub const AUDCLNT_E_ENDPOINT_OFFLOAD_NOT_CAPABLE: i32 = -2004287454;
pub const AUDCLNT_E_ENGINE_FORMAT_LOCKED: i32 = -2004287447;
pub const AUDCLNT_E_ENGINE_PERIODICITY_LOCKED: i32 = -2004287448;
pub const AUDCLNT_E_EVENTHANDLE_NOT_EXPECTED: i32 = -2004287471;
pub const AUDCLNT_E_EVENTHANDLE_NOT_SET: i32 = -2004287468;
pub const AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED: i32 = -2004287474;
pub const AUDCLNT_E_EXCLUSIVE_MODE_ONLY: i32 = -2004287470;
pub const AUDCLNT_E_HEADTRACKING_ENABLED: i32 = -2004287440;
pub const AUDCLNT_E_HEADTRACKING_UNSUPPORTED: i32 = -2004287424;
pub const AUDCLNT_E_INCORRECT_BUFFER_SIZE: i32 = -2004287467;
pub const AUDCLNT_E_INVALID_DEVICE_PERIOD: i32 = -2004287456;
pub const AUDCLNT_E_INVALID_SIZE: i32 = -2004287479;
pub const AUDCLNT_E_INVALID_STREAM_FLAG: i32 = -2004287455;
pub const AUDCLNT_E_NONOFFLOAD_MODE_ONLY: i32 = -2004287451;
pub const AUDCLNT_E_NOT_INITIALIZED: i32 = -2004287487;
pub const AUDCLNT_E_NOT_STOPPED: i32 = -2004287483;
pub const AUDCLNT_E_OFFLOAD_MODE_ONLY: i32 = -2004287452;
pub const AUDCLNT_E_OUT_OF_OFFLOAD_RESOURCES: i32 = -2004287453;
pub const AUDCLNT_E_OUT_OF_ORDER: i32 = -2004287481;
pub const AUDCLNT_E_POST_VOLUME_LOOPBACK_UNSUPPORTED: i32 = -2004287421;
pub const AUDCLNT_E_RAW_MODE_UNSUPPORTED: i32 = -2004287449;
pub const AUDCLNT_E_RESOURCES_INVALIDATED: i32 = -2004287450;
pub const AUDCLNT_E_SERVICE_NOT_RUNNING: i32 = -2004287472;
pub const AUDCLNT_E_THREAD_NOT_REGISTERED: i32 = -2004287476;
pub const AUDCLNT_E_UNSUPPORTED_FORMAT: i32 = -2004287480;
pub const AUDCLNT_E_WRONG_ENDPOINT_TYPE: i32 = -2004287485;
pub type AUDCLNT_STREAMOPTIONS = u32;
pub const AUDCLNT_STREAMOPTIONS_AMBISONICS: AUDCLNT_STREAMOPTIONS = 4;
pub const AUDCLNT_STREAMOPTIONS_MATCH_FORMAT: AUDCLNT_STREAMOPTIONS = 2;
pub const AUDCLNT_STREAMOPTIONS_NONE: AUDCLNT_STREAMOPTIONS = 0;
pub const AUDCLNT_STREAMOPTIONS_POST_VOLUME_LOOPBACK: AUDCLNT_STREAMOPTIONS = 8;
pub const AUDCLNT_STREAMOPTIONS_RAW: AUDCLNT_STREAMOPTIONS = 1;
pub const AUDCLNT_S_BUFFER_EMPTY: u32 = 143196161;
pub const AUDCLNT_S_POSITION_STALLED: u32 = 143196163;
pub const AUDCLNT_S_THREAD_ALREADY_REGISTERED: u32 = 143196162;
pub const AUDIOCLOCK_CHARACTERISTIC_FIXED_FREQ: u32 = 1;
pub type AUDIO_DUCKING_OPTIONS = u32;
pub const AUDIO_DUCKING_OPTIONS_DEFAULT: AUDIO_DUCKING_OPTIONS = 0;
pub const AUDIO_DUCKING_OPTIONS_DO_NOT_DUCK_OTHER_STREAMS: AUDIO_DUCKING_OPTIONS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AUDIO_EFFECT {
    pub id: windows_core::GUID,
    pub canSetState: windows_core::BOOL,
    pub state: AUDIO_EFFECT_STATE,
}
pub type AUDIO_EFFECT_STATE = i32;
pub const AUDIO_EFFECT_STATE_OFF: AUDIO_EFFECT_STATE = 0;
pub const AUDIO_EFFECT_STATE_ON: AUDIO_EFFECT_STATE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioClient3ActivationParams {
    pub tracingContextId: windows_core::GUID,
}
#[repr(C)]
#[cfg(feature = "audiosessiontypes")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AudioClientProperties {
    pub cbSize: u32,
    pub bIsOffload: windows_core::BOOL,
    pub eCategory: super::audiosessiontypes::AUDIO_STREAM_CATEGORY,
    pub Options: AUDCLNT_STREAMOPTIONS,
}
windows_core::imp::define_interface!(IAcousticEchoCancellationControl, IAcousticEchoCancellationControl_Vtbl, 0xf4ae25b5_aaa3_437d_b6b3_dbbe2d0e9549);
windows_core::imp::interface_hierarchy!(IAcousticEchoCancellationControl, windows_core::IUnknown);
impl IAcousticEchoCancellationControl {
    pub unsafe fn SetEchoCancellationRenderEndpoint<P0>(&self, endpointid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEchoCancellationRenderEndpoint)(windows_core::Interface::as_raw(self), endpointid.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAcousticEchoCancellationControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetEchoCancellationRenderEndpoint: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IAcousticEchoCancellationControl_Impl: windows_core::IUnknownImpl {
    fn SetEchoCancellationRenderEndpoint(&self, endpointid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IAcousticEchoCancellationControl_Vtbl {
    pub const fn new<Identity: IAcousticEchoCancellationControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetEchoCancellationRenderEndpoint<Identity: IAcousticEchoCancellationControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpointid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAcousticEchoCancellationControl_Impl::SetEchoCancellationRenderEndpoint(this, core::mem::transmute(&endpointid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetEchoCancellationRenderEndpoint: SetEchoCancellationRenderEndpoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAcousticEchoCancellationControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAcousticEchoCancellationControl {}
windows_core::imp::define_interface!(IAudioAmbisonicsControl, IAudioAmbisonicsControl_Vtbl, 0x28724c91_df35_4856_9f76_d6a26413f3df);
windows_core::imp::interface_hierarchy!(IAudioAmbisonicsControl, windows_core::IUnknown);
impl IAudioAmbisonicsControl {
    pub unsafe fn SetData(&self, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetData)(windows_core::Interface::as_raw(self), pambisonicsparams, cbambisonicsparams) }
    }
    pub unsafe fn SetHeadTracking(&self, benableheadtracking: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetHeadTracking)(windows_core::Interface::as_raw(self), benableheadtracking.into()) }
    }
    pub unsafe fn GetHeadTracking(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHeadTracking)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), x, y, z, w) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioAmbisonicsControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, *const AMBISONICS_PARAMS, u32) -> windows_core::HRESULT,
    pub SetHeadTracking: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetHeadTracking: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32) -> windows_core::HRESULT,
}
pub trait IAudioAmbisonicsControl_Impl: windows_core::IUnknownImpl {
    fn SetData(&self, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> windows_core::Result<()>;
    fn SetHeadTracking(&self, benableheadtracking: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetHeadTracking(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetRotation(&self, x: f32, y: f32, z: f32, w: f32) -> windows_core::Result<()>;
}
impl IAudioAmbisonicsControl_Vtbl {
    pub const fn new<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetData<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pambisonicsparams: *const AMBISONICS_PARAMS, cbambisonicsparams: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioAmbisonicsControl_Impl::SetData(this, core::mem::transmute_copy(&pambisonicsparams), core::mem::transmute_copy(&cbambisonicsparams)).into()
            }
        }
        unsafe extern "system" fn SetHeadTracking<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, benableheadtracking: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioAmbisonicsControl_Impl::SetHeadTracking(this, core::mem::transmute_copy(&benableheadtracking)).into()
            }
        }
        unsafe extern "system" fn GetHeadTracking<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbenableheadtracking: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioAmbisonicsControl_Impl::GetHeadTracking(this) {
                    Ok(ok__) => {
                        pbenableheadtracking.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IAudioAmbisonicsControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, w: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioAmbisonicsControl_Impl::SetRotation(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&w)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetData: SetData::<Identity, OFFSET>,
            SetHeadTracking: SetHeadTracking::<Identity, OFFSET>,
            GetHeadTracking: GetHeadTracking::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioAmbisonicsControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioAmbisonicsControl {}
windows_core::imp::define_interface!(IAudioCaptureClient, IAudioCaptureClient_Vtbl, 0xc8adbd64_e71e_48a0_a4de_185c395cd317);
windows_core::imp::interface_hierarchy!(IAudioCaptureClient, windows_core::IUnknown);
impl IAudioCaptureClient {
    pub unsafe fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: Option<*mut u64>, pu64qpcposition: Option<*mut u64>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), ppdata as _, pnumframestoread as _, pdwflags as _, pu64deviceposition.unwrap_or(core::mem::zeroed()) as _, pu64qpcposition.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn ReleaseBuffer(&self, numframesread: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseBuffer)(windows_core::Interface::as_raw(self), numframesread) }
    }
    pub unsafe fn GetNextPacketSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextPacketSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioCaptureClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32, *mut u32, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetNextPacketSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAudioCaptureClient_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> windows_core::Result<()>;
    fn ReleaseBuffer(&self, numframesread: u32) -> windows_core::Result<()>;
    fn GetNextPacketSize(&self) -> windows_core::Result<u32>;
}
impl IAudioCaptureClient_Vtbl {
    pub const fn new<Identity: IAudioCaptureClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdata: *mut *mut u8, pnumframestoread: *mut u32, pdwflags: *mut u32, pu64deviceposition: *mut u64, pu64qpcposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioCaptureClient_Impl::GetBuffer(this, core::mem::transmute_copy(&ppdata), core::mem::transmute_copy(&pnumframestoread), core::mem::transmute_copy(&pdwflags), core::mem::transmute_copy(&pu64deviceposition), core::mem::transmute_copy(&pu64qpcposition)).into()
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numframesread: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioCaptureClient_Impl::ReleaseBuffer(this, core::mem::transmute_copy(&numframesread)).into()
            }
        }
        unsafe extern "system" fn GetNextPacketSize<Identity: IAudioCaptureClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumframesinnextpacket: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioCaptureClient_Impl::GetNextPacketSize(this) {
                    Ok(ok__) => {
                        pnumframesinnextpacket.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Identity, OFFSET>,
            GetNextPacketSize: GetNextPacketSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioCaptureClient as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioCaptureClient {}
windows_core::imp::define_interface!(IAudioClient, IAudioClient_Vtbl, 0x1cb9ad4c_dbfa_4c32_b178_c2f568a703b2);
windows_core::imp::interface_hierarchy!(IAudioClient, windows_core::IUnknown);
impl IAudioClient {
    #[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi"))]
    pub unsafe fn Initialize(&self, sharemode: super::audiosessiontypes::AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: super::mediaobj::REFERENCE_TIME, hnsperiodicity: super::mediaobj::REFERENCE_TIME, pformat: *const super::mmeapi::WAVEFORMATEX, audiosessionguid: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), sharemode, streamflags, hnsbufferduration, hnsperiodicity, pformat, audiosessionguid.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetBufferSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mediaobj")]
    pub unsafe fn GetStreamLatency(&self) -> windows_core::Result<super::mediaobj::REFERENCE_TIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamLatency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentPadding(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPadding)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "audiosessiontypes", feature = "mmeapi"))]
    pub unsafe fn IsFormatSupported(&self, sharemode: super::audiosessiontypes::AUDCLNT_SHAREMODE, pformat: *const super::mmeapi::WAVEFORMATEX, ppclosestmatch: Option<*mut *mut super::mmeapi::WAVEFORMATEX>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsFormatSupported)(windows_core::Interface::as_raw(self), sharemode, pformat, ppclosestmatch.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "mmeapi")]
    pub unsafe fn GetMixFormat(&self) -> windows_core::Result<*mut super::mmeapi::WAVEFORMATEX> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMixFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mediaobj")]
    pub unsafe fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: Option<*mut super::mediaobj::REFERENCE_TIME>, phnsminimumdeviceperiod: Option<*mut super::mediaobj::REFERENCE_TIME>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevicePeriod)(windows_core::Interface::as_raw(self), phnsdefaultdeviceperiod.unwrap_or(core::mem::zeroed()) as _, phnsminimumdeviceperiod.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn Start(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "winnt")]
    pub unsafe fn SetEventHandle(&self, eventhandle: super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventHandle)(windows_core::Interface::as_raw(self), eventhandle) }
    }
    pub unsafe fn GetService<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi"))]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::audiosessiontypes::AUDCLNT_SHAREMODE, u32, super::mediaobj::REFERENCE_TIME, super::mediaobj::REFERENCE_TIME, *const super::mmeapi::WAVEFORMATEX, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi")))]
    Initialize: usize,
    pub GetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mediaobj")]
    pub GetStreamLatency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mediaobj::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "mediaobj"))]
    GetStreamLatency: usize,
    pub GetCurrentPadding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "audiosessiontypes", feature = "mmeapi"))]
    pub IsFormatSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::audiosessiontypes::AUDCLNT_SHAREMODE, *const super::mmeapi::WAVEFORMATEX, *mut *mut super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "audiosessiontypes", feature = "mmeapi")))]
    IsFormatSupported: usize,
    #[cfg(feature = "mmeapi")]
    pub GetMixFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    GetMixFormat: usize,
    #[cfg(feature = "mediaobj")]
    pub GetDevicePeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::mediaobj::REFERENCE_TIME, *mut super::mediaobj::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "mediaobj"))]
    GetDevicePeriod: usize,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "winnt")]
    pub SetEventHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "winnt"))]
    SetEventHandle: usize,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi", feature = "winnt"))]
pub trait IAudioClient_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, sharemode: super::audiosessiontypes::AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: super::mediaobj::REFERENCE_TIME, hnsperiodicity: super::mediaobj::REFERENCE_TIME, pformat: *const super::mmeapi::WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetBufferSize(&self) -> windows_core::Result<u32>;
    fn GetStreamLatency(&self) -> windows_core::Result<super::mediaobj::REFERENCE_TIME>;
    fn GetCurrentPadding(&self) -> windows_core::Result<u32>;
    fn IsFormatSupported(&self, sharemode: super::audiosessiontypes::AUDCLNT_SHAREMODE, pformat: *const super::mmeapi::WAVEFORMATEX, ppclosestmatch: *mut *mut super::mmeapi::WAVEFORMATEX) -> windows_core::Result<()>;
    fn GetMixFormat(&self) -> windows_core::Result<*mut super::mmeapi::WAVEFORMATEX>;
    fn GetDevicePeriod(&self, phnsdefaultdeviceperiod: *mut super::mediaobj::REFERENCE_TIME, phnsminimumdeviceperiod: *mut super::mediaobj::REFERENCE_TIME) -> windows_core::Result<()>;
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
    fn SetEventHandle(&self, eventhandle: super::winnt::HANDLE) -> windows_core::Result<()>;
    fn GetService(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi", feature = "winnt"))]
impl IAudioClient_Vtbl {
    pub const fn new<Identity: IAudioClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharemode: super::audiosessiontypes::AUDCLNT_SHAREMODE, streamflags: u32, hnsbufferduration: super::mediaobj::REFERENCE_TIME, hnsperiodicity: super::mediaobj::REFERENCE_TIME, pformat: *const super::mmeapi::WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::Initialize(this, core::mem::transmute_copy(&sharemode), core::mem::transmute_copy(&streamflags), core::mem::transmute_copy(&hnsbufferduration), core::mem::transmute_copy(&hnsperiodicity), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&audiosessionguid)).into()
            }
        }
        unsafe extern "system" fn GetBufferSize<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumbufferframes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient_Impl::GetBufferSize(this) {
                    Ok(ok__) => {
                        pnumbufferframes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamLatency<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnslatency: *mut super::mediaobj::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient_Impl::GetStreamLatency(this) {
                    Ok(ok__) => {
                        phnslatency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPadding<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnumpaddingframes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient_Impl::GetCurrentPadding(this) {
                    Ok(ok__) => {
                        pnumpaddingframes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFormatSupported<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sharemode: super::audiosessiontypes::AUDCLNT_SHAREMODE, pformat: *const super::mmeapi::WAVEFORMATEX, ppclosestmatch: *mut *mut super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::IsFormatSupported(this, core::mem::transmute_copy(&sharemode), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&ppclosestmatch)).into()
            }
        }
        unsafe extern "system" fn GetMixFormat<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdeviceformat: *mut *mut super::mmeapi::WAVEFORMATEX) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient_Impl::GetMixFormat(this) {
                    Ok(ok__) => {
                        ppdeviceformat.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDevicePeriod<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnsdefaultdeviceperiod: *mut super::mediaobj::REFERENCE_TIME, phnsminimumdeviceperiod: *mut super::mediaobj::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::GetDevicePeriod(this, core::mem::transmute_copy(&phnsdefaultdeviceperiod), core::mem::transmute_copy(&phnsminimumdeviceperiod)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn SetEventHandle<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventhandle: super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::SetEventHandle(this, core::mem::transmute_copy(&eventhandle)).into()
            }
        }
        unsafe extern "system" fn GetService<Identity: IAudioClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient_Impl::GetService(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, OFFSET>,
            GetStreamLatency: GetStreamLatency::<Identity, OFFSET>,
            GetCurrentPadding: GetCurrentPadding::<Identity, OFFSET>,
            IsFormatSupported: IsFormatSupported::<Identity, OFFSET>,
            GetMixFormat: GetMixFormat::<Identity, OFFSET>,
            GetDevicePeriod: GetDevicePeriod::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
            SetEventHandle: SetEventHandle::<Identity, OFFSET>,
            GetService: GetService::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClient as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi", feature = "winnt"))]
impl windows_core::RuntimeName for IAudioClient {}
windows_core::imp::define_interface!(IAudioClient2, IAudioClient2_Vtbl, 0x726778cd_f60a_4eda_82de_e47610cd78aa);
impl core::ops::Deref for IAudioClient2 {
    type Target = IAudioClient;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClient2, windows_core::IUnknown, IAudioClient);
impl IAudioClient2 {
    #[cfg(feature = "audiosessiontypes")]
    pub unsafe fn IsOffloadCapable(&self, category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsOffloadCapable)(windows_core::Interface::as_raw(self), category, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "audiosessiontypes")]
    pub unsafe fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientProperties)(windows_core::Interface::as_raw(self), pproperties) }
    }
    #[cfg(all(feature = "mediaobj", feature = "mmeapi"))]
    pub unsafe fn GetBufferSizeLimits(&self, pformat: *const super::mmeapi::WAVEFORMATEX, beventdriven: bool, phnsminbufferduration: *mut super::mediaobj::REFERENCE_TIME, phnsmaxbufferduration: *mut super::mediaobj::REFERENCE_TIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetBufferSizeLimits)(windows_core::Interface::as_raw(self), pformat, beventdriven.into(), phnsminbufferduration as _, phnsmaxbufferduration as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient2_Vtbl {
    pub base__: IAudioClient_Vtbl,
    #[cfg(feature = "audiosessiontypes")]
    pub IsOffloadCapable: unsafe extern "system" fn(*mut core::ffi::c_void, super::audiosessiontypes::AUDIO_STREAM_CATEGORY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "audiosessiontypes"))]
    IsOffloadCapable: usize,
    #[cfg(feature = "audiosessiontypes")]
    pub SetClientProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *const AudioClientProperties) -> windows_core::HRESULT,
    #[cfg(not(feature = "audiosessiontypes"))]
    SetClientProperties: usize,
    #[cfg(all(feature = "mediaobj", feature = "mmeapi"))]
    pub GetBufferSizeLimits: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::mmeapi::WAVEFORMATEX, windows_core::BOOL, *mut super::mediaobj::REFERENCE_TIME, *mut super::mediaobj::REFERENCE_TIME) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mediaobj", feature = "mmeapi")))]
    GetBufferSizeLimits: usize,
}
#[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi", feature = "winnt"))]
pub trait IAudioClient2_Impl: IAudioClient_Impl {
    fn IsOffloadCapable(&self, category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY) -> windows_core::Result<windows_core::BOOL>;
    fn SetClientProperties(&self, pproperties: *const AudioClientProperties) -> windows_core::Result<()>;
    fn GetBufferSizeLimits(&self, pformat: *const super::mmeapi::WAVEFORMATEX, beventdriven: windows_core::BOOL, phnsminbufferduration: *mut super::mediaobj::REFERENCE_TIME, phnsmaxbufferduration: *mut super::mediaobj::REFERENCE_TIME) -> windows_core::Result<()>;
}
#[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi", feature = "winnt"))]
impl IAudioClient2_Vtbl {
    pub const fn new<Identity: IAudioClient2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsOffloadCapable<Identity: IAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, category: super::audiosessiontypes::AUDIO_STREAM_CATEGORY, pboffloadcapable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClient2_Impl::IsOffloadCapable(this, core::mem::transmute_copy(&category)) {
                    Ok(ok__) => {
                        pboffloadcapable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientProperties<Identity: IAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproperties: *const AudioClientProperties) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient2_Impl::SetClientProperties(this, core::mem::transmute_copy(&pproperties)).into()
            }
        }
        unsafe extern "system" fn GetBufferSizeLimits<Identity: IAudioClient2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const super::mmeapi::WAVEFORMATEX, beventdriven: windows_core::BOOL, phnsminbufferduration: *mut super::mediaobj::REFERENCE_TIME, phnsmaxbufferduration: *mut super::mediaobj::REFERENCE_TIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient2_Impl::GetBufferSizeLimits(this, core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&beventdriven), core::mem::transmute_copy(&phnsminbufferduration), core::mem::transmute_copy(&phnsmaxbufferduration)).into()
            }
        }
        Self {
            base__: IAudioClient_Vtbl::new::<Identity, OFFSET>(),
            IsOffloadCapable: IsOffloadCapable::<Identity, OFFSET>,
            SetClientProperties: SetClientProperties::<Identity, OFFSET>,
            GetBufferSizeLimits: GetBufferSizeLimits::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClient2 as windows_core::Interface>::IID || iid == &<IAudioClient as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi", feature = "winnt"))]
impl windows_core::RuntimeName for IAudioClient2 {}
windows_core::imp::define_interface!(IAudioClient3, IAudioClient3_Vtbl, 0x7ed4ee07_8e67_4cd4_8c1a_2b7a5987ad42);
impl core::ops::Deref for IAudioClient3 {
    type Target = IAudioClient2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IAudioClient3, windows_core::IUnknown, IAudioClient, IAudioClient2);
impl IAudioClient3 {
    #[cfg(feature = "mmeapi")]
    pub unsafe fn GetSharedModeEnginePeriod(&self, pformat: *const super::mmeapi::WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSharedModeEnginePeriod)(windows_core::Interface::as_raw(self), pformat, pdefaultperiodinframes as _, pfundamentalperiodinframes as _, pminperiodinframes as _, pmaxperiodinframes as _) }
    }
    #[cfg(feature = "mmeapi")]
    pub unsafe fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut super::mmeapi::WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentSharedModeEnginePeriod)(windows_core::Interface::as_raw(self), ppformat as _, pcurrentperiodinframes as _) }
    }
    #[cfg(feature = "mmeapi")]
    pub unsafe fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const super::mmeapi::WAVEFORMATEX, audiosessionguid: Option<*const windows_core::GUID>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeSharedAudioStream)(windows_core::Interface::as_raw(self), streamflags, periodinframes, pformat, audiosessionguid.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClient3_Vtbl {
    pub base__: IAudioClient2_Vtbl,
    #[cfg(feature = "mmeapi")]
    pub GetSharedModeEnginePeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::mmeapi::WAVEFORMATEX, *mut u32, *mut u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    GetSharedModeEnginePeriod: usize,
    #[cfg(feature = "mmeapi")]
    pub GetCurrentSharedModeEnginePeriod: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::mmeapi::WAVEFORMATEX, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    GetCurrentSharedModeEnginePeriod: usize,
    #[cfg(feature = "mmeapi")]
    pub InitializeSharedAudioStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const super::mmeapi::WAVEFORMATEX, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "mmeapi"))]
    InitializeSharedAudioStream: usize,
}
#[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi", feature = "winnt"))]
pub trait IAudioClient3_Impl: IAudioClient2_Impl {
    fn GetSharedModeEnginePeriod(&self, pformat: *const super::mmeapi::WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> windows_core::Result<()>;
    fn GetCurrentSharedModeEnginePeriod(&self, ppformat: *mut *mut super::mmeapi::WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> windows_core::Result<()>;
    fn InitializeSharedAudioStream(&self, streamflags: u32, periodinframes: u32, pformat: *const super::mmeapi::WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::Result<()>;
}
#[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi", feature = "winnt"))]
impl IAudioClient3_Vtbl {
    pub const fn new<Identity: IAudioClient3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSharedModeEnginePeriod<Identity: IAudioClient3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pformat: *const super::mmeapi::WAVEFORMATEX, pdefaultperiodinframes: *mut u32, pfundamentalperiodinframes: *mut u32, pminperiodinframes: *mut u32, pmaxperiodinframes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient3_Impl::GetSharedModeEnginePeriod(this, core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&pdefaultperiodinframes), core::mem::transmute_copy(&pfundamentalperiodinframes), core::mem::transmute_copy(&pminperiodinframes), core::mem::transmute_copy(&pmaxperiodinframes)).into()
            }
        }
        unsafe extern "system" fn GetCurrentSharedModeEnginePeriod<Identity: IAudioClient3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppformat: *mut *mut super::mmeapi::WAVEFORMATEX, pcurrentperiodinframes: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient3_Impl::GetCurrentSharedModeEnginePeriod(this, core::mem::transmute_copy(&ppformat), core::mem::transmute_copy(&pcurrentperiodinframes)).into()
            }
        }
        unsafe extern "system" fn InitializeSharedAudioStream<Identity: IAudioClient3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamflags: u32, periodinframes: u32, pformat: *const super::mmeapi::WAVEFORMATEX, audiosessionguid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClient3_Impl::InitializeSharedAudioStream(this, core::mem::transmute_copy(&streamflags), core::mem::transmute_copy(&periodinframes), core::mem::transmute_copy(&pformat), core::mem::transmute_copy(&audiosessionguid)).into()
            }
        }
        Self {
            base__: IAudioClient2_Vtbl::new::<Identity, OFFSET>(),
            GetSharedModeEnginePeriod: GetSharedModeEnginePeriod::<Identity, OFFSET>,
            GetCurrentSharedModeEnginePeriod: GetCurrentSharedModeEnginePeriod::<Identity, OFFSET>,
            InitializeSharedAudioStream: InitializeSharedAudioStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClient3 as windows_core::Interface>::IID || iid == &<IAudioClient as windows_core::Interface>::IID || iid == &<IAudioClient2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "audiosessiontypes", feature = "mediaobj", feature = "mmeapi", feature = "winnt"))]
impl windows_core::RuntimeName for IAudioClient3 {}
windows_core::imp::define_interface!(IAudioClientDuckingControl, IAudioClientDuckingControl_Vtbl, 0xc789d381_a28c_4168_b28f_d3a837924dc3);
windows_core::imp::interface_hierarchy!(IAudioClientDuckingControl, windows_core::IUnknown);
impl IAudioClientDuckingControl {
    pub unsafe fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDuckingOptionsForCurrentStream)(windows_core::Interface::as_raw(self), options) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClientDuckingControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDuckingOptionsForCurrentStream: unsafe extern "system" fn(*mut core::ffi::c_void, AUDIO_DUCKING_OPTIONS) -> windows_core::HRESULT,
}
pub trait IAudioClientDuckingControl_Impl: windows_core::IUnknownImpl {
    fn SetDuckingOptionsForCurrentStream(&self, options: AUDIO_DUCKING_OPTIONS) -> windows_core::Result<()>;
}
impl IAudioClientDuckingControl_Vtbl {
    pub const fn new<Identity: IAudioClientDuckingControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDuckingOptionsForCurrentStream<Identity: IAudioClientDuckingControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: AUDIO_DUCKING_OPTIONS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClientDuckingControl_Impl::SetDuckingOptionsForCurrentStream(this, core::mem::transmute_copy(&options)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDuckingOptionsForCurrentStream: SetDuckingOptionsForCurrentStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClientDuckingControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioClientDuckingControl {}
windows_core::imp::define_interface!(IAudioClock, IAudioClock_Vtbl, 0xcd63314f_3fba_4a1b_812c_ef96358728e7);
windows_core::imp::interface_hierarchy!(IAudioClock, windows_core::IUnknown);
impl IAudioClock {
    pub unsafe fn GetFrequency(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFrequency)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: Option<*mut u64>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPosition)(windows_core::Interface::as_raw(self), pu64position as _, pu64qpcposition.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetCharacteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFrequency: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IAudioClock_Impl: windows_core::IUnknownImpl {
    fn GetFrequency(&self) -> windows_core::Result<u64>;
    fn GetPosition(&self, pu64position: *mut u64, pu64qpcposition: *mut u64) -> windows_core::Result<()>;
    fn GetCharacteristics(&self) -> windows_core::Result<u32>;
}
impl IAudioClock_Vtbl {
    pub const fn new<Identity: IAudioClock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFrequency<Identity: IAudioClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu64frequency: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClock_Impl::GetFrequency(this) {
                    Ok(ok__) => {
                        pu64frequency.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPosition<Identity: IAudioClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pu64position: *mut u64, pu64qpcposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClock_Impl::GetPosition(this, core::mem::transmute_copy(&pu64position), core::mem::transmute_copy(&pu64qpcposition)).into()
            }
        }
        unsafe extern "system" fn GetCharacteristics<Identity: IAudioClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcharacteristics: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioClock_Impl::GetCharacteristics(this) {
                    Ok(ok__) => {
                        pdwcharacteristics.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrequency: GetFrequency::<Identity, OFFSET>,
            GetPosition: GetPosition::<Identity, OFFSET>,
            GetCharacteristics: GetCharacteristics::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioClock {}
windows_core::imp::define_interface!(IAudioClock2, IAudioClock2_Vtbl, 0x6f49ff73_6727_49ac_a008_d98cf5e70048);
windows_core::imp::interface_hierarchy!(IAudioClock2, windows_core::IUnknown);
impl IAudioClock2 {
    pub unsafe fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: Option<*mut u64>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDevicePosition)(windows_core::Interface::as_raw(self), deviceposition as _, qpcposition.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClock2_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDevicePosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64) -> windows_core::HRESULT,
}
pub trait IAudioClock2_Impl: windows_core::IUnknownImpl {
    fn GetDevicePosition(&self, deviceposition: *mut u64, qpcposition: *mut u64) -> windows_core::Result<()>;
}
impl IAudioClock2_Vtbl {
    pub const fn new<Identity: IAudioClock2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDevicePosition<Identity: IAudioClock2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, deviceposition: *mut u64, qpcposition: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClock2_Impl::GetDevicePosition(this, core::mem::transmute_copy(&deviceposition), core::mem::transmute_copy(&qpcposition)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDevicePosition: GetDevicePosition::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClock2 as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioClock2 {}
windows_core::imp::define_interface!(IAudioClockAdjustment, IAudioClockAdjustment_Vtbl, 0xf6e4c0a0_46d9_4fb8_be21_57a3ef2b626c);
windows_core::imp::interface_hierarchy!(IAudioClockAdjustment, windows_core::IUnknown);
impl IAudioClockAdjustment {
    pub unsafe fn SetSampleRate(&self, flsamplerate: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSampleRate)(windows_core::Interface::as_raw(self), flsamplerate) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioClockAdjustment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetSampleRate: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
}
pub trait IAudioClockAdjustment_Impl: windows_core::IUnknownImpl {
    fn SetSampleRate(&self, flsamplerate: f32) -> windows_core::Result<()>;
}
impl IAudioClockAdjustment_Vtbl {
    pub const fn new<Identity: IAudioClockAdjustment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSampleRate<Identity: IAudioClockAdjustment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flsamplerate: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioClockAdjustment_Impl::SetSampleRate(this, core::mem::transmute_copy(&flsamplerate)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSampleRate: SetSampleRate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioClockAdjustment as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioClockAdjustment {}
windows_core::imp::define_interface!(IAudioEffectsChangedNotificationClient, IAudioEffectsChangedNotificationClient_Vtbl, 0xa5ded44f_3c5d_4b2b_bd1e_5dc1ee20bbf6);
windows_core::imp::interface_hierarchy!(IAudioEffectsChangedNotificationClient, windows_core::IUnknown);
impl IAudioEffectsChangedNotificationClient {
    pub unsafe fn OnAudioEffectsChanged(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnAudioEffectsChanged)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsChangedNotificationClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAudioEffectsChanged: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAudioEffectsChangedNotificationClient_Impl: windows_core::IUnknownImpl {
    fn OnAudioEffectsChanged(&self) -> windows_core::Result<()>;
}
impl IAudioEffectsChangedNotificationClient_Vtbl {
    pub const fn new<Identity: IAudioEffectsChangedNotificationClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAudioEffectsChanged<Identity: IAudioEffectsChangedNotificationClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsChangedNotificationClient_Impl::OnAudioEffectsChanged(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnAudioEffectsChanged: OnAudioEffectsChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEffectsChangedNotificationClient as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEffectsChangedNotificationClient {}
windows_core::imp::define_interface!(IAudioEffectsManager, IAudioEffectsManager_Vtbl, 0x4460b3ae_4b44_4527_8676_7548a8acd260);
windows_core::imp::interface_hierarchy!(IAudioEffectsManager, windows_core::IUnknown);
impl IAudioEffectsManager {
    pub unsafe fn RegisterAudioEffectsChangedNotificationCallback<P0>(&self, client: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioEffectsChangedNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterAudioEffectsChangedNotificationCallback)(windows_core::Interface::as_raw(self), client.param().abi()) }
    }
    pub unsafe fn UnregisterAudioEffectsChangedNotificationCallback<P0>(&self, client: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IAudioEffectsChangedNotificationClient>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnregisterAudioEffectsChangedNotificationCallback)(windows_core::Interface::as_raw(self), client.param().abi()) }
    }
    pub unsafe fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAudioEffects)(windows_core::Interface::as_raw(self), effects as _, numeffects as _) }
    }
    pub unsafe fn SetAudioEffectState(&self, effectid: windows_core::GUID, state: AUDIO_EFFECT_STATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAudioEffectState)(windows_core::Interface::as_raw(self), core::mem::transmute(effectid), state) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioEffectsManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnregisterAudioEffectsChangedNotificationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAudioEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut AUDIO_EFFECT, *mut u32) -> windows_core::HRESULT,
    pub SetAudioEffectState: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, AUDIO_EFFECT_STATE) -> windows_core::HRESULT,
}
pub trait IAudioEffectsManager_Impl: windows_core::IUnknownImpl {
    fn RegisterAudioEffectsChangedNotificationCallback(&self, client: windows_core::Ref<IAudioEffectsChangedNotificationClient>) -> windows_core::Result<()>;
    fn UnregisterAudioEffectsChangedNotificationCallback(&self, client: windows_core::Ref<IAudioEffectsChangedNotificationClient>) -> windows_core::Result<()>;
    fn GetAudioEffects(&self, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> windows_core::Result<()>;
    fn SetAudioEffectState(&self, effectid: &windows_core::GUID, state: AUDIO_EFFECT_STATE) -> windows_core::Result<()>;
}
impl IAudioEffectsManager_Vtbl {
    pub const fn new<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterAudioEffectsChangedNotificationCallback<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, client: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsManager_Impl::RegisterAudioEffectsChangedNotificationCallback(this, core::mem::transmute_copy(&client)).into()
            }
        }
        unsafe extern "system" fn UnregisterAudioEffectsChangedNotificationCallback<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, client: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsManager_Impl::UnregisterAudioEffectsChangedNotificationCallback(this, core::mem::transmute_copy(&client)).into()
            }
        }
        unsafe extern "system" fn GetAudioEffects<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effects: *mut *mut AUDIO_EFFECT, numeffects: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsManager_Impl::GetAudioEffects(this, core::mem::transmute_copy(&effects), core::mem::transmute_copy(&numeffects)).into()
            }
        }
        unsafe extern "system" fn SetAudioEffectState<Identity: IAudioEffectsManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, effectid: windows_core::GUID, state: AUDIO_EFFECT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioEffectsManager_Impl::SetAudioEffectState(this, core::mem::transmute(&effectid), core::mem::transmute_copy(&state)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterAudioEffectsChangedNotificationCallback: RegisterAudioEffectsChangedNotificationCallback::<Identity, OFFSET>,
            UnregisterAudioEffectsChangedNotificationCallback: UnregisterAudioEffectsChangedNotificationCallback::<Identity, OFFSET>,
            GetAudioEffects: GetAudioEffects::<Identity, OFFSET>,
            SetAudioEffectState: SetAudioEffectState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioEffectsManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioEffectsManager {}
windows_core::imp::define_interface!(IAudioRenderClient, IAudioRenderClient_Vtbl, 0xf294acfc_3146_4483_a7bf_addca7c260e2);
windows_core::imp::interface_hierarchy!(IAudioRenderClient, windows_core::IUnknown);
impl IAudioRenderClient {
    pub unsafe fn GetBuffer(&self, numframesrequested: u32) -> windows_core::Result<*mut u8> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBuffer)(windows_core::Interface::as_raw(self), numframesrequested, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseBuffer)(windows_core::Interface::as_raw(self), numframeswritten, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioRenderClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8) -> windows_core::HRESULT,
    pub ReleaseBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
}
pub trait IAudioRenderClient_Impl: windows_core::IUnknownImpl {
    fn GetBuffer(&self, numframesrequested: u32) -> windows_core::Result<*mut u8>;
    fn ReleaseBuffer(&self, numframeswritten: u32, dwflags: u32) -> windows_core::Result<()>;
}
impl IAudioRenderClient_Vtbl {
    pub const fn new<Identity: IAudioRenderClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetBuffer<Identity: IAudioRenderClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numframesrequested: u32, ppdata: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioRenderClient_Impl::GetBuffer(this, core::mem::transmute_copy(&numframesrequested)) {
                    Ok(ok__) => {
                        ppdata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReleaseBuffer<Identity: IAudioRenderClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numframeswritten: u32, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioRenderClient_Impl::ReleaseBuffer(this, core::mem::transmute_copy(&numframeswritten), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, OFFSET>,
            ReleaseBuffer: ReleaseBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioRenderClient as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioRenderClient {}
windows_core::imp::define_interface!(IAudioStreamVolume, IAudioStreamVolume_Vtbl, 0x93014887_242d_4068_8a15_cf5e93b90fe3);
windows_core::imp::interface_hierarchy!(IAudioStreamVolume, windows_core::IUnknown);
impl IAudioStreamVolume {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, flevel) }
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr())) }
    }
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr())) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioStreamVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32) -> windows_core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
pub trait IAudioStreamVolume_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> windows_core::Result<()>;
    fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32>;
    fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32) -> windows_core::Result<()>;
    fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> windows_core::Result<()>;
}
impl IAudioStreamVolume_Vtbl {
    pub const fn new<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioStreamVolume_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChannelVolume<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, flevel: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioStreamVolume_Impl::SetChannelVolume(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&flevel)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolume<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAudioStreamVolume_Impl::GetChannelVolume(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pflevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllVolumes<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioStreamVolume_Impl::SetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
            }
        }
        unsafe extern "system" fn GetAllVolumes<Identity: IAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioStreamVolume_Impl::GetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            SetChannelVolume: SetChannelVolume::<Identity, OFFSET>,
            GetChannelVolume: GetChannelVolume::<Identity, OFFSET>,
            SetAllVolumes: SetAllVolumes::<Identity, OFFSET>,
            GetAllVolumes: GetAllVolumes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioStreamVolume as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAudioStreamVolume {}
windows_core::imp::define_interface!(IAudioViewManagerService, IAudioViewManagerService_Vtbl, 0xa7a7ef10_1f49_45e0_ad35_612057cc8f74);
windows_core::imp::interface_hierarchy!(IAudioViewManagerService, windows_core::IUnknown);
impl IAudioViewManagerService {
    #[cfg(feature = "windef")]
    pub unsafe fn SetAudioStreamWindow(&self, hwnd: super::windef::HWND) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAudioStreamWindow)(windows_core::Interface::as_raw(self), hwnd) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioViewManagerService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub SetAudioStreamWindow: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetAudioStreamWindow: usize,
}
#[cfg(feature = "windef")]
pub trait IAudioViewManagerService_Impl: windows_core::IUnknownImpl {
    fn SetAudioStreamWindow(&self, hwnd: super::windef::HWND) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IAudioViewManagerService_Vtbl {
    pub const fn new<Identity: IAudioViewManagerService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAudioStreamWindow<Identity: IAudioViewManagerService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAudioViewManagerService_Impl::SetAudioStreamWindow(this, core::mem::transmute_copy(&hwnd)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetAudioStreamWindow: SetAudioStreamWindow::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAudioViewManagerService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IAudioViewManagerService {}
windows_core::imp::define_interface!(IChannelAudioVolume, IChannelAudioVolume_Vtbl, 0x1c158861_b533_4b30_b1cf_e853e51c59b8);
windows_core::imp::interface_hierarchy!(IChannelAudioVolume, windows_core::IUnknown);
impl IChannelAudioVolume {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, flevel, eventcontext) }
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllVolumes(&self, pfvolumes: &[f32], eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr()), eventcontext) }
    }
    pub unsafe fn GetAllVolumes(&self, pfvolumes: &mut [f32]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllVolumes)(windows_core::Interface::as_raw(self), pfvolumes.len().try_into().unwrap(), core::mem::transmute(pfvolumes.as_ptr())) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChannelAudioVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
pub trait IChannelAudioVolume_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetChannelVolume(&self, dwindex: u32, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32>;
    fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetAllVolumes(&self, dwcount: u32, pfvolumes: *mut f32) -> windows_core::Result<()>;
}
impl IChannelAudioVolume_Vtbl {
    pub const fn new<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IChannelAudioVolume_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChannelVolume<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelAudioVolume_Impl::SetChannelVolume(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolume<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IChannelAudioVolume_Impl::GetChannelVolume(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pflevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllVolumes<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *const f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelAudioVolume_Impl::SetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetAllVolumes<Identity: IChannelAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IChannelAudioVolume_Impl::GetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            SetChannelVolume: SetChannelVolume::<Identity, OFFSET>,
            GetChannelVolume: GetChannelVolume::<Identity, OFFSET>,
            SetAllVolumes: SetAllVolumes::<Identity, OFFSET>,
            GetAllVolumes: GetAllVolumes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IChannelAudioVolume as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IChannelAudioVolume {}
windows_core::imp::define_interface!(ISimpleAudioVolume, ISimpleAudioVolume_Vtbl, 0x87ce5498_68d6_44e5_9215_6da47ef883d8);
windows_core::imp::interface_hierarchy!(ISimpleAudioVolume, windows_core::IUnknown);
impl ISimpleAudioVolume {
    pub unsafe fn SetMasterVolume(&self, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMasterVolume)(windows_core::Interface::as_raw(self), flevel, eventcontext) }
    }
    pub unsafe fn GetMasterVolume(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMasterVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMute(&self, bmute: bool, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmute.into(), eventcontext) }
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISimpleAudioVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMasterVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMasterVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ISimpleAudioVolume_Impl: windows_core::IUnknownImpl {
    fn SetMasterVolume(&self, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMasterVolume(&self) -> windows_core::Result<f32>;
    fn SetMute(&self, bmute: windows_core::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl ISimpleAudioVolume_Vtbl {
    pub const fn new<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMasterVolume<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flevel: f32, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimpleAudioVolume_Impl::SetMasterVolume(this, core::mem::transmute_copy(&flevel), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMasterVolume<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimpleAudioVolume_Impl::GetMasterVolume(this) {
                    Ok(ok__) => {
                        pflevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMute<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: windows_core::BOOL, eventcontext: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISimpleAudioVolume_Impl::SetMute(this, core::mem::transmute_copy(&bmute), core::mem::transmute_copy(&eventcontext)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: ISimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmute: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISimpleAudioVolume_Impl::GetMute(this) {
                    Ok(ok__) => {
                        pbmute.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMasterVolume: SetMasterVolume::<Identity, OFFSET>,
            GetMasterVolume: GetMasterVolume::<Identity, OFFSET>,
            SetMute: SetMute::<Identity, OFFSET>,
            GetMute: GetMute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISimpleAudioVolume as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISimpleAudioVolume {}
pub type _AUDCLNT_BUFFERFLAGS = i32;
