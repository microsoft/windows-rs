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
impl ::windows::core::RuntimeName for IDirectSound {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSound";
}
impl IDirectSoundVtbl {
    pub const fn new<Impl: IDirectSoundImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundVtbl {
        unsafe extern "system" fn CreateSoundBuffer<Impl: IDirectSoundImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsbufferdesc: *const DSBUFFERDESC, ppdsbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSoundBuffer(&*(&pcdsbufferdesc as *const <DSBUFFERDESC as ::windows::core::Abi>::Abi as *const <DSBUFFERDESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdsbuffer), &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscaps: *mut DSCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(::core::mem::transmute_copy(&pdscaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DuplicateSoundBuffer<Impl: IDirectSoundImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsbufferoriginal: ::windows::core::RawPtr, ppdsbufferduplicate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).DuplicateSoundBuffer(&*(&pdsbufferoriginal as *const <IDirectSoundBuffer as ::windows::core::Abi>::Abi as *const <IDirectSoundBuffer as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdsbufferduplicate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCooperativeLevel<Impl: IDirectSoundImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, hwnd: super::super::super::Foundation::HWND, dwlevel: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCooperativeLevel(&*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwlevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Compact<Impl: IDirectSoundImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Compact() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSpeakerConfig<Impl: IDirectSoundImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwspeakerconfig: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetSpeakerConfig(::core::mem::transmute_copy(&pdwspeakerconfig)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSpeakerConfig<Impl: IDirectSoundImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwspeakerconfig: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSpeakerConfig(dwspeakerconfig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pcguiddevice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSound>, base.5, CreateSoundBuffer::<Impl, OFFSET>, GetCaps::<Impl, OFFSET>, DuplicateSoundBuffer::<Impl, OFFSET>, SetCooperativeLevel::<Impl, OFFSET>, Compact::<Impl, OFFSET>, GetSpeakerConfig::<Impl, OFFSET>, SetSpeakerConfig::<Impl, OFFSET>, Initialize::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDirectSound3DBuffer {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSound3DBuffer";
}
impl IDirectSound3DBufferVtbl {
    pub const fn new<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSound3DBufferVtbl {
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pds3dbuffer: *mut DS3DBUFFER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pds3dbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConeAngles<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwinsideconeangle: *mut u32, pdwoutsideconeangle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConeAngles(::core::mem::transmute_copy(&pdwinsideconeangle), ::core::mem::transmute_copy(&pdwoutsideconeangle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConeOrientation<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvorientation: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConeOrientation(::core::mem::transmute_copy(&pvorientation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConeOutsideVolume<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plconeoutsidevolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetConeOutsideVolume(::core::mem::transmute_copy(&plconeoutsidevolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxDistance<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflmaxdistance: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMaxDistance(::core::mem::transmute_copy(&pflmaxdistance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinDistance<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflmindistance: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMinDistance(::core::mem::transmute_copy(&pflmindistance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMode<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwmode: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetMode(::core::mem::transmute_copy(&pdwmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPosition(::core::mem::transmute_copy(&pvposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocity<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVelocity(::core::mem::transmute_copy(&pvvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcds3dbuffer: *const DS3DBUFFER, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcds3dbuffer as *const <DS3DBUFFER as ::windows::core::Abi>::Abi as *const <DS3DBUFFER as ::windows::core::DefaultType>::DefaultType), dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConeAngles<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwinsideconeangle: u32, dwoutsideconeangle: u32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConeAngles(dwinsideconeangle, dwoutsideconeangle, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConeOrientation<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConeOrientation(x, y, z, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetConeOutsideVolume<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lconeoutsidevolume: i32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetConeOutsideVolume(lconeoutsidevolume, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxDistance<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flmaxdistance: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMaxDistance(flmaxdistance, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinDistance<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flmindistance: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMinDistance(flmindistance, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMode<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwmode: u32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMode(dwmode, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPosition(x, y, z, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVelocity<Impl: IDirectSound3DBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVelocity(x, y, z, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectSound3DBuffer>,
            base.5,
            GetAllParameters::<Impl, OFFSET>,
            GetConeAngles::<Impl, OFFSET>,
            GetConeOrientation::<Impl, OFFSET>,
            GetConeOutsideVolume::<Impl, OFFSET>,
            GetMaxDistance::<Impl, OFFSET>,
            GetMinDistance::<Impl, OFFSET>,
            GetMode::<Impl, OFFSET>,
            GetPosition::<Impl, OFFSET>,
            GetVelocity::<Impl, OFFSET>,
            SetAllParameters::<Impl, OFFSET>,
            SetConeAngles::<Impl, OFFSET>,
            SetConeOrientation::<Impl, OFFSET>,
            SetConeOutsideVolume::<Impl, OFFSET>,
            SetMaxDistance::<Impl, OFFSET>,
            SetMinDistance::<Impl, OFFSET>,
            SetMode::<Impl, OFFSET>,
            SetPosition::<Impl, OFFSET>,
            SetVelocity::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDirectSound3DListener {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSound3DListener";
}
impl IDirectSound3DListenerVtbl {
    pub const fn new<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSound3DListenerVtbl {
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plistener: *mut DS3DLISTENER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&plistener)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDistanceFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfldistancefactor: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDistanceFactor(::core::mem::transmute_copy(&pfldistancefactor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDopplerFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pfldopplerfactor: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDopplerFactor(::core::mem::transmute_copy(&pfldopplerfactor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOrientation<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvorientfront: *mut super::super::super::Graphics::Direct3D::D3DVECTOR, pvorienttop: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOrientation(::core::mem::transmute_copy(&pvorientfront), ::core::mem::transmute_copy(&pvorienttop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPosition<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvposition: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPosition(::core::mem::transmute_copy(&pvposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRolloffFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pflrollofffactor: *mut f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetRolloffFactor(::core::mem::transmute_copy(&pflrollofffactor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVelocity<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvvelocity: *mut super::super::super::Graphics::Direct3D::D3DVECTOR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVelocity(::core::mem::transmute_copy(&pvvelocity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pclistener: *const DS3DLISTENER, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pclistener as *const <DS3DLISTENER as ::windows::core::Abi>::Abi as *const <DS3DLISTENER as ::windows::core::DefaultType>::DefaultType), dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDistanceFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fldistancefactor: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDistanceFactor(fldistancefactor, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDopplerFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fldopplerfactor: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDopplerFactor(fldopplerfactor, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOrientation<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xfront: f32, yfront: f32, zfront: f32, xtop: f32, ytop: f32, ztop: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetOrientation(xfront, yfront, zfront, xtop, ytop, ztop, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPosition<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPosition(x, y, z, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRolloffFactor<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flrollofffactor: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRolloffFactor(flrollofffactor, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVelocity<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, x: f32, y: f32, z: f32, dwapply: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVelocity(x, y, z, dwapply) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitDeferredSettings<Impl: IDirectSound3DListenerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommitDeferredSettings() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectSound3DListener>,
            base.5,
            GetAllParameters::<Impl, OFFSET>,
            GetDistanceFactor::<Impl, OFFSET>,
            GetDopplerFactor::<Impl, OFFSET>,
            GetOrientation::<Impl, OFFSET>,
            GetPosition::<Impl, OFFSET>,
            GetRolloffFactor::<Impl, OFFSET>,
            GetVelocity::<Impl, OFFSET>,
            SetAllParameters::<Impl, OFFSET>,
            SetDistanceFactor::<Impl, OFFSET>,
            SetDopplerFactor::<Impl, OFFSET>,
            SetOrientation::<Impl, OFFSET>,
            SetPosition::<Impl, OFFSET>,
            SetRolloffFactor::<Impl, OFFSET>,
            SetVelocity::<Impl, OFFSET>,
            CommitDeferredSettings::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectSound8Impl: Sized + IDirectSoundImpl {
    fn VerifyCertification();
}
impl ::windows::core::RuntimeName for IDirectSound8 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSound8";
}
impl IDirectSound8Vtbl {
    pub const fn new<Impl: IDirectSound8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSound8Vtbl {
        unsafe extern "system" fn VerifyCertification<Impl: IDirectSound8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcertified: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).VerifyCertification(::core::mem::transmute_copy(&pdwcertified)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSound8>, base.5, VerifyCertification::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDirectSoundBuffer {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundBuffer";
}
impl IDirectSoundBufferVtbl {
    pub const fn new<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundBufferVtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsbuffercaps: *mut DSBCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(::core::mem::transmute_copy(&pdsbuffercaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcurrentplaycursor: *mut u32, pdwcurrentwritecursor: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentPosition(::core::mem::transmute_copy(&pdwcurrentplaycursor), ::core::mem::transmute_copy(&pdwcurrentwritecursor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&pwfxformat), dwsizeallocated, ::core::mem::transmute_copy(&pdwsizewritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVolume<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plvolume: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetVolume(::core::mem::transmute_copy(&plvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPan<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plpan: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPan(::core::mem::transmute_copy(&plpan)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrequency<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwfrequency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFrequency(::core::mem::transmute_copy(&pdwfrequency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectsound: ::windows::core::RawPtr, pcdsbufferdesc: *const DSBUFFERDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pdirectsound as *const <IDirectSound as ::windows::core::Abi>::Abi as *const <IDirectSound as ::windows::core::DefaultType>::DefaultType), &*(&pcdsbufferdesc as *const <DSBUFFERDESC as ::windows::core::Abi>::Abi as *const <DSBUFFERDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lock<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(dwoffset, dwbytes, ::core::mem::transmute_copy(&ppvaudioptr1), ::core::mem::transmute_copy(&pdwaudiobytes1), ::core::mem::transmute_copy(&ppvaudioptr2), ::core::mem::transmute_copy(&pdwaudiobytes2), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Play<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwreserved1: u32, dwpriority: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Play(dwreserved1, dwpriority, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPosition<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwnewposition: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCurrentPosition(dwnewposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFormat<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcfxformat: *const super::WAVEFORMATEX) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFormat(&*(&pcfxformat as *const <super::WAVEFORMATEX as ::windows::core::Abi>::Abi as *const <super::WAVEFORMATEX as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolume<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lvolume: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetVolume(lvolume) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPan<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lpan: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPan(lpan) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFrequency<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwfrequency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFrequency(dwfrequency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock(&*(&pvaudioptr1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwaudiobytes1, &*(&pvaudioptr2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwaudiobytes2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Restore<Impl: IDirectSoundBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Restore() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IDirectSoundBuffer>,
            base.5,
            GetCaps::<Impl, OFFSET>,
            GetCurrentPosition::<Impl, OFFSET>,
            GetFormat::<Impl, OFFSET>,
            GetVolume::<Impl, OFFSET>,
            GetPan::<Impl, OFFSET>,
            GetFrequency::<Impl, OFFSET>,
            GetStatus::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            Lock::<Impl, OFFSET>,
            Play::<Impl, OFFSET>,
            SetCurrentPosition::<Impl, OFFSET>,
            SetFormat::<Impl, OFFSET>,
            SetVolume::<Impl, OFFSET>,
            SetPan::<Impl, OFFSET>,
            SetFrequency::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Unlock::<Impl, OFFSET>,
            Restore::<Impl, OFFSET>,
        )
    }
}
pub trait IDirectSoundBuffer8Impl: Sized + IDirectSoundBufferImpl {
    fn SetFX();
    fn AcquireResources();
    fn GetObjectInPath();
}
impl ::windows::core::RuntimeName for IDirectSoundBuffer8 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundBuffer8";
}
impl IDirectSoundBuffer8Vtbl {
    pub const fn new<Impl: IDirectSoundBuffer8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundBuffer8Vtbl {
        unsafe extern "system" fn SetFX<Impl: IDirectSoundBuffer8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdsfxdesc: *const DSEFFECTDESC, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFX(dweffectscount, &*(&pdsfxdesc as *const <DSEFFECTDESC as ::windows::core::Abi>::Abi as *const <DSEFFECTDESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwresultcodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireResources<Impl: IDirectSoundBuffer8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32, dweffectscount: u32, pdwresultcodes: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AcquireResources(dwflags, dweffectscount, ::core::mem::transmute_copy(&pdwresultcodes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObjectInPath<Impl: IDirectSoundBuffer8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectInPath(&*(&rguidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwindex, &*(&rguidinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundBuffer8>, base.5, SetFX::<Impl, OFFSET>, AcquireResources::<Impl, OFFSET>, GetObjectInPath::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundCaptureImpl: Sized {
    fn CreateCaptureBuffer();
    fn GetCaps();
    fn Initialize();
}
impl ::windows::core::RuntimeName for IDirectSoundCapture {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundCapture";
}
impl IDirectSoundCaptureVtbl {
    pub const fn new<Impl: IDirectSoundCaptureImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundCaptureVtbl {
        unsafe extern "system" fn CreateCaptureBuffer<Impl: IDirectSoundCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdscbufferdesc: *const DSCBUFFERDESC, ppdscbuffer: *mut ::windows::core::RawPtr, punkouter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateCaptureBuffer(&*(&pcdscbufferdesc as *const <DSCBUFFERDESC as ::windows::core::Abi>::Abi as *const <DSCBUFFERDESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdscbuffer), &*(&punkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsccaps: *mut DSCCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(::core::mem::transmute_copy(&pdsccaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundCaptureImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcguiddevice: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pcguiddevice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundCapture>, base.5, CreateCaptureBuffer::<Impl, OFFSET>, GetCaps::<Impl, OFFSET>, Initialize::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDirectSoundCaptureBuffer {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundCaptureBuffer";
}
impl IDirectSoundCaptureBufferVtbl {
    pub const fn new<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundCaptureBufferVtbl {
        unsafe extern "system" fn GetCaps<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscbcaps: *mut DSCBCAPS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCaps(::core::mem::transmute_copy(&pdscbcaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentPosition<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwcaptureposition: *mut u32, pdwreadposition: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCurrentPosition(::core::mem::transmute_copy(&pdwcaptureposition), ::core::mem::transmute_copy(&pdwreadposition)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFormat<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwfxformat: *mut super::WAVEFORMATEX, dwsizeallocated: u32, pdwsizewritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFormat(::core::mem::transmute_copy(&pwfxformat), dwsizeallocated, ::core::mem::transmute_copy(&pdwsizewritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdirectsoundcapture: ::windows::core::RawPtr, pcdscbufferdesc: *const DSCBUFFERDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pdirectsoundcapture as *const <IDirectSoundCapture as ::windows::core::Abi>::Abi as *const <IDirectSoundCapture as ::windows::core::DefaultType>::DefaultType), &*(&pcdscbufferdesc as *const <DSCBUFFERDESC as ::windows::core::Abi>::Abi as *const <DSCBUFFERDESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Lock<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwoffset: u32, dwbytes: u32, ppvaudioptr1: *mut *mut ::core::ffi::c_void, pdwaudiobytes1: *mut u32, ppvaudioptr2: *mut *mut ::core::ffi::c_void, pdwaudiobytes2: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Lock(dwoffset, dwbytes, ::core::mem::transmute_copy(&ppvaudioptr1), ::core::mem::transmute_copy(&pdwaudiobytes1), ::core::mem::transmute_copy(&ppvaudioptr2), ::core::mem::transmute_copy(&pdwaudiobytes2), dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Start(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unlock<Impl: IDirectSoundCaptureBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvaudioptr1: *const ::core::ffi::c_void, dwaudiobytes1: u32, pvaudioptr2: *const ::core::ffi::c_void, dwaudiobytes2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Unlock(&*(&pvaudioptr1 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwaudiobytes1, &*(&pvaudioptr2 as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), dwaudiobytes2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundCaptureBuffer>, base.5, GetCaps::<Impl, OFFSET>, GetCurrentPosition::<Impl, OFFSET>, GetFormat::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>, Initialize::<Impl, OFFSET>, Lock::<Impl, OFFSET>, Start::<Impl, OFFSET>, Stop::<Impl, OFFSET>, Unlock::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundCaptureBuffer8Impl: Sized + IDirectSoundCaptureBufferImpl {
    fn GetObjectInPath();
    fn GetFXStatus();
}
impl ::windows::core::RuntimeName for IDirectSoundCaptureBuffer8 {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundCaptureBuffer8";
}
impl IDirectSoundCaptureBuffer8Vtbl {
    pub const fn new<Impl: IDirectSoundCaptureBuffer8Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundCaptureBuffer8Vtbl {
        unsafe extern "system" fn GetObjectInPath<Impl: IDirectSoundCaptureBuffer8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rguidobject: *const ::windows::core::GUID, dwindex: u32, rguidinterface: *const ::windows::core::GUID, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetObjectInPath(&*(&rguidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwindex, &*(&rguidinterface as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFXStatus<Impl: IDirectSoundCaptureBuffer8Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweffectscount: u32, pdwfxstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetFXStatus(dweffectscount, ::core::mem::transmute_copy(&pdwfxstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundCaptureBuffer8>, base.5, GetObjectInPath::<Impl, OFFSET>, GetFXStatus::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundCaptureFXAecImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
    fn GetStatus();
    fn Reset();
}
impl ::windows::core::RuntimeName for IDirectSoundCaptureFXAec {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundCaptureFXAec";
}
impl IDirectSoundCaptureFXAecVtbl {
    pub const fn new<Impl: IDirectSoundCaptureFXAecImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundCaptureFXAecVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundCaptureFXAecImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscfxaec: *const DSCFXAec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pdscfxaec as *const <DSCFXAec as ::windows::core::Abi>::Abi as *const <DSCFXAec as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundCaptureFXAecImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscfxaec: *mut DSCFXAec) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdscfxaec)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatus<Impl: IDirectSoundCaptureFXAecImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetStatus(::core::mem::transmute_copy(&pdwstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IDirectSoundCaptureFXAecImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundCaptureFXAec>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>, GetStatus::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundCaptureFXNoiseSuppressImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
    fn Reset();
}
impl ::windows::core::RuntimeName for IDirectSoundCaptureFXNoiseSuppress {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundCaptureFXNoiseSuppress";
}
impl IDirectSoundCaptureFXNoiseSuppressVtbl {
    pub const fn new<Impl: IDirectSoundCaptureFXNoiseSuppressImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundCaptureFXNoiseSuppressVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundCaptureFXNoiseSuppressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdscfxnoisesuppress: *const DSCFXNoiseSuppress) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdscfxnoisesuppress as *const <DSCFXNoiseSuppress as ::windows::core::Abi>::Abi as *const <DSCFXNoiseSuppress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundCaptureFXNoiseSuppressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdscfxnoisesuppress: *mut DSCFXNoiseSuppress) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdscfxnoisesuppress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IDirectSoundCaptureFXNoiseSuppressImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundCaptureFXNoiseSuppress>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundFXChorusImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl ::windows::core::RuntimeName for IDirectSoundFXChorus {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFXChorus";
}
impl IDirectSoundFXChorusVtbl {
    pub const fn new<Impl: IDirectSoundFXChorusImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFXChorusVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXChorusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxchorus: *const DSFXChorus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdsfxchorus as *const <DSFXChorus as ::windows::core::Abi>::Abi as *const <DSFXChorus as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXChorusImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxchorus: *mut DSFXChorus) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdsfxchorus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFXChorus>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundFXCompressorImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl ::windows::core::RuntimeName for IDirectSoundFXCompressor {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFXCompressor";
}
impl IDirectSoundFXCompressorVtbl {
    pub const fn new<Impl: IDirectSoundFXCompressorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFXCompressorVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXCompressorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxcompressor: *const DSFXCompressor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdsfxcompressor as *const <DSFXCompressor as ::windows::core::Abi>::Abi as *const <DSFXCompressor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXCompressorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxcompressor: *mut DSFXCompressor) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdsfxcompressor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFXCompressor>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundFXDistortionImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl ::windows::core::RuntimeName for IDirectSoundFXDistortion {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFXDistortion";
}
impl IDirectSoundFXDistortionVtbl {
    pub const fn new<Impl: IDirectSoundFXDistortionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFXDistortionVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXDistortionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxdistortion: *const DSFXDistortion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdsfxdistortion as *const <DSFXDistortion as ::windows::core::Abi>::Abi as *const <DSFXDistortion as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXDistortionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxdistortion: *mut DSFXDistortion) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdsfxdistortion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFXDistortion>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundFXEchoImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl ::windows::core::RuntimeName for IDirectSoundFXEcho {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFXEcho";
}
impl IDirectSoundFXEchoVtbl {
    pub const fn new<Impl: IDirectSoundFXEchoImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFXEchoVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXEchoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxecho: *const DSFXEcho) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdsfxecho as *const <DSFXEcho as ::windows::core::Abi>::Abi as *const <DSFXEcho as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXEchoImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxecho: *mut DSFXEcho) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdsfxecho)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFXEcho>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundFXFlangerImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl ::windows::core::RuntimeName for IDirectSoundFXFlanger {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFXFlanger";
}
impl IDirectSoundFXFlangerVtbl {
    pub const fn new<Impl: IDirectSoundFXFlangerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFXFlangerVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXFlangerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxflanger: *const DSFXFlanger) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdsfxflanger as *const <DSFXFlanger as ::windows::core::Abi>::Abi as *const <DSFXFlanger as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXFlangerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxflanger: *mut DSFXFlanger) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdsfxflanger)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFXFlanger>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundFXGargleImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl ::windows::core::RuntimeName for IDirectSoundFXGargle {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFXGargle";
}
impl IDirectSoundFXGargleVtbl {
    pub const fn new<Impl: IDirectSoundFXGargleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFXGargleVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXGargleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxgargle: *const DSFXGargle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdsfxgargle as *const <DSFXGargle as ::windows::core::Abi>::Abi as *const <DSFXGargle as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXGargleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxgargle: *mut DSFXGargle) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdsfxgargle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFXGargle>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDirectSoundFXI3DL2Reverb {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFXI3DL2Reverb";
}
impl IDirectSoundFXI3DL2ReverbVtbl {
    pub const fn new<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFXI3DL2ReverbVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxi3dl2reverb: *const DSFXI3DL2Reverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdsfxi3dl2reverb as *const <DSFXI3DL2Reverb as ::windows::core::Abi>::Abi as *const <DSFXI3DL2Reverb as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxi3dl2reverb: *mut DSFXI3DL2Reverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdsfxi3dl2reverb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreset<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpreset: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPreset(dwpreset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPreset<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdwpreset: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPreset(::core::mem::transmute_copy(&pdwpreset)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQuality<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lquality: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetQuality(lquality) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuality<Impl: IDirectSoundFXI3DL2ReverbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plquality: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetQuality(::core::mem::transmute_copy(&plquality)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFXI3DL2Reverb>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>, SetPreset::<Impl, OFFSET>, GetPreset::<Impl, OFFSET>, SetQuality::<Impl, OFFSET>, GetQuality::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundFXParamEqImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl ::windows::core::RuntimeName for IDirectSoundFXParamEq {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFXParamEq";
}
impl IDirectSoundFXParamEqVtbl {
    pub const fn new<Impl: IDirectSoundFXParamEqImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFXParamEqVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXParamEqImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxparameq: *const DSFXParamEq) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdsfxparameq as *const <DSFXParamEq as ::windows::core::Abi>::Abi as *const <DSFXParamEq as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXParamEqImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxparameq: *mut DSFXParamEq) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdsfxparameq)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFXParamEq>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundFXWavesReverbImpl: Sized {
    fn SetAllParameters();
    fn GetAllParameters();
}
impl ::windows::core::RuntimeName for IDirectSoundFXWavesReverb {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFXWavesReverb";
}
impl IDirectSoundFXWavesReverbVtbl {
    pub const fn new<Impl: IDirectSoundFXWavesReverbImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFXWavesReverbVtbl {
        unsafe extern "system" fn SetAllParameters<Impl: IDirectSoundFXWavesReverbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcdsfxwavesreverb: *const DSFXWavesReverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAllParameters(&*(&pcdsfxwavesreverb as *const <DSFXWavesReverb as ::windows::core::Abi>::Abi as *const <DSFXWavesReverb as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParameters<Impl: IDirectSoundFXWavesReverbImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdsfxwavesreverb: *mut DSFXWavesReverb) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllParameters(::core::mem::transmute_copy(&pdsfxwavesreverb)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFXWavesReverb>, base.5, SetAllParameters::<Impl, OFFSET>, GetAllParameters::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundFullDuplexImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IDirectSoundFullDuplex {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundFullDuplex";
}
impl IDirectSoundFullDuplexVtbl {
    pub const fn new<Impl: IDirectSoundFullDuplexImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundFullDuplexVtbl {
        unsafe extern "system" fn Initialize<Impl: IDirectSoundFullDuplexImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcaptureguid: *const ::windows::core::GUID, prenderguid: *const ::windows::core::GUID, lpdscbufferdesc: *const DSCBUFFERDESC, lpdsbufferdesc: *const DSBUFFERDESC, hwnd: super::super::super::Foundation::HWND, dwlevel: u32, lplpdirectsoundcapturebuffer8: *mut ::windows::core::RawPtr, lplpdirectsoundbuffer8: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&pcaptureguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&prenderguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&lpdscbufferdesc as *const <DSCBUFFERDESC as ::windows::core::Abi>::Abi as *const <DSCBUFFERDESC as ::windows::core::DefaultType>::DefaultType),
                &*(&lpdsbufferdesc as *const <DSBUFFERDESC as ::windows::core::Abi>::Abi as *const <DSBUFFERDESC as ::windows::core::DefaultType>::DefaultType),
                &*(&hwnd as *const <super::super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwlevel,
                ::core::mem::transmute_copy(&lplpdirectsoundcapturebuffer8),
                ::core::mem::transmute_copy(&lplpdirectsoundbuffer8),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundFullDuplex>, base.5, Initialize::<Impl, OFFSET>)
    }
}
pub trait IDirectSoundNotifyImpl: Sized {
    fn SetNotificationPositions();
}
impl ::windows::core::RuntimeName for IDirectSoundNotify {
    const NAME: &'static str = "Windows.Win32.Media.Audio.DirectSound.IDirectSoundNotify";
}
impl IDirectSoundNotifyVtbl {
    pub const fn new<Impl: IDirectSoundNotifyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDirectSoundNotifyVtbl {
        unsafe extern "system" fn SetNotificationPositions<Impl: IDirectSoundNotifyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dwpositionnotifies: u32, pcpositionnotifies: *const DSBPOSITIONNOTIFY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetNotificationPositions(dwpositionnotifies, &*(&pcpositionnotifies as *const <DSBPOSITIONNOTIFY as ::windows::core::Abi>::Abi as *const <DSBPOSITIONNOTIFY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDirectSoundNotify>, base.5, SetNotificationPositions::<Impl, OFFSET>)
    }
}
