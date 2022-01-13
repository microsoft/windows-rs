#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapterImpl: Sized + IDXGIObjectImpl {
    fn EnumOutputs(&mut self, output: u32) -> ::windows::core::Result<IDXGIOutput>;
    fn GetDesc(&mut self) -> ::windows::core::Result<DXGI_ADAPTER_DESC>;
    fn CheckInterfaceSupport(&mut self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapterVtbl {
        unsafe extern "system" fn EnumOutputs<Impl: IDXGIAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumOutputs(::core::mem::transmute_copy(&output)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: IDXGIAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckInterfaceSupport<Impl: IDXGIAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacename: *const ::windows::core::GUID, pumdversion: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckInterfaceSupport(::core::mem::transmute_copy(&interfacename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pumdversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumOutputs: EnumOutputs::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter1Impl: Sized + IDXGIObjectImpl + IDXGIAdapterImpl {
    fn GetDesc1(&mut self) -> ::windows::core::Result<DXGI_ADAPTER_DESC1>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapter1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: IDXGIAdapter1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc1() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIAdapterVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc1: GetDesc1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter2Impl: Sized + IDXGIObjectImpl + IDXGIAdapterImpl + IDXGIAdapter1Impl {
    fn GetDesc2(&mut self) -> ::windows::core::Result<DXGI_ADAPTER_DESC2>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapter2Vtbl {
        unsafe extern "system" fn GetDesc2<Impl: IDXGIAdapter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc2() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIAdapter1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc2: GetDesc2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter3Impl: Sized + IDXGIObjectImpl + IDXGIAdapterImpl + IDXGIAdapter1Impl + IDXGIAdapter2Impl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterHardwareContentProtectionTeardownStatus(&mut self, dwcookie: u32);
    fn QueryVideoMemoryInfo(&mut self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP) -> ::windows::core::Result<DXGI_QUERY_VIDEO_MEMORY_INFO>;
    fn SetVideoMemoryReservation(&mut self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::Result<()>;
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterVideoMemoryBudgetChangeNotification(&mut self, dwcookie: u32);
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapter3Vtbl {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterHardwareContentProtectionTeardownStatusEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterHardwareContentProtectionTeardownStatus(::core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryVideoMemoryInfo(::core::mem::transmute_copy(&nodeindex), ::core::mem::transmute_copy(&memorysegmentgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvideomemoryinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVideoMemoryReservation(::core::mem::transmute_copy(&nodeindex), ::core::mem::transmute_copy(&memorysegmentgroup), ::core::mem::transmute_copy(&reservation)).into()
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterVideoMemoryBudgetChangeNotificationEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterVideoMemoryBudgetChangeNotification(::core::mem::transmute_copy(&dwcookie))
        }
        Self {
            base: IDXGIAdapter2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegisterHardwareContentProtectionTeardownStatusEvent: RegisterHardwareContentProtectionTeardownStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus: UnregisterHardwareContentProtectionTeardownStatus::<Impl, IMPL_OFFSET>,
            QueryVideoMemoryInfo: QueryVideoMemoryInfo::<Impl, IMPL_OFFSET>,
            SetVideoMemoryReservation: SetVideoMemoryReservation::<Impl, IMPL_OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent: RegisterVideoMemoryBudgetChangeNotificationEvent::<Impl, IMPL_OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification: UnregisterVideoMemoryBudgetChangeNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter4Impl: Sized + IDXGIObjectImpl + IDXGIAdapterImpl + IDXGIAdapter1Impl + IDXGIAdapter2Impl + IDXGIAdapter3Impl {
    fn GetDesc3(&mut self) -> ::windows::core::Result<DXGI_ADAPTER_DESC3>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapter4Vtbl {
        unsafe extern "system" fn GetDesc3<Impl: IDXGIAdapter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc3() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIAdapter3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDesc3: GetDesc3::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter4 as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIDebugImpl: Sized {
    fn ReportLiveObjects(&mut self, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::Result<()>;
}
impl IDXGIDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDebugVtbl {
        unsafe extern "system" fn ReportLiveObjects<Impl: IDXGIDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReportLiveObjects(::core::mem::transmute_copy(&apiid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ReportLiveObjects: ReportLiveObjects::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDebug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDebug1Impl: Sized + IDXGIDebugImpl {
    fn EnableLeakTrackingForThread(&mut self);
    fn DisableLeakTrackingForThread(&mut self);
    fn IsLeakTrackingEnabledForThread(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDebug1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebug1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDebug1Vtbl {
        unsafe extern "system" fn EnableLeakTrackingForThread<Impl: IDXGIDebug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableLeakTrackingForThread()
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Impl: IDXGIDebug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableLeakTrackingForThread()
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Impl: IDXGIDebug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsLeakTrackingEnabledForThread()
        }
        Self {
            base: IDXGIDebugVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnableLeakTrackingForThread: EnableLeakTrackingForThread::<Impl, IMPL_OFFSET>,
            DisableLeakTrackingForThread: DisableLeakTrackingForThread::<Impl, IMPL_OFFSET>,
            IsLeakTrackingEnabledForThread: IsLeakTrackingEnabledForThread::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDebug1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDecodeSwapChainImpl: Sized {
    fn PresentBuffer(&mut self, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::Result<()>;
    fn SetSourceRect(&mut self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetTargetRect(&mut self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetDestSize(&mut self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn GetSourceRect(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetTargetRect(&mut self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetDestSize(&mut self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()>;
    fn SetColorSpace(&mut self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::Result<()>;
    fn GetColorSpace(&mut self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDecodeSwapChainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDecodeSwapChainVtbl {
        unsafe extern "system" fn PresentBuffer<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PresentBuffer(::core::mem::transmute_copy(&buffertopresent), ::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetSourceRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetTargetRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTargetRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetDestSize<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceRect() {
                ::core::result::Result::Ok(ok__) => {
                    *prect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *prect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestSize<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDestSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetColorSpace<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorSpace(::core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn GetColorSpace<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetColorSpace()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PresentBuffer: PresentBuffer::<Impl, IMPL_OFFSET>,
            SetSourceRect: SetSourceRect::<Impl, IMPL_OFFSET>,
            SetTargetRect: SetTargetRect::<Impl, IMPL_OFFSET>,
            SetDestSize: SetDestSize::<Impl, IMPL_OFFSET>,
            GetSourceRect: GetSourceRect::<Impl, IMPL_OFFSET>,
            GetTargetRect: GetTargetRect::<Impl, IMPL_OFFSET>,
            GetDestSize: GetDestSize::<Impl, IMPL_OFFSET>,
            SetColorSpace: SetColorSpace::<Impl, IMPL_OFFSET>,
            GetColorSpace: GetColorSpace::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDecodeSwapChain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDeviceImpl: Sized + IDXGIObjectImpl {
    fn GetAdapter(&mut self) -> ::windows::core::Result<IDXGIAdapter>;
    fn CreateSurface(&mut self, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut ::core::option::Option<IDXGISurface>) -> ::windows::core::Result<()>;
    fn QueryResourceResidency(&mut self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()>;
    fn SetGPUThreadPriority(&mut self, priority: i32) -> ::windows::core::Result<()>;
    fn GetGPUThreadPriority(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDeviceVtbl {
        unsafe extern "system" fn GetAdapter<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    *padapter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numsurfaces), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&psharedresource), ::core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn QueryResourceResidency<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryResourceResidency(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&presidencystatus), ::core::mem::transmute_copy(&numresources)).into()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGPUThreadPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGPUThreadPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *ppriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAdapter: GetAdapter::<Impl, IMPL_OFFSET>,
            CreateSurface: CreateSurface::<Impl, IMPL_OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Impl, IMPL_OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice1Impl: Sized + IDXGIObjectImpl + IDXGIDeviceImpl {
    fn SetMaximumFrameLatency(&mut self, maxlatency: u32) -> ::windows::core::Result<()>;
    fn GetMaximumFrameLatency(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDevice1Vtbl {
        unsafe extern "system" fn SetMaximumFrameLatency<Impl: IDXGIDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumFrameLatency(::core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Impl: IDXGIDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumFrameLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *pmaxlatency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIDeviceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice2Impl: Sized + IDXGIObjectImpl + IDXGIDeviceImpl + IDXGIDevice1Impl {
    fn OfferResources(&mut self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()>;
    fn ReclaimResources(&mut self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EnqueueSetEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDevice2Vtbl {
        unsafe extern "system" fn OfferResources<Impl: IDXGIDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OfferResources(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn ReclaimResources<Impl: IDXGIDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReclaimResources(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdiscarded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnqueueSetEvent<Impl: IDXGIDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnqueueSetEvent(::core::mem::transmute_copy(&hevent)).into()
        }
        Self {
            base: IDXGIDevice1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OfferResources: OfferResources::<Impl, IMPL_OFFSET>,
            ReclaimResources: ReclaimResources::<Impl, IMPL_OFFSET>,
            EnqueueSetEvent: EnqueueSetEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice3Impl: Sized + IDXGIObjectImpl + IDXGIDeviceImpl + IDXGIDevice1Impl + IDXGIDevice2Impl {
    fn Trim(&mut self);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDevice3Vtbl {
        unsafe extern "system" fn Trim<Impl: IDXGIDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Trim()
        }
        Self { base: IDXGIDevice2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Trim: Trim::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice4Impl: Sized + IDXGIObjectImpl + IDXGIDeviceImpl + IDXGIDevice1Impl + IDXGIDevice2Impl + IDXGIDevice3Impl {
    fn OfferResources1(&mut self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::Result<()>;
    fn ReclaimResources1(&mut self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>) -> ::windows::core::Result<DXGI_RECLAIM_RESOURCE_RESULTS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDevice4Vtbl {
        unsafe extern "system" fn OfferResources1<Impl: IDXGIDevice4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OfferResources1(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ReclaimResources1<Impl: IDXGIDevice4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReclaimResources1(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *presults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIDevice3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OfferResources1: OfferResources1::<Impl, IMPL_OFFSET>,
            ReclaimResources1: ReclaimResources1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice4 as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIDeviceSubObjectImpl: Sized + IDXGIObjectImpl {
    fn GetDevice(&mut self, riid: *const ::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDXGIDeviceSubObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDeviceSubObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDeviceSubObjectVtbl {
        unsafe extern "system" fn GetDevice<Impl: IDXGIDeviceSubObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdevice)).into()
        }
        Self { base: IDXGIObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetDevice: GetDevice::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDisplayControlImpl: Sized {
    fn IsStereoEnabled(&mut self) -> super::super::Foundation::BOOL;
    fn SetStereoEnabled(&mut self, enabled: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDisplayControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDisplayControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDisplayControlVtbl {
        unsafe extern "system" fn IsStereoEnabled<Impl: IDXGIDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsStereoEnabled()
        }
        unsafe extern "system" fn SetStereoEnabled<Impl: IDXGIDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStereoEnabled(::core::mem::transmute_copy(&enabled))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsStereoEnabled: IsStereoEnabled::<Impl, IMPL_OFFSET>,
            SetStereoEnabled: SetStereoEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDisplayControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactoryImpl: Sized + IDXGIObjectImpl {
    fn EnumAdapters(&mut self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter>;
    fn MakeWindowAssociation(&mut self, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::Result<()>;
    fn GetWindowAssociation(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn CreateSwapChain(&mut self, pdevice: ::core::option::Option<::windows::core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain>;
    fn CreateSoftwareAdapter(&mut self, module: super::super::Foundation::HINSTANCE) -> ::windows::core::Result<IDXGIAdapter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactoryVtbl {
        unsafe extern "system" fn EnumAdapters<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdapters(::core::mem::transmute_copy(&adapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppadapter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeWindowAssociation<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MakeWindowAssociation(::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetWindowAssociation<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindowAssociation() {
                ::core::result::Result::Ok(ok__) => {
                    *pwindowhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChain<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChain(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::HINSTANCE, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSoftwareAdapter(::core::mem::transmute_copy(&module)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppadapter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumAdapters: EnumAdapters::<Impl, IMPL_OFFSET>,
            MakeWindowAssociation: MakeWindowAssociation::<Impl, IMPL_OFFSET>,
            GetWindowAssociation: GetWindowAssociation::<Impl, IMPL_OFFSET>,
            CreateSwapChain: CreateSwapChain::<Impl, IMPL_OFFSET>,
            CreateSoftwareAdapter: CreateSoftwareAdapter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory1Impl: Sized + IDXGIObjectImpl + IDXGIFactoryImpl {
    fn EnumAdapters1(&mut self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1>;
    fn IsCurrent(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory1Vtbl {
        unsafe extern "system" fn EnumAdapters1<Impl: IDXGIFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdapters1(::core::mem::transmute_copy(&adapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppadapter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrent<Impl: IDXGIFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsCurrent()
        }
        Self {
            base: IDXGIFactoryVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumAdapters1: EnumAdapters1::<Impl, IMPL_OFFSET>,
            IsCurrent: IsCurrent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory2Impl: Sized + IDXGIObjectImpl + IDXGIFactoryImpl + IDXGIFactory1Impl {
    fn IsWindowedStereoEnabled(&mut self) -> super::super::Foundation::BOOL;
    fn CreateSwapChainForHwnd(&mut self, pdevice: ::core::option::Option<::windows::core::IUnknown>, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
    fn CreateSwapChainForCoreWindow(&mut self, pdevice: ::core::option::Option<::windows::core::IUnknown>, pwindow: ::core::option::Option<::windows::core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
    fn GetSharedResourceAdapterLuid(&mut self, hresource: super::super::Foundation::HANDLE) -> ::windows::core::Result<super::super::Foundation::LUID>;
    fn RegisterStereoStatusWindow(&mut self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> ::windows::core::Result<u32>;
    fn RegisterStereoStatusEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterStereoStatus(&mut self, dwcookie: u32);
    fn RegisterOcclusionStatusWindow(&mut self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> ::windows::core::Result<u32>;
    fn RegisterOcclusionStatusEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterOcclusionStatus(&mut self, dwcookie: u32);
    fn CreateSwapChainForComposition(&mut self, pdevice: ::core::option::Option<::windows::core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory2Vtbl {
        unsafe extern "system" fn IsWindowedStereoEnabled<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsWindowedStereoEnabled()
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChainForHwnd(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pfullscreendesc), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChainForCoreWindow(::core::mem::transmute(&pdevice), ::core::mem::transmute(&pwindow), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSharedResourceAdapterLuid(::core::mem::transmute_copy(&hresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *pluid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterStereoStatusWindow(::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&wmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterStereoStatusEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterStereoStatus<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterStereoStatus(::core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterOcclusionStatusWindow(::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&wmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterOcclusionStatusEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterOcclusionStatus(::core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChainForComposition(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIFactory1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsWindowedStereoEnabled: IsWindowedStereoEnabled::<Impl, IMPL_OFFSET>,
            CreateSwapChainForHwnd: CreateSwapChainForHwnd::<Impl, IMPL_OFFSET>,
            CreateSwapChainForCoreWindow: CreateSwapChainForCoreWindow::<Impl, IMPL_OFFSET>,
            GetSharedResourceAdapterLuid: GetSharedResourceAdapterLuid::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusWindow: RegisterStereoStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusEvent: RegisterStereoStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterStereoStatus: UnregisterStereoStatus::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusWindow: RegisterOcclusionStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusEvent: RegisterOcclusionStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterOcclusionStatus: UnregisterOcclusionStatus::<Impl, IMPL_OFFSET>,
            CreateSwapChainForComposition: CreateSwapChainForComposition::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory3Impl: Sized + IDXGIObjectImpl + IDXGIFactoryImpl + IDXGIFactory1Impl + IDXGIFactory2Impl {
    fn GetCreationFlags(&mut self) -> u32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory3Vtbl {
        unsafe extern "system" fn GetCreationFlags<Impl: IDXGIFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCreationFlags()
        }
        Self { base: IDXGIFactory2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetCreationFlags: GetCreationFlags::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory4Impl: Sized + IDXGIObjectImpl + IDXGIFactoryImpl + IDXGIFactory1Impl + IDXGIFactory2Impl + IDXGIFactory3Impl {
    fn EnumAdapterByLuid(&mut self, adapterluid: super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn EnumWarpAdapter(&mut self, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory4Vtbl {
        unsafe extern "system" fn EnumAdapterByLuid<Impl: IDXGIFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumAdapterByLuid(::core::mem::transmute_copy(&adapterluid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn EnumWarpAdapter<Impl: IDXGIFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumWarpAdapter(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self {
            base: IDXGIFactory3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumAdapterByLuid: EnumAdapterByLuid::<Impl, IMPL_OFFSET>,
            EnumWarpAdapter: EnumWarpAdapter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory5Impl: Sized + IDXGIObjectImpl + IDXGIFactoryImpl + IDXGIFactory1Impl + IDXGIFactory2Impl + IDXGIFactory3Impl + IDXGIFactory4Impl {
    fn CheckFeatureSupport(&mut self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory5Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: IDXGIFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        Self { base: IDXGIFactory4Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CheckFeatureSupport: CheckFeatureSupport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory6Impl: Sized + IDXGIObjectImpl + IDXGIFactoryImpl + IDXGIFactory1Impl + IDXGIFactory2Impl + IDXGIFactory3Impl + IDXGIFactory4Impl + IDXGIFactory5Impl {
    fn EnumAdapterByGpuPreference(&mut self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory6Vtbl {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Impl: IDXGIFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumAdapterByGpuPreference(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&gpupreference), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self {
            base: IDXGIFactory5Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EnumAdapterByGpuPreference: EnumAdapterByGpuPreference::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory7Impl: Sized + IDXGIObjectImpl + IDXGIFactoryImpl + IDXGIFactory1Impl + IDXGIFactory2Impl + IDXGIFactory3Impl + IDXGIFactory4Impl + IDXGIFactory5Impl + IDXGIFactory6Impl {
    fn RegisterAdaptersChangedEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterAdaptersChangedEvent(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory7Vtbl {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Impl: IDXGIFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterAdaptersChangedEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Impl: IDXGIFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterAdaptersChangedEvent(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: IDXGIFactory6Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RegisterAdaptersChangedEvent: RegisterAdaptersChangedEvent::<Impl, IMPL_OFFSET>,
            UnregisterAdaptersChangedEvent: UnregisterAdaptersChangedEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactoryMediaImpl: Sized {
    fn CreateSwapChainForCompositionSurfaceHandle(&mut self, pdevice: ::core::option::Option<::windows::core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(&mut self, pdevice: ::core::option::Option<::windows::core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: ::core::option::Option<IDXGIResource>, prestricttooutput: ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGIDecodeSwapChain>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactoryMediaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactoryMediaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactoryMediaVtbl {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Impl: IDXGIFactoryMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChainForCompositionSurfaceHandle(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hsurface), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Impl: IDXGIFactoryMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: ::windows::core::RawPtr, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecodeSwapChainForCompositionSurfaceHandle(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hsurface), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&pyuvdecodebuffers), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateSwapChainForCompositionSurfaceHandle: CreateSwapChainForCompositionSurfaceHandle::<Impl, IMPL_OFFSET>,
            CreateDecodeSwapChainForCompositionSurfaceHandle: CreateDecodeSwapChainForCompositionSurfaceHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactoryMedia as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIInfoQueueImpl: Sized {
    fn SetMessageCountLimit(&mut self, producer: ::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::Result<()>;
    fn ClearStoredMessages(&mut self, producer: ::windows::core::GUID);
    fn GetMessage(&mut self, producer: ::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()>;
    fn GetNumStoredMessagesAllowedByRetrievalFilters(&mut self, producer: ::windows::core::GUID) -> u64;
    fn GetNumStoredMessages(&mut self, producer: ::windows::core::GUID) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&mut self, producer: ::windows::core::GUID) -> u64;
    fn GetMessageCountLimit(&mut self, producer: ::windows::core::GUID) -> u64;
    fn GetNumMessagesAllowedByStorageFilter(&mut self, producer: ::windows::core::GUID) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&mut self, producer: ::windows::core::GUID) -> u64;
    fn AddStorageFilterEntries(&mut self, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetStorageFilter(&mut self, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearStorageFilter(&mut self, producer: ::windows::core::GUID);
    fn PushEmptyStorageFilter(&mut self, producer: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushDenyAllStorageFilter(&mut self, producer: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushCopyOfStorageFilter(&mut self, producer: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushStorageFilter(&mut self, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopStorageFilter(&mut self, producer: ::windows::core::GUID);
    fn GetStorageFilterStackSize(&mut self, producer: ::windows::core::GUID) -> u32;
    fn AddRetrievalFilterEntries(&mut self, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetRetrievalFilter(&mut self, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearRetrievalFilter(&mut self, producer: ::windows::core::GUID);
    fn PushEmptyRetrievalFilter(&mut self, producer: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushDenyAllRetrievalFilter(&mut self, producer: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushCopyOfRetrievalFilter(&mut self, producer: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushRetrievalFilter(&mut self, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopRetrievalFilter(&mut self, producer: ::windows::core::GUID);
    fn GetRetrievalFilterStackSize(&mut self, producer: ::windows::core::GUID) -> u32;
    fn AddMessage(&mut self, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn AddApplicationMessage(&mut self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn SetBreakOnCategory(&mut self, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnSeverity(&mut self, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnID(&mut self, producer: ::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBreakOnCategory(&mut self, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&mut self, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&mut self, producer: ::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&mut self, producer: ::windows::core::GUID, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&mut self, producer: ::windows::core::GUID) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIInfoQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIInfoQueueVtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageCountLimit(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStoredMessages(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetMessage<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessage(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumStoredMessagesAllowedByRetrievalFilters(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumStoredMessages(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesDiscardedByMessageCountLimit(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMessageCountLimit(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesAllowedByStorageFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetNumMessagesDeniedByStorageFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStorageFilterEntries(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStorageFilter(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStorageFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushEmptyStorageFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushDenyAllStorageFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushCopyOfStorageFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushStorageFilter(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopStorageFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStorageFilterStackSize(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRetrievalFilterEntries(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRetrievalFilter(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRetrievalFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushEmptyRetrievalFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushDenyAllRetrievalFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushCopyOfRetrievalFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PushRetrievalFilter(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopRetrievalFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRetrievalFilterStackSize(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn AddMessage<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMessage(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddApplicationMessage(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnCategory(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnSeverity(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBreakOnID(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnCategory(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnSeverity(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBreakOnID(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMuteDebugOutput(::core::mem::transmute_copy(&producer))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Impl, IMPL_OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Impl, IMPL_OFFSET>,
            GetMessage: GetMessage::<Impl, IMPL_OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters: GetNumStoredMessagesAllowedByRetrievalFilters::<Impl, IMPL_OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Impl, IMPL_OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Impl, IMPL_OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Impl, IMPL_OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Impl, IMPL_OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Impl, IMPL_OFFSET>,
            GetStorageFilter: GetStorageFilter::<Impl, IMPL_OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Impl, IMPL_OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Impl, IMPL_OFFSET>,
            PushDenyAllStorageFilter: PushDenyAllStorageFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Impl, IMPL_OFFSET>,
            PushStorageFilter: PushStorageFilter::<Impl, IMPL_OFFSET>,
            PopStorageFilter: PopStorageFilter::<Impl, IMPL_OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Impl, IMPL_OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Impl, IMPL_OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Impl, IMPL_OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushDenyAllRetrievalFilter: PushDenyAllRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Impl, IMPL_OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Impl, IMPL_OFFSET>,
            AddMessage: AddMessage::<Impl, IMPL_OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Impl, IMPL_OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Impl, IMPL_OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            SetBreakOnID: SetBreakOnID::<Impl, IMPL_OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Impl, IMPL_OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            GetBreakOnID: GetBreakOnID::<Impl, IMPL_OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Impl, IMPL_OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIInfoQueue as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIKeyedMutexImpl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl {
    fn AcquireSync(&mut self, key: u64, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn ReleaseSync(&mut self, key: u64) -> ::windows::core::Result<()>;
}
impl IDXGIKeyedMutexVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIKeyedMutexImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIKeyedMutexVtbl {
        unsafe extern "system" fn AcquireSync<Impl: IDXGIKeyedMutexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireSync(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn ReleaseSync<Impl: IDXGIKeyedMutexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseSync(::core::mem::transmute_copy(&key)).into()
        }
        Self {
            base: IDXGIDeviceSubObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AcquireSync: AcquireSync::<Impl, IMPL_OFFSET>,
            ReleaseSync: ReleaseSync::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIKeyedMutex as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIObjectImpl: Sized {
    fn SetPrivateData(&mut self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, name: *const ::windows::core::GUID, punknown: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetPrivateData(&mut self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetParent(&mut self, riid: *const ::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDXGIObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIObjectVtbl {
        unsafe extern "system" fn SetPrivateData<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&name), ::core::mem::transmute(&punknown)).into()
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetParent<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetParent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparent)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetPrivateData: SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData: GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent: GetParent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutputImpl: Sized + IDXGIObjectImpl {
    fn GetDesc(&mut self) -> ::windows::core::Result<DXGI_OUTPUT_DESC>;
    fn GetDisplayModeList(&mut self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()>;
    fn FindClosestMatchingMode(&mut self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn WaitForVBlank(&mut self) -> ::windows::core::Result<()>;
    fn TakeOwnership(&mut self, pdevice: ::core::option::Option<::windows::core::IUnknown>, exclusive: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ReleaseOwnership(&mut self);
    fn GetGammaControlCapabilities(&mut self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL_CAPABILITIES>;
    fn SetGammaControl(&mut self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()>;
    fn GetGammaControl(&mut self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL>;
    fn SetDisplaySurface(&mut self, pscanoutsurface: ::core::option::Option<IDXGISurface>) -> ::windows::core::Result<()>;
    fn GetDisplaySurfaceData(&mut self, pdestination: ::core::option::Option<IDXGISurface>) -> ::windows::core::Result<()>;
    fn GetFrameStatistics(&mut self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutputVtbl {
        unsafe extern "system" fn GetDesc<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayModeList<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayModeList(::core::mem::transmute_copy(&enumformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pnummodes), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindClosestMatchingMode(::core::mem::transmute_copy(&pmodetomatch), ::core::mem::transmute_copy(&pclosestmatch), ::core::mem::transmute(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn WaitForVBlank<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WaitForVBlank().into()
        }
        unsafe extern "system" fn TakeOwnership<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TakeOwnership(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&exclusive)).into()
        }
        unsafe extern "system" fn ReleaseOwnership<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseOwnership()
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGammaControlCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pgammacaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGammaControl<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGammaControl(::core::mem::transmute_copy(&parray)).into()
        }
        unsafe extern "system" fn GetGammaControl<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGammaControl() {
                ::core::result::Result::Ok(ok__) => {
                    *parray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplaySurface<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscanoutsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplaySurface(::core::mem::transmute(&pscanoutsurface)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplaySurfaceData(::core::mem::transmute(&pdestination)).into()
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            GetDisplayModeList: GetDisplayModeList::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode: FindClosestMatchingMode::<Impl, IMPL_OFFSET>,
            WaitForVBlank: WaitForVBlank::<Impl, IMPL_OFFSET>,
            TakeOwnership: TakeOwnership::<Impl, IMPL_OFFSET>,
            ReleaseOwnership: ReleaseOwnership::<Impl, IMPL_OFFSET>,
            GetGammaControlCapabilities: GetGammaControlCapabilities::<Impl, IMPL_OFFSET>,
            SetGammaControl: SetGammaControl::<Impl, IMPL_OFFSET>,
            GetGammaControl: GetGammaControl::<Impl, IMPL_OFFSET>,
            SetDisplaySurface: SetDisplaySurface::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData: GetDisplaySurfaceData::<Impl, IMPL_OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput1Impl: Sized + IDXGIObjectImpl + IDXGIOutputImpl {
    fn GetDisplayModeList1(&mut self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::Result<()>;
    fn FindClosestMatchingMode1(&mut self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetDisplaySurfaceData1(&mut self, pdestination: ::core::option::Option<IDXGIResource>) -> ::windows::core::Result<()>;
    fn DuplicateOutput(&mut self, pdevice: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput1Vtbl {
        unsafe extern "system" fn GetDisplayModeList1<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayModeList1(::core::mem::transmute_copy(&enumformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pnummodes), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FindClosestMatchingMode1(::core::mem::transmute_copy(&pmodetomatch), ::core::mem::transmute_copy(&pclosestmatch), ::core::mem::transmute(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplaySurfaceData1(::core::mem::transmute(&pdestination)).into()
        }
        unsafe extern "system" fn DuplicateOutput<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DuplicateOutput(::core::mem::transmute(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutputduplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIOutputVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDisplayModeList1: GetDisplayModeList1::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode1: FindClosestMatchingMode1::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData1: GetDisplaySurfaceData1::<Impl, IMPL_OFFSET>,
            DuplicateOutput: DuplicateOutput::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput2Impl: Sized + IDXGIObjectImpl + IDXGIOutputImpl + IDXGIOutput1Impl {
    fn SupportsOverlays(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput2Vtbl {
        unsafe extern "system" fn SupportsOverlays<Impl: IDXGIOutput2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SupportsOverlays()
        }
        Self { base: IDXGIOutput1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SupportsOverlays: SupportsOverlays::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput3Impl: Sized + IDXGIObjectImpl + IDXGIOutputImpl + IDXGIOutput1Impl + IDXGIOutput2Impl {
    fn CheckOverlaySupport(&mut self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput3Vtbl {
        unsafe extern "system" fn CheckOverlaySupport<Impl: IDXGIOutput3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckOverlaySupport(::core::mem::transmute_copy(&enumformat), ::core::mem::transmute(&pconcerneddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIOutput2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), CheckOverlaySupport: CheckOverlaySupport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput4Impl: Sized + IDXGIObjectImpl + IDXGIOutputImpl + IDXGIOutput1Impl + IDXGIOutput2Impl + IDXGIOutput3Impl {
    fn CheckOverlayColorSpaceSupport(&mut self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput4Vtbl {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Impl: IDXGIOutput4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckOverlayColorSpaceSupport(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&colorspace), ::core::mem::transmute(&pconcerneddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIOutput3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CheckOverlayColorSpaceSupport: CheckOverlayColorSpaceSupport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput5Impl: Sized + IDXGIObjectImpl + IDXGIOutputImpl + IDXGIOutput1Impl + IDXGIOutput2Impl + IDXGIOutput3Impl + IDXGIOutput4Impl {
    fn DuplicateOutput1(&mut self, pdevice: ::core::option::Option<::windows::core::IUnknown>, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT) -> ::windows::core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput5Vtbl {
        unsafe extern "system" fn DuplicateOutput1<Impl: IDXGIOutput5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DuplicateOutput1(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&supportedformatscount), ::core::mem::transmute_copy(&psupportedformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutputduplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIOutput4Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), DuplicateOutput1: DuplicateOutput1::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput6Impl: Sized + IDXGIObjectImpl + IDXGIOutputImpl + IDXGIOutput1Impl + IDXGIOutput2Impl + IDXGIOutput3Impl + IDXGIOutput4Impl + IDXGIOutput5Impl {
    fn GetDesc1(&mut self) -> ::windows::core::Result<DXGI_OUTPUT_DESC1>;
    fn CheckHardwareCompositionSupport(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput6Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: IDXGIOutput6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc1() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Impl: IDXGIOutput6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHardwareCompositionSupport() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIOutput5Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDesc1: GetDesc1::<Impl, IMPL_OFFSET>,
            CheckHardwareCompositionSupport: CheckHardwareCompositionSupport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIOutputDuplicationImpl: Sized + IDXGIObjectImpl {
    fn GetDesc(&mut self, pdesc: *mut DXGI_OUTDUPL_DESC);
    fn AcquireNextFrame(&mut self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::core::option::Option<IDXGIResource>) -> ::windows::core::Result<()>;
    fn GetFrameDirtyRects(&mut self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetFrameMoveRects(&mut self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetFramePointerShape(&mut self, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::Result<()>;
    fn MapDesktopSurface(&mut self) -> ::windows::core::Result<DXGI_MAPPED_RECT>;
    fn UnMapDesktopSurface(&mut self) -> ::windows::core::Result<()>;
    fn ReleaseFrame(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIOutputDuplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutputDuplicationVtbl {
        unsafe extern "system" fn GetDesc<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn AcquireNextFrame<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireNextFrame(::core::mem::transmute_copy(&timeoutinmilliseconds), ::core::mem::transmute_copy(&pframeinfo), ::core::mem::transmute_copy(&ppdesktopresource)).into()
        }
        unsafe extern "system" fn GetFrameDirtyRects<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFrameDirtyRects(::core::mem::transmute_copy(&dirtyrectsbuffersize), ::core::mem::transmute_copy(&pdirtyrectsbuffer), ::core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFrameMoveRects<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFrameMoveRects(::core::mem::transmute_copy(&moverectsbuffersize), ::core::mem::transmute_copy(&pmoverectbuffer), ::core::mem::transmute_copy(&pmoverectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFramePointerShape<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFramePointerShape(::core::mem::transmute_copy(&pointershapebuffersize), ::core::mem::transmute_copy(&ppointershapebuffer), ::core::mem::transmute_copy(&ppointershapebuffersizerequired), ::core::mem::transmute_copy(&ppointershapeinfo)).into()
        }
        unsafe extern "system" fn MapDesktopSurface<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapDesktopSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *plockedrect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnMapDesktopSurface<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnMapDesktopSurface().into()
        }
        unsafe extern "system" fn ReleaseFrame<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseFrame().into()
        }
        Self {
            base: IDXGIObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            AcquireNextFrame: AcquireNextFrame::<Impl, IMPL_OFFSET>,
            GetFrameDirtyRects: GetFrameDirtyRects::<Impl, IMPL_OFFSET>,
            GetFrameMoveRects: GetFrameMoveRects::<Impl, IMPL_OFFSET>,
            GetFramePointerShape: GetFramePointerShape::<Impl, IMPL_OFFSET>,
            MapDesktopSurface: MapDesktopSurface::<Impl, IMPL_OFFSET>,
            UnMapDesktopSurface: UnMapDesktopSurface::<Impl, IMPL_OFFSET>,
            ReleaseFrame: ReleaseFrame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutputDuplication as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIResourceImpl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl {
    fn GetSharedHandle(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn GetUsage(&mut self) -> ::windows::core::Result<u32>;
    fn SetEvictionPriority(&mut self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::Result<()>;
    fn GetEvictionPriority(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIResourceVtbl {
        unsafe extern "system" fn GetSharedHandle<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSharedHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *psharedhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsage<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *pusage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEvictionPriority(::core::mem::transmute_copy(&evictionpriority)).into()
        }
        unsafe extern "system" fn GetEvictionPriority<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevictionpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEvictionPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *pevictionpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIDeviceSubObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSharedHandle: GetSharedHandle::<Impl, IMPL_OFFSET>,
            GetUsage: GetUsage::<Impl, IMPL_OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait IDXGIResource1Impl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl + IDXGIResourceImpl {
    fn CreateSubresourceSurface(&mut self, index: u32) -> ::windows::core::Result<IDXGISurface2>;
    fn CreateSharedHandle(&mut self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IDXGIResource1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIResource1Vtbl {
        unsafe extern "system" fn CreateSubresourceSurface<Impl: IDXGIResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSubresourceSurface(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Impl: IDXGIResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSharedHandle(::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&dwaccess), ::core::mem::transmute_copy(&lpname)) {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIResourceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateSubresourceSurface: CreateSubresourceSurface::<Impl, IMPL_OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIResource1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISurfaceImpl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl {
    fn GetDesc(&mut self) -> ::windows::core::Result<DXGI_SURFACE_DESC>;
    fn Map(&mut self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()>;
    fn Unmap(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISurfaceVtbl {
        unsafe extern "system" fn GetDesc<Impl: IDXGISurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Map<Impl: IDXGISurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Map(::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&mapflags)).into()
        }
        unsafe extern "system" fn Unmap<Impl: IDXGISurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unmap().into()
        }
        Self {
            base: IDXGIDeviceSubObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            Map: Map::<Impl, IMPL_OFFSET>,
            Unmap: Unmap::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface1Impl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl + IDXGISurfaceImpl {
    fn GetDC(&mut self, discard: super::super::Foundation::BOOL) -> ::windows::core::Result<super::Gdi::HDC>;
    fn ReleaseDC(&mut self, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISurface1Vtbl {
        unsafe extern "system" fn GetDC<Impl: IDXGISurface1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDC(::core::mem::transmute_copy(&discard)) {
                ::core::result::Result::Ok(ok__) => {
                    *phdc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDXGISurface1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self {
            base: IDXGISurfaceVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDC: GetDC::<Impl, IMPL_OFFSET>,
            ReleaseDC: ReleaseDC::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface2Impl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl + IDXGISurfaceImpl + IDXGISurface1Impl {
    fn GetResource(&mut self, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISurface2Vtbl {
        unsafe extern "system" fn GetResource<Impl: IDXGISurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetResource(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparentresource), ::core::mem::transmute_copy(&psubresourceindex)).into()
        }
        Self { base: IDXGISurface1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetResource: GetResource::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChainImpl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl {
    fn Present(&mut self, syncinterval: u32, flags: u32) -> ::windows::core::Result<()>;
    fn GetBuffer(&mut self, buffer: u32, riid: *const ::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetFullscreenState(&mut self, fullscreen: super::super::Foundation::BOOL, ptarget: ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()>;
    fn GetFullscreenState(&mut self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()>;
    fn GetDesc(&mut self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC>;
    fn ResizeBuffers(&mut self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()>;
    fn ResizeTarget(&mut self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()>;
    fn GetContainingOutput(&mut self) -> ::windows::core::Result<IDXGIOutput>;
    fn GetFrameStatistics(&mut self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS>;
    fn GetLastPresentCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChainVtbl {
        unsafe extern "system" fn Present<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Present(::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetBuffer<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: u32, riid: *const ::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn SetFullscreenState<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFullscreenState(::core::mem::transmute_copy(&fullscreen), ::core::mem::transmute(&ptarget)).into()
        }
        unsafe extern "system" fn GetFullscreenState<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFullscreenState(::core::mem::transmute_copy(&pfullscreen), ::core::mem::transmute_copy(&pptarget)).into()
        }
        unsafe extern "system" fn GetDesc<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeBuffers<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResizeBuffers(::core::mem::transmute_copy(&buffercount), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&newformat), ::core::mem::transmute_copy(&swapchainflags)).into()
        }
        unsafe extern "system" fn ResizeTarget<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResizeTarget(::core::mem::transmute_copy(&pnewtargetparameters)).into()
        }
        unsafe extern "system" fn GetContainingOutput<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainingOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPresentCount<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPresentCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plastpresentcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIDeviceSubObjectVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Present: Present::<Impl, IMPL_OFFSET>,
            GetBuffer: GetBuffer::<Impl, IMPL_OFFSET>,
            SetFullscreenState: SetFullscreenState::<Impl, IMPL_OFFSET>,
            GetFullscreenState: GetFullscreenState::<Impl, IMPL_OFFSET>,
            GetDesc: GetDesc::<Impl, IMPL_OFFSET>,
            ResizeBuffers: ResizeBuffers::<Impl, IMPL_OFFSET>,
            ResizeTarget: ResizeTarget::<Impl, IMPL_OFFSET>,
            GetContainingOutput: GetContainingOutput::<Impl, IMPL_OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetLastPresentCount: GetLastPresentCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain1Impl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl + IDXGISwapChainImpl {
    fn GetDesc1(&mut self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC1>;
    fn GetFullscreenDesc(&mut self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_FULLSCREEN_DESC>;
    fn GetHwnd(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn GetCoreWindow(&mut self, refiid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Present1(&mut self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::Result<()>;
    fn IsTemporaryMonoSupported(&mut self) -> super::super::Foundation::BOOL;
    fn GetRestrictToOutput(&mut self) -> ::windows::core::Result<IDXGIOutput>;
    fn SetBackgroundColor(&mut self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()>;
    fn GetBackgroundColor(&mut self) -> ::windows::core::Result<DXGI_RGBA>;
    fn SetRotation(&mut self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()>;
    fn GetRotation(&mut self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChain1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc1() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullscreenDesc<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullscreenDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHwnd<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCoreWindow<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCoreWindow(::core::mem::transmute_copy(&refiid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn Present1<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Present1(::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&presentflags), ::core::mem::transmute_copy(&ppresentparameters)).into()
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsTemporaryMonoSupported()
        }
        unsafe extern "system" fn GetRestrictToOutput<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprestricttooutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictToOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *pprestricttooutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *const DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBackgroundColor(::core::mem::transmute_copy(&pcolor)).into()
        }
        unsafe extern "system" fn GetBackgroundColor<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRotation(::core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn GetRotation<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *protation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGISwapChainVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDesc1: GetDesc1::<Impl, IMPL_OFFSET>,
            GetFullscreenDesc: GetFullscreenDesc::<Impl, IMPL_OFFSET>,
            GetHwnd: GetHwnd::<Impl, IMPL_OFFSET>,
            GetCoreWindow: GetCoreWindow::<Impl, IMPL_OFFSET>,
            Present1: Present1::<Impl, IMPL_OFFSET>,
            IsTemporaryMonoSupported: IsTemporaryMonoSupported::<Impl, IMPL_OFFSET>,
            GetRestrictToOutput: GetRestrictToOutput::<Impl, IMPL_OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Impl, IMPL_OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Impl, IMPL_OFFSET>,
            SetRotation: SetRotation::<Impl, IMPL_OFFSET>,
            GetRotation: GetRotation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain2Impl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl + IDXGISwapChainImpl + IDXGISwapChain1Impl {
    fn SetSourceSize(&mut self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn GetSourceSize(&mut self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()>;
    fn SetMaximumFrameLatency(&mut self, maxlatency: u32) -> ::windows::core::Result<()>;
    fn GetMaximumFrameLatency(&mut self) -> ::windows::core::Result<u32>;
    fn GetFrameLatencyWaitableObject(&mut self) -> super::super::Foundation::HANDLE;
    fn SetMatrixTransform(&mut self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()>;
    fn GetMatrixTransform(&mut self) -> ::windows::core::Result<DXGI_MATRIX_3X2_F>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChain2Vtbl {
        unsafe extern "system" fn SetSourceSize<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourceSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceSize<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSourceSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaximumFrameLatency(::core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumFrameLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *pmaxlatency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFrameLatencyWaitableObject()
        }
        unsafe extern "system" fn SetMatrixTransform<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMatrixTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetMatrixTransform<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrixTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *pmatrix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGISwapChain1Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSourceSize: SetSourceSize::<Impl, IMPL_OFFSET>,
            GetSourceSize: GetSourceSize::<Impl, IMPL_OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetFrameLatencyWaitableObject: GetFrameLatencyWaitableObject::<Impl, IMPL_OFFSET>,
            SetMatrixTransform: SetMatrixTransform::<Impl, IMPL_OFFSET>,
            GetMatrixTransform: GetMatrixTransform::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain3Impl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl + IDXGISwapChainImpl + IDXGISwapChain1Impl + IDXGISwapChain2Impl {
    fn GetCurrentBackBufferIndex(&mut self) -> u32;
    fn CheckColorSpaceSupport(&mut self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<u32>;
    fn SetColorSpace1(&mut self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()>;
    fn ResizeBuffers1(&mut self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChain3Vtbl {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentBackBufferIndex()
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckColorSpaceSupport(::core::mem::transmute_copy(&colorspace)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcolorspacesupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorSpace1<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetColorSpace1(::core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn ResizeBuffers1<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResizeBuffers1(::core::mem::transmute_copy(&buffercount), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&swapchainflags), ::core::mem::transmute_copy(&pcreationnodemask), ::core::mem::transmute_copy(&pppresentqueue)).into()
        }
        Self {
            base: IDXGISwapChain2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCurrentBackBufferIndex: GetCurrentBackBufferIndex::<Impl, IMPL_OFFSET>,
            CheckColorSpaceSupport: CheckColorSpaceSupport::<Impl, IMPL_OFFSET>,
            SetColorSpace1: SetColorSpace1::<Impl, IMPL_OFFSET>,
            ResizeBuffers1: ResizeBuffers1::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain4Impl: Sized + IDXGIObjectImpl + IDXGIDeviceSubObjectImpl + IDXGISwapChainImpl + IDXGISwapChain1Impl + IDXGISwapChain2Impl + IDXGISwapChain3Impl {
    fn SetHDRMetaData(&mut self, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChain4Vtbl {
        unsafe extern "system" fn SetHDRMetaData<Impl: IDXGISwapChain4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHDRMetaData(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata)).into()
        }
        Self { base: IDXGISwapChain3Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), SetHDRMetaData: SetHDRMetaData::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain4 as ::windows::core::Interface>::IID
    }
}
pub trait IDXGISwapChainMediaImpl: Sized {
    fn GetFrameStatisticsMedia(&mut self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS_MEDIA>;
    fn SetPresentDuration(&mut self, duration: u32) -> ::windows::core::Result<()>;
    fn CheckPresentDurationSupport(&mut self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::Result<()>;
}
impl IDXGISwapChainMediaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainMediaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChainMediaVtbl {
        unsafe extern "system" fn GetFrameStatisticsMedia<Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatisticsMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentDuration<Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPresentDuration(::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckPresentDurationSupport(::core::mem::transmute_copy(&desiredpresentduration), ::core::mem::transmute_copy(&pclosestsmallerpresentduration), ::core::mem::transmute_copy(&pclosestlargerpresentduration)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetFrameStatisticsMedia: GetFrameStatisticsMedia::<Impl, IMPL_OFFSET>,
            SetPresentDuration: SetPresentDuration::<Impl, IMPL_OFFSET>,
            CheckPresentDurationSupport: CheckPresentDurationSupport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChainMedia as ::windows::core::Interface>::IID
    }
}
pub trait IDXGraphicsAnalysisImpl: Sized {
    fn BeginCapture(&mut self);
    fn EndCapture(&mut self);
}
impl IDXGraphicsAnalysisVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGraphicsAnalysisImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGraphicsAnalysisVtbl {
        unsafe extern "system" fn BeginCapture<Impl: IDXGraphicsAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginCapture()
        }
        unsafe extern "system" fn EndCapture<Impl: IDXGraphicsAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndCapture()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BeginCapture: BeginCapture::<Impl, IMPL_OFFSET>,
            EndCapture: EndCapture::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGraphicsAnalysis as ::windows::core::Interface>::IID
    }
}
