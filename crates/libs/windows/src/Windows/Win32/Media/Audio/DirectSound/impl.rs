#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSound_Impl: Sized {
    fn CreateSoundBuffer(&self, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::core::option::Option<IDirectSoundBuffer>, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetCaps(&self, pdscaps: *mut DSCAPS) -> ::windows_core::Result<()>;
    fn DuplicateSoundBuffer(&self, pdsbufferoriginal: ::core::option::Option<&IDirectSoundBuffer>) -> ::windows_core::Result<IDirectSoundBuffer>;
    fn SetCooperativeLevel(&self, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows_core::Result<()>;
    fn Compact(&self) -> ::windows_core::Result<()>;
    fn GetSpeakerConfig(&self) -> ::windows_core::Result<u32>;
    fn SetSpeakerConfig(&self, dwspeakerconfig: u32) -> ::windows_core::Result<()>;
    fn Initialize(&self, pcguiddevice: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDirectSound {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSound_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: isize>() -> IDirectSound_Vtbl {
        unsafe extern "system" fn CreateSoundBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSoundBuffer(::core::mem::transmute_copy(&pcdsbufferdesc), ::core::mem::transmute_copy(&ppdsbuffer), ::windows_core::from_raw_borrowed(&punkouter)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscaps: *mut DSCAPS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&pdscaps)).into()
        }
        unsafe extern "system" fn DuplicateSoundBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsbufferoriginal: *mut ::core::ffi::c_void, ppdsbufferduplicate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DuplicateSoundBuffer(::windows_core::from_raw_borrowed(&pdsbufferoriginal)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdsbufferduplicate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCooperativeLevel(::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwlevel)).into()
        }
        unsafe extern "system" fn Compact<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Compact().into()
        }
        unsafe extern "system" fn GetSpeakerConfig<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwspeakerconfig: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSpeakerConfig() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwspeakerconfig, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeakerConfig<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwspeakerconfig: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSpeakerConfig(::core::mem::transmute_copy(&dwspeakerconfig)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&pcguiddevice)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSoundBuffer: CreateSoundBuffer::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            DuplicateSoundBuffer: DuplicateSoundBuffer::<Identity, Impl, OFFSET>,
            SetCooperativeLevel: SetCooperativeLevel::<Identity, Impl, OFFSET>,
            Compact: Compact::<Identity, Impl, OFFSET>,
            GetSpeakerConfig: GetSpeakerConfig::<Identity, Impl, OFFSET>,
            SetSpeakerConfig: SetSpeakerConfig::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSound as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DBuffer_Impl: Sized {
    fn GetAllParameters(&self, pds3dbuffer: *mut DS3DBUFFER) -> ::windows_core::Result<()>;
    fn GetConeAngles(&self, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows_core::Result<()>;
    fn GetConeOrientation(&self) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetConeOutsideVolume(&self) -> ::windows_core::Result<i32>;
    fn GetMaxDistance(&self) -> ::windows_core::Result<f32>;
    fn GetMinDistance(&self) -> ::windows_core::Result<f32>;
    fn GetMode(&self) -> ::windows_core::Result<u32>;
    fn GetPosition(&self) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetVelocity(&self) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn SetAllParameters(&self, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetConeAngles(&self, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetConeOrientation(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetConeOutsideVolume(&self, lconeoutsidevolume: i32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetMaxDistance(&self, flmaxdistance: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetMinDistance(&self, flmindistance: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetMode(&self, dwmode: u32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::RuntimeName for IDirectSound3DBuffer {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDirectSound3DBuffer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>() -> IDirectSound3DBuffer_Vtbl {
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pds3dbuffer: *mut DS3DBUFFER) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllParameters(::core::mem::transmute_copy(&pds3dbuffer)).into()
        }
        unsafe extern "system" fn GetConeAngles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetConeAngles(::core::mem::transmute_copy(&pdwinsideconeangle), ::core::mem::transmute_copy(&pdwoutsideconeangle)).into()
        }
        unsafe extern "system" fn GetConeOrientation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConeOrientation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvorientation, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConeOutsideVolume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plconeoutsidevolume: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConeOutsideVolume() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plconeoutsidevolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxDistance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflmaxdistance: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxDistance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflmaxdistance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinDistance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflmindistance: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinDistance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflmindistance, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvvelocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcds3dbuffer), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetConeAngles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConeAngles(::core::mem::transmute_copy(&dwinsideconeangle), ::core::mem::transmute_copy(&dwoutsideconeangle), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetConeOrientation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConeOrientation(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetConeOutsideVolume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lconeoutsidevolume: i32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetConeOutsideVolume(::core::mem::transmute_copy(&lconeoutsidevolume), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetMaxDistance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flmaxdistance: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxDistance(::core::mem::transmute_copy(&flmaxdistance), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetMinDistance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flmindistance: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinDistance(::core::mem::transmute_copy(&flmindistance), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmode: u32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMode(::core::mem::transmute_copy(&dwmode), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVelocity(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            GetConeAngles: GetConeAngles::<Identity, Impl, OFFSET>,
            GetConeOrientation: GetConeOrientation::<Identity, Impl, OFFSET>,
            GetConeOutsideVolume: GetConeOutsideVolume::<Identity, Impl, OFFSET>,
            GetMaxDistance: GetMaxDistance::<Identity, Impl, OFFSET>,
            GetMinDistance: GetMinDistance::<Identity, Impl, OFFSET>,
            GetMode: GetMode::<Identity, Impl, OFFSET>,
            GetPosition: GetPosition::<Identity, Impl, OFFSET>,
            GetVelocity: GetVelocity::<Identity, Impl, OFFSET>,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            SetConeAngles: SetConeAngles::<Identity, Impl, OFFSET>,
            SetConeOrientation: SetConeOrientation::<Identity, Impl, OFFSET>,
            SetConeOutsideVolume: SetConeOutsideVolume::<Identity, Impl, OFFSET>,
            SetMaxDistance: SetMaxDistance::<Identity, Impl, OFFSET>,
            SetMinDistance: SetMinDistance::<Identity, Impl, OFFSET>,
            SetMode: SetMode::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            SetVelocity: SetVelocity::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSound3DBuffer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Graphics_Direct3D\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Direct3D")]
pub trait IDirectSound3DListener_Impl: Sized {
    fn GetAllParameters(&self, plistener: *mut DS3DLISTENER) -> ::windows_core::Result<()>;
    fn GetDistanceFactor(&self) -> ::windows_core::Result<f32>;
    fn GetDopplerFactor(&self) -> ::windows_core::Result<f32>;
    fn GetOrientation(&self, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::Result<()>;
    fn GetPosition(&self) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn GetRolloffFactor(&self) -> ::windows_core::Result<f32>;
    fn GetVelocity(&self) -> ::windows_core::Result<super::super::super::Graphics::Direct3D::D3DVECTOR>;
    fn SetAllParameters(&self, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetDistanceFactor(&self, fldistancefactor: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetDopplerFactor(&self, fldopplerfactor: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetOrientation(&self, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetPosition(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetRolloffFactor(&self, flrollofffactor: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn SetVelocity(&self, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::Result<()>;
    fn CommitDeferredSettings(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::windows_core::RuntimeName for IDirectSound3DListener {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl IDirectSound3DListener_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>() -> IDirectSound3DListener_Vtbl {
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plistener: *mut DS3DLISTENER) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllParameters(::core::mem::transmute_copy(&plistener)).into()
        }
        unsafe extern "system" fn GetDistanceFactor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfldistancefactor: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDistanceFactor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfldistancefactor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDopplerFactor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfldopplerfactor: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDopplerFactor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfldopplerfactor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOrientation(::core::mem::transmute_copy(&pvorientfront), ::core::mem::transmute_copy(&pvorienttop)).into()
        }
        unsafe extern "system" fn GetPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPosition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvposition, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRolloffFactor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflrollofffactor: *mut f32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRolloffFactor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflrollofffactor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVelocity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvvelocity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pclistener), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetDistanceFactor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fldistancefactor: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDistanceFactor(::core::mem::transmute_copy(&fldistancefactor), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetDopplerFactor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fldopplerfactor: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDopplerFactor(::core::mem::transmute_copy(&fldopplerfactor), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetOrientation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOrientation(::core::mem::transmute_copy(&xfront), ::core::mem::transmute_copy(&yfront), ::core::mem::transmute_copy(&zfront), ::core::mem::transmute_copy(&xtop), ::core::mem::transmute_copy(&ytop), ::core::mem::transmute_copy(&ztop), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPosition(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetRolloffFactor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flrollofffactor: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRolloffFactor(::core::mem::transmute_copy(&flrollofffactor), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn SetVelocity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVelocity(::core::mem::transmute_copy(&x), ::core::mem::transmute_copy(&y), ::core::mem::transmute_copy(&z), ::core::mem::transmute_copy(&dwapply)).into()
        }
        unsafe extern "system" fn CommitDeferredSettings<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound3DListener_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CommitDeferredSettings().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            GetDistanceFactor: GetDistanceFactor::<Identity, Impl, OFFSET>,
            GetDopplerFactor: GetDopplerFactor::<Identity, Impl, OFFSET>,
            GetOrientation: GetOrientation::<Identity, Impl, OFFSET>,
            GetPosition: GetPosition::<Identity, Impl, OFFSET>,
            GetRolloffFactor: GetRolloffFactor::<Identity, Impl, OFFSET>,
            GetVelocity: GetVelocity::<Identity, Impl, OFFSET>,
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            SetDistanceFactor: SetDistanceFactor::<Identity, Impl, OFFSET>,
            SetDopplerFactor: SetDopplerFactor::<Identity, Impl, OFFSET>,
            SetOrientation: SetOrientation::<Identity, Impl, OFFSET>,
            SetPosition: SetPosition::<Identity, Impl, OFFSET>,
            SetRolloffFactor: SetRolloffFactor::<Identity, Impl, OFFSET>,
            SetVelocity: SetVelocity::<Identity, Impl, OFFSET>,
            CommitDeferredSettings: CommitDeferredSettings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSound3DListener as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSound8_Impl: Sized + IDirectSound_Impl {
    fn VerifyCertification(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDirectSound8 {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSound8_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound8_Impl, const OFFSET: isize>() -> IDirectSound8_Vtbl {
        unsafe extern "system" fn VerifyCertification<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSound8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcertified: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VerifyCertification() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcertified, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IDirectSound_Vtbl::new::<Identity, Impl, OFFSET>(), VerifyCertification: VerifyCertification::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSound8 as ::windows_core::ComInterface>::IID || iid == &<IDirectSound as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundBuffer_Impl: Sized {
    fn GetCaps(&self, pdsbuffercaps: *mut DSBCAPS) -> ::windows_core::Result<()>;
    fn GetCurrentPosition(&self, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows_core::Result<()>;
    fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_core::Result<()>;
    fn GetVolume(&self) -> ::windows_core::Result<i32>;
    fn GetPan(&self) -> ::windows_core::Result<i32>;
    fn GetFrequency(&self) -> ::windows_core::Result<u32>;
    fn GetStatus(&self) -> ::windows_core::Result<u32>;
    fn Initialize(&self, pdirectsound: ::core::option::Option<&IDirectSound>, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows_core::Result<()>;
    fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn Play(&self, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn SetCurrentPosition(&self, dwnewposition: u32) -> ::windows_core::Result<()>;
    fn SetFormat(&self, pcfxformat: *const super::WAVEFORMATEX) -> ::windows_core::Result<()>;
    fn SetVolume(&self, lvolume: i32) -> ::windows_core::Result<()>;
    fn SetPan(&self, lpan: i32) -> ::windows_core::Result<()>;
    fn SetFrequency(&self, dwfrequency: u32) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
    fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_core::Result<()>;
    fn Restore(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundBuffer {}
impl IDirectSoundBuffer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>() -> IDirectSoundBuffer_Vtbl {
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsbuffercaps: *mut DSBCAPS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCaps(::core::mem::transmute_copy(&pdsbuffercaps)).into()
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentPosition(::core::mem::transmute_copy(&pdwcurrentplaycursor), ::core::mem::transmute_copy(&pdwcurrentwritecursor)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFormat(::core::mem::transmute_copy(&pwfxformat), ::core::mem::transmute_copy(&dwsizeallocated), ::core::mem::transmute_copy(&pdwsizewritten)).into()
        }
        unsafe extern "system" fn GetVolume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVolume() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plpan: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPan() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plpan, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrequency<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwfrequency: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFrequency() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwfrequency, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsound: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pdirectsound), ::core::mem::transmute_copy(&pcdsbufferdesc)).into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&dwoffset), ::core::mem::transmute_copy(&dwbytes), ::core::mem::transmute_copy(&ppvaudioptr1), ::core::mem::transmute_copy(&pdwaudiobytes1), ::core::mem::transmute_copy(&ppvaudioptr2), ::core::mem::transmute_copy(&pdwaudiobytes2), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Play<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Play(::core::mem::transmute_copy(&dwreserved1), ::core::mem::transmute_copy(&dwpriority), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn SetCurrentPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnewposition: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentPosition(::core::mem::transmute_copy(&dwnewposition)).into()
        }
        unsafe extern "system" fn SetFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcfxformat: *const super::WAVEFORMATEX) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFormat(::core::mem::transmute_copy(&pcfxformat)).into()
        }
        unsafe extern "system" fn SetVolume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVolume(::core::mem::transmute_copy(&lvolume)).into()
        }
        unsafe extern "system" fn SetPan<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpan: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPan(::core::mem::transmute_copy(&lpan)).into()
        }
        unsafe extern "system" fn SetFrequency<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwfrequency: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFrequency(::core::mem::transmute_copy(&dwfrequency)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock(::core::mem::transmute_copy(&pvaudioptr1), ::core::mem::transmute_copy(&dwaudiobytes1), ::core::mem::transmute_copy(&pvaudioptr2), ::core::mem::transmute_copy(&dwaudiobytes2)).into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetVolume: GetVolume::<Identity, Impl, OFFSET>,
            GetPan: GetPan::<Identity, Impl, OFFSET>,
            GetFrequency: GetFrequency::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Play: Play::<Identity, Impl, OFFSET>,
            SetCurrentPosition: SetCurrentPosition::<Identity, Impl, OFFSET>,
            SetFormat: SetFormat::<Identity, Impl, OFFSET>,
            SetVolume: SetVolume::<Identity, Impl, OFFSET>,
            SetPan: SetPan::<Identity, Impl, OFFSET>,
            SetFrequency: SetFrequency::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundBuffer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundBuffer8_Impl: Sized + IDirectSoundBuffer_Impl {
    fn SetFX(&self, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows_core::Result<()>;
    fn AcquireResources(&self, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows_core::Result<()>;
    fn GetObjectInPath(&self, rguidobject: *const ::windows_core::GUID, dwindex: u32, rguidinterface: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundBuffer8 {}
impl IDirectSoundBuffer8_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer8_Impl, const OFFSET: isize>() -> IDirectSoundBuffer8_Vtbl {
        unsafe extern "system" fn SetFX<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFX(::core::mem::transmute_copy(&dweffectscount), ::core::mem::transmute_copy(&pdsfxdesc), ::core::mem::transmute_copy(&pdwresultcodes)).into()
        }
        unsafe extern "system" fn AcquireResources<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireResources(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dweffectscount), ::core::mem::transmute_copy(&pdwresultcodes)).into()
        }
        unsafe extern "system" fn GetObjectInPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows_core::GUID, dwindex: u32, rguidinterface: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectInPath(::core::mem::transmute_copy(&rguidobject), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&rguidinterface), ::core::mem::transmute_copy(&ppobject)).into()
        }
        Self {
            base__: IDirectSoundBuffer_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetFX: SetFX::<Identity, Impl, OFFSET>,
            AcquireResources: AcquireResources::<Identity, Impl, OFFSET>,
            GetObjectInPath: GetObjectInPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundBuffer8 as ::windows_core::ComInterface>::IID || iid == &<IDirectSoundBuffer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundCapture_Impl: Sized {
    fn CreateCaptureBuffer(&self, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::core::option::Option<IDirectSoundCaptureBuffer>, punkouter: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetCaps(&self) -> ::windows_core::Result<DSCCAPS>;
    fn Initialize(&self, pcguiddevice: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundCapture {}
impl IDirectSoundCapture_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCapture_Impl, const OFFSET: isize>() -> IDirectSoundCapture_Vtbl {
        unsafe extern "system" fn CreateCaptureBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateCaptureBuffer(::core::mem::transmute_copy(&pcdscbufferdesc), ::core::mem::transmute_copy(&ppdscbuffer), ::windows_core::from_raw_borrowed(&punkouter)).into()
        }
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsccaps: *mut DSCCAPS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCaps() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdsccaps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCapture_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&pcguiddevice)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateCaptureBuffer: CreateCaptureBuffer::<Identity, Impl, OFFSET>,
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundCapture as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundCaptureBuffer_Impl: Sized {
    fn GetCaps(&self) -> ::windows_core::Result<DSCBCAPS>;
    fn GetCurrentPosition(&self, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows_core::Result<()>;
    fn GetFormat(&self, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_core::Result<()>;
    fn GetStatus(&self) -> ::windows_core::Result<u32>;
    fn Initialize(&self, pdirectsoundcapture: ::core::option::Option<&IDirectSoundCapture>, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows_core::Result<()>;
    fn Lock(&self, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn Start(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
    fn Unlock(&self, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundCaptureBuffer {}
impl IDirectSoundCaptureBuffer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>() -> IDirectSoundCaptureBuffer_Vtbl {
        unsafe extern "system" fn GetCaps<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscbcaps: *mut DSCBCAPS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCaps() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdscbcaps, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentPosition(::core::mem::transmute_copy(&pdwcaptureposition), ::core::mem::transmute_copy(&pdwreadposition)).into()
        }
        unsafe extern "system" fn GetFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFormat(::core::mem::transmute_copy(&pwfxformat), ::core::mem::transmute_copy(&dwsizeallocated), ::core::mem::transmute_copy(&pdwsizewritten)).into()
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirectsoundcapture: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&pdirectsoundcapture), ::core::mem::transmute_copy(&pcdscbufferdesc)).into()
        }
        unsafe extern "system" fn Lock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&dwoffset), ::core::mem::transmute_copy(&dwbytes), ::core::mem::transmute_copy(&ppvaudioptr1), ::core::mem::transmute_copy(&pdwaudiobytes1), ::core::mem::transmute_copy(&ppvaudioptr2), ::core::mem::transmute_copy(&pdwaudiobytes2), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock(::core::mem::transmute_copy(&pvaudioptr1), ::core::mem::transmute_copy(&dwaudiobytes1), ::core::mem::transmute_copy(&pvaudioptr2), ::core::mem::transmute_copy(&dwaudiobytes2)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCaps: GetCaps::<Identity, Impl, OFFSET>,
            GetCurrentPosition: GetCurrentPosition::<Identity, Impl, OFFSET>,
            GetFormat: GetFormat::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Lock: Lock::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Unlock: Unlock::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundCaptureBuffer8_Impl: Sized + IDirectSoundCaptureBuffer_Impl {
    fn GetObjectInPath(&self, rguidobject: *const ::windows_core::GUID, dwindex: u32, rguidinterface: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn GetFXStatus(&self, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundCaptureBuffer8 {}
impl IDirectSoundCaptureBuffer8_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>() -> IDirectSoundCaptureBuffer8_Vtbl {
        unsafe extern "system" fn GetObjectInPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows_core::GUID, dwindex: u32, rguidinterface: *const ::windows_core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObjectInPath(::core::mem::transmute_copy(&rguidobject), ::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&rguidinterface), ::core::mem::transmute_copy(&ppobject)).into()
        }
        unsafe extern "system" fn GetFXStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureBuffer8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFXStatus(::core::mem::transmute_copy(&dweffectscount), ::core::mem::transmute_copy(&pdwfxstatus)).into()
        }
        Self {
            base__: IDirectSoundCaptureBuffer_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetObjectInPath: GetObjectInPath::<Identity, Impl, OFFSET>,
            GetFXStatus: GetFXStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureBuffer8 as ::windows_core::ComInterface>::IID || iid == &<IDirectSoundCaptureBuffer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundCaptureFXAec_Impl: Sized {
    fn SetAllParameters(&self, pdscfxaec: *const DSCFXAec) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self) -> ::windows_core::Result<DSCFXAec>;
    fn GetStatus(&self) -> ::windows_core::Result<u32>;
    fn Reset(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDirectSoundCaptureFXAec {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundCaptureFXAec_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>() -> IDirectSoundCaptureFXAec_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscfxaec: *const DSCFXAec) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pdscfxaec)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscfxaec: *mut DSCFXAec) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdscfxaec, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureFXAec_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            GetStatus: GetStatus::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXAec as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundCaptureFXNoiseSuppress_Impl: Sized {
    fn SetAllParameters(&self, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self) -> ::windows_core::Result<DSCFXNoiseSuppress>;
    fn Reset(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDirectSoundCaptureFXNoiseSuppress {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundCaptureFXNoiseSuppress_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>() -> IDirectSoundCaptureFXNoiseSuppress_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdscfxnoisesuppress)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdscfxnoisesuppress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundCaptureFXNoiseSuppress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundCaptureFXNoiseSuppress as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundFXChorus_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxchorus: *const DSFXChorus) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxchorus: *mut DSFXChorus) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundFXChorus {}
impl IDirectSoundFXChorus_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXChorus_Impl, const OFFSET: isize>() -> IDirectSoundFXChorus_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXChorus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxchorus: *const DSFXChorus) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdsfxchorus)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXChorus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxchorus: *mut DSFXChorus) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllParameters(::core::mem::transmute_copy(&pdsfxchorus)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXChorus as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundFXCompressor_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxcompressor: *const DSFXCompressor) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxcompressor: *mut DSFXCompressor) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundFXCompressor {}
impl IDirectSoundFXCompressor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXCompressor_Impl, const OFFSET: isize>() -> IDirectSoundFXCompressor_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXCompressor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxcompressor: *const DSFXCompressor) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdsfxcompressor)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXCompressor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxcompressor: *mut DSFXCompressor) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllParameters(::core::mem::transmute_copy(&pdsfxcompressor)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXCompressor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundFXDistortion_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxdistortion: *const DSFXDistortion) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxdistortion: *mut DSFXDistortion) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundFXDistortion {}
impl IDirectSoundFXDistortion_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXDistortion_Impl, const OFFSET: isize>() -> IDirectSoundFXDistortion_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXDistortion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxdistortion: *const DSFXDistortion) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdsfxdistortion)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXDistortion_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxdistortion: *mut DSFXDistortion) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllParameters(::core::mem::transmute_copy(&pdsfxdistortion)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXDistortion as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundFXEcho_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxecho: *const DSFXEcho) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxecho: *mut DSFXEcho) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundFXEcho {}
impl IDirectSoundFXEcho_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXEcho_Impl, const OFFSET: isize>() -> IDirectSoundFXEcho_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXEcho_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxecho: *const DSFXEcho) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdsfxecho)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXEcho_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxecho: *mut DSFXEcho) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllParameters(::core::mem::transmute_copy(&pdsfxecho)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXEcho as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundFXFlanger_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxflanger: *const DSFXFlanger) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxflanger: *mut DSFXFlanger) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDirectSoundFXFlanger {}
impl IDirectSoundFXFlanger_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXFlanger_Impl, const OFFSET: isize>() -> IDirectSoundFXFlanger_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXFlanger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxflanger: *const DSFXFlanger) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdsfxflanger)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXFlanger_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxflanger: *mut DSFXFlanger) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllParameters(::core::mem::transmute_copy(&pdsfxflanger)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXFlanger as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundFXGargle_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxgargle: *const DSFXGargle) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self) -> ::windows_core::Result<DSFXGargle>;
}
impl ::windows_core::RuntimeName for IDirectSoundFXGargle {}
impl IDirectSoundFXGargle_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXGargle_Impl, const OFFSET: isize>() -> IDirectSoundFXGargle_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXGargle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxgargle: *const DSFXGargle) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdsfxgargle)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXGargle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxgargle: *mut DSFXGargle) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdsfxgargle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXGargle as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundFXI3DL2Reverb_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows_core::Result<()>;
    fn SetPreset(&self, dwpreset: u32) -> ::windows_core::Result<()>;
    fn GetPreset(&self) -> ::windows_core::Result<u32>;
    fn SetQuality(&self, lquality: i32) -> ::windows_core::Result<()>;
    fn GetQuality(&self) -> ::windows_core::Result<i32>;
}
impl ::windows_core::RuntimeName for IDirectSoundFXI3DL2Reverb {}
impl IDirectSoundFXI3DL2Reverb_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>() -> IDirectSoundFXI3DL2Reverb_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdsfxi3dl2reverb)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAllParameters(::core::mem::transmute_copy(&pdsfxi3dl2reverb)).into()
        }
        unsafe extern "system" fn SetPreset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpreset: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreset(::core::mem::transmute_copy(&dwpreset)).into()
        }
        unsafe extern "system" fn GetPreset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwpreset: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPreset() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwpreset, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lquality: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuality(::core::mem::transmute_copy(&lquality)).into()
        }
        unsafe extern "system" fn GetQuality<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXI3DL2Reverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plquality: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQuality() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plquality, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
            SetPreset: SetPreset::<Identity, Impl, OFFSET>,
            GetPreset: GetPreset::<Identity, Impl, OFFSET>,
            SetQuality: SetQuality::<Identity, Impl, OFFSET>,
            GetQuality: GetQuality::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXI3DL2Reverb as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundFXParamEq_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxparameq: *const DSFXParamEq) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self) -> ::windows_core::Result<DSFXParamEq>;
}
impl ::windows_core::RuntimeName for IDirectSoundFXParamEq {}
impl IDirectSoundFXParamEq_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXParamEq_Impl, const OFFSET: isize>() -> IDirectSoundFXParamEq_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXParamEq_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxparameq: *const DSFXParamEq) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdsfxparameq)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXParamEq_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxparameq: *mut DSFXParamEq) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdsfxparameq, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXParamEq as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"implement\"`*"]
pub trait IDirectSoundFXWavesReverb_Impl: Sized {
    fn SetAllParameters(&self, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows_core::Result<()>;
    fn GetAllParameters(&self) -> ::windows_core::Result<DSFXWavesReverb>;
}
impl ::windows_core::RuntimeName for IDirectSoundFXWavesReverb {}
impl IDirectSoundFXWavesReverb_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>() -> IDirectSoundFXWavesReverb_Vtbl {
        unsafe extern "system" fn SetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllParameters(::core::mem::transmute_copy(&pcdsfxwavesreverb)).into()
        }
        unsafe extern "system" fn GetAllParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFXWavesReverb_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdsfxwavesreverb, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAllParameters: SetAllParameters::<Identity, Impl, OFFSET>,
            GetAllParameters: GetAllParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFXWavesReverb as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundFullDuplex_Impl: Sized {
    fn Initialize(&self, pcaptureguid: *const ::windows_core::GUID, prenderguid: *const ::windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::core::option::Option<IDirectSoundCaptureBuffer8>, lplpdirectsoundbuffer8: *mut ::core::option::Option<IDirectSoundBuffer8>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDirectSoundFullDuplex {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundFullDuplex_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFullDuplex_Impl, const OFFSET: isize>() -> IDirectSoundFullDuplex_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundFullDuplex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcaptureguid: *const ::windows_core::GUID, prenderguid: *const ::windows_core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut *mut ::core::ffi::c_void, lplpdirectsoundbuffer8: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&pcaptureguid), ::core::mem::transmute_copy(&prenderguid), ::core::mem::transmute_copy(&lpdscbufferdesc), ::core::mem::transmute_copy(&lpdsbufferdesc), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwlevel), ::core::mem::transmute_copy(&lplpdirectsoundcapturebuffer8), ::core::mem::transmute_copy(&lplpdirectsoundbuffer8)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundFullDuplex as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Media_Audio_DirectSound\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDirectSoundNotify_Impl: Sized {
    fn SetNotificationPositions(&self, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDirectSoundNotify {}
#[cfg(feature = "Win32_Foundation")]
impl IDirectSoundNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundNotify_Impl, const OFFSET: isize>() -> IDirectSoundNotify_Vtbl {
        unsafe extern "system" fn SetNotificationPositions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDirectSoundNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNotificationPositions(::core::mem::transmute_copy(&dwpositionnotifies), ::core::mem::transmute_copy(&pcpositionnotifies)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetNotificationPositions: SetNotificationPositions::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDirectSoundNotify as ::windows_core::ComInterface>::IID
    }
}
