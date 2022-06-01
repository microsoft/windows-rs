#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait ICompositionFramePresentStatistics_Impl: Sized + IPresentStatistics_Impl {
    fn GetContentTag(&self) -> usize;
    fn GetCompositionFrameId(&self) -> u64;
    fn GetDisplayInstanceArray(&self, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for ICompositionFramePresentStatistics {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ICompositionFramePresentStatistics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>() -> ICompositionFramePresentStatistics_Vtbl {
        unsafe extern "system" fn GetContentTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContentTag()
        }
        unsafe extern "system" fn GetCompositionFrameId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCompositionFrameId()
        }
        unsafe extern "system" fn GetDisplayInstanceArray<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICompositionFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, displayinstancearraycount: *mut u32, displayinstancearray: *mut *mut CompositionFrameDisplayInstance) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayInstanceArray(::core::mem::transmute_copy(&displayinstancearraycount), ::core::mem::transmute_copy(&displayinstancearray))
        }
        Self {
            base__: IPresentStatistics_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetContentTag: GetContentTag::<Identity, Impl, OFFSET>,
            GetCompositionFrameId: GetCompositionFrameId::<Identity, Impl, OFFSET>,
            GetDisplayInstanceArray: GetDisplayInstanceArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICompositionFramePresentStatistics as ::windows::core::Interface>::IID || iid == &<IPresentStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IIndependentFlipFramePresentStatistics_Impl: Sized + IPresentStatistics_Impl {
    fn GetOutputAdapterLUID(&self) -> super::super::Foundation::LUID;
    fn GetOutputVidPnSourceId(&self) -> u32;
    fn GetContentTag(&self) -> usize;
    fn GetDisplayedTime(&self) -> SystemInterruptTime;
    fn GetPresentDuration(&self) -> SystemInterruptTime;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IIndependentFlipFramePresentStatistics {}
#[cfg(feature = "Win32_Foundation")]
impl IIndependentFlipFramePresentStatistics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>() -> IIndependentFlipFramePresentStatistics_Vtbl {
        unsafe extern "system" fn GetOutputAdapterLUID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::LUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = this.GetOutputAdapterLUID()
        }
        unsafe extern "system" fn GetOutputVidPnSourceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOutputVidPnSourceId()
        }
        unsafe extern "system" fn GetContentTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContentTag()
        }
        unsafe extern "system" fn GetDisplayedTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = this.GetDisplayedTime()
        }
        unsafe extern "system" fn GetPresentDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IIndependentFlipFramePresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut SystemInterruptTime) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            *result__ = this.GetPresentDuration()
        }
        Self {
            base__: IPresentStatistics_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetOutputAdapterLUID: GetOutputAdapterLUID::<Identity, Impl, OFFSET>,
            GetOutputVidPnSourceId: GetOutputVidPnSourceId::<Identity, Impl, OFFSET>,
            GetContentTag: GetContentTag::<Identity, Impl, OFFSET>,
            GetDisplayedTime: GetDisplayedTime::<Identity, Impl, OFFSET>,
            GetPresentDuration: GetPresentDuration::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIndependentFlipFramePresentStatistics as ::windows::core::Interface>::IID || iid == &<IPresentStatistics as ::windows::core::Interface>::IID
    }
}
pub trait IPresentStatistics_Impl: Sized {
    fn GetPresentId(&self) -> u64;
    fn GetKind(&self) -> PresentStatisticsKind;
}
impl ::windows::core::RuntimeName for IPresentStatistics {}
impl IPresentStatistics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatistics_Impl, const OFFSET: isize>() -> IPresentStatistics_Vtbl {
        unsafe extern "system" fn GetPresentId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresentId()
        }
        unsafe extern "system" fn GetKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> PresentStatisticsKind {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKind()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPresentId: GetPresentId::<Identity, Impl, OFFSET>,
            GetKind: GetKind::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentStatistics as ::windows::core::Interface>::IID
    }
}
pub trait IPresentStatusPresentStatistics_Impl: Sized + IPresentStatistics_Impl {
    fn GetCompositionFrameId(&self) -> u64;
    fn GetPresentStatus(&self) -> PresentStatus;
}
impl ::windows::core::RuntimeName for IPresentStatusPresentStatistics {}
impl IPresentStatusPresentStatistics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatusPresentStatistics_Impl, const OFFSET: isize>() -> IPresentStatusPresentStatistics_Vtbl {
        unsafe extern "system" fn GetCompositionFrameId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatusPresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCompositionFrameId()
        }
        unsafe extern "system" fn GetPresentStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentStatusPresentStatistics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> PresentStatus {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPresentStatus()
        }
        Self {
            base__: IPresentStatistics_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCompositionFrameId: GetCompositionFrameId::<Identity, Impl, OFFSET>,
            GetPresentStatus: GetPresentStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentStatusPresentStatistics as ::windows::core::Interface>::IID || iid == &<IPresentStatistics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPresentationBuffer_Impl: Sized {
    fn GetAvailableEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn IsAvailable(&self) -> ::windows::core::Result<u8>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPresentationBuffer {}
#[cfg(feature = "Win32_Foundation")]
impl IPresentationBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationBuffer_Impl, const OFFSET: isize>() -> IPresentationBuffer_Vtbl {
        unsafe extern "system" fn GetAvailableEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAvailableEvent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(availableeventhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, isavailable: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isavailable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetAvailableEvent: GetAvailableEvent::<Identity, Impl, OFFSET>,
            IsAvailable: IsAvailable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IPresentationContent_Impl: Sized {
    fn SetTag(&self, tag: usize);
}
impl ::windows::core::RuntimeName for IPresentationContent {}
impl IPresentationContent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationContent_Impl, const OFFSET: isize>() -> IPresentationContent_Vtbl {
        unsafe extern "system" fn SetTag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationContent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, tag: usize) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTag(::core::mem::transmute_copy(&tag))
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetTag: SetTag::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationContent as ::windows::core::Interface>::IID
    }
}
pub trait IPresentationFactory_Impl: Sized {
    fn IsPresentationSupported(&self) -> u8;
    fn IsPresentationSupportedWithIndependentFlip(&self) -> u8;
    fn CreatePresentationManager(&self) -> ::windows::core::Result<IPresentationManager>;
}
impl ::windows::core::RuntimeName for IPresentationFactory {}
impl IPresentationFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: isize>() -> IPresentationFactory_Vtbl {
        unsafe extern "system" fn IsPresentationSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsPresentationSupported()
        }
        unsafe extern "system" fn IsPresentationSupportedWithIndependentFlip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u8 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsPresentationSupportedWithIndependentFlip()
        }
        unsafe extern "system" fn CreatePresentationManager<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppresentationmanager: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePresentationManager() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppresentationmanager, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsPresentationSupported: IsPresentationSupported::<Identity, Impl, OFFSET>,
            IsPresentationSupportedWithIndependentFlip: IsPresentationSupportedWithIndependentFlip::<Identity, Impl, OFFSET>,
            CreatePresentationManager: CreatePresentationManager::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPresentationManager_Impl: Sized {
    fn AddBufferFromResource(&self, resource: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IPresentationBuffer>;
    fn CreatePresentationSurface(&self, compositionsurfacehandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<IPresentationSurface>;
    fn GetNextPresentId(&self) -> u64;
    fn SetTargetTime(&self, targettime: &SystemInterruptTime) -> ::windows::core::Result<()>;
    fn SetPreferredPresentDuration(&self, preferredduration: &SystemInterruptTime, deviationtolerance: &SystemInterruptTime) -> ::windows::core::Result<()>;
    fn ForceVSyncInterrupt(&self, forcevsyncinterrupt: u8) -> ::windows::core::Result<()>;
    fn Present(&self) -> ::windows::core::Result<()>;
    fn GetPresentRetiringFence(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<*mut ::core::ffi::c_void>;
    fn CancelPresentsFrom(&self, presentidtocancelfrom: u64) -> ::windows::core::Result<()>;
    fn GetLostEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn GetPresentStatisticsAvailableEvent(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn EnablePresentStatisticsKind(&self, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::Result<()>;
    fn GetNextPresentStatistics(&self) -> ::windows::core::Result<IPresentStatistics>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPresentationManager {}
#[cfg(feature = "Win32_Foundation")]
impl IPresentationManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>() -> IPresentationManager_Vtbl {
        unsafe extern "system" fn AddBufferFromResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut ::core::ffi::c_void, presentationbuffer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddBufferFromResource(::core::mem::transmute(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presentationbuffer, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreatePresentationSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, compositionsurfacehandle: super::super::Foundation::HANDLE, presentationsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreatePresentationSurface(::core::mem::transmute_copy(&compositionsurfacehandle)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presentationsurface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNextPresentId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNextPresentId()
        }
        unsafe extern "system" fn SetTargetTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, targettime: SystemInterruptTime) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTargetTime(::core::mem::transmute(&targettime)).into()
        }
        unsafe extern "system" fn SetPreferredPresentDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preferredduration: SystemInterruptTime, deviationtolerance: SystemInterruptTime) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPreferredPresentDuration(::core::mem::transmute(&preferredduration), ::core::mem::transmute(&deviationtolerance)).into()
        }
        unsafe extern "system" fn ForceVSyncInterrupt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, forcevsyncinterrupt: u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForceVSyncInterrupt(::core::mem::transmute_copy(&forcevsyncinterrupt)).into()
        }
        unsafe extern "system" fn Present<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Present().into()
        }
        unsafe extern "system" fn GetPresentRetiringFence<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, fence: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPresentRetiringFence(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(fence, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelPresentsFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentidtocancelfrom: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelPresentsFrom(::core::mem::transmute_copy(&presentidtocancelfrom)).into()
        }
        unsafe extern "system" fn GetLostEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, losteventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLostEvent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(losteventhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPresentStatisticsAvailableEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentstatisticsavailableeventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPresentStatisticsAvailableEvent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presentstatisticsavailableeventhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnablePresentStatisticsKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentstatisticskind: PresentStatisticsKind, enabled: u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnablePresentStatisticsKind(::core::mem::transmute_copy(&presentstatisticskind), ::core::mem::transmute_copy(&enabled)).into()
        }
        unsafe extern "system" fn GetNextPresentStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nextpresentstatistics: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNextPresentStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nextpresentstatistics, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddBufferFromResource: AddBufferFromResource::<Identity, Impl, OFFSET>,
            CreatePresentationSurface: CreatePresentationSurface::<Identity, Impl, OFFSET>,
            GetNextPresentId: GetNextPresentId::<Identity, Impl, OFFSET>,
            SetTargetTime: SetTargetTime::<Identity, Impl, OFFSET>,
            SetPreferredPresentDuration: SetPreferredPresentDuration::<Identity, Impl, OFFSET>,
            ForceVSyncInterrupt: ForceVSyncInterrupt::<Identity, Impl, OFFSET>,
            Present: Present::<Identity, Impl, OFFSET>,
            GetPresentRetiringFence: GetPresentRetiringFence::<Identity, Impl, OFFSET>,
            CancelPresentsFrom: CancelPresentsFrom::<Identity, Impl, OFFSET>,
            GetLostEvent: GetLostEvent::<Identity, Impl, OFFSET>,
            GetPresentStatisticsAvailableEvent: GetPresentStatisticsAvailableEvent::<Identity, Impl, OFFSET>,
            EnablePresentStatisticsKind: EnablePresentStatisticsKind::<Identity, Impl, OFFSET>,
            GetNextPresentStatistics: GetNextPresentStatistics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationManager as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IPresentationSurface_Impl: Sized + IPresentationContent_Impl {
    fn SetBuffer(&self, presentationbuffer: &::core::option::Option<IPresentationBuffer>) -> ::windows::core::Result<()>;
    fn SetColorSpace(&self, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()>;
    fn SetAlphaMode(&self, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::Result<()>;
    fn SetSourceRect(&self, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetTransform(&self, transform: *const PresentationTransform) -> ::windows::core::Result<()>;
    fn RestrictToOutput(&self, output: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetDisableReadback(&self, value: u8) -> ::windows::core::Result<()>;
    fn SetLetterboxingMargins(&self, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IPresentationSurface {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IPresentationSurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>() -> IPresentationSurface_Vtbl {
        unsafe extern "system" fn SetBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presentationbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBuffer(::core::mem::transmute(&presentationbuffer)).into()
        }
        unsafe extern "system" fn SetColorSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorSpace(::core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn SetAlphaMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, alphamode: super::Dxgi::Common::DXGI_ALPHA_MODE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAlphaMode(::core::mem::transmute_copy(&alphamode)).into()
        }
        unsafe extern "system" fn SetSourceRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcerect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourceRect(::core::mem::transmute_copy(&sourcerect)).into()
        }
        unsafe extern "system" fn SetTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transform: *const PresentationTransform) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransform(::core::mem::transmute_copy(&transform)).into()
        }
        unsafe extern "system" fn RestrictToOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestrictToOutput(::core::mem::transmute(&output)).into()
        }
        unsafe extern "system" fn SetDisableReadback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisableReadback(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetLetterboxingMargins<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPresentationSurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, leftletterboxsize: f32, topletterboxsize: f32, rightletterboxsize: f32, bottomletterboxsize: f32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLetterboxingMargins(::core::mem::transmute_copy(&leftletterboxsize), ::core::mem::transmute_copy(&topletterboxsize), ::core::mem::transmute_copy(&rightletterboxsize), ::core::mem::transmute_copy(&bottomletterboxsize)).into()
        }
        Self {
            base__: IPresentationContent_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBuffer: SetBuffer::<Identity, Impl, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, Impl, OFFSET>,
            SetAlphaMode: SetAlphaMode::<Identity, Impl, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, Impl, OFFSET>,
            SetTransform: SetTransform::<Identity, Impl, OFFSET>,
            RestrictToOutput: RestrictToOutput::<Identity, Impl, OFFSET>,
            SetDisableReadback: SetDisableReadback::<Identity, Impl, OFFSET>,
            SetLetterboxingMargins: SetLetterboxingMargins::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPresentationSurface as ::windows::core::Interface>::IID || iid == &<IPresentationContent as ::windows::core::Interface>::IID
    }
}
