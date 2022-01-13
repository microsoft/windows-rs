#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ICompositionFramePresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetContentTag(&mut self) -> usize;
    fn GetCompositionFrameId(&mut self) -> u64;
    fn GetDisplayInstanceArray(&mut self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ICompositionFramePresentStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionFramePresentStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionFramePresentStatisticsVtbl {
        unsafe extern "system" fn GetContentTag<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContentTag()
        }
        unsafe extern "system" fn GetCompositionFrameId<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCompositionFrameId()
        }
        unsafe extern "system" fn GetDisplayInstanceArray<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayInstanceArray(::core::mem::transmute_copy(&displayinstancearraycount), ::core::mem::transmute_copy(&displayinstancearray))
        }
        Self {
            base: IPresentStatisticsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetContentTag: GetContentTag::<Impl, IMPL_OFFSET>,
            GetCompositionFrameId: GetCompositionFrameId::<Impl, IMPL_OFFSET>,
            GetDisplayInstanceArray: GetDisplayInstanceArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionFramePresentStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IIndependentFlipFramePresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetOutputAdapterLUID(&mut self) -> super::super::Foundation::LUID;
    fn GetOutputVidPnSourceId(&mut self) -> u32;
    fn GetContentTag(&mut self) -> usize;
    fn GetDisplayedTime(&mut self) -> SystemInterruptTime;
    fn GetPresentDuration(&mut self) -> SystemInterruptTime;
}
#[cfg(feature = "Win32_Foundation")]
impl IIndependentFlipFramePresentStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndependentFlipFramePresentStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIndependentFlipFramePresentStatisticsVtbl {
        unsafe extern "system" fn GetOutputAdapterLUID<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputAdapterLUID()
        }
        unsafe extern "system" fn GetOutputVidPnSourceId<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOutputVidPnSourceId()
        }
        unsafe extern "system" fn GetContentTag<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContentTag()
        }
        unsafe extern "system" fn GetDisplayedTime<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayedTime()
        }
        unsafe extern "system" fn GetPresentDuration<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresentDuration()
        }
        Self {
            base: IPresentStatisticsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetOutputAdapterLUID: GetOutputAdapterLUID::<Impl, IMPL_OFFSET>,
            GetOutputVidPnSourceId: GetOutputVidPnSourceId::<Impl, IMPL_OFFSET>,
            GetContentTag: GetContentTag::<Impl, IMPL_OFFSET>,
            GetDisplayedTime: GetDisplayedTime::<Impl, IMPL_OFFSET>,
            GetPresentDuration: GetPresentDuration::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndependentFlipFramePresentStatistics as ::windows::core::Interface>::IID
    }
}
pub trait IPresentStatisticsImpl: Sized {
    fn GetPresentId(&mut self) -> u64;
    fn GetKind(&mut self) -> PresentStatisticsKind;
}
impl IPresentStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentStatisticsVtbl {
        unsafe extern "system" fn GetPresentId<Impl: IPresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresentId()
        }
        unsafe extern "system" fn GetKind<Impl: IPresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKind()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPresentId: GetPresentId::<Impl, IMPL_OFFSET>,
            GetKind: GetKind::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentStatistics as ::windows::core::Interface>::IID
    }
}
pub trait IPresentStatusPresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetCompositionFrameId(&mut self) -> u64;
    fn GetPresentStatus(&mut self) -> PresentStatus;
}
impl IPresentStatusPresentStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentStatusPresentStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentStatusPresentStatisticsVtbl {
        unsafe extern "system" fn GetCompositionFrameId<Impl: IPresentStatusPresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCompositionFrameId()
        }
        unsafe extern "system" fn GetPresentStatus<Impl: IPresentStatusPresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> PresentStatus {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresentStatus()
        }
        Self {
            base: IPresentStatisticsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCompositionFrameId: GetCompositionFrameId::<Impl, IMPL_OFFSET>,
            GetPresentStatus: GetPresentStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentStatusPresentStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPresentationBufferImpl: Sized {
    fn GetAvailableEvent(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn IsAvailable(&mut self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPresentationBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationBufferVtbl {
        unsafe extern "system" fn GetAvailableEvent<Impl: IPresentationBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *availableeventhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Impl: IPresentationBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isavailable: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    *isavailable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAvailableEvent: GetAvailableEvent::<Impl, IMPL_OFFSET>,
            IsAvailable: IsAvailable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IPresentationContentImpl: Sized {
    fn SetTag(&mut self, tag: usize);
}
impl IPresentationContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationContentVtbl {
        unsafe extern "system" fn SetTag<Impl: IPresentationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTag(::core::mem::transmute_copy(&tag))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetTag: SetTag::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationContent as ::windows::core::Interface>::IID
    }
}
pub trait IPresentationFactoryImpl: Sized {
    fn IsPresentationSupported(&mut self) -> u8;
    fn IsPresentationSupportedWithIndependentFlip(&mut self) -> u8;
    fn CreatePresentationManager(&mut self) -> ::windows::core::Result<IPresentationManager>;
}
impl IPresentationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationFactoryVtbl {
        unsafe extern "system" fn IsPresentationSupported<Impl: IPresentationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPresentationSupported()
        }
        unsafe extern "system" fn IsPresentationSupportedWithIndependentFlip<Impl: IPresentationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPresentationSupportedWithIndependentFlip()
        }
        unsafe extern "system" fn CreatePresentationManager<Impl: IPresentationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppresentationmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePresentationManager() {
                ::core::result::Result::Ok(ok__) => {
                    *pppresentationmanager = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsPresentationSupported: IsPresentationSupported::<Impl, IMPL_OFFSET>,
            IsPresentationSupportedWithIndependentFlip: IsPresentationSupportedWithIndependentFlip::<Impl, IMPL_OFFSET>,
            CreatePresentationManager: CreatePresentationManager::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPresentationManagerImpl: Sized {
    fn AddBufferFromResource(&mut self, resource: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IPresentationBuffer>;
    fn CreatePresentationSurface(&mut self, compositionsurfacehandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<IPresentationSurface>;
    fn GetNextPresentId(&mut self) -> u64;
    fn SetTargetTime(&mut self, targettime: SystemInterruptTime) -> ::windows::core::Result<()>;
    fn SetPreferredPresentDuration(&mut self, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows::core::Result<()>;
    fn ForceVSyncInterrupt(&mut self, forcevsyncinterrupt: u8) -> ::windows::core::Result<()>;
    fn Present(&mut self) -> ::windows::core::Result<()>;
    fn GetPresentRetiringFence(&mut self, riid: *const ::windows::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CancelPresentsFrom(&mut self, presentidtocancelfrom: u64) -> ::windows::core::Result<()>;
    fn GetLostEvent(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn GetPresentStatisticsAvailableEvent(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn EnablePresentStatisticsKind(&mut self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::Result<()>;
    fn GetNextPresentStatistics(&mut self) -> ::windows::core::Result<IPresentStatistics>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPresentationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationManagerVtbl {
        unsafe extern "system" fn AddBufferFromResource<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void, presentationbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddBufferFromResource(::core::mem::transmute(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    *presentationbuffer = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePresentationSurface<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePresentationSurface(::core::mem::transmute_copy(&compositionsurfacehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *presentationsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextPresentId<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNextPresentId()
        }
        unsafe extern "system" fn SetTargetTime<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targettime: SystemInterruptTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetTime(::core::mem::transmute_copy(&targettime)).into()
        }
        unsafe extern "system" fn SetPreferredPresentDuration<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPreferredPresentDuration(::core::mem::transmute_copy(&preferredduration), ::core::mem::transmute_copy(&deviationtolerance)).into()
        }
        unsafe extern "system" fn ForceVSyncInterrupt<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forcevsyncinterrupt: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForceVSyncInterrupt(::core::mem::transmute_copy(&forcevsyncinterrupt)).into()
        }
        unsafe extern "system" fn Present<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Present().into()
        }
        unsafe extern "system" fn GetPresentRetiringFence<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPresentRetiringFence(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&fence)).into()
        }
        unsafe extern "system" fn CancelPresentsFrom<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentidtocancelfrom: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelPresentsFrom(::core::mem::transmute_copy(&presentidtocancelfrom)).into()
        }
        unsafe extern "system" fn GetLostEvent<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLostEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *losteventhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentStatisticsAvailableEvent<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPresentStatisticsAvailableEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *presentstatisticsavailableeventhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnablePresentStatisticsKind<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnablePresentStatisticsKind(::core::mem::transmute_copy(&presentstatisticskind), ::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn GetNextPresentStatistics<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextpresentstatistics: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNextPresentStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *nextpresentstatistics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddBufferFromResource: AddBufferFromResource::<Impl, IMPL_OFFSET>,
            CreatePresentationSurface: CreatePresentationSurface::<Impl, IMPL_OFFSET>,
            GetNextPresentId: GetNextPresentId::<Impl, IMPL_OFFSET>,
            SetTargetTime: SetTargetTime::<Impl, IMPL_OFFSET>,
            SetPreferredPresentDuration: SetPreferredPresentDuration::<Impl, IMPL_OFFSET>,
            ForceVSyncInterrupt: ForceVSyncInterrupt::<Impl, IMPL_OFFSET>,
            Present: Present::<Impl, IMPL_OFFSET>,
            GetPresentRetiringFence: GetPresentRetiringFence::<Impl, IMPL_OFFSET>,
            CancelPresentsFrom: CancelPresentsFrom::<Impl, IMPL_OFFSET>,
            GetLostEvent: GetLostEvent::<Impl, IMPL_OFFSET>,
            GetPresentStatisticsAvailableEvent: GetPresentStatisticsAvailableEvent::<Impl, IMPL_OFFSET>,
            EnablePresentStatisticsKind: EnablePresentStatisticsKind::<Impl, IMPL_OFFSET>,
            GetNextPresentStatistics: GetNextPresentStatistics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IPresentationSurfaceImpl: Sized + IPresentationContentImpl {
    fn SetBuffer(&mut self, presentationbuffer: ::core::option::Option<IPresentationBuffer>) -> ::windows::core::Result<()>;
    fn SetColorSpace(&mut self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()>;
    fn SetAlphaMode(&mut self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<()>;
    fn SetSourceRect(&mut self, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetTransform(&mut self, transform: *const PresentationTransform) -> ::windows::core::Result<()>;
    fn RestrictToOutput(&mut self, output: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetDisableReadback(&mut self, value: u8) -> ::windows::core::Result<()>;
    fn SetLetterboxingMargins(&mut self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IPresentationSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationSurfaceVtbl {
        unsafe extern "system" fn SetBuffer<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentationbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBuffer(::core::mem::transmute(&presentationbuffer)).into()
        }
        unsafe extern "system" fn SetColorSpace<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorSpace(::core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn SetAlphaMode<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlphaMode(::core::mem::transmute_copy(&alphamode)).into()
        }
        unsafe extern "system" fn SetSourceRect<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceRect(::core::mem::transmute_copy(&sourcerect)).into()
        }
        unsafe extern "system" fn SetTransform<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const PresentationTransform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransform(::core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn RestrictToOutput<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestrictToOutput(::core::mem::transmute(&output)).into()
        }
        unsafe extern "system" fn SetDisableReadback<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisableReadback(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetLetterboxingMargins<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLetterboxingMargins(::core::mem::transmute_copy(&leftletterboxsize), ::core::mem::transmute_copy(&topletterboxsize), ::core::mem::transmute_copy(&rightletterboxsize), ::core::mem::transmute_copy(&bottomletterboxsize)).into()
        }
        Self {
            base: IPresentationContentVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetBuffer: SetBuffer::<Impl, IMPL_OFFSET>,
            SetColorSpace: SetColorSpace::<Impl, IMPL_OFFSET>,
            SetAlphaMode: SetAlphaMode::<Impl, IMPL_OFFSET>,
            SetSourceRect: SetSourceRect::<Impl, IMPL_OFFSET>,
            SetTransform: SetTransform::<Impl, IMPL_OFFSET>,
            RestrictToOutput: RestrictToOutput::<Impl, IMPL_OFFSET>,
            SetDisableReadback: SetDisableReadback::<Impl, IMPL_OFFSET>,
            SetLetterboxingMargins: SetLetterboxingMargins::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationSurface as ::windows::core::Interface>::IID
    }
}
