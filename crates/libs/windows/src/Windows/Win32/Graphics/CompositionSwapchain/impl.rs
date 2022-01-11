#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ICompositionFramePresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetContentTag();
    fn GetCompositionFrameId();
    fn GetDisplayInstanceArray();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ICompositionFramePresentStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICompositionFramePresentStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICompositionFramePresentStatisticsVtbl {
        unsafe extern "system" fn GetContentTag<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCompositionFrameId<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayInstanceArray<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPresentId::<Impl, IMPL_OFFSET>, GetKind::<Impl, IMPL_OFFSET>, GetContentTag::<Impl, IMPL_OFFSET>, GetCompositionFrameId::<Impl, IMPL_OFFSET>, GetDisplayInstanceArray::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionFramePresentStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IIndependentFlipFramePresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetOutputAdapterLUID();
    fn GetOutputVidPnSourceId();
    fn GetContentTag();
    fn GetDisplayedTime();
    fn GetPresentDuration();
}
#[cfg(feature = "Win32_Foundation")]
impl IIndependentFlipFramePresentStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIndependentFlipFramePresentStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIndependentFlipFramePresentStatisticsVtbl {
        unsafe extern "system" fn GetOutputAdapterLUID<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOutputVidPnSourceId<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContentTag<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayedTime<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentDuration<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPresentId::<Impl, IMPL_OFFSET>, GetKind::<Impl, IMPL_OFFSET>, GetOutputAdapterLUID::<Impl, IMPL_OFFSET>, GetOutputVidPnSourceId::<Impl, IMPL_OFFSET>, GetContentTag::<Impl, IMPL_OFFSET>, GetDisplayedTime::<Impl, IMPL_OFFSET>, GetPresentDuration::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndependentFlipFramePresentStatistics as ::windows::core::Interface>::IID
    }
}
pub trait IPresentStatisticsImpl: Sized {
    fn GetPresentId();
    fn GetKind();
}
impl IPresentStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentStatisticsVtbl {
        unsafe extern "system" fn GetPresentId<Impl: IPresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetKind<Impl: IPresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPresentId::<Impl, IMPL_OFFSET>, GetKind::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentStatistics as ::windows::core::Interface>::IID
    }
}
pub trait IPresentStatusPresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetCompositionFrameId();
    fn GetPresentStatus();
}
impl IPresentStatusPresentStatisticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentStatusPresentStatisticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentStatusPresentStatisticsVtbl {
        unsafe extern "system" fn GetCompositionFrameId<Impl: IPresentStatusPresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentStatus<Impl: IPresentStatusPresentStatisticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> PresentStatus {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPresentId::<Impl, IMPL_OFFSET>, GetKind::<Impl, IMPL_OFFSET>, GetCompositionFrameId::<Impl, IMPL_OFFSET>, GetPresentStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentStatusPresentStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPresentationBufferImpl: Sized {
    fn GetAvailableEvent();
    fn IsAvailable();
}
#[cfg(feature = "Win32_Foundation")]
impl IPresentationBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationBufferVtbl {
        unsafe extern "system" fn GetAvailableEvent<Impl: IPresentationBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAvailable<Impl: IPresentationBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isavailable: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetAvailableEvent::<Impl, IMPL_OFFSET>, IsAvailable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IPresentationContentImpl: Sized {
    fn SetTag();
}
impl IPresentationContentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationContentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationContentVtbl {
        unsafe extern "system" fn SetTag<Impl: IPresentationContentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag: usize) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetTag::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationContent as ::windows::core::Interface>::IID
    }
}
pub trait IPresentationFactoryImpl: Sized {
    fn IsPresentationSupported();
    fn IsPresentationSupportedWithIndependentFlip();
    fn CreatePresentationManager();
}
impl IPresentationFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationFactoryVtbl {
        unsafe extern "system" fn IsPresentationSupported<Impl: IPresentationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPresentationSupportedWithIndependentFlip<Impl: IPresentationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePresentationManager<Impl: IPresentationFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppresentationmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsPresentationSupported::<Impl, IMPL_OFFSET>, IsPresentationSupportedWithIndependentFlip::<Impl, IMPL_OFFSET>, CreatePresentationManager::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPresentationManagerImpl: Sized {
    fn AddBufferFromResource();
    fn CreatePresentationSurface();
    fn GetNextPresentId();
    fn SetTargetTime();
    fn SetPreferredPresentDuration();
    fn ForceVSyncInterrupt();
    fn Present();
    fn GetPresentRetiringFence();
    fn CancelPresentsFrom();
    fn GetLostEvent();
    fn GetPresentStatisticsAvailableEvent();
    fn EnablePresentStatisticsKind();
    fn GetNextPresentStatistics();
}
#[cfg(feature = "Win32_Foundation")]
impl IPresentationManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationManagerVtbl {
        unsafe extern "system" fn AddBufferFromResource<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void, presentationbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePresentationSurface<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextPresentId<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetTime<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targettime: SystemInterruptTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPreferredPresentDuration<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForceVSyncInterrupt<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forcevsyncinterrupt: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Present<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentRetiringFence<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelPresentsFrom<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentidtocancelfrom: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLostEvent<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPresentStatisticsAvailableEvent<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnablePresentStatisticsKind<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNextPresentStatistics<Impl: IPresentationManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextpresentstatistics: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            AddBufferFromResource::<Impl, IMPL_OFFSET>,
            CreatePresentationSurface::<Impl, IMPL_OFFSET>,
            GetNextPresentId::<Impl, IMPL_OFFSET>,
            SetTargetTime::<Impl, IMPL_OFFSET>,
            SetPreferredPresentDuration::<Impl, IMPL_OFFSET>,
            ForceVSyncInterrupt::<Impl, IMPL_OFFSET>,
            Present::<Impl, IMPL_OFFSET>,
            GetPresentRetiringFence::<Impl, IMPL_OFFSET>,
            CancelPresentsFrom::<Impl, IMPL_OFFSET>,
            GetLostEvent::<Impl, IMPL_OFFSET>,
            GetPresentStatisticsAvailableEvent::<Impl, IMPL_OFFSET>,
            EnablePresentStatisticsKind::<Impl, IMPL_OFFSET>,
            GetNextPresentStatistics::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IPresentationSurfaceImpl: Sized + IPresentationContentImpl {
    fn SetBuffer();
    fn SetColorSpace();
    fn SetAlphaMode();
    fn SetSourceRect();
    fn SetTransform();
    fn RestrictToOutput();
    fn SetDisableReadback();
    fn SetLetterboxingMargins();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IPresentationSurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPresentationSurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPresentationSurfaceVtbl {
        unsafe extern "system" fn SetBuffer<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentationbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorSpace<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAlphaMode<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceRect<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransform<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const PresentationTransform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestrictToOutput<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisableReadback<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLetterboxingMargins<Impl: IPresentationSurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetTag::<Impl, IMPL_OFFSET>,
            SetBuffer::<Impl, IMPL_OFFSET>,
            SetColorSpace::<Impl, IMPL_OFFSET>,
            SetAlphaMode::<Impl, IMPL_OFFSET>,
            SetSourceRect::<Impl, IMPL_OFFSET>,
            SetTransform::<Impl, IMPL_OFFSET>,
            RestrictToOutput::<Impl, IMPL_OFFSET>,
            SetDisableReadback::<Impl, IMPL_OFFSET>,
            SetLetterboxingMargins::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationSurface as ::windows::core::Interface>::IID
    }
}
