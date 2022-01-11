#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapterImpl: Sized + IDXGIObjectImpl {
    fn EnumOutputs();
    fn GetDesc();
    fn CheckInterfaceSupport();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapterVtbl {
        unsafe extern "system" fn EnumOutputs<Impl: IDXGIAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: IDXGIAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckInterfaceSupport<Impl: IDXGIAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacename: *const ::windows::core::GUID, pumdversion: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, GetParent::<Impl, IMPL_OFFSET>, EnumOutputs::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, CheckInterfaceSupport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter1Impl: Sized + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn GetDesc1();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapter1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: IDXGIAdapter1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, GetParent::<Impl, IMPL_OFFSET>, EnumOutputs::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, CheckInterfaceSupport::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter2Impl: Sized + IDXGIAdapter1Impl + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn GetDesc2();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapter2Vtbl {
        unsafe extern "system" fn GetDesc2<Impl: IDXGIAdapter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, GetParent::<Impl, IMPL_OFFSET>, EnumOutputs::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, CheckInterfaceSupport::<Impl, IMPL_OFFSET>, GetDesc1::<Impl, IMPL_OFFSET>, GetDesc2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter3Impl: Sized + IDXGIAdapter2Impl + IDXGIAdapter1Impl + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent();
    fn UnregisterHardwareContentProtectionTeardownStatus();
    fn QueryVideoMemoryInfo();
    fn SetVideoMemoryReservation();
    fn RegisterVideoMemoryBudgetChangeNotificationEvent();
    fn UnregisterVideoMemoryBudgetChangeNotification();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapter3Vtbl {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumOutputs::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            CheckInterfaceSupport::<Impl, IMPL_OFFSET>,
            GetDesc1::<Impl, IMPL_OFFSET>,
            GetDesc2::<Impl, IMPL_OFFSET>,
            RegisterHardwareContentProtectionTeardownStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus::<Impl, IMPL_OFFSET>,
            QueryVideoMemoryInfo::<Impl, IMPL_OFFSET>,
            SetVideoMemoryReservation::<Impl, IMPL_OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent::<Impl, IMPL_OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter4Impl: Sized + IDXGIAdapter3Impl + IDXGIAdapter2Impl + IDXGIAdapter1Impl + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn GetDesc3();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIAdapter4Vtbl {
        unsafe extern "system" fn GetDesc3<Impl: IDXGIAdapter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumOutputs::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            CheckInterfaceSupport::<Impl, IMPL_OFFSET>,
            GetDesc1::<Impl, IMPL_OFFSET>,
            GetDesc2::<Impl, IMPL_OFFSET>,
            RegisterHardwareContentProtectionTeardownStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus::<Impl, IMPL_OFFSET>,
            QueryVideoMemoryInfo::<Impl, IMPL_OFFSET>,
            SetVideoMemoryReservation::<Impl, IMPL_OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent::<Impl, IMPL_OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification::<Impl, IMPL_OFFSET>,
            GetDesc3::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter4 as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIDebugImpl: Sized {
    fn ReportLiveObjects();
}
impl IDXGIDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebugImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDebugVtbl {
        unsafe extern "system" fn ReportLiveObjects<Impl: IDXGIDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReportLiveObjects::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDebug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDebug1Impl: Sized + IDXGIDebugImpl {
    fn EnableLeakTrackingForThread();
    fn DisableLeakTrackingForThread();
    fn IsLeakTrackingEnabledForThread();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDebug1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebug1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDebug1Vtbl {
        unsafe extern "system" fn EnableLeakTrackingForThread<Impl: IDXGIDebug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Impl: IDXGIDebug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Impl: IDXGIDebug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReportLiveObjects::<Impl, IMPL_OFFSET>, EnableLeakTrackingForThread::<Impl, IMPL_OFFSET>, DisableLeakTrackingForThread::<Impl, IMPL_OFFSET>, IsLeakTrackingEnabledForThread::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDebug1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDecodeSwapChainImpl: Sized {
    fn PresentBuffer();
    fn SetSourceRect();
    fn SetTargetRect();
    fn SetDestSize();
    fn GetSourceRect();
    fn GetTargetRect();
    fn GetDestSize();
    fn SetColorSpace();
    fn GetColorSpace();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDecodeSwapChainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDecodeSwapChainVtbl {
        unsafe extern "system" fn PresentBuffer<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourceRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTargetRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDestSize<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTargetRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDestSize<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorSpace<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColorSpace<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, PresentBuffer::<Impl, IMPL_OFFSET>, SetSourceRect::<Impl, IMPL_OFFSET>, SetTargetRect::<Impl, IMPL_OFFSET>, SetDestSize::<Impl, IMPL_OFFSET>, GetSourceRect::<Impl, IMPL_OFFSET>, GetTargetRect::<Impl, IMPL_OFFSET>, GetDestSize::<Impl, IMPL_OFFSET>, SetColorSpace::<Impl, IMPL_OFFSET>, GetColorSpace::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDecodeSwapChain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDeviceImpl: Sized + IDXGIObjectImpl {
    fn GetAdapter();
    fn CreateSurface();
    fn QueryResourceResidency();
    fn SetGPUThreadPriority();
    fn GetGPUThreadPriority();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDeviceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDeviceVtbl {
        unsafe extern "system" fn GetAdapter<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSurface<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryResourceResidency<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetAdapter::<Impl, IMPL_OFFSET>,
            CreateSurface::<Impl, IMPL_OFFSET>,
            QueryResourceResidency::<Impl, IMPL_OFFSET>,
            SetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            GetGPUThreadPriority::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice1Impl: Sized + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn SetMaximumFrameLatency();
    fn GetMaximumFrameLatency();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDevice1Vtbl {
        unsafe extern "system" fn SetMaximumFrameLatency<Impl: IDXGIDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Impl: IDXGIDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetAdapter::<Impl, IMPL_OFFSET>,
            CreateSurface::<Impl, IMPL_OFFSET>,
            QueryResourceResidency::<Impl, IMPL_OFFSET>,
            SetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            GetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice2Impl: Sized + IDXGIDevice1Impl + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn OfferResources();
    fn ReclaimResources();
    fn EnqueueSetEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDevice2Vtbl {
        unsafe extern "system" fn OfferResources<Impl: IDXGIDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReclaimResources<Impl: IDXGIDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnqueueSetEvent<Impl: IDXGIDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetAdapter::<Impl, IMPL_OFFSET>,
            CreateSurface::<Impl, IMPL_OFFSET>,
            QueryResourceResidency::<Impl, IMPL_OFFSET>,
            SetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            GetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            OfferResources::<Impl, IMPL_OFFSET>,
            ReclaimResources::<Impl, IMPL_OFFSET>,
            EnqueueSetEvent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice3Impl: Sized + IDXGIDevice2Impl + IDXGIDevice1Impl + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn Trim();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDevice3Vtbl {
        unsafe extern "system" fn Trim<Impl: IDXGIDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetAdapter::<Impl, IMPL_OFFSET>,
            CreateSurface::<Impl, IMPL_OFFSET>,
            QueryResourceResidency::<Impl, IMPL_OFFSET>,
            SetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            GetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            OfferResources::<Impl, IMPL_OFFSET>,
            ReclaimResources::<Impl, IMPL_OFFSET>,
            EnqueueSetEvent::<Impl, IMPL_OFFSET>,
            Trim::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice4Impl: Sized + IDXGIDevice3Impl + IDXGIDevice2Impl + IDXGIDevice1Impl + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn OfferResources1();
    fn ReclaimResources1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDevice4Vtbl {
        unsafe extern "system" fn OfferResources1<Impl: IDXGIDevice4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReclaimResources1<Impl: IDXGIDevice4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetAdapter::<Impl, IMPL_OFFSET>,
            CreateSurface::<Impl, IMPL_OFFSET>,
            QueryResourceResidency::<Impl, IMPL_OFFSET>,
            SetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            GetGPUThreadPriority::<Impl, IMPL_OFFSET>,
            SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            OfferResources::<Impl, IMPL_OFFSET>,
            ReclaimResources::<Impl, IMPL_OFFSET>,
            EnqueueSetEvent::<Impl, IMPL_OFFSET>,
            Trim::<Impl, IMPL_OFFSET>,
            OfferResources1::<Impl, IMPL_OFFSET>,
            ReclaimResources1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice4 as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIDeviceSubObjectImpl: Sized + IDXGIObjectImpl {
    fn GetDevice();
}
impl IDXGIDeviceSubObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDeviceSubObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDeviceSubObjectVtbl {
        unsafe extern "system" fn GetDevice<Impl: IDXGIDeviceSubObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, GetParent::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDisplayControlImpl: Sized {
    fn IsStereoEnabled();
    fn SetStereoEnabled();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDisplayControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDisplayControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIDisplayControlVtbl {
        unsafe extern "system" fn IsStereoEnabled<Impl: IDXGIDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStereoEnabled<Impl: IDXGIDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsStereoEnabled::<Impl, IMPL_OFFSET>, SetStereoEnabled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDisplayControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactoryImpl: Sized + IDXGIObjectImpl {
    fn EnumAdapters();
    fn MakeWindowAssociation();
    fn GetWindowAssociation();
    fn CreateSwapChain();
    fn CreateSoftwareAdapter();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactoryVtbl {
        unsafe extern "system" fn EnumAdapters<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MakeWindowAssociation<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWindowAssociation<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSwapChain<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::HINSTANCE, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumAdapters::<Impl, IMPL_OFFSET>,
            MakeWindowAssociation::<Impl, IMPL_OFFSET>,
            GetWindowAssociation::<Impl, IMPL_OFFSET>,
            CreateSwapChain::<Impl, IMPL_OFFSET>,
            CreateSoftwareAdapter::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory1Impl: Sized + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn EnumAdapters1();
    fn IsCurrent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory1Vtbl {
        unsafe extern "system" fn EnumAdapters1<Impl: IDXGIFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCurrent<Impl: IDXGIFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumAdapters::<Impl, IMPL_OFFSET>,
            MakeWindowAssociation::<Impl, IMPL_OFFSET>,
            GetWindowAssociation::<Impl, IMPL_OFFSET>,
            CreateSwapChain::<Impl, IMPL_OFFSET>,
            CreateSoftwareAdapter::<Impl, IMPL_OFFSET>,
            EnumAdapters1::<Impl, IMPL_OFFSET>,
            IsCurrent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory2Impl: Sized + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn IsWindowedStereoEnabled();
    fn CreateSwapChainForHwnd();
    fn CreateSwapChainForCoreWindow();
    fn GetSharedResourceAdapterLuid();
    fn RegisterStereoStatusWindow();
    fn RegisterStereoStatusEvent();
    fn UnregisterStereoStatus();
    fn RegisterOcclusionStatusWindow();
    fn RegisterOcclusionStatusEvent();
    fn UnregisterOcclusionStatus();
    fn CreateSwapChainForComposition();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory2Vtbl {
        unsafe extern "system" fn IsWindowedStereoEnabled<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterStereoStatus<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumAdapters::<Impl, IMPL_OFFSET>,
            MakeWindowAssociation::<Impl, IMPL_OFFSET>,
            GetWindowAssociation::<Impl, IMPL_OFFSET>,
            CreateSwapChain::<Impl, IMPL_OFFSET>,
            CreateSoftwareAdapter::<Impl, IMPL_OFFSET>,
            EnumAdapters1::<Impl, IMPL_OFFSET>,
            IsCurrent::<Impl, IMPL_OFFSET>,
            IsWindowedStereoEnabled::<Impl, IMPL_OFFSET>,
            CreateSwapChainForHwnd::<Impl, IMPL_OFFSET>,
            CreateSwapChainForCoreWindow::<Impl, IMPL_OFFSET>,
            GetSharedResourceAdapterLuid::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterStereoStatus::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterOcclusionStatus::<Impl, IMPL_OFFSET>,
            CreateSwapChainForComposition::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory3Impl: Sized + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn GetCreationFlags();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory3Vtbl {
        unsafe extern "system" fn GetCreationFlags<Impl: IDXGIFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumAdapters::<Impl, IMPL_OFFSET>,
            MakeWindowAssociation::<Impl, IMPL_OFFSET>,
            GetWindowAssociation::<Impl, IMPL_OFFSET>,
            CreateSwapChain::<Impl, IMPL_OFFSET>,
            CreateSoftwareAdapter::<Impl, IMPL_OFFSET>,
            EnumAdapters1::<Impl, IMPL_OFFSET>,
            IsCurrent::<Impl, IMPL_OFFSET>,
            IsWindowedStereoEnabled::<Impl, IMPL_OFFSET>,
            CreateSwapChainForHwnd::<Impl, IMPL_OFFSET>,
            CreateSwapChainForCoreWindow::<Impl, IMPL_OFFSET>,
            GetSharedResourceAdapterLuid::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterStereoStatus::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterOcclusionStatus::<Impl, IMPL_OFFSET>,
            CreateSwapChainForComposition::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory4Impl: Sized + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn EnumAdapterByLuid();
    fn EnumWarpAdapter();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory4Vtbl {
        unsafe extern "system" fn EnumAdapterByLuid<Impl: IDXGIFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumWarpAdapter<Impl: IDXGIFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumAdapters::<Impl, IMPL_OFFSET>,
            MakeWindowAssociation::<Impl, IMPL_OFFSET>,
            GetWindowAssociation::<Impl, IMPL_OFFSET>,
            CreateSwapChain::<Impl, IMPL_OFFSET>,
            CreateSoftwareAdapter::<Impl, IMPL_OFFSET>,
            EnumAdapters1::<Impl, IMPL_OFFSET>,
            IsCurrent::<Impl, IMPL_OFFSET>,
            IsWindowedStereoEnabled::<Impl, IMPL_OFFSET>,
            CreateSwapChainForHwnd::<Impl, IMPL_OFFSET>,
            CreateSwapChainForCoreWindow::<Impl, IMPL_OFFSET>,
            GetSharedResourceAdapterLuid::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterStereoStatus::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterOcclusionStatus::<Impl, IMPL_OFFSET>,
            CreateSwapChainForComposition::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            EnumAdapterByLuid::<Impl, IMPL_OFFSET>,
            EnumWarpAdapter::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory5Impl: Sized + IDXGIFactory4Impl + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn CheckFeatureSupport();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory5Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: IDXGIFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumAdapters::<Impl, IMPL_OFFSET>,
            MakeWindowAssociation::<Impl, IMPL_OFFSET>,
            GetWindowAssociation::<Impl, IMPL_OFFSET>,
            CreateSwapChain::<Impl, IMPL_OFFSET>,
            CreateSoftwareAdapter::<Impl, IMPL_OFFSET>,
            EnumAdapters1::<Impl, IMPL_OFFSET>,
            IsCurrent::<Impl, IMPL_OFFSET>,
            IsWindowedStereoEnabled::<Impl, IMPL_OFFSET>,
            CreateSwapChainForHwnd::<Impl, IMPL_OFFSET>,
            CreateSwapChainForCoreWindow::<Impl, IMPL_OFFSET>,
            GetSharedResourceAdapterLuid::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterStereoStatus::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterOcclusionStatus::<Impl, IMPL_OFFSET>,
            CreateSwapChainForComposition::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            EnumAdapterByLuid::<Impl, IMPL_OFFSET>,
            EnumWarpAdapter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory6Impl: Sized + IDXGIFactory5Impl + IDXGIFactory4Impl + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn EnumAdapterByGpuPreference();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory6Vtbl {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Impl: IDXGIFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumAdapters::<Impl, IMPL_OFFSET>,
            MakeWindowAssociation::<Impl, IMPL_OFFSET>,
            GetWindowAssociation::<Impl, IMPL_OFFSET>,
            CreateSwapChain::<Impl, IMPL_OFFSET>,
            CreateSoftwareAdapter::<Impl, IMPL_OFFSET>,
            EnumAdapters1::<Impl, IMPL_OFFSET>,
            IsCurrent::<Impl, IMPL_OFFSET>,
            IsWindowedStereoEnabled::<Impl, IMPL_OFFSET>,
            CreateSwapChainForHwnd::<Impl, IMPL_OFFSET>,
            CreateSwapChainForCoreWindow::<Impl, IMPL_OFFSET>,
            GetSharedResourceAdapterLuid::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterStereoStatus::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterOcclusionStatus::<Impl, IMPL_OFFSET>,
            CreateSwapChainForComposition::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            EnumAdapterByLuid::<Impl, IMPL_OFFSET>,
            EnumWarpAdapter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            EnumAdapterByGpuPreference::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory7Impl: Sized + IDXGIFactory6Impl + IDXGIFactory5Impl + IDXGIFactory4Impl + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn RegisterAdaptersChangedEvent();
    fn UnregisterAdaptersChangedEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory7Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactory7Vtbl {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Impl: IDXGIFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Impl: IDXGIFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            EnumAdapters::<Impl, IMPL_OFFSET>,
            MakeWindowAssociation::<Impl, IMPL_OFFSET>,
            GetWindowAssociation::<Impl, IMPL_OFFSET>,
            CreateSwapChain::<Impl, IMPL_OFFSET>,
            CreateSoftwareAdapter::<Impl, IMPL_OFFSET>,
            EnumAdapters1::<Impl, IMPL_OFFSET>,
            IsCurrent::<Impl, IMPL_OFFSET>,
            IsWindowedStereoEnabled::<Impl, IMPL_OFFSET>,
            CreateSwapChainForHwnd::<Impl, IMPL_OFFSET>,
            CreateSwapChainForCoreWindow::<Impl, IMPL_OFFSET>,
            GetSharedResourceAdapterLuid::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterStereoStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterStereoStatus::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusWindow::<Impl, IMPL_OFFSET>,
            RegisterOcclusionStatusEvent::<Impl, IMPL_OFFSET>,
            UnregisterOcclusionStatus::<Impl, IMPL_OFFSET>,
            CreateSwapChainForComposition::<Impl, IMPL_OFFSET>,
            GetCreationFlags::<Impl, IMPL_OFFSET>,
            EnumAdapterByLuid::<Impl, IMPL_OFFSET>,
            EnumWarpAdapter::<Impl, IMPL_OFFSET>,
            CheckFeatureSupport::<Impl, IMPL_OFFSET>,
            EnumAdapterByGpuPreference::<Impl, IMPL_OFFSET>,
            RegisterAdaptersChangedEvent::<Impl, IMPL_OFFSET>,
            UnregisterAdaptersChangedEvent::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory7 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactoryMediaImpl: Sized {
    fn CreateSwapChainForCompositionSurfaceHandle();
    fn CreateDecodeSwapChainForCompositionSurfaceHandle();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactoryMediaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactoryMediaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIFactoryMediaVtbl {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Impl: IDXGIFactoryMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Impl: IDXGIFactoryMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: ::windows::core::RawPtr, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateSwapChainForCompositionSurfaceHandle::<Impl, IMPL_OFFSET>, CreateDecodeSwapChainForCompositionSurfaceHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactoryMedia as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIInfoQueueImpl: Sized {
    fn SetMessageCountLimit();
    fn ClearStoredMessages();
    fn GetMessage();
    fn GetNumStoredMessagesAllowedByRetrievalFilters();
    fn GetNumStoredMessages();
    fn GetNumMessagesDiscardedByMessageCountLimit();
    fn GetMessageCountLimit();
    fn GetNumMessagesAllowedByStorageFilter();
    fn GetNumMessagesDeniedByStorageFilter();
    fn AddStorageFilterEntries();
    fn GetStorageFilter();
    fn ClearStorageFilter();
    fn PushEmptyStorageFilter();
    fn PushDenyAllStorageFilter();
    fn PushCopyOfStorageFilter();
    fn PushStorageFilter();
    fn PopStorageFilter();
    fn GetStorageFilterStackSize();
    fn AddRetrievalFilterEntries();
    fn GetRetrievalFilter();
    fn ClearRetrievalFilter();
    fn PushEmptyRetrievalFilter();
    fn PushDenyAllRetrievalFilter();
    fn PushCopyOfRetrievalFilter();
    fn PushRetrievalFilter();
    fn PopRetrievalFilter();
    fn GetRetrievalFilterStackSize();
    fn AddMessage();
    fn AddApplicationMessage();
    fn SetBreakOnCategory();
    fn SetBreakOnSeverity();
    fn SetBreakOnID();
    fn GetBreakOnCategory();
    fn GetBreakOnSeverity();
    fn GetBreakOnID();
    fn SetMuteDebugOutput();
    fn GetMuteDebugOutput();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIInfoQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueueImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIInfoQueueVtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMessage<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddMessage<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBreakOnID<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBreakOnID<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetMessageCountLimit::<Impl, IMPL_OFFSET>,
            ClearStoredMessages::<Impl, IMPL_OFFSET>,
            GetMessage::<Impl, IMPL_OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters::<Impl, IMPL_OFFSET>,
            GetNumStoredMessages::<Impl, IMPL_OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit::<Impl, IMPL_OFFSET>,
            GetMessageCountLimit::<Impl, IMPL_OFFSET>,
            GetNumMessagesAllowedByStorageFilter::<Impl, IMPL_OFFSET>,
            GetNumMessagesDeniedByStorageFilter::<Impl, IMPL_OFFSET>,
            AddStorageFilterEntries::<Impl, IMPL_OFFSET>,
            GetStorageFilter::<Impl, IMPL_OFFSET>,
            ClearStorageFilter::<Impl, IMPL_OFFSET>,
            PushEmptyStorageFilter::<Impl, IMPL_OFFSET>,
            PushDenyAllStorageFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfStorageFilter::<Impl, IMPL_OFFSET>,
            PushStorageFilter::<Impl, IMPL_OFFSET>,
            PopStorageFilter::<Impl, IMPL_OFFSET>,
            GetStorageFilterStackSize::<Impl, IMPL_OFFSET>,
            AddRetrievalFilterEntries::<Impl, IMPL_OFFSET>,
            GetRetrievalFilter::<Impl, IMPL_OFFSET>,
            ClearRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushEmptyRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushDenyAllRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushCopyOfRetrievalFilter::<Impl, IMPL_OFFSET>,
            PushRetrievalFilter::<Impl, IMPL_OFFSET>,
            PopRetrievalFilter::<Impl, IMPL_OFFSET>,
            GetRetrievalFilterStackSize::<Impl, IMPL_OFFSET>,
            AddMessage::<Impl, IMPL_OFFSET>,
            AddApplicationMessage::<Impl, IMPL_OFFSET>,
            SetBreakOnCategory::<Impl, IMPL_OFFSET>,
            SetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            SetBreakOnID::<Impl, IMPL_OFFSET>,
            GetBreakOnCategory::<Impl, IMPL_OFFSET>,
            GetBreakOnSeverity::<Impl, IMPL_OFFSET>,
            GetBreakOnID::<Impl, IMPL_OFFSET>,
            SetMuteDebugOutput::<Impl, IMPL_OFFSET>,
            GetMuteDebugOutput::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIInfoQueue as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIKeyedMutexImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn AcquireSync();
    fn ReleaseSync();
}
impl IDXGIKeyedMutexVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIKeyedMutexImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIKeyedMutexVtbl {
        unsafe extern "system" fn AcquireSync<Impl: IDXGIKeyedMutexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseSync<Impl: IDXGIKeyedMutexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, GetParent::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, AcquireSync::<Impl, IMPL_OFFSET>, ReleaseSync::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIKeyedMutex as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIObjectImpl: Sized {
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn GetPrivateData();
    fn GetParent();
}
impl IDXGIObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIObjectVtbl {
        unsafe extern "system" fn SetPrivateData<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParent<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, GetParent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutputImpl: Sized + IDXGIObjectImpl {
    fn GetDesc();
    fn GetDisplayModeList();
    fn FindClosestMatchingMode();
    fn WaitForVBlank();
    fn TakeOwnership();
    fn ReleaseOwnership();
    fn GetGammaControlCapabilities();
    fn SetGammaControl();
    fn GetGammaControl();
    fn SetDisplaySurface();
    fn GetDisplaySurfaceData();
    fn GetFrameStatistics();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutputVtbl {
        unsafe extern "system" fn GetDesc<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayModeList<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindClosestMatchingMode<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitForVBlank<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TakeOwnership<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseOwnership<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetGammaControl<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetGammaControl<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplaySurface<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscanoutsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetDisplayModeList::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode::<Impl, IMPL_OFFSET>,
            WaitForVBlank::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            ReleaseOwnership::<Impl, IMPL_OFFSET>,
            GetGammaControlCapabilities::<Impl, IMPL_OFFSET>,
            SetGammaControl::<Impl, IMPL_OFFSET>,
            GetGammaControl::<Impl, IMPL_OFFSET>,
            SetDisplaySurface::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput1Impl: Sized + IDXGIOutputImpl + IDXGIObjectImpl {
    fn GetDisplayModeList1();
    fn FindClosestMatchingMode1();
    fn GetDisplaySurfaceData1();
    fn DuplicateOutput();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput1Vtbl {
        unsafe extern "system" fn GetDisplayModeList1<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DuplicateOutput<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetDisplayModeList::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode::<Impl, IMPL_OFFSET>,
            WaitForVBlank::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            ReleaseOwnership::<Impl, IMPL_OFFSET>,
            GetGammaControlCapabilities::<Impl, IMPL_OFFSET>,
            SetGammaControl::<Impl, IMPL_OFFSET>,
            GetGammaControl::<Impl, IMPL_OFFSET>,
            SetDisplaySurface::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetDisplayModeList1::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode1::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData1::<Impl, IMPL_OFFSET>,
            DuplicateOutput::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput2Impl: Sized + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn SupportsOverlays();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput2Vtbl {
        unsafe extern "system" fn SupportsOverlays<Impl: IDXGIOutput2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetDisplayModeList::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode::<Impl, IMPL_OFFSET>,
            WaitForVBlank::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            ReleaseOwnership::<Impl, IMPL_OFFSET>,
            GetGammaControlCapabilities::<Impl, IMPL_OFFSET>,
            SetGammaControl::<Impl, IMPL_OFFSET>,
            GetGammaControl::<Impl, IMPL_OFFSET>,
            SetDisplaySurface::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetDisplayModeList1::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode1::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData1::<Impl, IMPL_OFFSET>,
            DuplicateOutput::<Impl, IMPL_OFFSET>,
            SupportsOverlays::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput3Impl: Sized + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn CheckOverlaySupport();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput3Vtbl {
        unsafe extern "system" fn CheckOverlaySupport<Impl: IDXGIOutput3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetDisplayModeList::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode::<Impl, IMPL_OFFSET>,
            WaitForVBlank::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            ReleaseOwnership::<Impl, IMPL_OFFSET>,
            GetGammaControlCapabilities::<Impl, IMPL_OFFSET>,
            SetGammaControl::<Impl, IMPL_OFFSET>,
            GetGammaControl::<Impl, IMPL_OFFSET>,
            SetDisplaySurface::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetDisplayModeList1::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode1::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData1::<Impl, IMPL_OFFSET>,
            DuplicateOutput::<Impl, IMPL_OFFSET>,
            SupportsOverlays::<Impl, IMPL_OFFSET>,
            CheckOverlaySupport::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput4Impl: Sized + IDXGIOutput3Impl + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn CheckOverlayColorSpaceSupport();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput4Vtbl {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Impl: IDXGIOutput4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetDisplayModeList::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode::<Impl, IMPL_OFFSET>,
            WaitForVBlank::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            ReleaseOwnership::<Impl, IMPL_OFFSET>,
            GetGammaControlCapabilities::<Impl, IMPL_OFFSET>,
            SetGammaControl::<Impl, IMPL_OFFSET>,
            GetGammaControl::<Impl, IMPL_OFFSET>,
            SetDisplaySurface::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetDisplayModeList1::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode1::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData1::<Impl, IMPL_OFFSET>,
            DuplicateOutput::<Impl, IMPL_OFFSET>,
            SupportsOverlays::<Impl, IMPL_OFFSET>,
            CheckOverlaySupport::<Impl, IMPL_OFFSET>,
            CheckOverlayColorSpaceSupport::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput5Impl: Sized + IDXGIOutput4Impl + IDXGIOutput3Impl + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn DuplicateOutput1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput5Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput5Vtbl {
        unsafe extern "system" fn DuplicateOutput1<Impl: IDXGIOutput5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetDisplayModeList::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode::<Impl, IMPL_OFFSET>,
            WaitForVBlank::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            ReleaseOwnership::<Impl, IMPL_OFFSET>,
            GetGammaControlCapabilities::<Impl, IMPL_OFFSET>,
            SetGammaControl::<Impl, IMPL_OFFSET>,
            GetGammaControl::<Impl, IMPL_OFFSET>,
            SetDisplaySurface::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetDisplayModeList1::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode1::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData1::<Impl, IMPL_OFFSET>,
            DuplicateOutput::<Impl, IMPL_OFFSET>,
            SupportsOverlays::<Impl, IMPL_OFFSET>,
            CheckOverlaySupport::<Impl, IMPL_OFFSET>,
            CheckOverlayColorSpaceSupport::<Impl, IMPL_OFFSET>,
            DuplicateOutput1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput6Impl: Sized + IDXGIOutput5Impl + IDXGIOutput4Impl + IDXGIOutput3Impl + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn GetDesc1();
    fn CheckHardwareCompositionSupport();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput6Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutput6Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: IDXGIOutput6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Impl: IDXGIOutput6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            GetDisplayModeList::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode::<Impl, IMPL_OFFSET>,
            WaitForVBlank::<Impl, IMPL_OFFSET>,
            TakeOwnership::<Impl, IMPL_OFFSET>,
            ReleaseOwnership::<Impl, IMPL_OFFSET>,
            GetGammaControlCapabilities::<Impl, IMPL_OFFSET>,
            SetGammaControl::<Impl, IMPL_OFFSET>,
            GetGammaControl::<Impl, IMPL_OFFSET>,
            SetDisplaySurface::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetDisplayModeList1::<Impl, IMPL_OFFSET>,
            FindClosestMatchingMode1::<Impl, IMPL_OFFSET>,
            GetDisplaySurfaceData1::<Impl, IMPL_OFFSET>,
            DuplicateOutput::<Impl, IMPL_OFFSET>,
            SupportsOverlays::<Impl, IMPL_OFFSET>,
            CheckOverlaySupport::<Impl, IMPL_OFFSET>,
            CheckOverlayColorSpaceSupport::<Impl, IMPL_OFFSET>,
            DuplicateOutput1::<Impl, IMPL_OFFSET>,
            GetDesc1::<Impl, IMPL_OFFSET>,
            CheckHardwareCompositionSupport::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIOutputDuplicationImpl: Sized + IDXGIObjectImpl {
    fn GetDesc();
    fn AcquireNextFrame();
    fn GetFrameDirtyRects();
    fn GetFrameMoveRects();
    fn GetFramePointerShape();
    fn MapDesktopSurface();
    fn UnMapDesktopSurface();
    fn ReleaseFrame();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIOutputDuplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplicationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIOutputDuplicationVtbl {
        unsafe extern "system" fn GetDesc<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AcquireNextFrame<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrameDirtyRects<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrameMoveRects<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFramePointerShape<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MapDesktopSurface<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnMapDesktopSurface<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseFrame<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            AcquireNextFrame::<Impl, IMPL_OFFSET>,
            GetFrameDirtyRects::<Impl, IMPL_OFFSET>,
            GetFrameMoveRects::<Impl, IMPL_OFFSET>,
            GetFramePointerShape::<Impl, IMPL_OFFSET>,
            MapDesktopSurface::<Impl, IMPL_OFFSET>,
            UnMapDesktopSurface::<Impl, IMPL_OFFSET>,
            ReleaseFrame::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutputDuplication as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIResourceImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetSharedHandle();
    fn GetUsage();
    fn SetEvictionPriority();
    fn GetEvictionPriority();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResourceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIResourceVtbl {
        unsafe extern "system" fn GetSharedHandle<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUsage<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEvictionPriority<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEvictionPriority<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevictionpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetSharedHandle::<Impl, IMPL_OFFSET>,
            GetUsage::<Impl, IMPL_OFFSET>,
            SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIResource as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait IDXGIResource1Impl: Sized + IDXGIResourceImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn CreateSubresourceSurface();
    fn CreateSharedHandle();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IDXGIResource1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGIResource1Vtbl {
        unsafe extern "system" fn CreateSubresourceSurface<Impl: IDXGIResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateSharedHandle<Impl: IDXGIResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetSharedHandle::<Impl, IMPL_OFFSET>,
            GetUsage::<Impl, IMPL_OFFSET>,
            SetEvictionPriority::<Impl, IMPL_OFFSET>,
            GetEvictionPriority::<Impl, IMPL_OFFSET>,
            CreateSubresourceSurface::<Impl, IMPL_OFFSET>,
            CreateSharedHandle::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIResource1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISurfaceImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetDesc();
    fn Map();
    fn Unmap();
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISurfaceVtbl {
        unsafe extern "system" fn GetDesc<Impl: IDXGISurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Map<Impl: IDXGISurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unmap<Impl: IDXGISurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetPrivateData::<Impl, IMPL_OFFSET>, SetPrivateDataInterface::<Impl, IMPL_OFFSET>, GetPrivateData::<Impl, IMPL_OFFSET>, GetParent::<Impl, IMPL_OFFSET>, GetDevice::<Impl, IMPL_OFFSET>, GetDesc::<Impl, IMPL_OFFSET>, Map::<Impl, IMPL_OFFSET>, Unmap::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface1Impl: Sized + IDXGISurfaceImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetDC();
    fn ReleaseDC();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISurface1Vtbl {
        unsafe extern "system" fn GetDC<Impl: IDXGISurface1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDXGISurface1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            GetDC::<Impl, IMPL_OFFSET>,
            ReleaseDC::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface2Impl: Sized + IDXGISurface1Impl + IDXGISurfaceImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetResource();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISurface2Vtbl {
        unsafe extern "system" fn GetResource<Impl: IDXGISurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            Map::<Impl, IMPL_OFFSET>,
            Unmap::<Impl, IMPL_OFFSET>,
            GetDC::<Impl, IMPL_OFFSET>,
            ReleaseDC::<Impl, IMPL_OFFSET>,
            GetResource::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChainImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn Present();
    fn GetBuffer();
    fn SetFullscreenState();
    fn GetFullscreenState();
    fn GetDesc();
    fn ResizeBuffers();
    fn ResizeTarget();
    fn GetContainingOutput();
    fn GetFrameStatistics();
    fn GetLastPresentCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChainVtbl {
        unsafe extern "system" fn Present<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBuffer<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: u32, riid: *const ::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFullscreenState<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFullscreenState<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDesc<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResizeBuffers<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResizeTarget<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainingOutput<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLastPresentCount<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            Present::<Impl, IMPL_OFFSET>,
            GetBuffer::<Impl, IMPL_OFFSET>,
            SetFullscreenState::<Impl, IMPL_OFFSET>,
            GetFullscreenState::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            ResizeBuffers::<Impl, IMPL_OFFSET>,
            ResizeTarget::<Impl, IMPL_OFFSET>,
            GetContainingOutput::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetLastPresentCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain1Impl: Sized + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetDesc1();
    fn GetFullscreenDesc();
    fn GetHwnd();
    fn GetCoreWindow();
    fn Present1();
    fn IsTemporaryMonoSupported();
    fn GetRestrictToOutput();
    fn SetBackgroundColor();
    fn GetBackgroundColor();
    fn SetRotation();
    fn GetRotation();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChain1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFullscreenDesc<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHwnd<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCoreWindow<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Present1<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRestrictToOutput<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprestricttooutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *const DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBackgroundColor<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRotation<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRotation<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            Present::<Impl, IMPL_OFFSET>,
            GetBuffer::<Impl, IMPL_OFFSET>,
            SetFullscreenState::<Impl, IMPL_OFFSET>,
            GetFullscreenState::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            ResizeBuffers::<Impl, IMPL_OFFSET>,
            ResizeTarget::<Impl, IMPL_OFFSET>,
            GetContainingOutput::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetLastPresentCount::<Impl, IMPL_OFFSET>,
            GetDesc1::<Impl, IMPL_OFFSET>,
            GetFullscreenDesc::<Impl, IMPL_OFFSET>,
            GetHwnd::<Impl, IMPL_OFFSET>,
            GetCoreWindow::<Impl, IMPL_OFFSET>,
            Present1::<Impl, IMPL_OFFSET>,
            IsTemporaryMonoSupported::<Impl, IMPL_OFFSET>,
            GetRestrictToOutput::<Impl, IMPL_OFFSET>,
            SetBackgroundColor::<Impl, IMPL_OFFSET>,
            GetBackgroundColor::<Impl, IMPL_OFFSET>,
            SetRotation::<Impl, IMPL_OFFSET>,
            GetRotation::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain2Impl: Sized + IDXGISwapChain1Impl + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn SetSourceSize();
    fn GetSourceSize();
    fn SetMaximumFrameLatency();
    fn GetMaximumFrameLatency();
    fn GetFrameLatencyWaitableObject();
    fn SetMatrixTransform();
    fn GetMatrixTransform();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChain2Vtbl {
        unsafe extern "system" fn SetSourceSize<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSourceSize<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMatrixTransform<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatrixTransform<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            Present::<Impl, IMPL_OFFSET>,
            GetBuffer::<Impl, IMPL_OFFSET>,
            SetFullscreenState::<Impl, IMPL_OFFSET>,
            GetFullscreenState::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            ResizeBuffers::<Impl, IMPL_OFFSET>,
            ResizeTarget::<Impl, IMPL_OFFSET>,
            GetContainingOutput::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetLastPresentCount::<Impl, IMPL_OFFSET>,
            GetDesc1::<Impl, IMPL_OFFSET>,
            GetFullscreenDesc::<Impl, IMPL_OFFSET>,
            GetHwnd::<Impl, IMPL_OFFSET>,
            GetCoreWindow::<Impl, IMPL_OFFSET>,
            Present1::<Impl, IMPL_OFFSET>,
            IsTemporaryMonoSupported::<Impl, IMPL_OFFSET>,
            GetRestrictToOutput::<Impl, IMPL_OFFSET>,
            SetBackgroundColor::<Impl, IMPL_OFFSET>,
            GetBackgroundColor::<Impl, IMPL_OFFSET>,
            SetRotation::<Impl, IMPL_OFFSET>,
            GetRotation::<Impl, IMPL_OFFSET>,
            SetSourceSize::<Impl, IMPL_OFFSET>,
            GetSourceSize::<Impl, IMPL_OFFSET>,
            SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetFrameLatencyWaitableObject::<Impl, IMPL_OFFSET>,
            SetMatrixTransform::<Impl, IMPL_OFFSET>,
            GetMatrixTransform::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain3Impl: Sized + IDXGISwapChain2Impl + IDXGISwapChain1Impl + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetCurrentBackBufferIndex();
    fn CheckColorSpaceSupport();
    fn SetColorSpace1();
    fn ResizeBuffers1();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChain3Vtbl {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetColorSpace1<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResizeBuffers1<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            Present::<Impl, IMPL_OFFSET>,
            GetBuffer::<Impl, IMPL_OFFSET>,
            SetFullscreenState::<Impl, IMPL_OFFSET>,
            GetFullscreenState::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            ResizeBuffers::<Impl, IMPL_OFFSET>,
            ResizeTarget::<Impl, IMPL_OFFSET>,
            GetContainingOutput::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetLastPresentCount::<Impl, IMPL_OFFSET>,
            GetDesc1::<Impl, IMPL_OFFSET>,
            GetFullscreenDesc::<Impl, IMPL_OFFSET>,
            GetHwnd::<Impl, IMPL_OFFSET>,
            GetCoreWindow::<Impl, IMPL_OFFSET>,
            Present1::<Impl, IMPL_OFFSET>,
            IsTemporaryMonoSupported::<Impl, IMPL_OFFSET>,
            GetRestrictToOutput::<Impl, IMPL_OFFSET>,
            SetBackgroundColor::<Impl, IMPL_OFFSET>,
            GetBackgroundColor::<Impl, IMPL_OFFSET>,
            SetRotation::<Impl, IMPL_OFFSET>,
            GetRotation::<Impl, IMPL_OFFSET>,
            SetSourceSize::<Impl, IMPL_OFFSET>,
            GetSourceSize::<Impl, IMPL_OFFSET>,
            SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetFrameLatencyWaitableObject::<Impl, IMPL_OFFSET>,
            SetMatrixTransform::<Impl, IMPL_OFFSET>,
            GetMatrixTransform::<Impl, IMPL_OFFSET>,
            GetCurrentBackBufferIndex::<Impl, IMPL_OFFSET>,
            CheckColorSpaceSupport::<Impl, IMPL_OFFSET>,
            SetColorSpace1::<Impl, IMPL_OFFSET>,
            ResizeBuffers1::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain4Impl: Sized + IDXGISwapChain3Impl + IDXGISwapChain2Impl + IDXGISwapChain1Impl + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn SetHDRMetaData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain4Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChain4Vtbl {
        unsafe extern "system" fn SetHDRMetaData<Impl: IDXGISwapChain4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetPrivateData::<Impl, IMPL_OFFSET>,
            SetPrivateDataInterface::<Impl, IMPL_OFFSET>,
            GetPrivateData::<Impl, IMPL_OFFSET>,
            GetParent::<Impl, IMPL_OFFSET>,
            GetDevice::<Impl, IMPL_OFFSET>,
            Present::<Impl, IMPL_OFFSET>,
            GetBuffer::<Impl, IMPL_OFFSET>,
            SetFullscreenState::<Impl, IMPL_OFFSET>,
            GetFullscreenState::<Impl, IMPL_OFFSET>,
            GetDesc::<Impl, IMPL_OFFSET>,
            ResizeBuffers::<Impl, IMPL_OFFSET>,
            ResizeTarget::<Impl, IMPL_OFFSET>,
            GetContainingOutput::<Impl, IMPL_OFFSET>,
            GetFrameStatistics::<Impl, IMPL_OFFSET>,
            GetLastPresentCount::<Impl, IMPL_OFFSET>,
            GetDesc1::<Impl, IMPL_OFFSET>,
            GetFullscreenDesc::<Impl, IMPL_OFFSET>,
            GetHwnd::<Impl, IMPL_OFFSET>,
            GetCoreWindow::<Impl, IMPL_OFFSET>,
            Present1::<Impl, IMPL_OFFSET>,
            IsTemporaryMonoSupported::<Impl, IMPL_OFFSET>,
            GetRestrictToOutput::<Impl, IMPL_OFFSET>,
            SetBackgroundColor::<Impl, IMPL_OFFSET>,
            GetBackgroundColor::<Impl, IMPL_OFFSET>,
            SetRotation::<Impl, IMPL_OFFSET>,
            GetRotation::<Impl, IMPL_OFFSET>,
            SetSourceSize::<Impl, IMPL_OFFSET>,
            GetSourceSize::<Impl, IMPL_OFFSET>,
            SetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetMaximumFrameLatency::<Impl, IMPL_OFFSET>,
            GetFrameLatencyWaitableObject::<Impl, IMPL_OFFSET>,
            SetMatrixTransform::<Impl, IMPL_OFFSET>,
            GetMatrixTransform::<Impl, IMPL_OFFSET>,
            GetCurrentBackBufferIndex::<Impl, IMPL_OFFSET>,
            CheckColorSpaceSupport::<Impl, IMPL_OFFSET>,
            SetColorSpace1::<Impl, IMPL_OFFSET>,
            ResizeBuffers1::<Impl, IMPL_OFFSET>,
            SetHDRMetaData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain4 as ::windows::core::Interface>::IID
    }
}
pub trait IDXGISwapChainMediaImpl: Sized {
    fn GetFrameStatisticsMedia();
    fn SetPresentDuration();
    fn CheckPresentDurationSupport();
}
impl IDXGISwapChainMediaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainMediaImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGISwapChainMediaVtbl {
        unsafe extern "system" fn GetFrameStatisticsMedia<Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPresentDuration<Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetFrameStatisticsMedia::<Impl, IMPL_OFFSET>, SetPresentDuration::<Impl, IMPL_OFFSET>, CheckPresentDurationSupport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChainMedia as ::windows::core::Interface>::IID
    }
}
pub trait IDXGraphicsAnalysisImpl: Sized {
    fn BeginCapture();
    fn EndCapture();
}
impl IDXGraphicsAnalysisVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGraphicsAnalysisImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXGraphicsAnalysisVtbl {
        unsafe extern "system" fn BeginCapture<Impl: IDXGraphicsAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndCapture<Impl: IDXGraphicsAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, BeginCapture::<Impl, IMPL_OFFSET>, EndCapture::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGraphicsAnalysis as ::windows::core::Interface>::IID
    }
}
