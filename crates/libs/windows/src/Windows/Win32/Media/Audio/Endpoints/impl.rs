pub trait IAudioEndpointFormatControlImpl: Sized {
    fn ResetToDefault();
}
impl ::windows::core::RuntimeName for IAudioEndpointFormatControl {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioEndpointFormatControl";
}
impl IAudioEndpointFormatControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointFormatControlImpl, const OFFSET: isize>() -> IAudioEndpointFormatControlVtbl {
        unsafe extern "system" fn ResetToDefault<Impl: IAudioEndpointFormatControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resetflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetToDefault(resetflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpointFormatControl>, ::windows::core::GetTrustLevel, ResetToDefault::<Impl, OFFSET>)
    }
}
pub trait IAudioEndpointLastBufferControlImpl: Sized {
    fn IsLastBufferControlSupported();
    fn ReleaseOutputDataPointerForLastBuffer();
}
impl ::windows::core::RuntimeName for IAudioEndpointLastBufferControl {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioEndpointLastBufferControl";
}
impl IAudioEndpointLastBufferControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointLastBufferControlImpl, const OFFSET: isize>() -> IAudioEndpointLastBufferControlVtbl {
        unsafe extern "system" fn IsLastBufferControlSupported<Impl: IAudioEndpointLastBufferControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLastBufferControlSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseOutputDataPointerForLastBuffer<Impl: IAudioEndpointLastBufferControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectionproperty: *const super::Apo::APO_CONNECTION_PROPERTY) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseOutputDataPointerForLastBuffer(&*(&pconnectionproperty as *const <super::Apo::APO_CONNECTION_PROPERTY as ::windows::core::Abi>::Abi as *const <super::Apo::APO_CONNECTION_PROPERTY as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpointLastBufferControl>, ::windows::core::GetTrustLevel, IsLastBufferControlSupported::<Impl, OFFSET>, ReleaseOutputDataPointerForLastBuffer::<Impl, OFFSET>)
    }
}
pub trait IAudioEndpointOffloadStreamMeterImpl: Sized {
    fn GetMeterChannelCount();
    fn GetMeteringData();
}
impl ::windows::core::RuntimeName for IAudioEndpointOffloadStreamMeter {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioEndpointOffloadStreamMeter";
}
impl IAudioEndpointOffloadStreamMeterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointOffloadStreamMeterImpl, const OFFSET: isize>() -> IAudioEndpointOffloadStreamMeterVtbl {
        unsafe extern "system" fn GetMeterChannelCount<Impl: IAudioEndpointOffloadStreamMeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMeterChannelCount(::core::mem::transmute_copy(&pu32channelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeteringData<Impl: IAudioEndpointOffloadStreamMeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32peakvalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMeteringData(u32channelcount, ::core::mem::transmute_copy(&pf32peakvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpointOffloadStreamMeter>, ::windows::core::GetTrustLevel, GetMeterChannelCount::<Impl, OFFSET>, GetMeteringData::<Impl, OFFSET>)
    }
}
pub trait IAudioEndpointOffloadStreamMuteImpl: Sized {
    fn SetMute();
    fn GetMute();
}
impl ::windows::core::RuntimeName for IAudioEndpointOffloadStreamMute {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioEndpointOffloadStreamMute";
}
impl IAudioEndpointOffloadStreamMuteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointOffloadStreamMuteImpl, const OFFSET: isize>() -> IAudioEndpointOffloadStreamMuteVtbl {
        unsafe extern "system" fn SetMute<Impl: IAudioEndpointOffloadStreamMuteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmuted: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMute(bmuted) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMute<Impl: IAudioEndpointOffloadStreamMuteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmuted: *mut u8) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpointOffloadStreamMute>, ::windows::core::GetTrustLevel, SetMute::<Impl, OFFSET>, GetMute::<Impl, OFFSET>)
    }
}
pub trait IAudioEndpointOffloadStreamVolumeImpl: Sized {
    fn GetVolumeChannelCount();
    fn SetChannelVolumes();
    fn GetChannelVolumes();
}
impl ::windows::core::RuntimeName for IAudioEndpointOffloadStreamVolume {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioEndpointOffloadStreamVolume";
}
impl IAudioEndpointOffloadStreamVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointOffloadStreamVolumeImpl, const OFFSET: isize>() -> IAudioEndpointOffloadStreamVolumeVtbl {
        unsafe extern "system" fn GetVolumeChannelCount<Impl: IAudioEndpointOffloadStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pu32channelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeChannelCount(::core::mem::transmute_copy(&pu32channelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolumes<Impl: IAudioEndpointOffloadStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *const f32, u32curvetype: super::super::KernelStreaming::AUDIO_CURVE_TYPE, pcurveduration: *const i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannelVolumes(u32channelcount, pf32volumes, u32curvetype, pcurveduration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelVolumes<Impl: IAudioEndpointOffloadStreamVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, pf32volumes: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolumes(u32channelcount, ::core::mem::transmute_copy(&pf32volumes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpointOffloadStreamVolume>, ::windows::core::GetTrustLevel, GetVolumeChannelCount::<Impl, OFFSET>, SetChannelVolumes::<Impl, OFFSET>, GetChannelVolumes::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IAudioEndpointVolume {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioEndpointVolume";
}
impl IAudioEndpointVolumeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>() -> IAudioEndpointVolumeVtbl {
        unsafe extern "system" fn RegisterControlChangeNotify<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterControlChangeNotify(&*(&pnotify as *const <IAudioEndpointVolumeCallback as ::windows::core::Abi>::Abi as *const <IAudioEndpointVolumeCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterControlChangeNotify<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterControlChangeNotify(&*(&pnotify as *const <IAudioEndpointVolumeCallback as ::windows::core::Abi>::Abi as *const <IAudioEndpointVolumeCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelCount<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelCount(::core::mem::transmute_copy(&pnchannelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterVolumeLevel<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMasterVolumeLevel(fleveldb, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMasterVolumeLevelScalar<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMasterVolumeLevelScalar(flevel, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMasterVolumeLevel<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMasterVolumeLevel(::core::mem::transmute_copy(&pfleveldb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMasterVolumeLevelScalar<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMasterVolumeLevelScalar(::core::mem::transmute_copy(&pflevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolumeLevel<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, fleveldb: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannelVolumeLevel(nchannel, fleveldb, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetChannelVolumeLevelScalar<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, flevel: f32, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetChannelVolumeLevelScalar(nchannel, flevel, &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelVolumeLevel<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pfleveldb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolumeLevel(nchannel, ::core::mem::transmute_copy(&pfleveldb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelVolumeLevelScalar<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nchannel: u32, pflevel: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelVolumeLevelScalar(nchannel, ::core::mem::transmute_copy(&pflevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMute<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmute: super::super::super::Foundation::BOOL, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMute(&*(&bmute as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMute<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmute: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetVolumeStepInfo<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnstep: *mut u32, pnstepcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeStepInfo(::core::mem::transmute_copy(&pnstep), ::core::mem::transmute_copy(&pnstepcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeStepUp<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeStepUp(&*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeStepDown<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguideventcontext: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeStepDown(&*(&pguideventcontext as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHardwareSupport<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHardwareSupport(::core::mem::transmute_copy(&pdwhardwaresupportmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolumeRange<Impl: IAudioEndpointVolumeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeRange(::core::mem::transmute_copy(&pflvolumemindb), ::core::mem::transmute_copy(&pflvolumemaxdb), ::core::mem::transmute_copy(&pflvolumeincrementdb)) {
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
            ::windows::core::GetRuntimeClassName::<IAudioEndpointVolume>,
            ::windows::core::GetTrustLevel,
            RegisterControlChangeNotify::<Impl, OFFSET>,
            UnregisterControlChangeNotify::<Impl, OFFSET>,
            GetChannelCount::<Impl, OFFSET>,
            SetMasterVolumeLevel::<Impl, OFFSET>,
            SetMasterVolumeLevelScalar::<Impl, OFFSET>,
            GetMasterVolumeLevel::<Impl, OFFSET>,
            GetMasterVolumeLevelScalar::<Impl, OFFSET>,
            SetChannelVolumeLevel::<Impl, OFFSET>,
            SetChannelVolumeLevelScalar::<Impl, OFFSET>,
            GetChannelVolumeLevel::<Impl, OFFSET>,
            GetChannelVolumeLevelScalar::<Impl, OFFSET>,
            SetMute::<Impl, OFFSET>,
            GetMute::<Impl, OFFSET>,
            GetVolumeStepInfo::<Impl, OFFSET>,
            VolumeStepUp::<Impl, OFFSET>,
            VolumeStepDown::<Impl, OFFSET>,
            QueryHardwareSupport::<Impl, OFFSET>,
            GetVolumeRange::<Impl, OFFSET>,
        )
    }
}
pub trait IAudioEndpointVolumeCallbackImpl: Sized {
    fn OnNotify();
}
impl ::windows::core::RuntimeName for IAudioEndpointVolumeCallback {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioEndpointVolumeCallback";
}
impl IAudioEndpointVolumeCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointVolumeCallbackImpl, const OFFSET: isize>() -> IAudioEndpointVolumeCallbackVtbl {
        unsafe extern "system" fn OnNotify<Impl: IAudioEndpointVolumeCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnotify: *mut super::AUDIO_VOLUME_NOTIFICATION_DATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNotify(&*(&pnotify as *const <super::AUDIO_VOLUME_NOTIFICATION_DATA as ::windows::core::Abi>::Abi as *const <super::AUDIO_VOLUME_NOTIFICATION_DATA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpointVolumeCallback>, ::windows::core::GetTrustLevel, OnNotify::<Impl, OFFSET>)
    }
}
pub trait IAudioEndpointVolumeExImpl: Sized + IAudioEndpointVolumeImpl {
    fn GetVolumeRangeChannel();
}
impl ::windows::core::RuntimeName for IAudioEndpointVolumeEx {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioEndpointVolumeEx";
}
impl IAudioEndpointVolumeExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioEndpointVolumeExImpl, const OFFSET: isize>() -> IAudioEndpointVolumeExVtbl {
        unsafe extern "system" fn GetVolumeRangeChannel<Impl: IAudioEndpointVolumeExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ichannel: u32, pflvolumemindb: *mut f32, pflvolumemaxdb: *mut f32, pflvolumeincrementdb: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolumeRangeChannel(ichannel, ::core::mem::transmute_copy(&pflvolumemindb), ::core::mem::transmute_copy(&pflvolumemaxdb), ::core::mem::transmute_copy(&pflvolumeincrementdb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioEndpointVolumeEx>, ::windows::core::GetTrustLevel, GetVolumeRangeChannel::<Impl, OFFSET>)
    }
}
pub trait IAudioLfxControlImpl: Sized {
    fn SetLocalEffectsState();
    fn GetLocalEffectsState();
}
impl ::windows::core::RuntimeName for IAudioLfxControl {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioLfxControl";
}
impl IAudioLfxControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioLfxControlImpl, const OFFSET: isize>() -> IAudioLfxControlVtbl {
        unsafe extern "system" fn SetLocalEffectsState<Impl: IAudioLfxControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalEffectsState(&*(&benabled as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalEffectsState<Impl: IAudioLfxControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalEffectsState(::core::mem::transmute_copy(&pbenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioLfxControl>, ::windows::core::GetTrustLevel, SetLocalEffectsState::<Impl, OFFSET>, GetLocalEffectsState::<Impl, OFFSET>)
    }
}
pub trait IAudioMeterInformationImpl: Sized {
    fn GetPeakValue();
    fn GetMeteringChannelCount();
    fn GetChannelsPeakValues();
    fn QueryHardwareSupport();
}
impl ::windows::core::RuntimeName for IAudioMeterInformation {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IAudioMeterInformation";
}
impl IAudioMeterInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAudioMeterInformationImpl, const OFFSET: isize>() -> IAudioMeterInformationVtbl {
        unsafe extern "system" fn GetPeakValue<Impl: IAudioMeterInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfpeak: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPeakValue(::core::mem::transmute_copy(&pfpeak)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMeteringChannelCount<Impl: IAudioMeterInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnchannelcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMeteringChannelCount(::core::mem::transmute_copy(&pnchannelcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetChannelsPeakValues<Impl: IAudioMeterInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, u32channelcount: u32, afpeakvalues: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChannelsPeakValues(u32channelcount, ::core::mem::transmute_copy(&afpeakvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryHardwareSupport<Impl: IAudioMeterInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhardwaresupportmask: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryHardwareSupport(::core::mem::transmute_copy(&pdwhardwaresupportmask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAudioMeterInformation>, ::windows::core::GetTrustLevel, GetPeakValue::<Impl, OFFSET>, GetMeteringChannelCount::<Impl, OFFSET>, GetChannelsPeakValues::<Impl, OFFSET>, QueryHardwareSupport::<Impl, OFFSET>)
    }
}
pub trait IHardwareAudioEngineBaseImpl: Sized {
    fn GetAvailableOffloadConnectorCount();
    fn GetEngineFormat();
    fn SetEngineDeviceFormat();
    fn SetGfxState();
    fn GetGfxState();
}
impl ::windows::core::RuntimeName for IHardwareAudioEngineBase {
    const NAME: &'static str = "Windows.Win32.Media.Audio.Endpoints.IHardwareAudioEngineBase";
}
impl IHardwareAudioEngineBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>() -> IHardwareAudioEngineBaseVtbl {
        unsafe extern "system" fn GetAvailableOffloadConnectorCount<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _pwstrdeviceid: super::super::super::Foundation::PWSTR, _uconnectorid: u32, _pavailableconnectorinstancecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableOffloadConnectorCount(&*(&_pwstrdeviceid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), _uconnectorid, ::core::mem::transmute_copy(&_pavailableconnectorinstancecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEngineFormat<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _brequestdeviceformat: super::super::super::Foundation::BOOL, _ppwfxformat: *mut *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEngineFormat(
                &*(&pdevice as *const <super::IMMDevice as ::windows::core::Abi>::Abi as *const <super::IMMDevice as ::windows::core::DefaultType>::DefaultType),
                &*(&_brequestdeviceformat as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&_ppwfxformat as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEngineDeviceFormat<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _pwfxformat: *mut super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEngineDeviceFormat(&*(&pdevice as *const <super::IMMDevice as ::windows::core::Abi>::Abi as *const <super::IMMDevice as ::windows::core::DefaultType>::DefaultType), &*(&_pwfxformat as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGfxState<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _benable: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGfxState(&*(&pdevice as *const <super::IMMDevice as ::windows::core::Abi>::Abi as *const <super::IMMDevice as ::windows::core::DefaultType>::DefaultType), &*(&_benable as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGfxState<Impl: IHardwareAudioEngineBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: ::windows::core::RawPtr, _pbenable: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGfxState(&*(&pdevice as *const <super::IMMDevice as ::windows::core::Abi>::Abi as *const <super::IMMDevice as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&_pbenable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IHardwareAudioEngineBase>, ::windows::core::GetTrustLevel, GetAvailableOffloadConnectorCount::<Impl, OFFSET>, GetEngineFormat::<Impl, OFFSET>, SetEngineDeviceFormat::<Impl, OFFSET>, SetGfxState::<Impl, OFFSET>, GetGfxState::<Impl, OFFSET>)
    }
}
