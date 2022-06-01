pub trait IAudioEndpointFormatControl_Impl: Sized {
    fn ResetToDefault(&self, resetflags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAudioEndpointFormatControl {}
impl IAudioEndpointFormatControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointFormatControl_Impl, const OFFSET: isize>() -> IAudioEndpointFormatControl_Vtbl {
        unsafe extern "system" fn ResetToDefault<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointFormatControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resetflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetToDefault(::core::mem::transmute_copy(&resetflags)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ResetToDefault: ResetToDefault::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointFormatControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
pub trait IAudioEndpointLastBufferControl_Impl: Sized {
    fn IsLastBufferControlSupported(&self) -> super::super::super::Foundation::BOOL;
    fn ReleaseOutputDataPointerForLastBuffer(&self, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
impl ::windows::core::RuntimeName for IAudioEndpointLastBufferControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
impl IAudioEndpointLastBufferControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointLastBufferControl_Impl, const OFFSET: isize>() -> IAudioEndpointLastBufferControl_Vtbl {
        unsafe extern "system" fn IsLastBufferControlSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointLastBufferControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLastBufferControlSupported()
        }
        unsafe extern "system" fn ReleaseOutputDataPointerForLastBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointLastBufferControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseOutputDataPointerForLastBuffer(::core::mem::transmute_copy(&pconnectionproperty))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsLastBufferControlSupported: IsLastBufferControlSupported::<Identity, Impl, OFFSET>,
            ReleaseOutputDataPointerForLastBuffer: ReleaseOutputDataPointerForLastBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointLastBufferControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointOffloadStreamMeter_Impl: Sized {
    fn GetMeterChannelCount(&self) -> ::windows::core::Result<u32>;
    fn GetMeteringData(&self, u32channelcount: u32) -> ::windows::core::Result<f32>;
}
impl ::windows::core::RuntimeName for IAudioEndpointOffloadStreamMeter {}
impl IAudioEndpointOffloadStreamMeter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: isize>() -> IAudioEndpointOffloadStreamMeter_Vtbl {
        unsafe extern "system" fn GetMeterChannelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMeterChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pu32channelcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeteringData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMeteringData(::core::mem::transmute_copy(&u32channelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pf32peakvalues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetMeterChannelCount: GetMeterChannelCount::<Identity, Impl, OFFSET>,
            GetMeteringData: GetMeteringData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamMeter as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointOffloadStreamMute_Impl: Sized {
    fn SetMute(&self, bmuted: u8) -> ::windows::core::Result<()>;
    fn GetMute(&self) -> ::windows::core::Result<u8>;
}
impl ::windows::core::RuntimeName for IAudioEndpointOffloadStreamMute {}
impl IAudioEndpointOffloadStreamMute_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: isize>() -> IAudioEndpointOffloadStreamMute_Vtbl {
        unsafe extern "system" fn SetMute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmuted: u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMute(::core::mem::transmute_copy(&bmuted)).into()
        }
        unsafe extern "system" fn GetMute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamMute_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmuted: *mut u8) -> ::windows::core::HRESULT {
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
        iid == &<IAudioEndpointOffloadStreamMute as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
pub trait IAudioEndpointOffloadStreamVolume_Impl: Sized {
    fn GetVolumeChannelCount(&self) -> ::windows::core::Result<u32>;
    fn SetChannelVolumes(&self, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::core::Result<()>;
    fn GetChannelVolumes(&self, u32channelcount: u32) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl ::windows::core::RuntimeName for IAudioEndpointOffloadStreamVolume {}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl IAudioEndpointOffloadStreamVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>() -> IAudioEndpointOffloadStreamVolume_Vtbl {
        unsafe extern "system" fn GetVolumeChannelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVolumeChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pu32channelcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolumes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChannelVolumes(::core::mem::transmute_copy(&u32channelcount), ::core::mem::transmute_copy(&pf32volumes), ::core::mem::transmute_copy(&u32curvetype), ::core::mem::transmute_copy(&pcurveduration)).into()
        }
        unsafe extern "system" fn GetChannelVolumes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointOffloadStreamVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelVolumes(::core::mem::transmute_copy(&u32channelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pf32volumes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetVolumeChannelCount: GetVolumeChannelCount::<Identity, Impl, OFFSET>,
            SetChannelVolumes: SetChannelVolumes::<Identity, Impl, OFFSET>,
            GetChannelVolumes: GetChannelVolumes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolume_Impl: Sized {
    fn RegisterControlChangeNotify(&self, pnotify: &::core::option::Option<IAudioEndpointVolumeCallback>) -> ::windows::core::Result<()>;
    fn UnregisterControlChangeNotify(&self, pnotify: &::core::option::Option<IAudioEndpointVolumeCallback>) -> ::windows::core::Result<()>;
    fn GetChannelCount(&self) -> ::windows::core::Result<u32>;
    fn SetMasterVolumeLevel(&self, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetMasterVolumeLevelScalar(&self, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMasterVolumeLevel(&self) -> ::windows::core::Result<f32>;
    fn GetMasterVolumeLevelScalar(&self) -> ::windows::core::Result<f32>;
    fn SetChannelVolumeLevel(&self, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetChannelVolumeLevelScalar(&self, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetChannelVolumeLevel(&self, nchannel: u32) -> ::windows::core::Result<f32>;
    fn GetChannelVolumeLevelScalar(&self, nchannel: u32) -> ::windows::core::Result<f32>;
    fn SetMute(&self, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMute(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn GetVolumeStepInfo(&self, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::core::Result<()>;
    fn VolumeStepUp(&self, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn VolumeStepDown(&self, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn QueryHardwareSupport(&self) -> ::windows::core::Result<u32>;
    fn GetVolumeRange(&self, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioEndpointVolume {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEndpointVolume_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>() -> IAudioEndpointVolume_Vtbl {
        unsafe extern "system" fn RegisterControlChangeNotify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterControlChangeNotify(::core::mem::transmute(&pnotify)).into()
        }
        unsafe extern "system" fn UnregisterControlChangeNotify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterControlChangeNotify(::core::mem::transmute(&pnotify)).into()
        }
        unsafe extern "system" fn GetChannelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnchannelcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterVolumeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMasterVolumeLevel(::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetMasterVolumeLevelScalar<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMasterVolumeLevelScalar(::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetMasterVolumeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMasterVolumeLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfleveldb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMasterVolumeLevelScalar<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMasterVolumeLevelScalar() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolumeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChannelVolumeLevel(::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&fleveldb), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn SetChannelVolumeLevelScalar<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetChannelVolumeLevelScalar(::core::mem::transmute_copy(&nchannel), ::core::mem::transmute_copy(&flevel), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetChannelVolumeLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelVolumeLevel(::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfleveldb, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelVolumeLevelScalar<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetChannelVolumeLevelScalar(::core::mem::transmute_copy(&nchannel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflevel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMute(::core::mem::transmute_copy(&bmute), ::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn GetMute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetVolumeStepInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVolumeStepInfo(::core::mem::transmute_copy(&pnstep), ::core::mem::transmute_copy(&pnstepcount)).into()
        }
        unsafe extern "system" fn VolumeStepUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VolumeStepUp(::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn VolumeStepDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.VolumeStepDown(::core::mem::transmute_copy(&pguideventcontext)).into()
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryHardwareSupport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhardwaresupportmask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeRange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolume_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVolumeRange(::core::mem::transmute_copy(&pflvolumemindb), ::core::mem::transmute_copy(&pflvolumemaxdb), ::core::mem::transmute_copy(&pflvolumeincrementdb)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterControlChangeNotify: RegisterControlChangeNotify::<Identity, Impl, OFFSET>,
            UnregisterControlChangeNotify: UnregisterControlChangeNotify::<Identity, Impl, OFFSET>,
            GetChannelCount: GetChannelCount::<Identity, Impl, OFFSET>,
            SetMasterVolumeLevel: SetMasterVolumeLevel::<Identity, Impl, OFFSET>,
            SetMasterVolumeLevelScalar: SetMasterVolumeLevelScalar::<Identity, Impl, OFFSET>,
            GetMasterVolumeLevel: GetMasterVolumeLevel::<Identity, Impl, OFFSET>,
            GetMasterVolumeLevelScalar: GetMasterVolumeLevelScalar::<Identity, Impl, OFFSET>,
            SetChannelVolumeLevel: SetChannelVolumeLevel::<Identity, Impl, OFFSET>,
            SetChannelVolumeLevelScalar: SetChannelVolumeLevelScalar::<Identity, Impl, OFFSET>,
            GetChannelVolumeLevel: GetChannelVolumeLevel::<Identity, Impl, OFFSET>,
            GetChannelVolumeLevelScalar: GetChannelVolumeLevelScalar::<Identity, Impl, OFFSET>,
            SetMute: SetMute::<Identity, Impl, OFFSET>,
            GetMute: GetMute::<Identity, Impl, OFFSET>,
            GetVolumeStepInfo: GetVolumeStepInfo::<Identity, Impl, OFFSET>,
            VolumeStepUp: VolumeStepUp::<Identity, Impl, OFFSET>,
            VolumeStepDown: VolumeStepDown::<Identity, Impl, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, Impl, OFFSET>,
            GetVolumeRange: GetVolumeRange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolumeCallback_Impl: Sized {
    fn OnNotify(&self, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioEndpointVolumeCallback {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEndpointVolumeCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolumeCallback_Impl, const OFFSET: isize>() -> IAudioEndpointVolumeCallback_Vtbl {
        unsafe extern "system" fn OnNotify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolumeCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNotify(::core::mem::transmute_copy(&pnotify)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnNotify: OnNotify::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolumeEx_Impl: Sized + IAudioEndpointVolume_Impl {
    fn GetVolumeRangeChannel(&self, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioEndpointVolumeEx {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEndpointVolumeEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolumeEx_Impl, const OFFSET: isize>() -> IAudioEndpointVolumeEx_Vtbl {
        unsafe extern "system" fn GetVolumeRangeChannel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioEndpointVolumeEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetVolumeRangeChannel(::core::mem::transmute_copy(&ichannel), ::core::mem::transmute_copy(&pflvolumemindb), ::core::mem::transmute_copy(&pflvolumemaxdb), ::core::mem::transmute_copy(&pflvolumeincrementdb)).into()
        }
        Self { base__: IAudioEndpointVolume_Vtbl::new::<Identity, Impl, OFFSET>(), GetVolumeRangeChannel: GetVolumeRangeChannel::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeEx as ::windows::core::Interface>::IID || iid == &<IAudioEndpointVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioLfxControl_Impl: Sized {
    fn SetLocalEffectsState(&self, benabled: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetLocalEffectsState(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAudioLfxControl {}
#[cfg(feature = "Win32_Foundation")]
impl IAudioLfxControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioLfxControl_Impl, const OFFSET: isize>() -> IAudioLfxControl_Vtbl {
        unsafe extern "system" fn SetLocalEffectsState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioLfxControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocalEffectsState(::core::mem::transmute_copy(&benabled)).into()
        }
        unsafe extern "system" fn GetLocalEffectsState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioLfxControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalEffectsState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetLocalEffectsState: SetLocalEffectsState::<Identity, Impl, OFFSET>,
            GetLocalEffectsState: GetLocalEffectsState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioLfxControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioMeterInformation_Impl: Sized {
    fn GetPeakValue(&self) -> ::windows::core::Result<f32>;
    fn GetMeteringChannelCount(&self) -> ::windows::core::Result<u32>;
    fn GetChannelsPeakValues(&self, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows::core::Result<()>;
    fn QueryHardwareSupport(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IAudioMeterInformation {}
impl IAudioMeterInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: isize>() -> IAudioMeterInformation_Vtbl {
        unsafe extern "system" fn GetPeakValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpeak: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPeakValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfpeak, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeteringChannelCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMeteringChannelCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnchannelcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelsPeakValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetChannelsPeakValues(::core::mem::transmute_copy(&u32channelcount), ::core::mem::transmute_copy(&afpeakvalues)).into()
        }
        unsafe extern "system" fn QueryHardwareSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAudioMeterInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryHardwareSupport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhardwaresupportmask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPeakValue: GetPeakValue::<Identity, Impl, OFFSET>,
            GetMeteringChannelCount: GetMeteringChannelCount::<Identity, Impl, OFFSET>,
            GetChannelsPeakValues: GetChannelsPeakValues::<Identity, Impl, OFFSET>,
            QueryHardwareSupport: QueryHardwareSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMeterInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHardwareAudioEngineBase_Impl: Sized {
    fn GetAvailableOffloadConnectorCount(&self, _pwstrdeviceid: &::windows::core::PCWSTR, _uconnectorid: u32) -> ::windows::core::Result<u32>;
    fn GetEngineFormat(&self, pdevice: &::core::option::Option<super::IMMDevice>, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn SetEngineDeviceFormat(&self, pdevice: &::core::option::Option<super::IMMDevice>, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn SetGfxState(&self, pdevice: &::core::option::Option<super::IMMDevice>, _benable: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetGfxState(&self, pdevice: &::core::option::Option<super::IMMDevice>) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IHardwareAudioEngineBase {}
#[cfg(feature = "Win32_Foundation")]
impl IHardwareAudioEngineBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>() -> IHardwareAudioEngineBase_Vtbl {
        unsafe extern "system" fn GetAvailableOffloadConnectorCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _pwstrdeviceid: ::windows::core::PCWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAvailableOffloadConnectorCount(::core::mem::transmute(&_pwstrdeviceid), ::core::mem::transmute_copy(&_uconnectorid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_pavailableconnectorinstancecount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEngineFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEngineFormat(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&_brequestdeviceformat), ::core::mem::transmute_copy(&_ppwfxformat)).into()
        }
        unsafe extern "system" fn SetEngineDeviceFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEngineDeviceFormat(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&_pwfxformat)).into()
        }
        unsafe extern "system" fn SetGfxState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _benable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGfxState(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&_benable)).into()
        }
        unsafe extern "system" fn GetGfxState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHardwareAudioEngineBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, _pbenable: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGfxState(::core::mem::transmute(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(_pbenable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAvailableOffloadConnectorCount: GetAvailableOffloadConnectorCount::<Identity, Impl, OFFSET>,
            GetEngineFormat: GetEngineFormat::<Identity, Impl, OFFSET>,
            SetEngineDeviceFormat: SetEngineDeviceFormat::<Identity, Impl, OFFSET>,
            SetGfxState: SetGfxState::<Identity, Impl, OFFSET>,
            GetGfxState: GetGfxState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHardwareAudioEngineBase as ::windows::core::Interface>::IID
    }
}
