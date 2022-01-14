pub trait IAudioEndpointFormatControl_Impl: Sized {
    fn ResetToDefault(&mut self, resetflags: u32) -> ::windows::core::Result<()>;
}
impl IAudioEndpointFormatControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointFormatControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointFormatControl_Vtbl {
        unsafe extern "system" fn ResetToDefault<Impl: IAudioEndpointFormatControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resetflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetToDefault(::core::mem::transmute_copy(&resetflags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ResetToDefault: ResetToDefault::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointFormatControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
pub trait IAudioEndpointLastBufferControl_Impl: Sized {
    fn IsLastBufferControlSupported(&mut self) -> super::super::super::Foundation::BOOL;
    fn ReleaseOutputDataPointerForLastBuffer(&mut self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
impl IAudioEndpointLastBufferControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointLastBufferControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointLastBufferControl_Vtbl {
        unsafe extern "system" fn IsLastBufferControlSupported<Impl: IAudioEndpointLastBufferControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLastBufferControlSupported()
        }
        unsafe extern "system" fn ReleaseOutputDataPointerForLastBuffer<Impl: IAudioEndpointLastBufferControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseOutputDataPointerForLastBuffer(::core::mem::transmute_copy(&pconnectionproperty))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsLastBufferControlSupported: IsLastBufferControlSupported::<Impl, IMPL_OFFSET>,
            ReleaseOutputDataPointerForLastBuffer: ReleaseOutputDataPointerForLastBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointLastBufferControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointOffloadStreamMeter_Impl: Sized {
    fn GetMeterChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetMeteringData(&mut self, u32channelcount: u32) -> ::windows::core::Result<f32>;
}
impl IAudioEndpointOffloadStreamMeter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointOffloadStreamMeter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointOffloadStreamMeter_Vtbl {
        unsafe extern "system" fn GetMeterChannelCount<Impl: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMeterChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pu32channelcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeteringData<Impl: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMeteringData(::core::mem::transmute_copy(&u32channelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *pf32peakvalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMeterChannelCount: GetMeterChannelCount::<Impl, IMPL_OFFSET>,
            GetMeteringData: GetMeteringData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamMeter as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointOffloadStreamMute_Impl: Sized {
    fn SetMute(&mut self, bmuted: u8) -> ::windows::core::Result<()>;
    fn GetMute(&mut self) -> ::windows::core::Result<u8>;
}
impl IAudioEndpointOffloadStreamMute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointOffloadStreamMute_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointOffloadStreamMute_Vtbl {
        unsafe extern "system" fn SetMute<Impl: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmuted: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMute(::core::mem::transmute_copy(&bmuted)).into()
        }
        unsafe extern "system" fn GetMute<Impl: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmuted: *mut u8) -> ::windows::core::HRESULT {
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
        iid == &<IAudioEndpointOffloadStreamMute as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
pub trait IAudioEndpointOffloadStreamVolume_Impl: Sized {
    fn GetVolumeChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetChannelVolumes(&mut self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::core::Result<()>;
    fn GetChannelVolumes(&mut self, u32channelcount: u32) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl IAudioEndpointOffloadStreamVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointOffloadStreamVolume_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointOffloadStreamVolume_Vtbl {
        unsafe extern "system" fn GetVolumeChannelCount<Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pu32channelcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolumes<Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelVolumes(::core::mem::transmute_copy(&u32channelcount), ::core::mem::transmute_copy(&pf32volumes), ::core::mem::transmute_copy(&u32curvetype), ::core::mem::transmute_copy(&pcurveduration)).into()
        }
        unsafe extern "system" fn GetChannelVolumes<Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolumes(::core::mem::transmute_copy(&u32channelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *pf32volumes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetVolumeChannelCount: GetVolumeChannelCount::<Impl, IMPL_OFFSET>,
            SetChannelVolumes: SetChannelVolumes::<Impl, IMPL_OFFSET>,
            GetChannelVolumes: GetChannelVolumes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolume_Impl: Sized {
    fn RegisterControlChangeNotify(&mut self, pnotify: ::core::option::Option<IAudioEndpointVolumeCallback>) -> ::windows::core::Result<()>;
    fn UnregisterControlChangeNotify(&mut self, pnotify: ::core::option::Option<IAudioEndpointVolumeCallback>) -> ::windows::core::Result<()>;
    fn GetChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetMasterVolumeLevel(&mut self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetMasterVolumeLevelScalar(&mut self, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMasterVolumeLevel(&mut self) -> ::windows::core::Result<f32>;
    fn GetMasterVolumeLevelScalar(&mut self) -> ::windows::core::Result<f32>;
    fn SetChannelVolumeLevel(&mut self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetChannelVolumeLevelScalar(&mut self, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetChannelVolumeLevel(&mut self, nchannel: u32) -> ::windows::core::Result<f32>;
    fn GetChannelVolumeLevelScalar(&mut self, nchannel: u32) -> ::windows::core::Result<f32>;
    fn SetMute(&mut self, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMute(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetVolumeStepInfo(&mut self, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::core::Result<()>;
    fn VolumeStepUp(&mut self, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn VolumeStepDown(&mut self, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn QueryHardwareSupport(&mut self) -> ::windows::core::Result<u32>;
    fn GetVolumeRange(&mut self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEndpointVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointVolume_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointVolume_Vtbl {
        unsafe extern "system" fn RegisterControlChangeNotify<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterControlChangeNotify(::core::mem::transmute(&pnotify)).into()
        }
        unsafe extern "system" fn UnregisterControlChangeNotify<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterControlChangeNotify(::core::mem::transmute(&pnotify)).into()
        }
        unsafe extern "system" fn GetChannelCount<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pnchannelcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterVolumeLevel<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMasterVolumeLevel(::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetMasterVolumeLevelScalar<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMasterVolumeLevelScalar(::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetMasterVolumeLevel<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMasterVolumeLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pfleveldb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMasterVolumeLevelScalar<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMasterVolumeLevelScalar() {
                ::core::result::Result::Ok(ok__) => {
                    *pflevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolumeLevel<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelVolumeLevel(::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetChannelVolumeLevelScalar<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetChannelVolumeLevelScalar(::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetChannelVolumeLevel<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolumeLevel(::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfleveldb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelVolumeLevelScalar<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolumeLevelScalar(::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflevel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMute<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMute(::core::mem::transmute_copy(&bmute), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetMute<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMute() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmute = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeStepInfo<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVolumeStepInfo(::core::mem::transmute_copy(&pnstep), ::core::mem::transmute_copy(&pnstepcount)).into()
        }
        unsafe extern "system" fn VolumeStepUp<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VolumeStepUp(::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn VolumeStepDown<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).VolumeStepDown(::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn QueryHardwareSupport<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHardwareSupport() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwhardwaresupportmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeRange<Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVolumeRange(::core::mem::transmute_copy(&pflvolumemindb), ::core::mem::transmute_copy(&pflvolumemaxdb), ::core::mem::transmute_copy(&pflvolumeincrementdb)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterControlChangeNotify: RegisterControlChangeNotify::<Impl, IMPL_OFFSET>,
            UnregisterControlChangeNotify: UnregisterControlChangeNotify::<Impl, IMPL_OFFSET>,
            GetChannelCount: GetChannelCount::<Impl, IMPL_OFFSET>,
            SetMasterVolumeLevel: SetMasterVolumeLevel::<Impl, IMPL_OFFSET>,
            SetMasterVolumeLevelScalar: SetMasterVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            GetMasterVolumeLevel: GetMasterVolumeLevel::<Impl, IMPL_OFFSET>,
            GetMasterVolumeLevelScalar: GetMasterVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            SetChannelVolumeLevel: SetChannelVolumeLevel::<Impl, IMPL_OFFSET>,
            SetChannelVolumeLevelScalar: SetChannelVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            GetChannelVolumeLevel: GetChannelVolumeLevel::<Impl, IMPL_OFFSET>,
            GetChannelVolumeLevelScalar: GetChannelVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            SetMute: SetMute::<Impl, IMPL_OFFSET>,
            GetMute: GetMute::<Impl, IMPL_OFFSET>,
            GetVolumeStepInfo: GetVolumeStepInfo::<Impl, IMPL_OFFSET>,
            VolumeStepUp: VolumeStepUp::<Impl, IMPL_OFFSET>,
            VolumeStepDown: VolumeStepDown::<Impl, IMPL_OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Impl, IMPL_OFFSET>,
            GetVolumeRange: GetVolumeRange::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolumeCallback_Impl: Sized {
    fn OnNotify(&mut self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEndpointVolumeCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointVolumeCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointVolumeCallback_Vtbl {
        unsafe extern "system" fn OnNotify<Impl: IAudioEndpointVolumeCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNotify(::core::mem::transmute_copy(&pnotify)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnNotify: OnNotify::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolumeEx_Impl: Sized + IAudioEndpointVolume_Impl {
    fn GetVolumeRangeChannel(&mut self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEndpointVolumeEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointVolumeEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointVolumeEx_Vtbl {
        unsafe extern "system" fn GetVolumeRangeChannel<Impl: IAudioEndpointVolumeEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetVolumeRangeChannel(::core::mem::transmute_copy(&ichannel), ::core::mem::transmute_copy(&pflvolumemindb), ::core::mem::transmute_copy(&pflvolumemaxdb), ::core::mem::transmute_copy(&pflvolumeincrementdb)).into()
        }
        Self {
            base: IAudioEndpointVolume_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetVolumeRangeChannel: GetVolumeRangeChannel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioLfxControl_Impl: Sized {
    fn SetLocalEffectsState(&mut self, benabled: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLocalEffectsState(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioLfxControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioLfxControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioLfxControl_Vtbl {
        unsafe extern "system" fn SetLocalEffectsState<Impl: IAudioLfxControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalEffectsState(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn GetLocalEffectsState<Impl: IAudioLfxControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalEffectsState() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetLocalEffectsState: SetLocalEffectsState::<Impl, IMPL_OFFSET>,
            GetLocalEffectsState: GetLocalEffectsState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioLfxControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioMeterInformation_Impl: Sized {
    fn GetPeakValue(&mut self) -> ::windows::core::Result<f32>;
    fn GetMeteringChannelCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetChannelsPeakValues(&mut self, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows::core::Result<()>;
    fn QueryHardwareSupport(&mut self) -> ::windows::core::Result<u32>;
}
impl IAudioMeterInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMeterInformation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioMeterInformation_Vtbl {
        unsafe extern "system" fn GetPeakValue<Impl: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpeak: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPeakValue() {
                ::core::result::Result::Ok(ok__) => {
                    *pfpeak = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeteringChannelCount<Impl: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMeteringChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pnchannelcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelsPeakValues<Impl: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetChannelsPeakValues(::core::mem::transmute_copy(&u32channelcount), ::core::mem::transmute_copy(&afpeakvalues)).into()
        }
        unsafe extern "system" fn QueryHardwareSupport<Impl: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHardwareSupport() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwhardwaresupportmask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPeakValue: GetPeakValue::<Impl, IMPL_OFFSET>,
            GetMeteringChannelCount: GetMeteringChannelCount::<Impl, IMPL_OFFSET>,
            GetChannelsPeakValues: GetChannelsPeakValues::<Impl, IMPL_OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMeterInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHardwareAudioEngineBase_Impl: Sized {
    fn GetAvailableOffloadConnectorCount(&mut self, _pwstrdeviceid: super::super::super::Foundation::PWSTR, _uconnectorid: u32) -> ::windows::core::Result<u32>;
    fn GetEngineFormat(&mut self, pdevice: ::core::option::Option<super::IMMDevice>, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn SetEngineDeviceFormat(&mut self, pdevice: ::core::option::Option<super::IMMDevice>, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn SetGfxState(&mut self, pdevice: ::core::option::Option<super::IMMDevice>, _benable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetGfxState(&mut self, pdevice: ::core::option::Option<super::IMMDevice>) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHardwareAudioEngineBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHardwareAudioEngineBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHardwareAudioEngineBase_Vtbl {
        unsafe extern "system" fn GetAvailableOffloadConnectorCount<Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _pwstrdeviceid: super::super::super::Foundation::PWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableOffloadConnectorCount(::core::mem::transmute_copy(&_pwstrdeviceid), ::core::mem::transmute_copy(&_uconnectorid)) {
                ::core::result::Result::Ok(ok__) => {
                    *_pavailableconnectorinstancecount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEngineFormat<Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEngineFormat(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&_brequestdeviceformat), ::core::mem::transmute_copy(&_ppwfxformat)).into()
        }
        unsafe extern "system" fn SetEngineDeviceFormat<Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEngineDeviceFormat(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&_pwfxformat)).into()
        }
        unsafe extern "system" fn SetGfxState<Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _benable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGfxState(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&_benable)).into()
        }
        unsafe extern "system" fn GetGfxState<Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _pbenable: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGfxState(::core::mem::transmute(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *_pbenable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAvailableOffloadConnectorCount: GetAvailableOffloadConnectorCount::<Impl, IMPL_OFFSET>,
            GetEngineFormat: GetEngineFormat::<Impl, IMPL_OFFSET>,
            SetEngineDeviceFormat: SetEngineDeviceFormat::<Impl, IMPL_OFFSET>,
            SetGfxState: SetGfxState::<Impl, IMPL_OFFSET>,
            GetGfxState: GetGfxState::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHardwareAudioEngineBase as ::windows::core::Interface>::IID
    }
}
