pub trait ICompositionFramePresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetContentTag();
    fn GetCompositionFrameId();
    fn GetDisplayInstanceArray();
}
impl ::windows::core::RuntimeName for ICompositionFramePresentStatistics {
    const NAME: &'static str = "Windows.Win32.Graphics.CompositionSwapchain.ICompositionFramePresentStatistics";
}
impl ICompositionFramePresentStatisticsVtbl {
    pub const fn new<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ICompositionFramePresentStatisticsVtbl {
        unsafe extern "system" fn GetContentTag<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentTag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCompositionFrameId<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCompositionFrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayInstanceArray<Impl: ICompositionFramePresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).GetDisplayInstanceArray(::core::mem::transmute_copy(&displayinstancearraycount), ::core::mem::transmute_copy(&displayinstancearray)).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ICompositionFramePresentStatistics>, base.5, GetContentTag::<Impl, OFFSET>, GetCompositionFrameId::<Impl, OFFSET>, GetDisplayInstanceArray::<Impl, OFFSET>)
    }
}
pub trait IIndependentFlipFramePresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetOutputAdapterLUID();
    fn GetOutputVidPnSourceId();
    fn GetContentTag();
    fn GetDisplayedTime();
    fn GetPresentDuration();
}
impl ::windows::core::RuntimeName for IIndependentFlipFramePresentStatistics {
    const NAME: &'static str = "Windows.Win32.Graphics.CompositionSwapchain.IIndependentFlipFramePresentStatistics";
}
impl IIndependentFlipFramePresentStatisticsVtbl {
    pub const fn new<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IIndependentFlipFramePresentStatisticsVtbl {
        unsafe extern "system" fn GetOutputAdapterLUID<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputAdapterLUID() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputVidPnSourceId<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetOutputVidPnSourceId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContentTag<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetContentTag() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayedTime<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetDisplayedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentDuration<Impl: IIndependentFlipFramePresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPresentDuration() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IIndependentFlipFramePresentStatistics>, base.5, GetOutputAdapterLUID::<Impl, OFFSET>, GetOutputVidPnSourceId::<Impl, OFFSET>, GetContentTag::<Impl, OFFSET>, GetDisplayedTime::<Impl, OFFSET>, GetPresentDuration::<Impl, OFFSET>)
    }
}
pub trait IPresentStatisticsImpl: Sized {
    fn GetPresentId();
    fn GetKind();
}
impl ::windows::core::RuntimeName for IPresentStatistics {
    const NAME: &'static str = "Windows.Win32.Graphics.CompositionSwapchain.IPresentStatistics";
}
impl IPresentStatisticsVtbl {
    pub const fn new<Impl: IPresentStatisticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPresentStatisticsVtbl {
        unsafe extern "system" fn GetPresentId<Impl: IPresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPresentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKind<Impl: IPresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPresentStatistics>, base.5, GetPresentId::<Impl, OFFSET>, GetKind::<Impl, OFFSET>)
    }
}
pub trait IPresentStatusPresentStatisticsImpl: Sized + IPresentStatisticsImpl {
    fn GetCompositionFrameId();
    fn GetPresentStatus();
}
impl ::windows::core::RuntimeName for IPresentStatusPresentStatistics {
    const NAME: &'static str = "Windows.Win32.Graphics.CompositionSwapchain.IPresentStatusPresentStatistics";
}
impl IPresentStatusPresentStatisticsVtbl {
    pub const fn new<Impl: IPresentStatusPresentStatisticsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPresentStatusPresentStatisticsVtbl {
        unsafe extern "system" fn GetCompositionFrameId<Impl: IPresentStatusPresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCompositionFrameId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentStatus<Impl: IPresentStatusPresentStatisticsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> PresentStatus {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPresentStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPresentStatusPresentStatistics>, base.5, GetCompositionFrameId::<Impl, OFFSET>, GetPresentStatus::<Impl, OFFSET>)
    }
}
pub trait IPresentationBufferImpl: Sized {
    fn GetAvailableEvent();
    fn IsAvailable();
}
impl ::windows::core::RuntimeName for IPresentationBuffer {
    const NAME: &'static str = "Windows.Win32.Graphics.CompositionSwapchain.IPresentationBuffer";
}
impl IPresentationBufferVtbl {
    pub const fn new<Impl: IPresentationBufferImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPresentationBufferVtbl {
        unsafe extern "system" fn GetAvailableEvent<Impl: IPresentationBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAvailableEvent(::core::mem::transmute_copy(&availableeventhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Impl: IPresentationBufferImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isavailable: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsAvailable(::core::mem::transmute_copy(&isavailable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPresentationBuffer>, base.5, GetAvailableEvent::<Impl, OFFSET>, IsAvailable::<Impl, OFFSET>)
    }
}
pub trait IPresentationContentImpl: Sized {
    fn SetTag();
}
impl ::windows::core::RuntimeName for IPresentationContent {
    const NAME: &'static str = "Windows.Win32.Graphics.CompositionSwapchain.IPresentationContent";
}
impl IPresentationContentVtbl {
    pub const fn new<Impl: IPresentationContentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPresentationContentVtbl {
        unsafe extern "system" fn SetTag<Impl: IPresentationContentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tag: usize) {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            (*this).SetTag(tag).into()
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPresentationContent>, base.5, SetTag::<Impl, OFFSET>)
    }
}
pub trait IPresentationFactoryImpl: Sized {
    fn IsPresentationSupported();
    fn IsPresentationSupportedWithIndependentFlip();
    fn CreatePresentationManager();
}
impl ::windows::core::RuntimeName for IPresentationFactory {
    const NAME: &'static str = "Windows.Win32.Graphics.CompositionSwapchain.IPresentationFactory";
}
impl IPresentationFactoryVtbl {
    pub const fn new<Impl: IPresentationFactoryImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPresentationFactoryVtbl {
        unsafe extern "system" fn IsPresentationSupported<Impl: IPresentationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPresentationSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPresentationSupportedWithIndependentFlip<Impl: IPresentationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).IsPresentationSupportedWithIndependentFlip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePresentationManager<Impl: IPresentationFactoryImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pppresentationmanager: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePresentationManager(::core::mem::transmute_copy(&pppresentationmanager)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPresentationFactory>, base.5, IsPresentationSupported::<Impl, OFFSET>, IsPresentationSupportedWithIndependentFlip::<Impl, OFFSET>, CreatePresentationManager::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IPresentationManager {
    const NAME: &'static str = "Windows.Win32.Graphics.CompositionSwapchain.IPresentationManager";
}
impl IPresentationManagerVtbl {
    pub const fn new<Impl: IPresentationManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPresentationManagerVtbl {
        unsafe extern "system" fn AddBufferFromResource<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void, presentationbuffer: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddBufferFromResource(&*(&resource as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presentationbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePresentationSurface<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreatePresentationSurface(&*(&compositionsurfacehandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presentationsurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextPresentId<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextPresentId() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetTime<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targettime: SystemInterruptTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTargetTime(&*(&targettime as *const <SystemInterruptTime as ::windows::core::Abi>::Abi as *const <SystemInterruptTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPreferredPresentDuration<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPreferredPresentDuration(&*(&preferredduration as *const <SystemInterruptTime as ::windows::core::Abi>::Abi as *const <SystemInterruptTime as ::windows::core::DefaultType>::DefaultType), &*(&deviationtolerance as *const <SystemInterruptTime as ::windows::core::Abi>::Abi as *const <SystemInterruptTime as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceVSyncInterrupt<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, forcevsyncinterrupt: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ForceVSyncInterrupt(forcevsyncinterrupt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Present<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Present() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentRetiringFence<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPresentRetiringFence(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fence)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelPresentsFrom<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presentidtocancelfrom: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CancelPresentsFrom(presentidtocancelfrom) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLostEvent<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetLostEvent(::core::mem::transmute_copy(&losteventhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentStatisticsAvailableEvent<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetPresentStatisticsAvailableEvent(::core::mem::transmute_copy(&presentstatisticsavailableeventhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnablePresentStatisticsKind<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnablePresentStatisticsKind(presentstatisticskind, enabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextPresentStatistics<Impl: IPresentationManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextpresentstatistics: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetNextPresentStatistics(::core::mem::transmute_copy(&nextpresentstatistics)) {
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
            ::windows::core::GetRuntimeClassName::<IPresentationManager>,
            base.5,
            AddBufferFromResource::<Impl, OFFSET>,
            CreatePresentationSurface::<Impl, OFFSET>,
            GetNextPresentId::<Impl, OFFSET>,
            SetTargetTime::<Impl, OFFSET>,
            SetPreferredPresentDuration::<Impl, OFFSET>,
            ForceVSyncInterrupt::<Impl, OFFSET>,
            Present::<Impl, OFFSET>,
            GetPresentRetiringFence::<Impl, OFFSET>,
            CancelPresentsFrom::<Impl, OFFSET>,
            GetLostEvent::<Impl, OFFSET>,
            GetPresentStatisticsAvailableEvent::<Impl, OFFSET>,
            EnablePresentStatisticsKind::<Impl, OFFSET>,
            GetNextPresentStatistics::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IPresentationSurface {
    const NAME: &'static str = "Windows.Win32.Graphics.CompositionSwapchain.IPresentationSurface";
}
impl IPresentationSurfaceVtbl {
    pub const fn new<Impl: IPresentationSurfaceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IPresentationSurfaceVtbl {
        unsafe extern "system" fn SetBuffer<Impl: IPresentationSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presentationbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBuffer(&*(&presentationbuffer as *const <IPresentationBuffer as ::windows::core::Abi>::Abi as *const <IPresentationBuffer as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorSpace<Impl: IPresentationSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetColorSpace(colorspace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlphaMode<Impl: IPresentationSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetAlphaMode(alphamode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceRect<Impl: IPresentationSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetSourceRect(&*(&sourcerect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransform<Impl: IPresentationSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, transform: *const PresentationTransform) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTransform(&*(&transform as *const <PresentationTransform as ::windows::core::Abi>::Abi as *const <PresentationTransform as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestrictToOutput<Impl: IPresentationSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, output: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RestrictToOutput(&*(&output as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableReadback<Impl: IPresentationSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetDisableReadback(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLetterboxingMargins<Impl: IPresentationSurfaceImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetLetterboxingMargins(leftletterboxsize, topletterboxsize, rightletterboxsize, bottomletterboxsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IPresentationSurface>, base.5, SetBuffer::<Impl, OFFSET>, SetColorSpace::<Impl, OFFSET>, SetAlphaMode::<Impl, OFFSET>, SetSourceRect::<Impl, OFFSET>, SetTransform::<Impl, OFFSET>, RestrictToOutput::<Impl, OFFSET>, SetDisableReadback::<Impl, OFFSET>, SetLetterboxingMargins::<Impl, OFFSET>)
    }
}
