#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSound_Impl: Sized {
    fn CreateSoundBuffer(&mut self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::core::option::Option<IDirectSoundBuffer>, punkouter: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self) -> ::windows::core::Result<DSCAPS>;
    fn DuplicateSoundBuffer(&mut self, pdsbufferoriginal: ::core::option::Option<IDirectSoundBuffer>) -> ::windows::core::Result<IDirectSoundBuffer>;
    fn SetCooperativeLevel(&mut self, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::core::Result<()>;
    fn Compact(&mut self) -> ::windows::core::Result<()>;
    fn GetSpeakerConfig(&mut self) -> ::windows::core::Result<u32>;
    fn SetSpeakerConfig(&mut self, dwspeakerconfig: u32) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSound_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSound_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSound_Vtbl {
        unsafe extern "system" fn CreateSoundBuffer<Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSoundBuffer(::core::mem::transmute_copy(&pcdsbufferdesc), ::core::mem::transmute_copy(&ppdsbuffer), ::core::mem::transmute(&punkouter)).into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscaps: *mut DSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pdscaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DuplicateSoundBuffer<Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsbufferoriginal: ::windows::core::RawPtr, ppdsbufferduplicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DuplicateSoundBuffer(::core::mem::transmute(&pdsbufferoriginal)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppdsbufferduplicate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCooperativeLevel(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwlevel)).into()
        }
        unsafe extern "system" fn Compact<Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Compact().into()
        }
        unsafe extern "system" fn GetSpeakerConfig<Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwspeakerconfig: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSpeakerConfig() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwspeakerconfig = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeakerConfig<Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspeakerconfig: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSpeakerConfig(::core::mem::transmute_copy(&dwspeakerconfig)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pcguiddevice)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateSoundBuffer: CreateSoundBuffer::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            DuplicateSoundBuffer: DuplicateSoundBuffer::<Impl, IMPL_OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Impl, IMPL_OFFSET>,
            Compact: Compact::<Impl, IMPL_OFFSET>,
            GetSpeakerConfig: GetSpeakerConfig::<Impl, IMPL_OFFSET>,
            SetSpeakerConfig: SetSpeakerConfig::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSound as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DBuffer_Impl: Sized {
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DS3DBUFFER>;
    fn GetConeAngles(&mut self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::core::Result<()>;
    fn GetConeOrientation(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetConeOutsideVolume(&mut self) -> ::windows::core::Result<i32>;
    fn GetMaxDistance(&mut self) -> ::windows::core::Result<f32>;
    fn GetMinDistance(&mut self) -> ::windows::core::Result<f32>;
    fn GetMode(&mut self) -> ::windows::core::Result<u32>;
    fn GetPosition(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetVelocity(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn SetAllParameters(&mut self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetConeAngles(&mut self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetConeOrientation(&mut self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetConeOutsideVolume(&mut self, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetMaxDistance(&mut self, flmaxdistance: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetMinDistance(&mut self, flmindistance: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetMode(&mut self, dwmode: u32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetPosition(&mut self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetVelocity(&mut self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDirectSound3DBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSound3DBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSound3DBuffer_Vtbl {
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pds3dbuffer: *mut DS3DBUFFER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pds3dbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConeAngles<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetConeAngles(::core::mem::transmute_copy(&pdwinsideconeangle), ::core::mem::transmute_copy(&pdwoutsideconeangle)).into()
        }
        unsafe extern "system" fn GetConeOrientation<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConeOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    *pvorientation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConeOutsideVolume<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plconeoutsidevolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetConeOutsideVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *plconeoutsidevolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxDistance<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflmaxdistance: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *pflmaxdistance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinDistance<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflmindistance: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinDistance() {
                ::core::result::Result::Ok(ok__) => {
                    *pflmindistance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMode<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMode() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pvposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocity<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *pvvelocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcds3dbuffer), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetConeAngles<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConeAngles(::core::mem::transmute_copy(&dwinsideconeangle), ::core::mem::transmute_copy(&dwoutsideconeangle), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetConeOrientation<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConeOrientation(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetConeOutsideVolume<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetConeOutsideVolume(::core::mem::transmute_copy(&lconeoutsidevolume), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetMaxDistance<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flmaxdistance: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxDistance(::core::mem::transmute_copy(&flmaxdistance), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetMinDistance<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flmindistance: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinDistance(::core::mem::transmute_copy(&flmindistance), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetMode<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMode(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetPosition<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetVelocity<Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVelocity(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
            GetConeAngles: GetConeAngles::<Impl, IMPL_OFFSET>,
            GetConeOrientation: GetConeOrientation::<Impl, IMPL_OFFSET>,
            GetConeOutsideVolume: GetConeOutsideVolume::<Impl, IMPL_OFFSET>,
            GetMaxDistance: GetMaxDistance::<Impl, IMPL_OFFSET>,
            GetMinDistance: GetMinDistance::<Impl, IMPL_OFFSET>,
            GetMode: GetMode::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
            GetVelocity: GetVelocity::<Impl, IMPL_OFFSET>,
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            SetConeAngles: SetConeAngles::<Impl, IMPL_OFFSET>,
            SetConeOrientation: SetConeOrientation::<Impl, IMPL_OFFSET>,
            SetConeOutsideVolume: SetConeOutsideVolume::<Impl, IMPL_OFFSET>,
            SetMaxDistance: SetMaxDistance::<Impl, IMPL_OFFSET>,
            SetMinDistance: SetMinDistance::<Impl, IMPL_OFFSET>,
            SetMode: SetMode::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            SetVelocity: SetVelocity::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSound3DBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DListener_Impl: Sized {
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DS3DLISTENER>;
    fn GetDistanceFactor(&mut self) -> ::windows::core::Result<f32>;
    fn GetDopplerFactor(&mut self) -> ::windows::core::Result<f32>;
    fn GetOrientation(&mut self, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::Result<()>;
    fn GetPosition(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetRolloffFactor(&mut self) -> ::windows::core::Result<f32>;
    fn GetVelocity(&mut self) -> ::windows::core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn SetAllParameters(&mut self, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetDistanceFactor(&mut self, fldistancefactor: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetDopplerFactor(&mut self, fldopplerfactor: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetOrientation(&mut self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetPosition(&mut self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetRolloffFactor(&mut self, flrollofffactor: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn SetVelocity(&mut self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::Result<()>;
    fn CommitDeferredSettings(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDirectSound3DListener_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSound3DListener_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSound3DListener_Vtbl {
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plistener: *mut DS3DLISTENER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *plistener = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDistanceFactor<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfldistancefactor: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDistanceFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *pfldistancefactor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDopplerFactor<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfldopplerfactor: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDopplerFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *pfldopplerfactor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientation<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOrientation(::core::mem::transmute_copy(&pvorientfront), ::core::mem::transmute_copy(&pvorienttop)).into()
        }
        unsafe extern "system" fn GetPosition<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPosition() {
                ::core::result::Result::Ok(ok__) => {
                    *pvposition = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRolloffFactor<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflrollofffactor: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRolloffFactor() {
                ::core::result::Result::Ok(ok__) => {
                    *pflrollofffactor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocity<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    *pvvelocity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pclistener), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetDistanceFactor<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fldistancefactor: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDistanceFactor(::core::mem::transmute_copy(&fldistancefactor), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetDopplerFactor<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fldopplerfactor: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDopplerFactor(::core::mem::transmute_copy(&fldopplerfactor), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetOrientation<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOrientation(::core::mem::transmute_copy(&xfront), ::core::mem::transmute_copy(&yfront), ::core::mem::transmute_copy(&zfront), ::core::mem::transmute_copy(&xtop), ::core::mem::transmute_copy(&ytop), ::core::mem::transmute_copy(&ztop), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetPosition<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetRolloffFactor<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flrollofffactor: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRolloffFactor(::core::mem::transmute_copy(&flrollofffactor), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetVelocity<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVelocity(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn CommitDeferredSettings<Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CommitDeferredSettings().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
            GetDistanceFactor: GetDistanceFactor::<Impl, IMPL_OFFSET>,
            GetDopplerFactor: GetDopplerFactor::<Impl, IMPL_OFFSET>,
            GetOrientation: GetOrientation::<Impl, IMPL_OFFSET>,
            GetPosition: GetPosition::<Impl, IMPL_OFFSET>,
            GetRolloffFactor: GetRolloffFactor::<Impl, IMPL_OFFSET>,
            GetVelocity: GetVelocity::<Impl, IMPL_OFFSET>,
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            SetDistanceFactor: SetDistanceFactor::<Impl, IMPL_OFFSET>,
            SetDopplerFactor: SetDopplerFactor::<Impl, IMPL_OFFSET>,
            SetOrientation: SetOrientation::<Impl, IMPL_OFFSET>,
            SetPosition: SetPosition::<Impl, IMPL_OFFSET>,
            SetRolloffFactor: SetRolloffFactor::<Impl, IMPL_OFFSET>,
            SetVelocity: SetVelocity::<Impl, IMPL_OFFSET>,
            CommitDeferredSettings: CommitDeferredSettings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSound3DListener as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSound8_Impl: Sized + IDirectSound_Impl {
    fn VerifyCertification(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSound8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSound8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSound8_Vtbl {
        unsafe extern "system" fn VerifyCertification<Impl: IDirectSound8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcertified: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VerifyCertification() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcertified = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDirectSound_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), VerifyCertification: VerifyCertification::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSound8 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundBuffer_Impl: Sized {
    fn GetCaps(&mut self) -> ::windows::core::Result<DSBCAPS>;
    fn GetCurrentPosition(&mut self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::Result<()>;
    fn GetFormat(&mut self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()>;
    fn GetVolume(&mut self) -> ::windows::core::Result<i32>;
    fn GetPan(&mut self) -> ::windows::core::Result<i32>;
    fn GetFrequency(&mut self) -> ::windows::core::Result<u32>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn Initialize(&mut self, pdirectsound: ::core::option::Option<IDirectSound>, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::Result<()>;
    fn Lock(&mut self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn Play(&mut self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn SetCurrentPosition(&mut self, dwnewposition: u32) -> ::windows::core::Result<()>;
    fn SetFormat(&mut self, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::Result<()>;
    fn SetVolume(&mut self, lvolume: i32) -> ::windows::core::Result<()>;
    fn SetPan(&mut self, lpan: i32) -> ::windows::core::Result<()>;
    fn SetFrequency(&mut self, dwfrequency: u32) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Unlock(&mut self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()>;
    fn Restore(&mut self) -> ::windows::core::Result<()>;
}
impl IDirectSoundBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundBuffer_Vtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsbuffercaps: *mut DSBCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsbuffercaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentPosition(::core::mem::transmute_copy(&pdwcurrentplaycursor), ::core::mem::transmute_copy(&pdwcurrentwritecursor)).into()
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormat(::core::mem::transmute_copy(&pwfxformat), ::core::mem::transmute_copy(&dwsizeallocated), ::core::mem::transmute_copy(&pdwsizewritten)).into()
        }
        unsafe extern "system" fn GetVolume<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *plvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPan<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpan: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPan() {
                ::core::result::Result::Ok(ok__) => {
                    *plpan = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrequency<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfrequency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwfrequency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pdirectsound), ::core::mem::transmute_copy(&pcdsbufferdesc)).into()
        }
        unsafe extern "system" fn Lock<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&dwoffset), ::core::mem::transmute_copy(&dwbytes), ::core::mem::transmute_copy(&ppvaudioptr1), ::core::mem::transmute_copy(&pdwaudiobytes1), ::core::mem::transmute_copy(&ppvaudioptr2), ::core::mem::transmute_copy(&pdwaudiobytes2), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Play<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Play(::core::mem::transmute_copy(&dwreserved1), ::core::mem::transmute_copy(&dwpriority), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetCurrentPosition<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnewposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentPosition(::core::mem::transmute_copy(&dwnewposition)).into()
        }
        unsafe extern "system" fn SetFormat<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFormat(::core::mem::transmute_copy(&pcfxformat)).into()
        }
        unsafe extern "system" fn SetVolume<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn SetPan<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpan: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPan(::core::mem::transmute_copy(&lpan)).into()
        }
        unsafe extern "system" fn SetFrequency<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfrequency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFrequency(::core::mem::transmute_copy(&dwfrequency)).into()
        }
        unsafe extern "system" fn Stop<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&pvaudioptr1), ::core::mem::transmute_copy(&dwaudiobytes1), ::core::mem::transmute_copy(&pvaudioptr2), ::core::mem::transmute_copy(&dwaudiobytes2)).into()
        }
        unsafe extern "system" fn Restore<Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Restore().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Impl, IMPL_OFFSET>,
            GetFormat: GetFormat::<Impl, IMPL_OFFSET>,
            GetVolume: GetVolume::<Impl, IMPL_OFFSET>,
            GetPan: GetPan::<Impl, IMPL_OFFSET>,
            GetFrequency: GetFrequency::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Lock: Lock::<Impl, IMPL_OFFSET>,
            Play: Play::<Impl, IMPL_OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Impl, IMPL_OFFSET>,
            SetFormat: SetFormat::<Impl, IMPL_OFFSET>,
            SetVolume: SetVolume::<Impl, IMPL_OFFSET>,
            SetPan: SetPan::<Impl, IMPL_OFFSET>,
            SetFrequency: SetFrequency::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
            Restore: Restore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundBuffer8_Impl: Sized + IDirectSoundBuffer_Impl {
    fn SetFX(&mut self, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::core::Result<()>;
    fn AcquireResources(&mut self, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::core::Result<()>;
    fn GetObjectInPath(&mut self, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDirectSoundBuffer8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundBuffer8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundBuffer8_Vtbl {
        unsafe extern "system" fn SetFX<Impl: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFX(::core::mem::transmute_copy(&dweffectscount), ::core::mem::transmute_copy(&pdsfxdesc), ::core::mem::transmute_copy(&pdwresultcodes)).into()
        }
        unsafe extern "system" fn AcquireResources<Impl: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireResources(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dweffectscount), ::core::mem::transmute_copy(&pdwresultcodes)).into()
        }
        unsafe extern "system" fn GetObjectInPath<Impl: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInPath(::core::mem::transmute_copy(&rguidobject), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&rguidinterface), ::core::mem::transmute_copy(&ppobject)).into()
        }
        Self {
            base: IDirectSoundBuffer_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetFX: SetFX::<Impl, IMPL_OFFSET>,
            AcquireResources: AcquireResources::<Impl, IMPL_OFFSET>,
            GetObjectInPath: GetObjectInPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundBuffer8 as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundCapture_Impl: Sized {
    fn CreateCaptureBuffer(&mut self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::core::option::Option<IDirectSoundCaptureBuffer>, punkouter: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetCaps(&mut self) -> ::windows::core::Result<DSCCAPS>;
    fn Initialize(&mut self, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IDirectSoundCapture_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCapture_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCapture_Vtbl {
        unsafe extern "system" fn CreateCaptureBuffer<Impl: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateCaptureBuffer(::core::mem::transmute_copy(&pcdscbufferdesc), ::core::mem::transmute_copy(&ppdscbuffer), ::core::mem::transmute(&punkouter)).into()
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsccaps: *mut DSCCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsccaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pcguiddevice)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateCaptureBuffer: CreateCaptureBuffer::<Impl, IMPL_OFFSET>,
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCapture as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundCaptureBuffer_Impl: Sized {
    fn GetCaps(&mut self) -> ::windows::core::Result<DSCBCAPS>;
    fn GetCurrentPosition(&mut self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::Result<()>;
    fn GetFormat(&mut self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::Result<()>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn Initialize(&mut self, pdirectsoundcapture: ::core::option::Option<IDirectSoundCapture>, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::Result<()>;
    fn Lock(&mut self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn Start(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Unlock(&mut self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::Result<()>;
}
impl IDirectSoundCaptureBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCaptureBuffer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCaptureBuffer_Vtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscbcaps: *mut DSCBCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCaps() {
                ::core::result::Result::Ok(ok__) => {
                    *pdscbcaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentPosition(::core::mem::transmute_copy(&pdwcaptureposition), ::core::mem::transmute_copy(&pdwreadposition)).into()
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFormat(::core::mem::transmute_copy(&pwfxformat), ::core::mem::transmute_copy(&dwsizeallocated), ::core::mem::transmute_copy(&pdwsizewritten)).into()
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsoundcapture: ::windows::core::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pdirectsoundcapture), ::core::mem::transmute_copy(&pcdscbufferdesc)).into()
        }
        unsafe extern "system" fn Lock<Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Lock(::core::mem::transmute_copy(&dwoffset), ::core::mem::transmute_copy(&dwbytes), ::core::mem::transmute_copy(&ppvaudioptr1), ::core::mem::transmute_copy(&pdwaudiobytes1), ::core::mem::transmute_copy(&ppvaudioptr2), ::core::mem::transmute_copy(&pdwaudiobytes2), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Start<Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Stop<Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Unlock<Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unlock(::core::mem::transmute_copy(&pvaudioptr1), ::core::mem::transmute_copy(&dwaudiobytes1), ::core::mem::transmute_copy(&pvaudioptr2), ::core::mem::transmute_copy(&dwaudiobytes2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCaps: GetCaps::<Impl, IMPL_OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Impl, IMPL_OFFSET>,
            GetFormat: GetFormat::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Lock: Lock::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Unlock: Unlock::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundCaptureBuffer8_Impl: Sized + IDirectSoundCaptureBuffer_Impl {
    fn GetObjectInPath(&mut self, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetFXStatus(&mut self, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::core::Result<()>;
}
impl IDirectSoundCaptureBuffer8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCaptureBuffer8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCaptureBuffer8_Vtbl {
        unsafe extern "system" fn GetObjectInPath<Impl: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObjectInPath(::core::mem::transmute_copy(&rguidobject), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&rguidinterface), ::core::mem::transmute_copy(&ppobject)).into()
        }
        unsafe extern "system" fn GetFXStatus<Impl: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFXStatus(::core::mem::transmute_copy(&dweffectscount), ::core::mem::transmute_copy(&pdwfxstatus)).into()
        }
        Self {
            base: IDirectSoundCaptureBuffer_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetObjectInPath: GetObjectInPath::<Impl, IMPL_OFFSET>,
            GetFXStatus: GetFXStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundCaptureFXAec_Impl: Sized {
    fn SetAllParameters(&mut self, pdscfxaec: *const DSCFXAec) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSCFXAec>;
    fn GetStatus(&mut self) -> ::windows::core::Result<u32>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundCaptureFXAec_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCaptureFXAec_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCaptureFXAec_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscfxaec: *const DSCFXAec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pdscfxaec)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscfxaec: *mut DSCFXAec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdscfxaec = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
            GetStatus: GetStatus::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXAec as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundCaptureFXNoiseSuppress_Impl: Sized {
    fn SetAllParameters(&mut self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSCFXNoiseSuppress>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundCaptureFXNoiseSuppress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundCaptureFXNoiseSuppress_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdscfxnoisesuppress)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdscfxnoisesuppress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXNoiseSuppress as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXChorus_Impl: Sized {
    fn SetAllParameters(&mut self, pcdsfxchorus: *const DSFXChorus) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSFXChorus>;
}
impl IDirectSoundFXChorus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXChorus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXChorus_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXChorus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxchorus: *const DSFXChorus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdsfxchorus)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXChorus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxchorus: *mut DSFXChorus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsfxchorus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXChorus as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXCompressor_Impl: Sized {
    fn SetAllParameters(&mut self, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSFXCompressor>;
}
impl IDirectSoundFXCompressor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXCompressor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXCompressor_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXCompressor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdsfxcompressor)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXCompressor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxcompressor: *mut DSFXCompressor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsfxcompressor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXCompressor as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXDistortion_Impl: Sized {
    fn SetAllParameters(&mut self, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSFXDistortion>;
}
impl IDirectSoundFXDistortion_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXDistortion_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXDistortion_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXDistortion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdsfxdistortion)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXDistortion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxdistortion: *mut DSFXDistortion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsfxdistortion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXDistortion as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXEcho_Impl: Sized {
    fn SetAllParameters(&mut self, pcdsfxecho: *const DSFXEcho) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSFXEcho>;
}
impl IDirectSoundFXEcho_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXEcho_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXEcho_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXEcho_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxecho: *const DSFXEcho) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdsfxecho)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXEcho_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxecho: *mut DSFXEcho) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsfxecho = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXEcho as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXFlanger_Impl: Sized {
    fn SetAllParameters(&mut self, pcdsfxflanger: *const DSFXFlanger) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSFXFlanger>;
}
impl IDirectSoundFXFlanger_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXFlanger_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXFlanger_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXFlanger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxflanger: *const DSFXFlanger) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdsfxflanger)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXFlanger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxflanger: *mut DSFXFlanger) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsfxflanger = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXFlanger as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXGargle_Impl: Sized {
    fn SetAllParameters(&mut self, pcdsfxgargle: *const DSFXGargle) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSFXGargle>;
}
impl IDirectSoundFXGargle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXGargle_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXGargle_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXGargle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxgargle: *const DSFXGargle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdsfxgargle)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXGargle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxgargle: *mut DSFXGargle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsfxgargle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXGargle as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXI3DL2Reverb_Impl: Sized {
    fn SetAllParameters(&mut self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSFXI3DL2Reverb>;
    fn SetPreset(&mut self, dwpreset: u32) -> ::windows::core::Result<()>;
    fn GetPreset(&mut self) -> ::windows::core::Result<u32>;
    fn SetQuality(&mut self, lquality: i32) -> ::windows::core::Result<()>;
    fn GetQuality(&mut self) -> ::windows::core::Result<i32>;
}
impl IDirectSoundFXI3DL2Reverb_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXI3DL2Reverb_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXI3DL2Reverb_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdsfxi3dl2reverb)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsfxi3dl2reverb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreset<Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpreset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreset(::core::mem::transmute_copy(&dwpreset)).into()
        }
        unsafe extern "system" fn GetPreset<Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpreset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPreset() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwpreset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquality: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQuality(::core::mem::transmute_copy(&lquality)).into()
        }
        unsafe extern "system" fn GetQuality<Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQuality() {
                ::core::result::Result::Ok(ok__) => {
                    *plquality = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
            SetPreset: SetPreset::<Impl, IMPL_OFFSET>,
            GetPreset: GetPreset::<Impl, IMPL_OFFSET>,
            SetQuality: SetQuality::<Impl, IMPL_OFFSET>,
            GetQuality: GetQuality::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXI3DL2Reverb as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXParamEq_Impl: Sized {
    fn SetAllParameters(&mut self, pcdsfxparameq: *const DSFXParamEq) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSFXParamEq>;
}
impl IDirectSoundFXParamEq_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXParamEq_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXParamEq_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXParamEq_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxparameq: *const DSFXParamEq) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdsfxparameq)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXParamEq_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxparameq: *mut DSFXParamEq) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsfxparameq = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXParamEq as ::windows::core::Interface>::IID
    }
}
pub trait IDirectSoundFXWavesReverb_Impl: Sized {
    fn SetAllParameters(&mut self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::core::Result<()>;
    fn GetAllParameters(&mut self) -> ::windows::core::Result<DSFXWavesReverb>;
}
impl IDirectSoundFXWavesReverb_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFXWavesReverb_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFXWavesReverb_Vtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllParameters(::core::mem::transmute_copy(&pcdsfxwavesreverb)).into()
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pdsfxwavesreverb = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAllParameters: SetAllParameters::<Impl, IMPL_OFFSET>,
            GetAllParameters: GetAllParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFXWavesReverb as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundFullDuplex_Impl: Sized {
    fn Initialize(&mut self, pcaptureguid: *const ::windows::core::GUID, prenderguid: *const ::windows::core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::core::option::Option<IDirectSoundCaptureBuffer8>, lplpdirectsoundbuffer8: *mut ::core::option::Option<IDirectSoundBuffer8>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundFullDuplex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundFullDuplex_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundFullDuplex_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IDirectSoundFullDuplex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaptureguid: *const ::windows::core::GUID, prenderguid: *const ::windows::core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::windows::core::RawPtr, lplpdirectsoundbuffer8: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pcaptureguid), ::core::mem::transmute_copy(&prenderguid), ::core::mem::transmute_copy(&lpdscbufferdesc), ::core::mem::transmute_copy(&lpdsbufferdesc), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&lplpdirectsoundcapturebuffer8), ::core::mem::transmute_copy(&lplpdirectsoundbuffer8)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundFullDuplex as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundNotify_Impl: Sized {
    fn SetNotificationPositions(&mut self, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDirectSoundNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDirectSoundNotify_Vtbl {
        unsafe extern "system" fn SetNotificationPositions<Impl: IDirectSoundNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNotificationPositions(::core::mem::transmute_copy(&dwpositionnotifies), ::core::mem::transmute_copy(&pcpositionnotifies)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetNotificationPositions: SetNotificationPositions::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDirectSoundNotify as ::windows::core::Interface>::IID
    }
}
