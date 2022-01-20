#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter_Impl: Sized + IDXGIObject_Impl {
    fn EnumOutputs(&mut self, output: u32) -> ::windows::core::Result<IDXGIOutput>;
    fn GetDesc(&mut self) -> ::windows::core::Result<DXGI_ADAPTER_DESC>;
    fn CheckInterfaceSupport(&mut self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter_Impl, const OFFSET: isize>() -> IDXGIAdapter_Vtbl {
        unsafe extern "system" fn EnumOutputs<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: u32, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumOutputs(::core::mem::transmute_copy(&output)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckInterfaceSupport<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacename: *const ::windows::core::GUID, pumdversion: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckInterfaceSupport(::core::mem::transmute_copy(&interfacename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pumdversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumOutputs: EnumOutputs::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter1_Impl: Sized + IDXGIObject_Impl + IDXGIAdapter_Impl {
    fn GetDesc1(&mut self) -> ::windows::core::Result<DXGI_ADAPTER_DESC1>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter1_Impl, const OFFSET: isize>() -> IDXGIAdapter1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc1() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIAdapter_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter1 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter2_Impl: Sized + IDXGIObject_Impl + IDXGIAdapter_Impl + IDXGIAdapter1_Impl {
    fn GetDesc2(&mut self) -> ::windows::core::Result<DXGI_ADAPTER_DESC2>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter2_Impl, const OFFSET: isize>() -> IDXGIAdapter2_Vtbl {
        unsafe extern "system" fn GetDesc2<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc2() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIAdapter1_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc2: GetDesc2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter2 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter1 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter3_Impl: Sized + IDXGIObject_Impl + IDXGIAdapter_Impl + IDXGIAdapter1_Impl + IDXGIAdapter2_Impl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterHardwareContentProtectionTeardownStatus(&mut self, dwcookie: u32);
    fn QueryVideoMemoryInfo(&mut self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP) -> ::windows::core::Result<DXGI_QUERY_VIDEO_MEMORY_INFO>;
    fn SetVideoMemoryReservation(&mut self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::Result<()>;
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterVideoMemoryBudgetChangeNotification(&mut self, dwcookie: u32);
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>() -> IDXGIAdapter3_Vtbl {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterHardwareContentProtectionTeardownStatusEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterHardwareContentProtectionTeardownStatus(::core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryVideoMemoryInfo(::core::mem::transmute_copy(&nodeindex), ::core::mem::transmute_copy(&memorysegmentgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvideomemoryinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVideoMemoryReservation(::core::mem::transmute_copy(&nodeindex), ::core::mem::transmute_copy(&memorysegmentgroup), ::core::mem::transmute_copy(&reservation)).into()
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterVideoMemoryBudgetChangeNotificationEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterVideoMemoryBudgetChangeNotification(::core::mem::transmute_copy(&dwcookie))
        }
        Self {
            base: IDXGIAdapter2_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterHardwareContentProtectionTeardownStatusEvent: RegisterHardwareContentProtectionTeardownStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus: UnregisterHardwareContentProtectionTeardownStatus::<Identity, Impl, OFFSET>,
            QueryVideoMemoryInfo: QueryVideoMemoryInfo::<Identity, Impl, OFFSET>,
            SetVideoMemoryReservation: SetVideoMemoryReservation::<Identity, Impl, OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent: RegisterVideoMemoryBudgetChangeNotificationEvent::<Identity, Impl, OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification: UnregisterVideoMemoryBudgetChangeNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter3 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter1 as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter4_Impl: Sized + IDXGIObject_Impl + IDXGIAdapter_Impl + IDXGIAdapter1_Impl + IDXGIAdapter2_Impl + IDXGIAdapter3_Impl {
    fn GetDesc3(&mut self) -> ::windows::core::Result<DXGI_ADAPTER_DESC3>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter4_Impl, const OFFSET: isize>() -> IDXGIAdapter4_Vtbl {
        unsafe extern "system" fn GetDesc3<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIAdapter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc3() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIAdapter3_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc3: GetDesc3::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter4 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter1 as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter2 as ::windows::core::Interface>::IID || iid == &<IDXGIAdapter3 as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIDebug_Impl: Sized {
    fn ReportLiveObjects(&mut self, apiid: &::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::Result<()>;
}
impl IDXGIDebug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebug_Impl, const OFFSET: isize>() -> IDXGIDebug_Vtbl {
        unsafe extern "system" fn ReportLiveObjects<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReportLiveObjects(::core::mem::transmute_copy(&apiid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ReportLiveObjects: ReportLiveObjects::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDebug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDebug1_Impl: Sized + IDXGIDebug_Impl {
    fn EnableLeakTrackingForThread(&mut self);
    fn DisableLeakTrackingForThread(&mut self);
    fn IsLeakTrackingEnabledForThread(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDebug1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebug1_Impl, const OFFSET: isize>() -> IDXGIDebug1_Vtbl {
        unsafe extern "system" fn EnableLeakTrackingForThread<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnableLeakTrackingForThread()
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DisableLeakTrackingForThread()
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsLeakTrackingEnabledForThread()
        }
        Self {
            base: IDXGIDebug_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnableLeakTrackingForThread: EnableLeakTrackingForThread::<Identity, Impl, OFFSET>,
            DisableLeakTrackingForThread: DisableLeakTrackingForThread::<Identity, Impl, OFFSET>,
            IsLeakTrackingEnabledForThread: IsLeakTrackingEnabledForThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDebug1 as ::windows::core::Interface>::IID || iid == &<IDXGIDebug as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDecodeSwapChain_Impl: Sized {
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
impl IDXGIDecodeSwapChain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>() -> IDXGIDecodeSwapChain_Vtbl {
        unsafe extern "system" fn PresentBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PresentBuffer(::core::mem::transmute_copy(&buffertopresent), ::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn SetSourceRect<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSourceRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetTargetRect<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTargetRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetDestSize<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDestSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceRect<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSourceRect() {
                ::core::result::Result::Ok(ok__) => {
                    *prect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetRect<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTargetRect() {
                ::core::result::Result::Ok(ok__) => {
                    *prect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestSize<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDestSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetColorSpace<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorSpace(::core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn GetColorSpace<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetColorSpace()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PresentBuffer: PresentBuffer::<Identity, Impl, OFFSET>,
            SetSourceRect: SetSourceRect::<Identity, Impl, OFFSET>,
            SetTargetRect: SetTargetRect::<Identity, Impl, OFFSET>,
            SetDestSize: SetDestSize::<Identity, Impl, OFFSET>,
            GetSourceRect: GetSourceRect::<Identity, Impl, OFFSET>,
            GetTargetRect: GetTargetRect::<Identity, Impl, OFFSET>,
            GetDestSize: GetDestSize::<Identity, Impl, OFFSET>,
            SetColorSpace: SetColorSpace::<Identity, Impl, OFFSET>,
            GetColorSpace: GetColorSpace::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDecodeSwapChain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice_Impl: Sized + IDXGIObject_Impl {
    fn GetAdapter(&mut self) -> ::windows::core::Result<IDXGIAdapter>;
    fn CreateSurface(&mut self, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut ::core::option::Option<IDXGISurface>) -> ::windows::core::Result<()>;
    fn QueryResourceResidency(&mut self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()>;
    fn SetGPUThreadPriority(&mut self, priority: i32) -> ::windows::core::Result<()>;
    fn GetGPUThreadPriority(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice_Impl, const OFFSET: isize>() -> IDXGIDevice_Vtbl {
        unsafe extern "system" fn GetAdapter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    *padapter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: u32, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateSurface(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numsurfaces), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&psharedresource), ::core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn QueryResourceResidency<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).QueryResourceResidency(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&presidencystatus), ::core::mem::transmute_copy(&numresources)).into()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGPUThreadPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGPUThreadPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *ppriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, Impl, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, Impl, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice1_Impl: Sized + IDXGIObject_Impl + IDXGIDevice_Impl {
    fn SetMaximumFrameLatency(&mut self, maxlatency: u32) -> ::windows::core::Result<()>;
    fn GetMaximumFrameLatency(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice1_Impl, const OFFSET: isize>() -> IDXGIDevice1_Vtbl {
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumFrameLatency(::core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaximumFrameLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *pmaxlatency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice1 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDevice as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice2_Impl: Sized + IDXGIObject_Impl + IDXGIDevice_Impl + IDXGIDevice1_Impl {
    fn OfferResources(&mut self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()>;
    fn ReclaimResources(&mut self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EnqueueSetEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice2_Impl, const OFFSET: isize>() -> IDXGIDevice2_Vtbl {
        unsafe extern "system" fn OfferResources<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OfferResources(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn ReclaimResources<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReclaimResources(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdiscarded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnqueueSetEvent<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnqueueSetEvent(::core::mem::transmute_copy(&hevent)).into()
        }
        Self {
            base: IDXGIDevice1_Vtbl::new::<Identity, Impl, OFFSET>(),
            OfferResources: OfferResources::<Identity, Impl, OFFSET>,
            ReclaimResources: ReclaimResources::<Identity, Impl, OFFSET>,
            EnqueueSetEvent: EnqueueSetEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice2 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDevice as ::windows::core::Interface>::IID || iid == &<IDXGIDevice1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice3_Impl: Sized + IDXGIObject_Impl + IDXGIDevice_Impl + IDXGIDevice1_Impl + IDXGIDevice2_Impl {
    fn Trim(&mut self);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice3_Impl, const OFFSET: isize>() -> IDXGIDevice3_Vtbl {
        unsafe extern "system" fn Trim<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Trim()
        }
        Self { base: IDXGIDevice2_Vtbl::new::<Identity, Impl, OFFSET>(), Trim: Trim::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice3 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDevice as ::windows::core::Interface>::IID || iid == &<IDXGIDevice1 as ::windows::core::Interface>::IID || iid == &<IDXGIDevice2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice4_Impl: Sized + IDXGIObject_Impl + IDXGIDevice_Impl + IDXGIDevice1_Impl + IDXGIDevice2_Impl + IDXGIDevice3_Impl {
    fn OfferResources1(&mut self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::Result<()>;
    fn ReclaimResources1(&mut self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>) -> ::windows::core::Result<DXGI_RECLAIM_RESOURCE_RESULTS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice4_Impl, const OFFSET: isize>() -> IDXGIDevice4_Vtbl {
        unsafe extern "system" fn OfferResources1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OfferResources1(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ReclaimResources1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const ::windows::core::RawPtr, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReclaimResources1(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources)) {
                ::core::result::Result::Ok(ok__) => {
                    *presults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIDevice3_Vtbl::new::<Identity, Impl, OFFSET>(),
            OfferResources1: OfferResources1::<Identity, Impl, OFFSET>,
            ReclaimResources1: ReclaimResources1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice4 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDevice as ::windows::core::Interface>::IID || iid == &<IDXGIDevice1 as ::windows::core::Interface>::IID || iid == &<IDXGIDevice2 as ::windows::core::Interface>::IID || iid == &<IDXGIDevice3 as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIDeviceSubObject_Impl: Sized + IDXGIObject_Impl {
    fn GetDevice(&mut self, riid: *const ::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDXGIDeviceSubObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDeviceSubObject_Impl, const OFFSET: isize>() -> IDXGIDeviceSubObject_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDeviceSubObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdevice)).into()
        }
        Self { base: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(), GetDevice: GetDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDisplayControl_Impl: Sized {
    fn IsStereoEnabled(&mut self) -> super::super::Foundation::BOOL;
    fn SetStereoEnabled(&mut self, enabled: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDisplayControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDisplayControl_Impl, const OFFSET: isize>() -> IDXGIDisplayControl_Vtbl {
        unsafe extern "system" fn IsStereoEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsStereoEnabled()
        }
        unsafe extern "system" fn SetStereoEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetStereoEnabled(::core::mem::transmute_copy(&enabled))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsStereoEnabled: IsStereoEnabled::<Identity, Impl, OFFSET>,
            SetStereoEnabled: SetStereoEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDisplayControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory_Impl: Sized + IDXGIObject_Impl {
    fn EnumAdapters(&mut self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter>;
    fn MakeWindowAssociation(&mut self, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::Result<()>;
    fn GetWindowAssociation(&mut self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn CreateSwapChain(&mut self, pdevice: &::core::option::Option<::windows::core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<IDXGISwapChain>;
    fn CreateSoftwareAdapter(&mut self, module: super::super::Foundation::HINSTANCE) -> ::windows::core::Result<IDXGIAdapter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory_Impl, const OFFSET: isize>() -> IDXGIFactory_Vtbl {
        unsafe extern "system" fn EnumAdapters<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumAdapters(::core::mem::transmute_copy(&adapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppadapter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeWindowAssociation<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MakeWindowAssociation(::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetWindowAssociation<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetWindowAssociation() {
                ::core::result::Result::Ok(ok__) => {
                    *pwindowhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChain<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSwapChain(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pdesc)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::HINSTANCE, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSoftwareAdapter(::core::mem::transmute_copy(&module)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppadapter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAdapters: EnumAdapters::<Identity, Impl, OFFSET>,
            MakeWindowAssociation: MakeWindowAssociation::<Identity, Impl, OFFSET>,
            GetWindowAssociation: GetWindowAssociation::<Identity, Impl, OFFSET>,
            CreateSwapChain: CreateSwapChain::<Identity, Impl, OFFSET>,
            CreateSoftwareAdapter: CreateSoftwareAdapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory1_Impl: Sized + IDXGIObject_Impl + IDXGIFactory_Impl {
    fn EnumAdapters1(&mut self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1>;
    fn IsCurrent(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory1_Impl, const OFFSET: isize>() -> IDXGIFactory1_Vtbl {
        unsafe extern "system" fn EnumAdapters1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumAdapters1(::core::mem::transmute_copy(&adapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppadapter = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrent<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsCurrent()
        }
        Self {
            base: IDXGIFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAdapters1: EnumAdapters1::<Identity, Impl, OFFSET>,
            IsCurrent: IsCurrent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory1 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory2_Impl: Sized + IDXGIObject_Impl + IDXGIFactory_Impl + IDXGIFactory1_Impl {
    fn IsWindowedStereoEnabled(&mut self) -> super::super::Foundation::BOOL;
    fn CreateSwapChainForHwnd(&mut self, pdevice: &::core::option::Option<::windows::core::IUnknown>, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: &::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
    fn CreateSwapChainForCoreWindow(&mut self, pdevice: &::core::option::Option<::windows::core::IUnknown>, pwindow: &::core::option::Option<::windows::core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: &::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
    fn GetSharedResourceAdapterLuid(&mut self, hresource: super::super::Foundation::HANDLE) -> ::windows::core::Result<super::super::Foundation::LUID>;
    fn RegisterStereoStatusWindow(&mut self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> ::windows::core::Result<u32>;
    fn RegisterStereoStatusEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterStereoStatus(&mut self, dwcookie: u32);
    fn RegisterOcclusionStatusWindow(&mut self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> ::windows::core::Result<u32>;
    fn RegisterOcclusionStatusEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterOcclusionStatus(&mut self, dwcookie: u32);
    fn CreateSwapChainForComposition(&mut self, pdevice: &::core::option::Option<::windows::core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: &::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>() -> IDXGIFactory2_Vtbl {
        unsafe extern "system" fn IsWindowedStereoEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsWindowedStereoEnabled()
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSwapChainForHwnd(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pfullscreendesc), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSwapChainForCoreWindow(::core::mem::transmute(&pdevice), ::core::mem::transmute(&pwindow), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSharedResourceAdapterLuid(::core::mem::transmute_copy(&hresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *pluid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterStereoStatusWindow(::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&wmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterStereoStatusEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterStereoStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterStereoStatus(::core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterOcclusionStatusWindow(::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&wmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterOcclusionStatusEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterOcclusionStatus(::core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSwapChainForComposition(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIFactory1_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsWindowedStereoEnabled: IsWindowedStereoEnabled::<Identity, Impl, OFFSET>,
            CreateSwapChainForHwnd: CreateSwapChainForHwnd::<Identity, Impl, OFFSET>,
            CreateSwapChainForCoreWindow: CreateSwapChainForCoreWindow::<Identity, Impl, OFFSET>,
            GetSharedResourceAdapterLuid: GetSharedResourceAdapterLuid::<Identity, Impl, OFFSET>,
            RegisterStereoStatusWindow: RegisterStereoStatusWindow::<Identity, Impl, OFFSET>,
            RegisterStereoStatusEvent: RegisterStereoStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterStereoStatus: UnregisterStereoStatus::<Identity, Impl, OFFSET>,
            RegisterOcclusionStatusWindow: RegisterOcclusionStatusWindow::<Identity, Impl, OFFSET>,
            RegisterOcclusionStatusEvent: RegisterOcclusionStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterOcclusionStatus: UnregisterOcclusionStatus::<Identity, Impl, OFFSET>,
            CreateSwapChainForComposition: CreateSwapChainForComposition::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory2 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIFactory as ::windows::core::Interface>::IID || iid == &<IDXGIFactory1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory3_Impl: Sized + IDXGIObject_Impl + IDXGIFactory_Impl + IDXGIFactory1_Impl + IDXGIFactory2_Impl {
    fn GetCreationFlags(&mut self) -> u32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory3_Impl, const OFFSET: isize>() -> IDXGIFactory3_Vtbl {
        unsafe extern "system" fn GetCreationFlags<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCreationFlags()
        }
        Self { base: IDXGIFactory2_Vtbl::new::<Identity, Impl, OFFSET>(), GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory3 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIFactory as ::windows::core::Interface>::IID || iid == &<IDXGIFactory1 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory4_Impl: Sized + IDXGIObject_Impl + IDXGIFactory_Impl + IDXGIFactory1_Impl + IDXGIFactory2_Impl + IDXGIFactory3_Impl {
    fn EnumAdapterByLuid(&mut self, adapterluid: &super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn EnumWarpAdapter(&mut self, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory4_Impl, const OFFSET: isize>() -> IDXGIFactory4_Vtbl {
        unsafe extern "system" fn EnumAdapterByLuid<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAdapterByLuid(::core::mem::transmute_copy(&adapterluid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn EnumWarpAdapter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumWarpAdapter(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self {
            base: IDXGIFactory3_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAdapterByLuid: EnumAdapterByLuid::<Identity, Impl, OFFSET>,
            EnumWarpAdapter: EnumWarpAdapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory4 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIFactory as ::windows::core::Interface>::IID || iid == &<IDXGIFactory1 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory2 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory5_Impl: Sized + IDXGIObject_Impl + IDXGIFactory_Impl + IDXGIFactory1_Impl + IDXGIFactory2_Impl + IDXGIFactory3_Impl + IDXGIFactory4_Impl {
    fn CheckFeatureSupport(&mut self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory5_Impl, const OFFSET: isize>() -> IDXGIFactory5_Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        Self { base: IDXGIFactory4_Vtbl::new::<Identity, Impl, OFFSET>(), CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory5 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIFactory as ::windows::core::Interface>::IID || iid == &<IDXGIFactory1 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory2 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory3 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory6_Impl: Sized + IDXGIObject_Impl + IDXGIFactory_Impl + IDXGIFactory1_Impl + IDXGIFactory2_Impl + IDXGIFactory3_Impl + IDXGIFactory4_Impl + IDXGIFactory5_Impl {
    fn EnumAdapterByGpuPreference(&mut self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory6_Impl, const OFFSET: isize>() -> IDXGIFactory6_Vtbl {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EnumAdapterByGpuPreference(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&gpupreference), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self { base: IDXGIFactory5_Vtbl::new::<Identity, Impl, OFFSET>(), EnumAdapterByGpuPreference: EnumAdapterByGpuPreference::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory6 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIFactory as ::windows::core::Interface>::IID || iid == &<IDXGIFactory1 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory2 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory3 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory4 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory7_Impl: Sized + IDXGIObject_Impl + IDXGIFactory_Impl + IDXGIFactory1_Impl + IDXGIFactory2_Impl + IDXGIFactory3_Impl + IDXGIFactory4_Impl + IDXGIFactory5_Impl + IDXGIFactory6_Impl {
    fn RegisterAdaptersChangedEvent(&mut self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterAdaptersChangedEvent(&mut self, dwcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory7_Impl, const OFFSET: isize>() -> IDXGIFactory7_Vtbl {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RegisterAdaptersChangedEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnregisterAdaptersChangedEvent(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base: IDXGIFactory6_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterAdaptersChangedEvent: RegisterAdaptersChangedEvent::<Identity, Impl, OFFSET>,
            UnregisterAdaptersChangedEvent: UnregisterAdaptersChangedEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory7 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIFactory as ::windows::core::Interface>::IID || iid == &<IDXGIFactory1 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory2 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory3 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory4 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory5 as ::windows::core::Interface>::IID || iid == &<IDXGIFactory6 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactoryMedia_Impl: Sized {
    fn CreateSwapChainForCompositionSurfaceHandle(&mut self, pdevice: &::core::option::Option<::windows::core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: &::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(&mut self, pdevice: &::core::option::Option<::windows::core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: &::core::option::Option<IDXGIResource>, prestricttooutput: &::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<IDXGIDecodeSwapChain>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactoryMedia_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactoryMedia_Impl, const OFFSET: isize>() -> IDXGIFactoryMedia_Vtbl {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSwapChainForCompositionSurfaceHandle(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hsurface), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: ::windows::core::RawPtr, prestricttooutput: ::windows::core::RawPtr, ppswapchain: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateDecodeSwapChainForCompositionSurfaceHandle(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&hsurface), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute(&pyuvdecodebuffers), ::core::mem::transmute(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppswapchain = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateSwapChainForCompositionSurfaceHandle: CreateSwapChainForCompositionSurfaceHandle::<Identity, Impl, OFFSET>,
            CreateDecodeSwapChainForCompositionSurfaceHandle: CreateDecodeSwapChainForCompositionSurfaceHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactoryMedia as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIInfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&mut self, producer: &::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::Result<()>;
    fn ClearStoredMessages(&mut self, producer: &::windows::core::GUID);
    fn GetMessage(&mut self, producer: &::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()>;
    fn GetNumStoredMessagesAllowedByRetrievalFilters(&mut self, producer: &::windows::core::GUID) -> u64;
    fn GetNumStoredMessages(&mut self, producer: &::windows::core::GUID) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&mut self, producer: &::windows::core::GUID) -> u64;
    fn GetMessageCountLimit(&mut self, producer: &::windows::core::GUID) -> u64;
    fn GetNumMessagesAllowedByStorageFilter(&mut self, producer: &::windows::core::GUID) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&mut self, producer: &::windows::core::GUID) -> u64;
    fn AddStorageFilterEntries(&mut self, producer: &::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetStorageFilter(&mut self, producer: &::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearStorageFilter(&mut self, producer: &::windows::core::GUID);
    fn PushEmptyStorageFilter(&mut self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushDenyAllStorageFilter(&mut self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushCopyOfStorageFilter(&mut self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushStorageFilter(&mut self, producer: &::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopStorageFilter(&mut self, producer: &::windows::core::GUID);
    fn GetStorageFilterStackSize(&mut self, producer: &::windows::core::GUID) -> u32;
    fn AddRetrievalFilterEntries(&mut self, producer: &::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetRetrievalFilter(&mut self, producer: &::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearRetrievalFilter(&mut self, producer: &::windows::core::GUID);
    fn PushEmptyRetrievalFilter(&mut self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushDenyAllRetrievalFilter(&mut self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushCopyOfRetrievalFilter(&mut self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushRetrievalFilter(&mut self, producer: &::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopRetrievalFilter(&mut self, producer: &::windows::core::GUID);
    fn GetRetrievalFilterStackSize(&mut self, producer: &::windows::core::GUID) -> u32;
    fn AddMessage(&mut self, producer: &::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn AddApplicationMessage(&mut self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::Result<()>;
    fn SetBreakOnCategory(&mut self, producer: &::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnSeverity(&mut self, producer: &::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnID(&mut self, producer: &::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBreakOnCategory(&mut self, producer: &::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&mut self, producer: &::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&mut self, producer: &::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&mut self, producer: &::windows::core::GUID, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&mut self, producer: &::windows::core::GUID) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIInfoQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>() -> IDXGIInfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMessageCountLimit(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearStoredMessages(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMessage(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumStoredMessagesAllowedByRetrievalFilters(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumStoredMessages(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumMessagesDiscardedByMessageCountLimit(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMessageCountLimit(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumMessagesAllowedByStorageFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetNumMessagesDeniedByStorageFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStorageFilterEntries(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStorageFilter(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearStorageFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushEmptyStorageFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushDenyAllStorageFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushCopyOfStorageFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushStorageFilter(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PopStorageFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStorageFilterStackSize(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRetrievalFilterEntries(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRetrievalFilter(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearRetrievalFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushEmptyRetrievalFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushDenyAllRetrievalFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushCopyOfRetrievalFilter(::core::mem::transmute_copy(&producer)).into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PushRetrievalFilter(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PopRetrievalFilter(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetRetrievalFilterStackSize(::core::mem::transmute_copy(&producer))
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMessage(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: super::super::Foundation::PSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddApplicationMessage(::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreakOnCategory(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreakOnSeverity(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBreakOnID(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBreakOnCategory(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBreakOnSeverity(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBreakOnID(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, bmute: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMuteDebugOutput(::core::mem::transmute_copy(&producer), ::core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetMuteDebugOutput(::core::mem::transmute_copy(&producer))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetMessageCountLimit: SetMessageCountLimit::<Identity, Impl, OFFSET>,
            ClearStoredMessages: ClearStoredMessages::<Identity, Impl, OFFSET>,
            GetMessage: GetMessage::<Identity, Impl, OFFSET>,
            GetNumStoredMessagesAllowedByRetrievalFilters: GetNumStoredMessagesAllowedByRetrievalFilters::<Identity, Impl, OFFSET>,
            GetNumStoredMessages: GetNumStoredMessages::<Identity, Impl, OFFSET>,
            GetNumMessagesDiscardedByMessageCountLimit: GetNumMessagesDiscardedByMessageCountLimit::<Identity, Impl, OFFSET>,
            GetMessageCountLimit: GetMessageCountLimit::<Identity, Impl, OFFSET>,
            GetNumMessagesAllowedByStorageFilter: GetNumMessagesAllowedByStorageFilter::<Identity, Impl, OFFSET>,
            GetNumMessagesDeniedByStorageFilter: GetNumMessagesDeniedByStorageFilter::<Identity, Impl, OFFSET>,
            AddStorageFilterEntries: AddStorageFilterEntries::<Identity, Impl, OFFSET>,
            GetStorageFilter: GetStorageFilter::<Identity, Impl, OFFSET>,
            ClearStorageFilter: ClearStorageFilter::<Identity, Impl, OFFSET>,
            PushEmptyStorageFilter: PushEmptyStorageFilter::<Identity, Impl, OFFSET>,
            PushDenyAllStorageFilter: PushDenyAllStorageFilter::<Identity, Impl, OFFSET>,
            PushCopyOfStorageFilter: PushCopyOfStorageFilter::<Identity, Impl, OFFSET>,
            PushStorageFilter: PushStorageFilter::<Identity, Impl, OFFSET>,
            PopStorageFilter: PopStorageFilter::<Identity, Impl, OFFSET>,
            GetStorageFilterStackSize: GetStorageFilterStackSize::<Identity, Impl, OFFSET>,
            AddRetrievalFilterEntries: AddRetrievalFilterEntries::<Identity, Impl, OFFSET>,
            GetRetrievalFilter: GetRetrievalFilter::<Identity, Impl, OFFSET>,
            ClearRetrievalFilter: ClearRetrievalFilter::<Identity, Impl, OFFSET>,
            PushEmptyRetrievalFilter: PushEmptyRetrievalFilter::<Identity, Impl, OFFSET>,
            PushDenyAllRetrievalFilter: PushDenyAllRetrievalFilter::<Identity, Impl, OFFSET>,
            PushCopyOfRetrievalFilter: PushCopyOfRetrievalFilter::<Identity, Impl, OFFSET>,
            PushRetrievalFilter: PushRetrievalFilter::<Identity, Impl, OFFSET>,
            PopRetrievalFilter: PopRetrievalFilter::<Identity, Impl, OFFSET>,
            GetRetrievalFilterStackSize: GetRetrievalFilterStackSize::<Identity, Impl, OFFSET>,
            AddMessage: AddMessage::<Identity, Impl, OFFSET>,
            AddApplicationMessage: AddApplicationMessage::<Identity, Impl, OFFSET>,
            SetBreakOnCategory: SetBreakOnCategory::<Identity, Impl, OFFSET>,
            SetBreakOnSeverity: SetBreakOnSeverity::<Identity, Impl, OFFSET>,
            SetBreakOnID: SetBreakOnID::<Identity, Impl, OFFSET>,
            GetBreakOnCategory: GetBreakOnCategory::<Identity, Impl, OFFSET>,
            GetBreakOnSeverity: GetBreakOnSeverity::<Identity, Impl, OFFSET>,
            GetBreakOnID: GetBreakOnID::<Identity, Impl, OFFSET>,
            SetMuteDebugOutput: SetMuteDebugOutput::<Identity, Impl, OFFSET>,
            GetMuteDebugOutput: GetMuteDebugOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIInfoQueue as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIKeyedMutex_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl {
    fn AcquireSync(&mut self, key: u64, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn ReleaseSync(&mut self, key: u64) -> ::windows::core::Result<()>;
}
impl IDXGIKeyedMutex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIKeyedMutex_Impl, const OFFSET: isize>() -> IDXGIKeyedMutex_Vtbl {
        unsafe extern "system" fn AcquireSync<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireSync(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn ReleaseSync<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseSync(::core::mem::transmute_copy(&key)).into()
        }
        Self {
            base: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            AcquireSync: AcquireSync::<Identity, Impl, OFFSET>,
            ReleaseSync: ReleaseSync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIKeyedMutex as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID
    }
}
pub trait IDXGIObject_Impl: Sized {
    fn SetPrivateData(&mut self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&mut self, name: *const ::windows::core::GUID, punknown: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetPrivateData(&mut self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetParent(&mut self, riid: *const ::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDXGIObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIObject_Impl, const OFFSET: isize>() -> IDXGIObject_Vtbl {
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateData(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPrivateDataInterface(::core::mem::transmute_copy(&name), ::core::mem::transmute(&punknown)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrivateData(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetParent<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetParent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparent)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput_Impl: Sized + IDXGIObject_Impl {
    fn GetDesc(&mut self) -> ::windows::core::Result<DXGI_OUTPUT_DESC>;
    fn GetDisplayModeList(&mut self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()>;
    fn FindClosestMatchingMode(&mut self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn WaitForVBlank(&mut self) -> ::windows::core::Result<()>;
    fn TakeOwnership(&mut self, pdevice: &::core::option::Option<::windows::core::IUnknown>, exclusive: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ReleaseOwnership(&mut self);
    fn GetGammaControlCapabilities(&mut self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL_CAPABILITIES>;
    fn SetGammaControl(&mut self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()>;
    fn GetGammaControl(&mut self) -> ::windows::core::Result<Common::DXGI_GAMMA_CONTROL>;
    fn SetDisplaySurface(&mut self, pscanoutsurface: &::core::option::Option<IDXGISurface>) -> ::windows::core::Result<()>;
    fn GetDisplaySurfaceData(&mut self, pdestination: &::core::option::Option<IDXGISurface>) -> ::windows::core::Result<()>;
    fn GetFrameStatistics(&mut self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>() -> IDXGIOutput_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayModeList<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayModeList(::core::mem::transmute_copy(&enumformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pnummodes), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindClosestMatchingMode(::core::mem::transmute_copy(&pmodetomatch), ::core::mem::transmute_copy(&pclosestmatch), ::core::mem::transmute(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn WaitForVBlank<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).WaitForVBlank().into()
        }
        unsafe extern "system" fn TakeOwnership<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).TakeOwnership(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&exclusive)).into()
        }
        unsafe extern "system" fn ReleaseOwnership<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseOwnership()
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGammaControlCapabilities() {
                ::core::result::Result::Ok(ok__) => {
                    *pgammacaps = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGammaControl<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGammaControl(::core::mem::transmute_copy(&parray)).into()
        }
        unsafe extern "system" fn GetGammaControl<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGammaControl() {
                ::core::result::Result::Ok(ok__) => {
                    *parray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplaySurface<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscanoutsurface: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDisplaySurface(::core::mem::transmute(&pscanoutsurface)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplaySurfaceData(::core::mem::transmute(&pdestination)).into()
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            GetDisplayModeList: GetDisplayModeList::<Identity, Impl, OFFSET>,
            FindClosestMatchingMode: FindClosestMatchingMode::<Identity, Impl, OFFSET>,
            WaitForVBlank: WaitForVBlank::<Identity, Impl, OFFSET>,
            TakeOwnership: TakeOwnership::<Identity, Impl, OFFSET>,
            ReleaseOwnership: ReleaseOwnership::<Identity, Impl, OFFSET>,
            GetGammaControlCapabilities: GetGammaControlCapabilities::<Identity, Impl, OFFSET>,
            SetGammaControl: SetGammaControl::<Identity, Impl, OFFSET>,
            GetGammaControl: GetGammaControl::<Identity, Impl, OFFSET>,
            SetDisplaySurface: SetDisplaySurface::<Identity, Impl, OFFSET>,
            GetDisplaySurfaceData: GetDisplaySurfaceData::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput1_Impl: Sized + IDXGIObject_Impl + IDXGIOutput_Impl {
    fn GetDisplayModeList1(&mut self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::Result<()>;
    fn FindClosestMatchingMode1(&mut self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetDisplaySurfaceData1(&mut self, pdestination: &::core::option::Option<IDXGIResource>) -> ::windows::core::Result<()>;
    fn DuplicateOutput(&mut self, pdevice: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput1_Impl, const OFFSET: isize>() -> IDXGIOutput1_Vtbl {
        unsafe extern "system" fn GetDisplayModeList1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplayModeList1(::core::mem::transmute_copy(&enumformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pnummodes), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FindClosestMatchingMode1(::core::mem::transmute_copy(&pmodetomatch), ::core::mem::transmute_copy(&pclosestmatch), ::core::mem::transmute(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDisplaySurfaceData1(::core::mem::transmute(&pdestination)).into()
        }
        unsafe extern "system" fn DuplicateOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DuplicateOutput(::core::mem::transmute(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutputduplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIOutput_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDisplayModeList1: GetDisplayModeList1::<Identity, Impl, OFFSET>,
            FindClosestMatchingMode1: FindClosestMatchingMode1::<Identity, Impl, OFFSET>,
            GetDisplaySurfaceData1: GetDisplaySurfaceData1::<Identity, Impl, OFFSET>,
            DuplicateOutput: DuplicateOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput1 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIOutput as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput2_Impl: Sized + IDXGIObject_Impl + IDXGIOutput_Impl + IDXGIOutput1_Impl {
    fn SupportsOverlays(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput2_Impl, const OFFSET: isize>() -> IDXGIOutput2_Vtbl {
        unsafe extern "system" fn SupportsOverlays<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SupportsOverlays()
        }
        Self { base: IDXGIOutput1_Vtbl::new::<Identity, Impl, OFFSET>(), SupportsOverlays: SupportsOverlays::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput2 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIOutput as ::windows::core::Interface>::IID || iid == &<IDXGIOutput1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput3_Impl: Sized + IDXGIObject_Impl + IDXGIOutput_Impl + IDXGIOutput1_Impl + IDXGIOutput2_Impl {
    fn CheckOverlaySupport(&mut self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput3_Impl, const OFFSET: isize>() -> IDXGIOutput3_Vtbl {
        unsafe extern "system" fn CheckOverlaySupport<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckOverlaySupport(::core::mem::transmute_copy(&enumformat), ::core::mem::transmute(&pconcerneddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIOutput2_Vtbl::new::<Identity, Impl, OFFSET>(), CheckOverlaySupport: CheckOverlaySupport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput3 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIOutput as ::windows::core::Interface>::IID || iid == &<IDXGIOutput1 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput4_Impl: Sized + IDXGIObject_Impl + IDXGIOutput_Impl + IDXGIOutput1_Impl + IDXGIOutput2_Impl + IDXGIOutput3_Impl {
    fn CheckOverlayColorSpaceSupport(&mut self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput4_Impl, const OFFSET: isize>() -> IDXGIOutput4_Vtbl {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckOverlayColorSpaceSupport(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&colorspace), ::core::mem::transmute(&pconcerneddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIOutput3_Vtbl::new::<Identity, Impl, OFFSET>(), CheckOverlayColorSpaceSupport: CheckOverlayColorSpaceSupport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput4 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIOutput as ::windows::core::Interface>::IID || iid == &<IDXGIOutput1 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput2 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput5_Impl: Sized + IDXGIObject_Impl + IDXGIOutput_Impl + IDXGIOutput1_Impl + IDXGIOutput2_Impl + IDXGIOutput3_Impl + IDXGIOutput4_Impl {
    fn DuplicateOutput1(&mut self, pdevice: &::core::option::Option<::windows::core::IUnknown>, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT) -> ::windows::core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput5_Impl, const OFFSET: isize>() -> IDXGIOutput5_Vtbl {
        unsafe extern "system" fn DuplicateOutput1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DuplicateOutput1(::core::mem::transmute(&pdevice), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&supportedformatscount), ::core::mem::transmute_copy(&psupportedformats)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutputduplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IDXGIOutput4_Vtbl::new::<Identity, Impl, OFFSET>(), DuplicateOutput1: DuplicateOutput1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput5 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIOutput as ::windows::core::Interface>::IID || iid == &<IDXGIOutput1 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput2 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput3 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput4 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput6_Impl: Sized + IDXGIObject_Impl + IDXGIOutput_Impl + IDXGIOutput1_Impl + IDXGIOutput2_Impl + IDXGIOutput3_Impl + IDXGIOutput4_Impl + IDXGIOutput5_Impl {
    fn GetDesc1(&mut self) -> ::windows::core::Result<DXGI_OUTPUT_DESC1>;
    fn CheckHardwareCompositionSupport(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput6_Impl, const OFFSET: isize>() -> IDXGIOutput6_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc1() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckHardwareCompositionSupport() {
                ::core::result::Result::Ok(ok__) => {
                    *pflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIOutput5_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, Impl, OFFSET>,
            CheckHardwareCompositionSupport: CheckHardwareCompositionSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput6 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIOutput as ::windows::core::Interface>::IID || iid == &<IDXGIOutput1 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput2 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput3 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput4 as ::windows::core::Interface>::IID || iid == &<IDXGIOutput5 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIOutputDuplication_Impl: Sized + IDXGIObject_Impl {
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
impl IDXGIOutputDuplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>() -> IDXGIOutputDuplication_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn AcquireNextFrame<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AcquireNextFrame(::core::mem::transmute_copy(&timeoutinmilliseconds), ::core::mem::transmute_copy(&pframeinfo), ::core::mem::transmute_copy(&ppdesktopresource)).into()
        }
        unsafe extern "system" fn GetFrameDirtyRects<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFrameDirtyRects(::core::mem::transmute_copy(&dirtyrectsbuffersize), ::core::mem::transmute_copy(&pdirtyrectsbuffer), ::core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFrameMoveRects<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFrameMoveRects(::core::mem::transmute_copy(&moverectsbuffersize), ::core::mem::transmute_copy(&pmoverectbuffer), ::core::mem::transmute_copy(&pmoverectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFramePointerShape<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFramePointerShape(::core::mem::transmute_copy(&pointershapebuffersize), ::core::mem::transmute_copy(&ppointershapebuffer), ::core::mem::transmute_copy(&ppointershapebuffersizerequired), ::core::mem::transmute_copy(&ppointershapeinfo)).into()
        }
        unsafe extern "system" fn MapDesktopSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MapDesktopSurface() {
                ::core::result::Result::Ok(ok__) => {
                    *plockedrect = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnMapDesktopSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnMapDesktopSurface().into()
        }
        unsafe extern "system" fn ReleaseFrame<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseFrame().into()
        }
        Self {
            base: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            AcquireNextFrame: AcquireNextFrame::<Identity, Impl, OFFSET>,
            GetFrameDirtyRects: GetFrameDirtyRects::<Identity, Impl, OFFSET>,
            GetFrameMoveRects: GetFrameMoveRects::<Identity, Impl, OFFSET>,
            GetFramePointerShape: GetFramePointerShape::<Identity, Impl, OFFSET>,
            MapDesktopSurface: MapDesktopSurface::<Identity, Impl, OFFSET>,
            UnMapDesktopSurface: UnMapDesktopSurface::<Identity, Impl, OFFSET>,
            ReleaseFrame: ReleaseFrame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutputDuplication as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIResource_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl {
    fn GetSharedHandle(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn GetUsage(&mut self) -> ::windows::core::Result<u32>;
    fn SetEvictionPriority(&mut self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::Result<()>;
    fn GetEvictionPriority(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource_Impl, const OFFSET: isize>() -> IDXGIResource_Vtbl {
        unsafe extern "system" fn GetSharedHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSharedHandle() {
                ::core::result::Result::Ok(ok__) => {
                    *psharedhandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsage<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusage: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUsage() {
                ::core::result::Result::Ok(ok__) => {
                    *pusage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEvictionPriority(::core::mem::transmute_copy(&evictionpriority)).into()
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevictionpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEvictionPriority() {
                ::core::result::Result::Ok(ok__) => {
                    *pevictionpriority = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSharedHandle: GetSharedHandle::<Identity, Impl, OFFSET>,
            GetUsage: GetUsage::<Identity, Impl, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, Impl, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIResource as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait IDXGIResource1_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl + IDXGIResource_Impl {
    fn CreateSubresourceSurface(&mut self, index: u32) -> ::windows::core::Result<IDXGISurface2>;
    fn CreateSharedHandle(&mut self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IDXGIResource1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource1_Impl, const OFFSET: isize>() -> IDXGIResource1_Vtbl {
        unsafe extern "system" fn CreateSubresourceSurface<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppsurface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSubresourceSurface(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsurface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: ::windows::core::IUnknownImpl, Impl: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: super::super::Foundation::PWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSharedHandle(::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&dwaccess), ::core::mem::transmute_copy(&lpname)) {
                ::core::result::Result::Ok(ok__) => {
                    *phandle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateSubresourceSurface: CreateSubresourceSurface::<Identity, Impl, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIResource1 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID || iid == &<IDXGIResource as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISurface_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl {
    fn GetDesc(&mut self) -> ::windows::core::Result<DXGI_SURFACE_DESC>;
    fn Map(&mut self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()>;
    fn Unmap(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface_Impl, const OFFSET: isize>() -> IDXGISurface_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Map<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Map(::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&mapflags)).into()
        }
        unsafe extern "system" fn Unmap<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unmap().into()
        }
        Self {
            base: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface1_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl + IDXGISurface_Impl {
    fn GetDC(&mut self, discard: super::super::Foundation::BOOL) -> ::windows::core::Result<super::Gdi::HDC>;
    fn ReleaseDC(&mut self, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface1_Impl, const OFFSET: isize>() -> IDXGISurface1_Vtbl {
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDC(::core::mem::transmute_copy(&discard)) {
                ::core::result::Result::Ok(ok__) => {
                    *phdc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ReleaseDC(::core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self { base: IDXGISurface_Vtbl::new::<Identity, Impl, OFFSET>(), GetDC: GetDC::<Identity, Impl, OFFSET>, ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface1 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID || iid == &<IDXGISurface as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface2_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl + IDXGISurface_Impl + IDXGISurface1_Impl {
    fn GetResource(&mut self, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface2_Impl, const OFFSET: isize>() -> IDXGISurface2_Vtbl {
        unsafe extern "system" fn GetResource<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetResource(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparentresource), ::core::mem::transmute_copy(&psubresourceindex)).into()
        }
        Self { base: IDXGISurface1_Vtbl::new::<Identity, Impl, OFFSET>(), GetResource: GetResource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface2 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID || iid == &<IDXGISurface as ::windows::core::Interface>::IID || iid == &<IDXGISurface1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl {
    fn Present(&mut self, syncinterval: u32, flags: u32) -> ::windows::core::Result<()>;
    fn GetBuffer(&mut self, buffer: u32, riid: *const ::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetFullscreenState(&mut self, fullscreen: super::super::Foundation::BOOL, ptarget: &::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()>;
    fn GetFullscreenState(&mut self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()>;
    fn GetDesc(&mut self) -> ::windows::core::Result<DXGI_SWAP_CHAIN_DESC>;
    fn ResizeBuffers(&mut self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()>;
    fn ResizeTarget(&mut self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()>;
    fn GetContainingOutput(&mut self) -> ::windows::core::Result<IDXGIOutput>;
    fn GetFrameStatistics(&mut self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS>;
    fn GetLastPresentCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>() -> IDXGISwapChain_Vtbl {
        unsafe extern "system" fn Present<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Present(::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: u32, riid: *const ::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn SetFullscreenState<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFullscreenState(::core::mem::transmute_copy(&fullscreen), ::core::mem::transmute(&ptarget)).into()
        }
        unsafe extern "system" fn GetFullscreenState<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFullscreenState(::core::mem::transmute_copy(&pfullscreen), ::core::mem::transmute_copy(&pptarget)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResizeBuffers<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResizeBuffers(::core::mem::transmute_copy(&buffercount), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&newformat), ::core::mem::transmute_copy(&swapchainflags)).into()
        }
        unsafe extern "system" fn ResizeTarget<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResizeTarget(::core::mem::transmute_copy(&pnewtargetparameters)).into()
        }
        unsafe extern "system" fn GetContainingOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContainingOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *ppoutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrameStatistics() {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLastPresentCount<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLastPresentCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plastpresentcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            Present: Present::<Identity, Impl, OFFSET>,
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            SetFullscreenState: SetFullscreenState::<Identity, Impl, OFFSET>,
            GetFullscreenState: GetFullscreenState::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            ResizeBuffers: ResizeBuffers::<Identity, Impl, OFFSET>,
            ResizeTarget: ResizeTarget::<Identity, Impl, OFFSET>,
            GetContainingOutput: GetContainingOutput::<Identity, Impl, OFFSET>,
            GetFrameStatistics: GetFrameStatistics::<Identity, Impl, OFFSET>,
            GetLastPresentCount: GetLastPresentCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain1_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl + IDXGISwapChain_Impl {
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
impl IDXGISwapChain1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>() -> IDXGISwapChain1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDesc1() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFullscreenDesc<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFullscreenDesc() {
                ::core::result::Result::Ok(ok__) => {
                    *pdesc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHwnd<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHwnd() {
                ::core::result::Result::Ok(ok__) => {
                    *phwnd = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCoreWindow<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCoreWindow(::core::mem::transmute_copy(&refiid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn Present1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Present1(::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&presentflags), ::core::mem::transmute_copy(&ppresentparameters)).into()
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsTemporaryMonoSupported()
        }
        unsafe extern "system" fn GetRestrictToOutput<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprestricttooutput: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRestrictToOutput() {
                ::core::result::Result::Ok(ok__) => {
                    *pprestricttooutput = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *const DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBackgroundColor(::core::mem::transmute_copy(&pcolor)).into()
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    *pcolor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRotation(::core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn GetRotation<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRotation() {
                ::core::result::Result::Ok(ok__) => {
                    *protation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGISwapChain_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, Impl, OFFSET>,
            GetFullscreenDesc: GetFullscreenDesc::<Identity, Impl, OFFSET>,
            GetHwnd: GetHwnd::<Identity, Impl, OFFSET>,
            GetCoreWindow: GetCoreWindow::<Identity, Impl, OFFSET>,
            Present1: Present1::<Identity, Impl, OFFSET>,
            IsTemporaryMonoSupported: IsTemporaryMonoSupported::<Identity, Impl, OFFSET>,
            GetRestrictToOutput: GetRestrictToOutput::<Identity, Impl, OFFSET>,
            SetBackgroundColor: SetBackgroundColor::<Identity, Impl, OFFSET>,
            GetBackgroundColor: GetBackgroundColor::<Identity, Impl, OFFSET>,
            SetRotation: SetRotation::<Identity, Impl, OFFSET>,
            GetRotation: GetRotation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain1 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain2_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl + IDXGISwapChain_Impl + IDXGISwapChain1_Impl {
    fn SetSourceSize(&mut self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn GetSourceSize(&mut self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()>;
    fn SetMaximumFrameLatency(&mut self, maxlatency: u32) -> ::windows::core::Result<()>;
    fn GetMaximumFrameLatency(&mut self) -> ::windows::core::Result<u32>;
    fn GetFrameLatencyWaitableObject(&mut self) -> super::super::Foundation::HANDLE;
    fn SetMatrixTransform(&mut self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()>;
    fn GetMatrixTransform(&mut self) -> ::windows::core::Result<DXGI_MATRIX_3X2_F>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>() -> IDXGISwapChain2_Vtbl {
        unsafe extern "system" fn SetSourceSize<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSourceSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceSize<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSourceSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaximumFrameLatency(::core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMaximumFrameLatency() {
                ::core::result::Result::Ok(ok__) => {
                    *pmaxlatency = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetFrameLatencyWaitableObject()
        }
        unsafe extern "system" fn SetMatrixTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMatrixTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetMatrixTransform<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMatrixTransform() {
                ::core::result::Result::Ok(ok__) => {
                    *pmatrix = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDXGISwapChain1_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSourceSize: SetSourceSize::<Identity, Impl, OFFSET>,
            GetSourceSize: GetSourceSize::<Identity, Impl, OFFSET>,
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetFrameLatencyWaitableObject: GetFrameLatencyWaitableObject::<Identity, Impl, OFFSET>,
            SetMatrixTransform: SetMatrixTransform::<Identity, Impl, OFFSET>,
            GetMatrixTransform: GetMatrixTransform::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain2 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain1 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain3_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl + IDXGISwapChain_Impl + IDXGISwapChain1_Impl + IDXGISwapChain2_Impl {
    fn GetCurrentBackBufferIndex(&mut self) -> u32;
    fn CheckColorSpaceSupport(&mut self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<u32>;
    fn SetColorSpace1(&mut self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()>;
    fn ResizeBuffers1(&mut self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>() -> IDXGISwapChain3_Vtbl {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCurrentBackBufferIndex()
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CheckColorSpaceSupport(::core::mem::transmute_copy(&colorspace)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcolorspacesupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorSpace1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetColorSpace1(::core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn ResizeBuffers1<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResizeBuffers1(::core::mem::transmute_copy(&buffercount), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&swapchainflags), ::core::mem::transmute_copy(&pcreationnodemask), ::core::mem::transmute_copy(&pppresentqueue)).into()
        }
        Self {
            base: IDXGISwapChain2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCurrentBackBufferIndex: GetCurrentBackBufferIndex::<Identity, Impl, OFFSET>,
            CheckColorSpaceSupport: CheckColorSpaceSupport::<Identity, Impl, OFFSET>,
            SetColorSpace1: SetColorSpace1::<Identity, Impl, OFFSET>,
            ResizeBuffers1: ResizeBuffers1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain3 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain1 as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain4_Impl: Sized + IDXGIObject_Impl + IDXGIDeviceSubObject_Impl + IDXGISwapChain_Impl + IDXGISwapChain1_Impl + IDXGISwapChain2_Impl + IDXGISwapChain3_Impl {
    fn SetHDRMetaData(&mut self, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain4_Impl, const OFFSET: isize>() -> IDXGISwapChain4_Vtbl {
        unsafe extern "system" fn SetHDRMetaData<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChain4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetHDRMetaData(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata)).into()
        }
        Self { base: IDXGISwapChain3_Vtbl::new::<Identity, Impl, OFFSET>(), SetHDRMetaData: SetHDRMetaData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain4 as ::windows::core::Interface>::IID || iid == &<IDXGIObject as ::windows::core::Interface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain1 as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain2 as ::windows::core::Interface>::IID || iid == &<IDXGISwapChain3 as ::windows::core::Interface>::IID
    }
}
pub trait IDXGISwapChainMedia_Impl: Sized {
    fn GetFrameStatisticsMedia(&mut self) -> ::windows::core::Result<DXGI_FRAME_STATISTICS_MEDIA>;
    fn SetPresentDuration(&mut self, duration: u32) -> ::windows::core::Result<()>;
    fn CheckPresentDurationSupport(&mut self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::Result<()>;
}
impl IDXGISwapChainMedia_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>() -> IDXGISwapChainMedia_Vtbl {
        unsafe extern "system" fn GetFrameStatisticsMedia<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFrameStatisticsMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *pstats = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPresentDuration<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPresentDuration(::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Identity: ::windows::core::IUnknownImpl, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CheckPresentDurationSupport(::core::mem::transmute_copy(&desiredpresentduration), ::core::mem::transmute_copy(&pclosestsmallerpresentduration), ::core::mem::transmute_copy(&pclosestlargerpresentduration)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetFrameStatisticsMedia: GetFrameStatisticsMedia::<Identity, Impl, OFFSET>,
            SetPresentDuration: SetPresentDuration::<Identity, Impl, OFFSET>,
            CheckPresentDurationSupport: CheckPresentDurationSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChainMedia as ::windows::core::Interface>::IID
    }
}
pub trait IDXGraphicsAnalysis_Impl: Sized {
    fn BeginCapture(&mut self);
    fn EndCapture(&mut self);
}
impl IDXGraphicsAnalysis_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: isize>() -> IDXGraphicsAnalysis_Vtbl {
        unsafe extern "system" fn BeginCapture<Identity: ::windows::core::IUnknownImpl, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).BeginCapture()
        }
        unsafe extern "system" fn EndCapture<Identity: ::windows::core::IUnknownImpl, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndCapture()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BeginCapture: BeginCapture::<Identity, Impl, OFFSET>,
            EndCapture: EndCapture::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGraphicsAnalysis as ::windows::core::Interface>::IID
    }
}
