pub trait IDXGIAdapterImpl: Sized + IDXGIObjectImpl {
    fn EnumOutputs();
    fn GetDesc();
    fn CheckInterfaceSupport();
}
impl ::windows::core::RuntimeName for IDXGIAdapter {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIAdapter";
}
impl IDXGIAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapterImpl, const OFFSET: isize>() -> IDXGIAdapterVtbl {
        unsafe extern "system" fn EnumOutputs<Impl: IDXGIAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumOutputs(output, ::core::mem::transmute_copy(&ppoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: IDXGIAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckInterfaceSupport<Impl: IDXGIAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacename: &::windows::core::GUID, pumdversion: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckInterfaceSupport(&*(&interfacename as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pumdversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIAdapter>, ::windows::core::GetTrustLevel, EnumOutputs::<Impl, OFFSET>, GetDesc::<Impl, OFFSET>, CheckInterfaceSupport::<Impl, OFFSET>)
    }
}
pub trait IDXGIAdapter1Impl: Sized + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn GetDesc1();
}
impl ::windows::core::RuntimeName for IDXGIAdapter1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIAdapter1";
}
impl IDXGIAdapter1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter1Impl, const OFFSET: isize>() -> IDXGIAdapter1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: IDXGIAdapter1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc1(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIAdapter1>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>)
    }
}
pub trait IDXGIAdapter2Impl: Sized + IDXGIAdapter1Impl + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn GetDesc2();
}
impl ::windows::core::RuntimeName for IDXGIAdapter2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIAdapter2";
}
impl IDXGIAdapter2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter2Impl, const OFFSET: isize>() -> IDXGIAdapter2Vtbl {
        unsafe extern "system" fn GetDesc2<Impl: IDXGIAdapter2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc2(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIAdapter2>, ::windows::core::GetTrustLevel, GetDesc2::<Impl, OFFSET>)
    }
}
pub trait IDXGIAdapter3Impl: Sized + IDXGIAdapter2Impl + IDXGIAdapter1Impl + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent();
    fn UnregisterHardwareContentProtectionTeardownStatus();
    fn QueryVideoMemoryInfo();
    fn SetVideoMemoryReservation();
    fn RegisterVideoMemoryBudgetChangeNotificationEvent();
    fn UnregisterVideoMemoryBudgetChangeNotification();
}
impl ::windows::core::RuntimeName for IDXGIAdapter3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIAdapter3";
}
impl IDXGIAdapter3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3Impl, const OFFSET: isize>() -> IDXGIAdapter3Vtbl {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterHardwareContentProtectionTeardownStatusEvent(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterHardwareContentProtectionTeardownStatus(dwcookie).into()
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryVideoMemoryInfo(nodeindex, memorysegmentgroup, ::core::mem::transmute_copy(&pvideomemoryinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVideoMemoryReservation(nodeindex, memorysegmentgroup, reservation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterVideoMemoryBudgetChangeNotificationEvent(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Impl: IDXGIAdapter3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterVideoMemoryBudgetChangeNotification(dwcookie).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXGIAdapter3>,
            ::windows::core::GetTrustLevel,
            RegisterHardwareContentProtectionTeardownStatusEvent::<Impl, OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus::<Impl, OFFSET>,
            QueryVideoMemoryInfo::<Impl, OFFSET>,
            SetVideoMemoryReservation::<Impl, OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent::<Impl, OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification::<Impl, OFFSET>,
        )
    }
}
pub trait IDXGIAdapter4Impl: Sized + IDXGIAdapter3Impl + IDXGIAdapter2Impl + IDXGIAdapter1Impl + IDXGIAdapterImpl + IDXGIObjectImpl {
    fn GetDesc3();
}
impl ::windows::core::RuntimeName for IDXGIAdapter4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIAdapter4";
}
impl IDXGIAdapter4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter4Impl, const OFFSET: isize>() -> IDXGIAdapter4Vtbl {
        unsafe extern "system" fn GetDesc3<Impl: IDXGIAdapter4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc3(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIAdapter4>, ::windows::core::GetTrustLevel, GetDesc3::<Impl, OFFSET>)
    }
}
pub trait IDXGIDebugImpl: Sized {
    fn ReportLiveObjects();
}
impl ::windows::core::RuntimeName for IDXGIDebug {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDebug";
}
impl IDXGIDebugVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebugImpl, const OFFSET: isize>() -> IDXGIDebugVtbl {
        unsafe extern "system" fn ReportLiveObjects<Impl: IDXGIDebugImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReportLiveObjects(&*(&apiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIDebug>, ::windows::core::GetTrustLevel, ReportLiveObjects::<Impl, OFFSET>)
    }
}
pub trait IDXGIDebug1Impl: Sized + IDXGIDebugImpl {
    fn EnableLeakTrackingForThread();
    fn DisableLeakTrackingForThread();
    fn IsLeakTrackingEnabledForThread();
}
impl ::windows::core::RuntimeName for IDXGIDebug1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDebug1";
}
impl IDXGIDebug1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebug1Impl, const OFFSET: isize>() -> IDXGIDebug1Vtbl {
        unsafe extern "system" fn EnableLeakTrackingForThread<Impl: IDXGIDebug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableLeakTrackingForThread().into()
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Impl: IDXGIDebug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableLeakTrackingForThread().into()
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Impl: IDXGIDebug1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsLeakTrackingEnabledForThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIDebug1>, ::windows::core::GetTrustLevel, EnableLeakTrackingForThread::<Impl, OFFSET>, DisableLeakTrackingForThread::<Impl, OFFSET>, IsLeakTrackingEnabledForThread::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDXGIDecodeSwapChain {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDecodeSwapChain";
}
impl IDXGIDecodeSwapChainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>() -> IDXGIDecodeSwapChainVtbl {
        unsafe extern "system" fn PresentBuffer<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PresentBuffer(buffertopresent, syncinterval, flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourceRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourceRect(&*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTargetRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTargetRect(&*(&prect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestSize<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDestSize(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceRect(::core::mem::transmute_copy(&prect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetRect<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTargetRect(::core::mem::transmute_copy(&prect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestSize<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDestSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorSpace<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColorSpace(colorspace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColorSpace<Impl: IDXGIDecodeSwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColorSpace() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXGIDecodeSwapChain>,
            ::windows::core::GetTrustLevel,
            PresentBuffer::<Impl, OFFSET>,
            SetSourceRect::<Impl, OFFSET>,
            SetTargetRect::<Impl, OFFSET>,
            SetDestSize::<Impl, OFFSET>,
            GetSourceRect::<Impl, OFFSET>,
            GetTargetRect::<Impl, OFFSET>,
            GetDestSize::<Impl, OFFSET>,
            SetColorSpace::<Impl, OFFSET>,
            GetColorSpace::<Impl, OFFSET>,
        )
    }
}
pub trait IDXGIDeviceImpl: Sized + IDXGIObjectImpl {
    fn GetAdapter();
    fn CreateSurface();
    fn QueryResourceResidency();
    fn SetGPUThreadPriority();
    fn GetGPUThreadPriority();
}
impl ::windows::core::RuntimeName for IDXGIDevice {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDevice";
}
impl IDXGIDeviceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDeviceImpl, const OFFSET: isize>() -> IDXGIDeviceVtbl {
        unsafe extern "system" fn GetAdapter<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdapter(::core::mem::transmute_copy(&padapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSurface(&*(&pdesc as *const <DXGI_SURFACE_DESC as ::windows::core::Abi>::Abi as *const <DXGI_SURFACE_DESC as ::windows::core::DefaultType>::DefaultType), numsurfaces, usage, &*(&psharedresource as *const <DXGI_SHARED_RESOURCE as ::windows::core::Abi>::Abi as *const <DXGI_SHARED_RESOURCE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryResourceResidency<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryResourceResidency(&*(&ppresources as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presidencystatus), numresources) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGPUThreadPriority<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGPUThreadPriority(priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGPUThreadPriority<Impl: IDXGIDeviceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGPUThreadPriority(::core::mem::transmute_copy(&ppriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIDevice>, ::windows::core::GetTrustLevel, GetAdapter::<Impl, OFFSET>, CreateSurface::<Impl, OFFSET>, QueryResourceResidency::<Impl, OFFSET>, SetGPUThreadPriority::<Impl, OFFSET>, GetGPUThreadPriority::<Impl, OFFSET>)
    }
}
pub trait IDXGIDevice1Impl: Sized + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn SetMaximumFrameLatency();
    fn GetMaximumFrameLatency();
}
impl ::windows::core::RuntimeName for IDXGIDevice1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDevice1";
}
impl IDXGIDevice1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice1Impl, const OFFSET: isize>() -> IDXGIDevice1Vtbl {
        unsafe extern "system" fn SetMaximumFrameLatency<Impl: IDXGIDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaximumFrameLatency(maxlatency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Impl: IDXGIDevice1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumFrameLatency(::core::mem::transmute_copy(&pmaxlatency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIDevice1>, ::windows::core::GetTrustLevel, SetMaximumFrameLatency::<Impl, OFFSET>, GetMaximumFrameLatency::<Impl, OFFSET>)
    }
}
pub trait IDXGIDevice2Impl: Sized + IDXGIDevice1Impl + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn OfferResources();
    fn ReclaimResources();
    fn EnqueueSetEvent();
}
impl ::windows::core::RuntimeName for IDXGIDevice2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDevice2";
}
impl IDXGIDevice2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice2Impl, const OFFSET: isize>() -> IDXGIDevice2Vtbl {
        unsafe extern "system" fn OfferResources<Impl: IDXGIDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfferResources(numresources, &*(&ppresources as *const <IDXGIResource as ::windows::core::Abi>::Abi as *const <IDXGIResource as ::windows::core::DefaultType>::DefaultType), priority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReclaimResources<Impl: IDXGIDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReclaimResources(numresources, &*(&ppresources as *const <IDXGIResource as ::windows::core::Abi>::Abi as *const <IDXGIResource as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdiscarded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnqueueSetEvent<Impl: IDXGIDevice2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnqueueSetEvent(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIDevice2>, ::windows::core::GetTrustLevel, OfferResources::<Impl, OFFSET>, ReclaimResources::<Impl, OFFSET>, EnqueueSetEvent::<Impl, OFFSET>)
    }
}
pub trait IDXGIDevice3Impl: Sized + IDXGIDevice2Impl + IDXGIDevice1Impl + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn Trim();
}
impl ::windows::core::RuntimeName for IDXGIDevice3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDevice3";
}
impl IDXGIDevice3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice3Impl, const OFFSET: isize>() -> IDXGIDevice3Vtbl {
        unsafe extern "system" fn Trim<Impl: IDXGIDevice3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Trim().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIDevice3>, ::windows::core::GetTrustLevel, Trim::<Impl, OFFSET>)
    }
}
pub trait IDXGIDevice4Impl: Sized + IDXGIDevice3Impl + IDXGIDevice2Impl + IDXGIDevice1Impl + IDXGIDeviceImpl + IDXGIObjectImpl {
    fn OfferResources1();
    fn ReclaimResources1();
}
impl ::windows::core::RuntimeName for IDXGIDevice4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDevice4";
}
impl IDXGIDevice4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice4Impl, const OFFSET: isize>() -> IDXGIDevice4Vtbl {
        unsafe extern "system" fn OfferResources1<Impl: IDXGIDevice4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OfferResources1(numresources, &*(&ppresources as *const <IDXGIResource as ::windows::core::Abi>::Abi as *const <IDXGIResource as ::windows::core::DefaultType>::DefaultType), priority, flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReclaimResources1<Impl: IDXGIDevice4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReclaimResources1(numresources, &*(&ppresources as *const <IDXGIResource as ::windows::core::Abi>::Abi as *const <IDXGIResource as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&presults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIDevice4>, ::windows::core::GetTrustLevel, OfferResources1::<Impl, OFFSET>, ReclaimResources1::<Impl, OFFSET>)
    }
}
pub trait IDXGIDeviceSubObjectImpl: Sized + IDXGIObjectImpl {
    fn GetDevice();
}
impl ::windows::core::RuntimeName for IDXGIDeviceSubObject {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDeviceSubObject";
}
impl IDXGIDeviceSubObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDeviceSubObjectImpl, const OFFSET: isize>() -> IDXGIDeviceSubObjectVtbl {
        unsafe extern "system" fn GetDevice<Impl: IDXGIDeviceSubObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevice(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIDeviceSubObject>, ::windows::core::GetTrustLevel, GetDevice::<Impl, OFFSET>)
    }
}
pub trait IDXGIDisplayControlImpl: Sized {
    fn IsStereoEnabled();
    fn SetStereoEnabled();
}
impl ::windows::core::RuntimeName for IDXGIDisplayControl {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIDisplayControl";
}
impl IDXGIDisplayControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDisplayControlImpl, const OFFSET: isize>() -> IDXGIDisplayControlVtbl {
        unsafe extern "system" fn IsStereoEnabled<Impl: IDXGIDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStereoEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStereoEnabled<Impl: IDXGIDisplayControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStereoEnabled(&*(&enabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIDisplayControl>, ::windows::core::GetTrustLevel, IsStereoEnabled::<Impl, OFFSET>, SetStereoEnabled::<Impl, OFFSET>)
    }
}
pub trait IDXGIFactoryImpl: Sized + IDXGIObjectImpl {
    fn EnumAdapters();
    fn MakeWindowAssociation();
    fn GetWindowAssociation();
    fn CreateSwapChain();
    fn CreateSoftwareAdapter();
}
impl ::windows::core::RuntimeName for IDXGIFactory {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIFactory";
}
impl IDXGIFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactoryImpl, const OFFSET: isize>() -> IDXGIFactoryVtbl {
        unsafe extern "system" fn EnumAdapters<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdapters(adapter, ::core::mem::transmute_copy(&ppadapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeWindowAssociation<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MakeWindowAssociation(&*(&windowhandle as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWindowAssociation<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWindowAssociation(::core::mem::transmute_copy(&pwindowhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChain<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChain(&*(&pdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&pdesc as *const <DXGI_SWAP_CHAIN_DESC as ::windows::core::Abi>::Abi as *const <DXGI_SWAP_CHAIN_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppswapchain)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Impl: IDXGIFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::HINSTANCE, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSoftwareAdapter(&*(&module as *const <super::super::Foundation::HINSTANCE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HINSTANCE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppadapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIFactory>, ::windows::core::GetTrustLevel, EnumAdapters::<Impl, OFFSET>, MakeWindowAssociation::<Impl, OFFSET>, GetWindowAssociation::<Impl, OFFSET>, CreateSwapChain::<Impl, OFFSET>, CreateSoftwareAdapter::<Impl, OFFSET>)
    }
}
pub trait IDXGIFactory1Impl: Sized + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn EnumAdapters1();
    fn IsCurrent();
}
impl ::windows::core::RuntimeName for IDXGIFactory1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIFactory1";
}
impl IDXGIFactory1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory1Impl, const OFFSET: isize>() -> IDXGIFactory1Vtbl {
        unsafe extern "system" fn EnumAdapters1<Impl: IDXGIFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdapters1(adapter, ::core::mem::transmute_copy(&ppadapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrent<Impl: IDXGIFactory1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIFactory1>, ::windows::core::GetTrustLevel, EnumAdapters1::<Impl, OFFSET>, IsCurrent::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDXGIFactory2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIFactory2";
}
impl IDXGIFactory2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2Impl, const OFFSET: isize>() -> IDXGIFactory2Vtbl {
        unsafe extern "system" fn IsWindowedStereoEnabled<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWindowedStereoEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChainForHwnd(
                &*(&pdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <DXGI_SWAP_CHAIN_DESC1 as ::windows::core::Abi>::Abi as *const <DXGI_SWAP_CHAIN_DESC1 as ::windows::core::DefaultType>::DefaultType),
                &*(&pfullscreendesc as *const <DXGI_SWAP_CHAIN_FULLSCREEN_DESC as ::windows::core::Abi>::Abi as *const <DXGI_SWAP_CHAIN_FULLSCREEN_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&prestricttooutput as *const <IDXGIOutput as ::windows::core::Abi>::Abi as *const <IDXGIOutput as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppswapchain),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChainForCoreWindow(
                &*(&pdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pwindow as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <DXGI_SWAP_CHAIN_DESC1 as ::windows::core::Abi>::Abi as *const <DXGI_SWAP_CHAIN_DESC1 as ::windows::core::DefaultType>::DefaultType),
                &*(&prestricttooutput as *const <IDXGIOutput as ::windows::core::Abi>::Abi as *const <IDXGIOutput as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppswapchain),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSharedResourceAdapterLuid(&*(&hresource as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pluid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterStereoStatusWindow(&*(&windowhandle as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), wmsg, ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterStereoStatusEvent(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterStereoStatus<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterStereoStatus(dwcookie).into()
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterOcclusionStatusWindow(&*(&windowhandle as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), wmsg, ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterOcclusionStatusEvent(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterOcclusionStatus(dwcookie).into()
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Impl: IDXGIFactory2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChainForComposition(
                &*(&pdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <DXGI_SWAP_CHAIN_DESC1 as ::windows::core::Abi>::Abi as *const <DXGI_SWAP_CHAIN_DESC1 as ::windows::core::DefaultType>::DefaultType),
                &*(&prestricttooutput as *const <IDXGIOutput as ::windows::core::Abi>::Abi as *const <IDXGIOutput as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppswapchain),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXGIFactory2>,
            ::windows::core::GetTrustLevel,
            IsWindowedStereoEnabled::<Impl, OFFSET>,
            CreateSwapChainForHwnd::<Impl, OFFSET>,
            CreateSwapChainForCoreWindow::<Impl, OFFSET>,
            GetSharedResourceAdapterLuid::<Impl, OFFSET>,
            RegisterStereoStatusWindow::<Impl, OFFSET>,
            RegisterStereoStatusEvent::<Impl, OFFSET>,
            UnregisterStereoStatus::<Impl, OFFSET>,
            RegisterOcclusionStatusWindow::<Impl, OFFSET>,
            RegisterOcclusionStatusEvent::<Impl, OFFSET>,
            UnregisterOcclusionStatus::<Impl, OFFSET>,
            CreateSwapChainForComposition::<Impl, OFFSET>,
        )
    }
}
pub trait IDXGIFactory3Impl: Sized + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn GetCreationFlags();
}
impl ::windows::core::RuntimeName for IDXGIFactory3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIFactory3";
}
impl IDXGIFactory3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory3Impl, const OFFSET: isize>() -> IDXGIFactory3Vtbl {
        unsafe extern "system" fn GetCreationFlags<Impl: IDXGIFactory3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreationFlags() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIFactory3>, ::windows::core::GetTrustLevel, GetCreationFlags::<Impl, OFFSET>)
    }
}
pub trait IDXGIFactory4Impl: Sized + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn EnumAdapterByLuid();
    fn EnumWarpAdapter();
}
impl ::windows::core::RuntimeName for IDXGIFactory4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIFactory4";
}
impl IDXGIFactory4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory4Impl, const OFFSET: isize>() -> IDXGIFactory4Vtbl {
        unsafe extern "system" fn EnumAdapterByLuid<Impl: IDXGIFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: &::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdapterByLuid(&*(&adapterluid as *const <super::super::Foundation::LUID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvadapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumWarpAdapter<Impl: IDXGIFactory4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumWarpAdapter(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvadapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIFactory4>, ::windows::core::GetTrustLevel, EnumAdapterByLuid::<Impl, OFFSET>, EnumWarpAdapter::<Impl, OFFSET>)
    }
}
pub trait IDXGIFactory5Impl: Sized + IDXGIFactory4Impl + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn CheckFeatureSupport();
}
impl ::windows::core::RuntimeName for IDXGIFactory5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIFactory5";
}
impl IDXGIFactory5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory5Impl, const OFFSET: isize>() -> IDXGIFactory5Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Impl: IDXGIFactory5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckFeatureSupport(feature, &*(&pfeaturesupportdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), featuresupportdatasize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIFactory5>, ::windows::core::GetTrustLevel, CheckFeatureSupport::<Impl, OFFSET>)
    }
}
pub trait IDXGIFactory6Impl: Sized + IDXGIFactory5Impl + IDXGIFactory4Impl + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn EnumAdapterByGpuPreference();
}
impl ::windows::core::RuntimeName for IDXGIFactory6 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIFactory6";
}
impl IDXGIFactory6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory6Impl, const OFFSET: isize>() -> IDXGIFactory6Vtbl {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Impl: IDXGIFactory6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: &::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumAdapterByGpuPreference(adapter, gpupreference, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvadapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIFactory6>, ::windows::core::GetTrustLevel, EnumAdapterByGpuPreference::<Impl, OFFSET>)
    }
}
pub trait IDXGIFactory7Impl: Sized + IDXGIFactory6Impl + IDXGIFactory5Impl + IDXGIFactory4Impl + IDXGIFactory3Impl + IDXGIFactory2Impl + IDXGIFactory1Impl + IDXGIFactoryImpl + IDXGIObjectImpl {
    fn RegisterAdaptersChangedEvent();
    fn UnregisterAdaptersChangedEvent();
}
impl ::windows::core::RuntimeName for IDXGIFactory7 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIFactory7";
}
impl IDXGIFactory7Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory7Impl, const OFFSET: isize>() -> IDXGIFactory7Vtbl {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Impl: IDXGIFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterAdaptersChangedEvent(&*(&hevent as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Impl: IDXGIFactory7Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterAdaptersChangedEvent(dwcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIFactory7>, ::windows::core::GetTrustLevel, RegisterAdaptersChangedEvent::<Impl, OFFSET>, UnregisterAdaptersChangedEvent::<Impl, OFFSET>)
    }
}
pub trait IDXGIFactoryMediaImpl: Sized {
    fn CreateSwapChainForCompositionSurfaceHandle();
    fn CreateDecodeSwapChainForCompositionSurfaceHandle();
}
impl ::windows::core::RuntimeName for IDXGIFactoryMedia {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIFactoryMedia";
}
impl IDXGIFactoryMediaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactoryMediaImpl, const OFFSET: isize>() -> IDXGIFactoryMediaVtbl {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Impl: IDXGIFactoryMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSwapChainForCompositionSurfaceHandle(
                &*(&pdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&hsurface as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <DXGI_SWAP_CHAIN_DESC1 as ::windows::core::Abi>::Abi as *const <DXGI_SWAP_CHAIN_DESC1 as ::windows::core::DefaultType>::DefaultType),
                &*(&prestricttooutput as *const <IDXGIOutput as ::windows::core::Abi>::Abi as *const <IDXGIOutput as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppswapchain),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Impl: IDXGIFactoryMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: ::windows::core::RawPtr, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDecodeSwapChainForCompositionSurfaceHandle(
                &*(&pdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&hsurface as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType),
                &*(&pdesc as *const <DXGI_DECODE_SWAP_CHAIN_DESC as ::windows::core::Abi>::Abi as *const <DXGI_DECODE_SWAP_CHAIN_DESC as ::windows::core::DefaultType>::DefaultType),
                &*(&pyuvdecodebuffers as *const <IDXGIResource as ::windows::core::Abi>::Abi as *const <IDXGIResource as ::windows::core::DefaultType>::DefaultType),
                &*(&prestricttooutput as *const <IDXGIOutput as ::windows::core::Abi>::Abi as *const <IDXGIOutput as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppswapchain),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIFactoryMedia>, ::windows::core::GetTrustLevel, CreateSwapChainForCompositionSurfaceHandle::<Impl, OFFSET>, CreateDecodeSwapChainForCompositionSurfaceHandle::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDXGIInfoQueue {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIInfoQueue";
}
impl IDXGIInfoQueueVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueueImpl, const OFFSET: isize>() -> IDXGIInfoQueueVtbl {
        unsafe extern "system" fn SetMessageCountLimit<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMessageCountLimit(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), messagecountlimit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStoredMessages<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStoredMessages(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetMessage<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessage(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), messageindex, ::core::mem::transmute_copy(&pmessage), pmessagebytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumStoredMessagesAllowedByRetrievalFilters(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumStoredMessages<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumStoredMessages(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesDiscardedByMessageCountLimit(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageCountLimit<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageCountLimit(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesAllowedByStorageFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNumMessagesDeniedByStorageFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStorageFilterEntries<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStorageFilterEntries(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pfilter as *const <DXGI_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <DXGI_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfilter), pfilterbytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearStorageFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushEmptyStorageFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushDenyAllStorageFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushCopyOfStorageFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushStorageFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pfilter as *const <DXGI_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <DXGI_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopStorageFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopStorageFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStorageFilterStackSize(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRetrievalFilterEntries(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pfilter as *const <DXGI_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <DXGI_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetrievalFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfilter), pfilterbytelength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearRetrievalFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushEmptyRetrievalFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushDenyAllRetrievalFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushCopyOfRetrievalFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PushRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PushRetrievalFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&pfilter as *const <DXGI_INFO_QUEUE_FILTER as ::windows::core::Abi>::Abi as *const <DXGI_INFO_QUEUE_FILTER as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopRetrievalFilter<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopRetrievalFilter(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetrievalFilterStackSize(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMessage<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMessage(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), category, severity, id, &*(&pdescription as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddApplicationMessage<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddApplicationMessage(severity, &*(&pdescription as *const <super::super::Foundation::PSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnCategory<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnCategory(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), category, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnSeverity<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnSeverity(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), severity, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBreakOnID<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBreakOnID(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), id, &*(&benable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnCategory<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnCategory(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), category) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnSeverity<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnSeverity(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), severity) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBreakOnID<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBreakOnID(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), id) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMuteDebugOutput<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMuteDebugOutput(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&bmute as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn GetMuteDebugOutput<Impl: IDXGIInfoQueueImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMuteDebugOutput(&*(&producer as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXGIInfoQueue>,
            ::windows::core::GetTrustLevel,
            SetMessageCountLimit::<Impl, OFFSET>,
            ClearStoredMessages::<Impl, OFFSET>,
            GetMessage::<Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters::<Impl, OFFSET>,
            GetNumStoredMessages::<Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit::<Impl, OFFSET>,
            GetMessageCountLimit::<Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter::<Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter::<Impl, OFFSET>,
            AddStorageFilterEntries::<Impl, OFFSET>,
            GetStorageFilter::<Impl, OFFSET>,
            ClearStorageFilter::<Impl, OFFSET>,
            PushEmptyStorageFilter::<Impl, OFFSET>,
            PushDenyAllStorageFilter::<Impl, OFFSET>,
            PushCopyOfStorageFilter::<Impl, OFFSET>,
            PushStorageFilter::<Impl, OFFSET>,
            PopStorageFilter::<Impl, OFFSET>,
            GetStorageFilterStackSize::<Impl, OFFSET>,
            AddRetrievalFilterEntries::<Impl, OFFSET>,
            GetRetrievalFilter::<Impl, OFFSET>,
            ClearRetrievalFilter::<Impl, OFFSET>,
            PushEmptyRetrievalFilter::<Impl, OFFSET>,
            PushDenyAllRetrievalFilter::<Impl, OFFSET>,
            PushCopyOfRetrievalFilter::<Impl, OFFSET>,
            PushRetrievalFilter::<Impl, OFFSET>,
            PopRetrievalFilter::<Impl, OFFSET>,
            GetRetrievalFilterStackSize::<Impl, OFFSET>,
            AddMessage::<Impl, OFFSET>,
            AddApplicationMessage::<Impl, OFFSET>,
            SetBreakOnCategory::<Impl, OFFSET>,
            SetBreakOnSeverity::<Impl, OFFSET>,
            SetBreakOnID::<Impl, OFFSET>,
            GetBreakOnCategory::<Impl, OFFSET>,
            GetBreakOnSeverity::<Impl, OFFSET>,
            GetBreakOnID::<Impl, OFFSET>,
            SetMuteDebugOutput::<Impl, OFFSET>,
            GetMuteDebugOutput::<Impl, OFFSET>,
        )
    }
}
pub trait IDXGIKeyedMutexImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn AcquireSync();
    fn ReleaseSync();
}
impl ::windows::core::RuntimeName for IDXGIKeyedMutex {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIKeyedMutex";
}
impl IDXGIKeyedMutexVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIKeyedMutexImpl, const OFFSET: isize>() -> IDXGIKeyedMutexVtbl {
        unsafe extern "system" fn AcquireSync<Impl: IDXGIKeyedMutexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireSync(key, dwmilliseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseSync<Impl: IDXGIKeyedMutexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseSync(key) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIKeyedMutex>, ::windows::core::GetTrustLevel, AcquireSync::<Impl, OFFSET>, ReleaseSync::<Impl, OFFSET>)
    }
}
pub trait IDXGIObjectImpl: Sized {
    fn SetPrivateData();
    fn SetPrivateDataInterface();
    fn GetPrivateData();
    fn GetParent();
}
impl ::windows::core::RuntimeName for IDXGIObject {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIObject";
}
impl IDXGIObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIObjectImpl, const OFFSET: isize>() -> IDXGIObjectVtbl {
        unsafe extern "system" fn SetPrivateData<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: &::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateData(&*(&name as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), datasize, &*(&pdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPrivateDataInterface<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: &::windows::core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPrivateDataInterface(&*(&name as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPrivateData<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: &::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrivateData(&*(&name as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), pdatasize, ::core::mem::transmute_copy(&pdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParent<Impl: IDXGIObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParent(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIObject>, ::windows::core::GetTrustLevel, SetPrivateData::<Impl, OFFSET>, SetPrivateDataInterface::<Impl, OFFSET>, GetPrivateData::<Impl, OFFSET>, GetParent::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDXGIOutput {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIOutput";
}
impl IDXGIOutputVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputImpl, const OFFSET: isize>() -> IDXGIOutputVtbl {
        unsafe extern "system" fn GetDesc<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayModeList<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayModeList(enumformat, flags, pnummodes, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClosestMatchingMode<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindClosestMatchingMode(&*(&pmodetomatch as *const <Common::DXGI_MODE_DESC as ::windows::core::Abi>::Abi as *const <Common::DXGI_MODE_DESC as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pclosestmatch), &*(&pconcerneddevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WaitForVBlank<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WaitForVBlank() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TakeOwnership<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TakeOwnership(&*(&pdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&exclusive as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseOwnership<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseOwnership().into()
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGammaControlCapabilities(::core::mem::transmute_copy(&pgammacaps)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGammaControl<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGammaControl(&*(&parray as *const <Common::DXGI_GAMMA_CONTROL as ::windows::core::Abi>::Abi as *const <Common::DXGI_GAMMA_CONTROL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGammaControl<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGammaControl(::core::mem::transmute_copy(&parray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplaySurface<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscanoutsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisplaySurface(&*(&pscanoutsurface as *const <IDXGISurface as ::windows::core::Abi>::Abi as *const <IDXGISurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplaySurfaceData(&*(&pdestination as *const <IDXGISurface as ::windows::core::Abi>::Abi as *const <IDXGISurface as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDXGIOutputImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics(::core::mem::transmute_copy(&pstats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXGIOutput>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            GetDisplayModeList::<Impl, OFFSET>,
            FindClosestMatchingMode::<Impl, OFFSET>,
            WaitForVBlank::<Impl, OFFSET>,
            TakeOwnership::<Impl, OFFSET>,
            ReleaseOwnership::<Impl, OFFSET>,
            GetGammaControlCapabilities::<Impl, OFFSET>,
            SetGammaControl::<Impl, OFFSET>,
            GetGammaControl::<Impl, OFFSET>,
            SetDisplaySurface::<Impl, OFFSET>,
            GetDisplaySurfaceData::<Impl, OFFSET>,
            GetFrameStatistics::<Impl, OFFSET>,
        )
    }
}
pub trait IDXGIOutput1Impl: Sized + IDXGIOutputImpl + IDXGIObjectImpl {
    fn GetDisplayModeList1();
    fn FindClosestMatchingMode1();
    fn GetDisplaySurfaceData1();
    fn DuplicateOutput();
}
impl ::windows::core::RuntimeName for IDXGIOutput1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIOutput1";
}
impl IDXGIOutput1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput1Impl, const OFFSET: isize>() -> IDXGIOutput1Vtbl {
        unsafe extern "system" fn GetDisplayModeList1<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayModeList1(enumformat, flags, pnummodes, ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindClosestMatchingMode1(&*(&pmodetomatch as *const <DXGI_MODE_DESC1 as ::windows::core::Abi>::Abi as *const <DXGI_MODE_DESC1 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pclosestmatch), &*(&pconcerneddevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplaySurfaceData1(&*(&pdestination as *const <IDXGIResource as ::windows::core::Abi>::Abi as *const <IDXGIResource as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DuplicateOutput<Impl: IDXGIOutput1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DuplicateOutput(&*(&pdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoutputduplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIOutput1>, ::windows::core::GetTrustLevel, GetDisplayModeList1::<Impl, OFFSET>, FindClosestMatchingMode1::<Impl, OFFSET>, GetDisplaySurfaceData1::<Impl, OFFSET>, DuplicateOutput::<Impl, OFFSET>)
    }
}
pub trait IDXGIOutput2Impl: Sized + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn SupportsOverlays();
}
impl ::windows::core::RuntimeName for IDXGIOutput2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIOutput2";
}
impl IDXGIOutput2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput2Impl, const OFFSET: isize>() -> IDXGIOutput2Vtbl {
        unsafe extern "system" fn SupportsOverlays<Impl: IDXGIOutput2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsOverlays() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIOutput2>, ::windows::core::GetTrustLevel, SupportsOverlays::<Impl, OFFSET>)
    }
}
pub trait IDXGIOutput3Impl: Sized + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn CheckOverlaySupport();
}
impl ::windows::core::RuntimeName for IDXGIOutput3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIOutput3";
}
impl IDXGIOutput3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput3Impl, const OFFSET: isize>() -> IDXGIOutput3Vtbl {
        unsafe extern "system" fn CheckOverlaySupport<Impl: IDXGIOutput3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckOverlaySupport(enumformat, &*(&pconcerneddevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIOutput3>, ::windows::core::GetTrustLevel, CheckOverlaySupport::<Impl, OFFSET>)
    }
}
pub trait IDXGIOutput4Impl: Sized + IDXGIOutput3Impl + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn CheckOverlayColorSpaceSupport();
}
impl ::windows::core::RuntimeName for IDXGIOutput4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIOutput4";
}
impl IDXGIOutput4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput4Impl, const OFFSET: isize>() -> IDXGIOutput4Vtbl {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Impl: IDXGIOutput4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckOverlayColorSpaceSupport(format, colorspace, &*(&pconcerneddevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIOutput4>, ::windows::core::GetTrustLevel, CheckOverlayColorSpaceSupport::<Impl, OFFSET>)
    }
}
pub trait IDXGIOutput5Impl: Sized + IDXGIOutput4Impl + IDXGIOutput3Impl + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn DuplicateOutput1();
}
impl ::windows::core::RuntimeName for IDXGIOutput5 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIOutput5";
}
impl IDXGIOutput5Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput5Impl, const OFFSET: isize>() -> IDXGIOutput5Vtbl {
        unsafe extern "system" fn DuplicateOutput1<Impl: IDXGIOutput5Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DuplicateOutput1(&*(&pdevice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), flags, supportedformatscount, psupportedformats, ::core::mem::transmute_copy(&ppoutputduplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIOutput5>, ::windows::core::GetTrustLevel, DuplicateOutput1::<Impl, OFFSET>)
    }
}
pub trait IDXGIOutput6Impl: Sized + IDXGIOutput5Impl + IDXGIOutput4Impl + IDXGIOutput3Impl + IDXGIOutput2Impl + IDXGIOutput1Impl + IDXGIOutputImpl + IDXGIObjectImpl {
    fn GetDesc1();
    fn CheckHardwareCompositionSupport();
}
impl ::windows::core::RuntimeName for IDXGIOutput6 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIOutput6";
}
impl IDXGIOutput6Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput6Impl, const OFFSET: isize>() -> IDXGIOutput6Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: IDXGIOutput6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc1(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Impl: IDXGIOutput6Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckHardwareCompositionSupport(::core::mem::transmute_copy(&pflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIOutput6>, ::windows::core::GetTrustLevel, GetDesc1::<Impl, OFFSET>, CheckHardwareCompositionSupport::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDXGIOutputDuplication {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIOutputDuplication";
}
impl IDXGIOutputDuplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>() -> IDXGIOutputDuplicationVtbl {
        unsafe extern "system" fn GetDesc<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn AcquireNextFrame<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireNextFrame(timeoutinmilliseconds, ::core::mem::transmute_copy(&pframeinfo), ::core::mem::transmute_copy(&ppdesktopresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameDirtyRects<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameDirtyRects(dirtyrectsbuffersize, ::core::mem::transmute_copy(&pdirtyrectsbuffer), ::core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameMoveRects<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameMoveRects(moverectsbuffersize, ::core::mem::transmute_copy(&pmoverectbuffer), ::core::mem::transmute_copy(&pmoverectsbuffersizerequired)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFramePointerShape<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFramePointerShape(pointershapebuffersize, ::core::mem::transmute_copy(&ppointershapebuffer), ::core::mem::transmute_copy(&ppointershapebuffersizerequired), ::core::mem::transmute_copy(&ppointershapeinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MapDesktopSurface<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MapDesktopSurface(::core::mem::transmute_copy(&plockedrect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnMapDesktopSurface<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnMapDesktopSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseFrame<Impl: IDXGIOutputDuplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseFrame() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXGIOutputDuplication>,
            ::windows::core::GetTrustLevel,
            GetDesc::<Impl, OFFSET>,
            AcquireNextFrame::<Impl, OFFSET>,
            GetFrameDirtyRects::<Impl, OFFSET>,
            GetFrameMoveRects::<Impl, OFFSET>,
            GetFramePointerShape::<Impl, OFFSET>,
            MapDesktopSurface::<Impl, OFFSET>,
            UnMapDesktopSurface::<Impl, OFFSET>,
            ReleaseFrame::<Impl, OFFSET>,
        )
    }
}
pub trait IDXGIResourceImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetSharedHandle();
    fn GetUsage();
    fn SetEvictionPriority();
    fn GetEvictionPriority();
}
impl ::windows::core::RuntimeName for IDXGIResource {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIResource";
}
impl IDXGIResourceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResourceImpl, const OFFSET: isize>() -> IDXGIResourceVtbl {
        unsafe extern "system" fn GetSharedHandle<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSharedHandle(::core::mem::transmute_copy(&psharedhandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsage<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUsage(::core::mem::transmute_copy(&pusage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEvictionPriority(evictionpriority) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEvictionPriority<Impl: IDXGIResourceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevictionpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEvictionPriority(::core::mem::transmute_copy(&pevictionpriority)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIResource>, ::windows::core::GetTrustLevel, GetSharedHandle::<Impl, OFFSET>, GetUsage::<Impl, OFFSET>, SetEvictionPriority::<Impl, OFFSET>, GetEvictionPriority::<Impl, OFFSET>)
    }
}
pub trait IDXGIResource1Impl: Sized + IDXGIResourceImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn CreateSubresourceSurface();
    fn CreateSharedHandle();
}
impl ::windows::core::RuntimeName for IDXGIResource1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGIResource1";
}
impl IDXGIResource1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource1Impl, const OFFSET: isize>() -> IDXGIResource1Vtbl {
        unsafe extern "system" fn CreateSubresourceSurface<Impl: IDXGIResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSubresourceSurface(index, ::core::mem::transmute_copy(&ppsurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Impl: IDXGIResource1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSharedHandle(&*(&pattributes as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::Abi>::Abi as *const <super::super::Security::SECURITY_ATTRIBUTES as ::windows::core::DefaultType>::DefaultType), dwaccess, &*(&lpname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phandle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGIResource1>, ::windows::core::GetTrustLevel, CreateSubresourceSurface::<Impl, OFFSET>, CreateSharedHandle::<Impl, OFFSET>)
    }
}
pub trait IDXGISurfaceImpl: Sized + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetDesc();
    fn Map();
    fn Unmap();
}
impl ::windows::core::RuntimeName for IDXGISurface {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGISurface";
}
impl IDXGISurfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurfaceImpl, const OFFSET: isize>() -> IDXGISurfaceVtbl {
        unsafe extern "system" fn GetDesc<Impl: IDXGISurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Map<Impl: IDXGISurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Map(::core::mem::transmute_copy(&plockedrect), mapflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unmap<Impl: IDXGISurfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unmap() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGISurface>, ::windows::core::GetTrustLevel, GetDesc::<Impl, OFFSET>, Map::<Impl, OFFSET>, Unmap::<Impl, OFFSET>)
    }
}
pub trait IDXGISurface1Impl: Sized + IDXGISurfaceImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetDC();
    fn ReleaseDC();
}
impl ::windows::core::RuntimeName for IDXGISurface1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGISurface1";
}
impl IDXGISurface1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface1Impl, const OFFSET: isize>() -> IDXGISurface1Vtbl {
        unsafe extern "system" fn GetDC<Impl: IDXGISurface1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDC(&*(&discard as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phdc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Impl: IDXGISurface1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseDC(&*(&pdirtyrect as *const <super::super::Foundation::RECT as ::windows::core::Abi>::Abi as *const <super::super::Foundation::RECT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGISurface1>, ::windows::core::GetTrustLevel, GetDC::<Impl, OFFSET>, ReleaseDC::<Impl, OFFSET>)
    }
}
pub trait IDXGISurface2Impl: Sized + IDXGISurface1Impl + IDXGISurfaceImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetResource();
}
impl ::windows::core::RuntimeName for IDXGISurface2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGISurface2";
}
impl IDXGISurface2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface2Impl, const OFFSET: isize>() -> IDXGISurface2Vtbl {
        unsafe extern "system" fn GetResource<Impl: IDXGISurface2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppparentresource), ::core::mem::transmute_copy(&psubresourceindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGISurface2>, ::windows::core::GetTrustLevel, GetResource::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IDXGISwapChain {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGISwapChain";
}
impl IDXGISwapChainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainImpl, const OFFSET: isize>() -> IDXGISwapChainVtbl {
        unsafe extern "system" fn Present<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Present(syncinterval, flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBuffer<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: u32, riid: &::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBuffer(buffer, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsurface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullscreenState<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFullscreenState(&*(&fullscreen as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&ptarget as *const <IDXGIOutput as ::windows::core::Abi>::Abi as *const <IDXGIOutput as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullscreenState<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullscreenState(::core::mem::transmute_copy(&pfullscreen), ::core::mem::transmute_copy(&pptarget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeBuffers<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResizeBuffers(buffercount, width, height, newformat, swapchainflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeTarget<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResizeTarget(&*(&pnewtargetparameters as *const <Common::DXGI_MODE_DESC as ::windows::core::Abi>::Abi as *const <Common::DXGI_MODE_DESC as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainingOutput<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContainingOutput(::core::mem::transmute_copy(&ppoutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatistics(::core::mem::transmute_copy(&pstats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPresentCount<Impl: IDXGISwapChainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLastPresentCount(::core::mem::transmute_copy(&plastpresentcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXGISwapChain>,
            ::windows::core::GetTrustLevel,
            Present::<Impl, OFFSET>,
            GetBuffer::<Impl, OFFSET>,
            SetFullscreenState::<Impl, OFFSET>,
            GetFullscreenState::<Impl, OFFSET>,
            GetDesc::<Impl, OFFSET>,
            ResizeBuffers::<Impl, OFFSET>,
            ResizeTarget::<Impl, OFFSET>,
            GetContainingOutput::<Impl, OFFSET>,
            GetFrameStatistics::<Impl, OFFSET>,
            GetLastPresentCount::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDXGISwapChain1 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGISwapChain1";
}
impl IDXGISwapChain1Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1Impl, const OFFSET: isize>() -> IDXGISwapChain1Vtbl {
        unsafe extern "system" fn GetDesc1<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDesc1(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullscreenDesc<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFullscreenDesc(::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHwnd<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHwnd(::core::mem::transmute_copy(&phwnd)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCoreWindow<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: &::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCoreWindow(&*(&refiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Present1<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Present1(syncinterval, presentflags, &*(&ppresentparameters as *const <DXGI_PRESENT_PARAMETERS as ::windows::core::Abi>::Abi as *const <DXGI_PRESENT_PARAMETERS as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsTemporaryMonoSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRestrictToOutput<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprestricttooutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRestrictToOutput(::core::mem::transmute_copy(&pprestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *const DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBackgroundColor(&*(&pcolor as *const <DXGI_RGBA as ::windows::core::Abi>::Abi as *const <DXGI_RGBA as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBackgroundColor<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBackgroundColor(::core::mem::transmute_copy(&pcolor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRotation(rotation) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRotation<Impl: IDXGISwapChain1Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRotation(::core::mem::transmute_copy(&protation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXGISwapChain1>,
            ::windows::core::GetTrustLevel,
            GetDesc1::<Impl, OFFSET>,
            GetFullscreenDesc::<Impl, OFFSET>,
            GetHwnd::<Impl, OFFSET>,
            GetCoreWindow::<Impl, OFFSET>,
            Present1::<Impl, OFFSET>,
            IsTemporaryMonoSupported::<Impl, OFFSET>,
            GetRestrictToOutput::<Impl, OFFSET>,
            SetBackgroundColor::<Impl, OFFSET>,
            GetBackgroundColor::<Impl, OFFSET>,
            SetRotation::<Impl, OFFSET>,
            GetRotation::<Impl, OFFSET>,
        )
    }
}
pub trait IDXGISwapChain2Impl: Sized + IDXGISwapChain1Impl + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn SetSourceSize();
    fn GetSourceSize();
    fn SetMaximumFrameLatency();
    fn GetMaximumFrameLatency();
    fn GetFrameLatencyWaitableObject();
    fn SetMatrixTransform();
    fn GetMatrixTransform();
}
impl ::windows::core::RuntimeName for IDXGISwapChain2 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGISwapChain2";
}
impl IDXGISwapChain2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2Impl, const OFFSET: isize>() -> IDXGISwapChain2Vtbl {
        unsafe extern "system" fn SetSourceSize<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourceSize(width, height) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSourceSize<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSourceSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaximumFrameLatency(maxlatency) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumFrameLatency(::core::mem::transmute_copy(&pmaxlatency)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameLatencyWaitableObject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMatrixTransform<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMatrixTransform(&*(&pmatrix as *const <DXGI_MATRIX_3X2_F as ::windows::core::Abi>::Abi as *const <DXGI_MATRIX_3X2_F as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatrixTransform<Impl: IDXGISwapChain2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatrixTransform(::core::mem::transmute_copy(&pmatrix)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXGISwapChain2>,
            ::windows::core::GetTrustLevel,
            SetSourceSize::<Impl, OFFSET>,
            GetSourceSize::<Impl, OFFSET>,
            SetMaximumFrameLatency::<Impl, OFFSET>,
            GetMaximumFrameLatency::<Impl, OFFSET>,
            GetFrameLatencyWaitableObject::<Impl, OFFSET>,
            SetMatrixTransform::<Impl, OFFSET>,
            GetMatrixTransform::<Impl, OFFSET>,
        )
    }
}
pub trait IDXGISwapChain3Impl: Sized + IDXGISwapChain2Impl + IDXGISwapChain1Impl + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn GetCurrentBackBufferIndex();
    fn CheckColorSpaceSupport();
    fn SetColorSpace1();
    fn ResizeBuffers1();
}
impl ::windows::core::RuntimeName for IDXGISwapChain3 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGISwapChain3";
}
impl IDXGISwapChain3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain3Impl, const OFFSET: isize>() -> IDXGISwapChain3Vtbl {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentBackBufferIndex() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckColorSpaceSupport(colorspace, ::core::mem::transmute_copy(&pcolorspacesupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorSpace1<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetColorSpace1(colorspace) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeBuffers1<Impl: IDXGISwapChain3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResizeBuffers1(buffercount, width, height, format, swapchainflags, pcreationnodemask, &*(&pppresentqueue as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGISwapChain3>, ::windows::core::GetTrustLevel, GetCurrentBackBufferIndex::<Impl, OFFSET>, CheckColorSpaceSupport::<Impl, OFFSET>, SetColorSpace1::<Impl, OFFSET>, ResizeBuffers1::<Impl, OFFSET>)
    }
}
pub trait IDXGISwapChain4Impl: Sized + IDXGISwapChain3Impl + IDXGISwapChain2Impl + IDXGISwapChain1Impl + IDXGISwapChainImpl + IDXGIDeviceSubObjectImpl + IDXGIObjectImpl {
    fn SetHDRMetaData();
}
impl ::windows::core::RuntimeName for IDXGISwapChain4 {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGISwapChain4";
}
impl IDXGISwapChain4Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain4Impl, const OFFSET: isize>() -> IDXGISwapChain4Vtbl {
        unsafe extern "system" fn SetHDRMetaData<Impl: IDXGISwapChain4Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHDRMetaData(r#type, size, &*(&pmetadata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGISwapChain4>, ::windows::core::GetTrustLevel, SetHDRMetaData::<Impl, OFFSET>)
    }
}
pub trait IDXGISwapChainMediaImpl: Sized {
    fn GetFrameStatisticsMedia();
    fn SetPresentDuration();
    fn CheckPresentDurationSupport();
}
impl ::windows::core::RuntimeName for IDXGISwapChainMedia {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGISwapChainMedia";
}
impl IDXGISwapChainMediaVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>() -> IDXGISwapChainMediaVtbl {
        unsafe extern "system" fn GetFrameStatisticsMedia<Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFrameStatisticsMedia(::core::mem::transmute_copy(&pstats)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentDuration<Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPresentDuration(duration) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Impl: IDXGISwapChainMediaImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckPresentDurationSupport(desiredpresentduration, ::core::mem::transmute_copy(&pclosestsmallerpresentduration), ::core::mem::transmute_copy(&pclosestlargerpresentduration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGISwapChainMedia>, ::windows::core::GetTrustLevel, GetFrameStatisticsMedia::<Impl, OFFSET>, SetPresentDuration::<Impl, OFFSET>, CheckPresentDurationSupport::<Impl, OFFSET>)
    }
}
pub trait IDXGraphicsAnalysisImpl: Sized {
    fn BeginCapture();
    fn EndCapture();
}
impl ::windows::core::RuntimeName for IDXGraphicsAnalysis {
    const NAME: &'static str = "Windows.Win32.Graphics.Dxgi.IDXGraphicsAnalysis";
}
impl IDXGraphicsAnalysisVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGraphicsAnalysisImpl, const OFFSET: isize>() -> IDXGraphicsAnalysisVtbl {
        unsafe extern "system" fn BeginCapture<Impl: IDXGraphicsAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginCapture().into()
        }
        unsafe extern "system" fn EndCapture<Impl: IDXGraphicsAnalysisImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndCapture().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXGraphicsAnalysis>, ::windows::core::GetTrustLevel, BeginCapture::<Impl, OFFSET>, EndCapture::<Impl, OFFSET>)
    }
}
