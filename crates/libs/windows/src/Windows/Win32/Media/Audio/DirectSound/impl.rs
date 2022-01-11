#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundImpl: Sized {
    fn CreateSoundBuffer();
    fn GetCaps();
    fn DuplicateSoundBuffer();
    fn SetCooperativeLevel();
    fn Compact();
    fn GetSpeakerConfig();
    fn SetSpeakerConfig();
    fn Initialize();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundVtbl {
        unsafe extern "system" fn CreateSoundBuffer<Impl: IDirectSoundImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscaps: *mut DSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DuplicateSoundBuffer<Impl: IDirectSoundImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsbufferoriginal: ::windows::core::RawPtr, ppdsbufferduplicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectSoundImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Compact<Impl: IDirectSoundImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSpeakerConfig<Impl: IDirectSoundImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwspeakerconfig: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSpeakerConfig<Impl: IDirectSoundImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspeakerconfig: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateSoundBuffer::<Impl, IMPL_OFFSET>, GetCaps::<Impl, IMPL_OFFSET>, DuplicateSoundBuffer::<Impl, IMPL_OFFSET>, SetCooperativeLevel::<Impl, IMPL_OFFSET>, Compact::<Impl, IMPL_OFFSET>, GetSpeakerConfig::<Impl, IMPL_OFFSET>, SetSpeakerConfig::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSound as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DBufferImpl: Sized {
    fn GetAllParameters();
    fn GetConeAngles();
    fn GetConeOrientation();
    fn GetConeOutsideVolume();
    fn GetMaxDistance();
    fn GetMinDistance();
    fn GetMode();
    fn GetPosition();
    fn GetVelocity();
    fn SetAllParameters();
    fn SetConeAngles();
    fn SetConeOrientation();
    fn SetConeOutsideVolume();
    fn SetMaxDistance();
    fn SetMinDistance();
    fn SetMode();
    fn SetPosition();
    fn SetVelocity();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDirectSound3DBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSound3DBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSound3DBufferVtbl {
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pds3dbuffer: *mut DS3DBUFFER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConeAngles<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConeOrientation<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConeOutsideVolume<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plconeoutsidevolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxDistance<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflmaxdistance: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinDistance<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflmindistance: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMode<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPosition<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVelocity<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConeAngles<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConeOrientation<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetConeOutsideVolume<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxDistance<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flmaxdistance: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinDistance<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flmindistance: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMode<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPosition<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVelocity<Impl: IDirectSound3DBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetAllParameters::<Impl, IMPL_OFFSET>,
            GetConeAngles::<Impl, IMPL_OFFSET>,
            GetConeOrientation::<Impl, IMPL_OFFSET>,
            GetConeOutsideVolume::<Impl, IMPL_OFFSET>,
            GetMaxDistance::<Impl, IMPL_OFFSET>,
            GetMinDistance::<Impl, IMPL_OFFSET>,
            GetMode::<Impl, IMPL_OFFSET>,
            GetPosition::<Impl, IMPL_OFFSET>,
            GetVelocity::<Impl, IMPL_OFFSET>,
            SetAllParameters::<Impl, IMPL_OFFSET>,
            SetConeAngles::<Impl, IMPL_OFFSET>,
            SetConeOrientation::<Impl, IMPL_OFFSET>,
            SetConeOutsideVolume::<Impl, IMPL_OFFSET>,
            SetMaxDistance::<Impl, IMPL_OFFSET>,
            SetMinDistance::<Impl, IMPL_OFFSET>,
            SetMode::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
            SetVelocity::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSound3DBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DListenerImpl: Sized {
    fn GetAllParameters();
    fn GetDistanceFactor();
    fn GetDopplerFactor();
    fn GetOrientation();
    fn GetPosition();
    fn GetRolloffFactor();
    fn GetVelocity();
    fn SetAllParameters();
    fn SetDistanceFactor();
    fn SetDopplerFactor();
    fn SetOrientation();
    fn SetPosition();
    fn SetRolloffFactor();
    fn SetVelocity();
    fn CommitDeferredSettings();
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDirectSound3DListenerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSound3DListenerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSound3DListenerVtbl {
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plistener: *mut DS3DLISTENER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDistanceFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfldistancefactor: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDopplerFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfldopplerfactor: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOrientation<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPosition<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRolloffFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflrollofffactor: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVelocity<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDistanceFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fldistancefactor: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDopplerFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fldopplerfactor: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOrientation<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPosition<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRolloffFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flrollofffactor: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVelocity<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitDeferredSettings<Impl: IDirectSound3DListenerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetAllParameters::<Impl, IMPL_OFFSET>,
            GetDistanceFactor::<Impl, IMPL_OFFSET>,
            GetDopplerFactor::<Impl, IMPL_OFFSET>,
            GetOrientation::<Impl, IMPL_OFFSET>,
            GetPosition::<Impl, IMPL_OFFSET>,
            GetRolloffFactor::<Impl, IMPL_OFFSET>,
            GetVelocity::<Impl, IMPL_OFFSET>,
            SetAllParameters::<Impl, IMPL_OFFSET>,
            SetDistanceFactor::<Impl, IMPL_OFFSET>,
            SetDopplerFactor::<Impl, IMPL_OFFSET>,
            SetOrientation::<Impl, IMPL_OFFSET>,
            SetPosition::<Impl, IMPL_OFFSET>,
            SetRolloffFactor::<Impl, IMPL_OFFSET>,
            SetVelocity::<Impl, IMPL_OFFSET>,
            CommitDeferredSettings::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSound3DListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSound8Impl: Sized + IDirectSoundImpl {
    fn VerifyCertification();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSound8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSound8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSound8Vtbl {
        unsafe extern "system" fn VerifyCertification<Impl: IDirectSound8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcertified: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateSoundBuffer::<Impl, IMPL_OFFSET>,
            GetCaps::<Impl, IMPL_OFFSET>,
            DuplicateSoundBuffer::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            Compact::<Impl, IMPL_OFFSET>,
            GetSpeakerConfig::<Impl, IMPL_OFFSET>,
            SetSpeakerConfig::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            VerifyCertification::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSound8 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundBufferImpl: Sized {
    fn GetCaps();
    fn GetCurrentPosition();
    fn GetFormat();
    fn GetVolume();
    fn GetPan();
    fn GetFrequency();
    fn GetStatus();
    fn Initialize();
    fn Lock();
    fn Play();
    fn SetCurrentPosition();
    fn SetFormat();
    fn SetVolume();
    fn SetPan();
    fn SetFrequency();
    fn Stop();
    fn Unlock();
    fn Restore();
}
impl IDirectSoundBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundBufferVtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsbuffercaps: *mut DSBCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPosition<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVolume<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPan<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpan: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrequency<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfrequency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lock<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Play<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentPosition<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnewposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFormat<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVolume<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPan<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpan: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFrequency<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfrequency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Restore<Impl: IDirectSoundBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCaps::<Impl, IMPL_OFFSET>,
            GetCurrentPosition::<Impl, IMPL_OFFSET>,
            GetFormat::<Impl, IMPL_OFFSET>,
            GetVolume::<Impl, IMPL_OFFSET>,
            GetPan::<Impl, IMPL_OFFSET>,
            GetFrequency::<Impl, IMPL_OFFSET>,
            GetStatus::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            Lock::<Impl, IMPL_OFFSET>,
            Play::<Impl, IMPL_OFFSET>,
            SetCurrentPosition::<Impl, IMPL_OFFSET>,
            SetFormat::<Impl, IMPL_OFFSET>,
            SetVolume::<Impl, IMPL_OFFSET>,
            SetPan::<Impl, IMPL_OFFSET>,
            SetFrequency::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Unlock::<Impl, IMPL_OFFSET>,
            Restore::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundBuffer8Impl: Sized + IDirectSoundBufferImpl {
    fn SetFX();
    fn AcquireResources();
    fn GetObjectInPath();
}
impl IDirectSoundBuffer8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundBuffer8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundBuffer8Vtbl {
        unsafe extern "system" fn SetFX<Impl: IDirectSoundBuffer8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AcquireResources<Impl: IDirectSoundBuffer8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectInPath<Impl: IDirectSoundBuffer8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCaps::<Impl, IMPL_OFFSET>,
            GetCurrentPosition::<Impl, IMPL_OFFSET>,
            GetFormat::<Impl, IMPL_OFFSET>,
            GetVolume::<Impl, IMPL_OFFSET>,
            GetPan::<Impl, IMPL_OFFSET>,
            GetFrequency::<Impl, IMPL_OFFSET>,
            GetStatus::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            Lock::<Impl, IMPL_OFFSET>,
            Play::<Impl, IMPL_OFFSET>,
            SetCurrentPosition::<Impl, IMPL_OFFSET>,
            SetFormat::<Impl, IMPL_OFFSET>,
            SetVolume::<Impl, IMPL_OFFSET>,
            SetPan::<Impl, IMPL_OFFSET>,
            SetFrequency::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Unlock::<Impl, IMPL_OFFSET>,
            Restore::<Impl, IMPL_OFFSET>,
            SetFX::<Impl, IMPL_OFFSET>,
            AcquireResources::<Impl, IMPL_OFFSET>,
            GetObjectInPath::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundBuffer8 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundCaptureImpl: Sized {
    fn CreateCaptureBuffer();
    fn GetCaps();
    fn Initialize();
}
impl IDirectSoundCaptureVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCaptureImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCaptureVtbl {
        unsafe extern "system" fn CreateCaptureBuffer<Impl: IDirectSoundCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsccaps: *mut DSCCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundCaptureImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateCaptureBuffer::<Impl, IMPL_OFFSET>, GetCaps::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCapture as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundCaptureBufferImpl: Sized {
    fn GetCaps();
    fn GetCurrentPosition();
    fn GetFormat();
    fn GetStatus();
    fn Initialize();
    fn Lock();
    fn Start();
    fn Stop();
    fn Unlock();
}
impl IDirectSoundCaptureBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCaptureBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCaptureBufferVtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscbcaps: *mut DSCBCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentPosition<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsoundcapture: ::windows::core::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Lock<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Start<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stop<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCaps::<Impl, IMPL_OFFSET>, GetCurrentPosition::<Impl, IMPL_OFFSET>, GetFormat::<Impl, IMPL_OFFSET>, GetStatus::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Lock::<Impl, IMPL_OFFSET>, Start::<Impl, IMPL_OFFSET>, Stop::<Impl, IMPL_OFFSET>, Unlock::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundCaptureBuffer8Impl: Sized + IDirectSoundCaptureBufferImpl {
    fn GetObjectInPath();
    fn GetFXStatus();
}
impl IDirectSoundCaptureBuffer8Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCaptureBuffer8Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCaptureBuffer8Vtbl {
        unsafe extern "system" fn GetObjectInPath<Impl: IDirectSoundCaptureBuffer8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFXStatus<Impl: IDirectSoundCaptureBuffer8Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetCaps::<Impl, IMPL_OFFSET>,
            GetCurrentPosition::<Impl, IMPL_OFFSET>,
            GetFormat::<Impl, IMPL_OFFSET>,
            GetStatus::<Impl, IMPL_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            Lock::<Impl, IMPL_OFFSET>,
            Start::<Impl, IMPL_OFFSET>,
            Stop::<Impl, IMPL_OFFSET>,
            Unlock::<Impl, IMPL_OFFSET>,
            GetObjectInPath::<Impl, IMPL_OFFSET>,
            GetFXStatus::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundCaptureFXAecImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
    fn GetStatus();
    fn Reset();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundCaptureFXAecVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCaptureFXAecImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCaptureFXAecVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundCaptureFXAecImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscfxaec: *const DSCFXAec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundCaptureFXAecImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscfxaec: *mut DSCFXAec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectSoundCaptureFXAecImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IDirectSoundCaptureFXAecImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>, GetStatus::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXAec as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundCaptureFXNoiseSuppressImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
    fn Reset();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundCaptureFXNoiseSuppressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCaptureFXNoiseSuppressImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCaptureFXNoiseSuppressVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundCaptureFXNoiseSuppressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundCaptureFXNoiseSuppressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IDirectSoundCaptureFXNoiseSuppressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXNoiseSuppress as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXChorusImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl IDirectSoundFXChorusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXChorusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXChorusVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXChorusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxchorus: *const DSFXChorus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXChorusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxchorus: *mut DSFXChorus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXChorus as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXCompressorImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl IDirectSoundFXCompressorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXCompressorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXCompressorVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXCompressorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXCompressorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxcompressor: *mut DSFXCompressor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXCompressor as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXDistortionImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl IDirectSoundFXDistortionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXDistortionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXDistortionVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXDistortionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXDistortionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxdistortion: *mut DSFXDistortion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXDistortion as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXEchoImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl IDirectSoundFXEchoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXEchoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXEchoVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXEchoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxecho: *const DSFXEcho) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXEchoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxecho: *mut DSFXEcho) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXEcho as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXFlangerImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl IDirectSoundFXFlangerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXFlangerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXFlangerVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXFlangerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxflanger: *const DSFXFlanger) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXFlangerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxflanger: *mut DSFXFlanger) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXFlanger as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXGargleImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl IDirectSoundFXGargleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXGargleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXGargleVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXGargleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxgargle: *const DSFXGargle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXGargleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxgargle: *mut DSFXGargle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXGargle as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXI3DL2ReverbImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
    fn SetPreset();
    fn GetPreset();
    fn SetQuality();
    fn GetQuality();
}
impl IDirectSoundFXI3DL2ReverbVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXI3DL2ReverbImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXI3DL2ReverbVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreset<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpreset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPreset<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpreset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuality<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquality: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuality<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>, SetPreset::<Impl, IMPL_OFFSET>, GetPreset::<Impl, IMPL_OFFSET>, SetQuality::<Impl, IMPL_OFFSET>, GetQuality::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXI3DL2Reverb as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXParamEqImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl IDirectSoundFXParamEqVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXParamEqImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXParamEqVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXParamEqImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxparameq: *const DSFXParamEq) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXParamEqImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxparameq: *mut DSFXParamEq) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXParamEq as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXWavesReverbImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl IDirectSoundFXWavesReverbVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXWavesReverbImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXWavesReverbVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXWavesReverbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXWavesReverbImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllParameters::<Impl, IMPL_OFFSET>, GetAllParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXWavesReverb as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundFullDuplexImpl: Sized {
    fn Initialize();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundFullDuplexVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFullDuplexImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFullDuplexVtbl {
        unsafe extern "system" fn Initialize<Impl: IDirectSoundFullDuplexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaptureguid: *const ::windows::core::GUID, prenderguid: *const ::windows::core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::windows::core::RawPtr, lplpdirectsoundbuffer8: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFullDuplex as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundNotifyImpl: Sized {
    fn SetNotificationPositions();
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundNotifyVtbl {
        unsafe extern "system" fn SetNotificationPositions<Impl: IDirectSoundNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetNotificationPositions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundNotify as ::windows::core::Interface>::IID
    }
}
