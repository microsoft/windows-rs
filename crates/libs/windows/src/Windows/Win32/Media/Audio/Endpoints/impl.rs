pub trait IAudioEndpointFormatControlImpl: Sized {
    fn ResetToDefault();
}
impl IAudioEndpointFormatControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointFormatControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointFormatControlVtbl {
        unsafe extern "system" fn ResetToDefault<Impl: IAudioEndpointFormatControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resetflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ResetToDefault::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointFormatControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
pub trait IAudioEndpointLastBufferControlImpl: Sized {
    fn IsLastBufferControlSupported();
    fn ReleaseOutputDataPointerForLastBuffer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_Audio_Apo"))]
impl IAudioEndpointLastBufferControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointLastBufferControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointLastBufferControlVtbl {
        unsafe extern "system" fn IsLastBufferControlSupported<Impl: IAudioEndpointLastBufferControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseOutputDataPointerForLastBuffer<Impl: IAudioEndpointLastBufferControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsLastBufferControlSupported::<Impl, IMPL_OFFSET>, ReleaseOutputDataPointerForLastBuffer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointLastBufferControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointOffloadStreamMeterImpl: Sized {
    fn GetMeterChannelCount();
    fn GetMeteringData();
}
impl IAudioEndpointOffloadStreamMeterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointOffloadStreamMeterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointOffloadStreamMeterVtbl {
        unsafe extern "system" fn GetMeterChannelCount<Impl: IAudioEndpointOffloadStreamMeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMeteringData<Impl: IAudioEndpointOffloadStreamMeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetMeterChannelCount::<Impl, IMPL_OFFSET>, GetMeteringData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamMeter as ::windows::core::Interface>::IID
    }
}
pub trait IAudioEndpointOffloadStreamMuteImpl: Sized {
    fn SetMute();
    fn GetMute();
}
impl IAudioEndpointOffloadStreamMuteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointOffloadStreamMuteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointOffloadStreamMuteVtbl {
        unsafe extern "system" fn SetMute<Impl: IAudioEndpointOffloadStreamMuteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmuted: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMute<Impl: IAudioEndpointOffloadStreamMuteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmuted: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetMute::<Impl, IMPL_OFFSET>, GetMute::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamMute as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
pub trait IAudioEndpointOffloadStreamVolumeImpl: Sized {
    fn GetVolumeChannelCount();
    fn SetChannelVolumes();
    fn GetChannelVolumes();
}
#[cfg(feature = "Win32_Media_KernelStreaming")]
impl IAudioEndpointOffloadStreamVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointOffloadStreamVolumeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointOffloadStreamVolumeVtbl {
        unsafe extern "system" fn GetVolumeChannelCount<Impl: IAudioEndpointOffloadStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetChannelVolumes<Impl: IAudioEndpointOffloadStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelVolumes<Impl: IAudioEndpointOffloadStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetVolumeChannelCount::<Impl, IMPL_OFFSET>, SetChannelVolumes::<Impl, IMPL_OFFSET>, GetChannelVolumes::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointOffloadStreamVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolumeImpl: Sized {
    fn RegisterControlChangeNotify();
    fn UnregisterControlChangeNotify();
    fn GetChannelCount();
    fn SetMasterVolumeLevel();
    fn SetMasterVolumeLevelScalar();
    fn GetMasterVolumeLevel();
    fn GetMasterVolumeLevelScalar();
    fn SetChannelVolumeLevel();
    fn SetChannelVolumeLevelScalar();
    fn GetChannelVolumeLevel();
    fn GetChannelVolumeLevelScalar();
    fn SetMute();
    fn GetMute();
    fn GetVolumeStepInfo();
    fn VolumeStepUp();
    fn VolumeStepDown();
    fn QueryHardwareSupport();
    fn GetVolumeRange();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEndpointVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointVolumeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointVolumeVtbl {
        unsafe extern "system" fn RegisterControlChangeNotify<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterControlChangeNotify<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelCount<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMasterVolumeLevel<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMasterVolumeLevelScalar<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMasterVolumeLevel<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMasterVolumeLevelScalar<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetChannelVolumeLevel<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetChannelVolumeLevelScalar<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelVolumeLevel<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelVolumeLevelScalar<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMute<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMute<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVolumeStepInfo<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeStepUp<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeStepDown<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryHardwareSupport<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVolumeRange<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            RegisterControlChangeNotify::<Impl, IMPL_OFFSET>,
            UnregisterControlChangeNotify::<Impl, IMPL_OFFSET>,
            GetChannelCount::<Impl, IMPL_OFFSET>,
            SetMasterVolumeLevel::<Impl, IMPL_OFFSET>,
            SetMasterVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            GetMasterVolumeLevel::<Impl, IMPL_OFFSET>,
            GetMasterVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            SetChannelVolumeLevel::<Impl, IMPL_OFFSET>,
            SetChannelVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            GetChannelVolumeLevel::<Impl, IMPL_OFFSET>,
            GetChannelVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            SetMute::<Impl, IMPL_OFFSET>,
            GetMute::<Impl, IMPL_OFFSET>,
            GetVolumeStepInfo::<Impl, IMPL_OFFSET>,
            VolumeStepUp::<Impl, IMPL_OFFSET>,
            VolumeStepDown::<Impl, IMPL_OFFSET>,
            QueryHardwareSupport::<Impl, IMPL_OFFSET>,
            GetVolumeRange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointVolume as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolumeCallbackImpl: Sized {
    fn OnNotify();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEndpointVolumeCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointVolumeCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointVolumeCallbackVtbl {
        unsafe extern "system" fn OnNotify<Impl: IAudioEndpointVolumeCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnNotify::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioEndpointVolumeExImpl: Sized + IAudioEndpointVolumeImpl {
    fn GetVolumeRangeChannel();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioEndpointVolumeExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointVolumeExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioEndpointVolumeExVtbl {
        unsafe extern "system" fn GetVolumeRangeChannel<Impl: IAudioEndpointVolumeExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            RegisterControlChangeNotify::<Impl, IMPL_OFFSET>,
            UnregisterControlChangeNotify::<Impl, IMPL_OFFSET>,
            GetChannelCount::<Impl, IMPL_OFFSET>,
            SetMasterVolumeLevel::<Impl, IMPL_OFFSET>,
            SetMasterVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            GetMasterVolumeLevel::<Impl, IMPL_OFFSET>,
            GetMasterVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            SetChannelVolumeLevel::<Impl, IMPL_OFFSET>,
            SetChannelVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            GetChannelVolumeLevel::<Impl, IMPL_OFFSET>,
            GetChannelVolumeLevelScalar::<Impl, IMPL_OFFSET>,
            SetMute::<Impl, IMPL_OFFSET>,
            GetMute::<Impl, IMPL_OFFSET>,
            GetVolumeStepInfo::<Impl, IMPL_OFFSET>,
            VolumeStepUp::<Impl, IMPL_OFFSET>,
            VolumeStepDown::<Impl, IMPL_OFFSET>,
            QueryHardwareSupport::<Impl, IMPL_OFFSET>,
            GetVolumeRange::<Impl, IMPL_OFFSET>,
            GetVolumeRangeChannel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioEndpointVolumeEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAudioLfxControlImpl: Sized {
    fn SetLocalEffectsState();
    fn GetLocalEffectsState();
}
#[cfg(feature = "Win32_Foundation")]
impl IAudioLfxControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioLfxControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioLfxControlVtbl {
        unsafe extern "system" fn SetLocalEffectsState<Impl: IAudioLfxControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLocalEffectsState<Impl: IAudioLfxControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetLocalEffectsState::<Impl, IMPL_OFFSET>, GetLocalEffectsState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioLfxControl as ::windows::core::Interface>::IID
    }
}
pub trait IAudioMeterInformationImpl: Sized {
    fn GetPeakValue();
    fn GetMeteringChannelCount();
    fn GetChannelsPeakValues();
    fn QueryHardwareSupport();
}
impl IAudioMeterInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMeterInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAudioMeterInformationVtbl {
        unsafe extern "system" fn GetPeakValue<Impl: IAudioMeterInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpeak: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMeteringChannelCount<Impl: IAudioMeterInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetChannelsPeakValues<Impl: IAudioMeterInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryHardwareSupport<Impl: IAudioMeterInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPeakValue::<Impl, IMPL_OFFSET>, GetMeteringChannelCount::<Impl, IMPL_OFFSET>, GetChannelsPeakValues::<Impl, IMPL_OFFSET>, QueryHardwareSupport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAudioMeterInformation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHardwareAudioEngineBaseImpl: Sized {
    fn GetAvailableOffloadConnectorCount();
    fn GetEngineFormat();
    fn SetEngineDeviceFormat();
    fn SetGfxState();
    fn GetGfxState();
}
#[cfg(feature = "Win32_Foundation")]
impl IHardwareAudioEngineBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHardwareAudioEngineBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHardwareAudioEngineBaseVtbl {
        unsafe extern "system" fn GetAvailableOffloadConnectorCount<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _pwstrdeviceid: super::super::super::Foundation::PWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEngineFormat<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEngineDeviceFormat<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGfxState<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _benable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGfxState<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _pbenable: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetAvailableOffloadConnectorCount::<Impl, IMPL_OFFSET>, GetEngineFormat::<Impl, IMPL_OFFSET>, SetEngineDeviceFormat::<Impl, IMPL_OFFSET>, SetGfxState::<Impl, IMPL_OFFSET>, GetGfxState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHardwareAudioEngineBase as ::windows::core::Interface>::IID
    }
}
