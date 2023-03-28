#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter_Impl: Sized + IDXGIObject_Impl {
    fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput>;
    fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()>;
    fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIAdapter {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: isize>() -> IDXGIAdapter_Vtbl {
        unsafe extern "system" fn EnumOutputs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, output: u32, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumOutputs(::core::mem::transmute_copy(&output)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn CheckInterfaceSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, interfacename: *const ::windows::core::GUID, pumdversion: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckInterfaceSupport(::core::mem::transmute_copy(&interfacename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pumdversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumOutputs: EnumOutputs::<Identity, Impl, OFFSET>,
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            CheckInterfaceSupport: CheckInterfaceSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter1_Impl: Sized + IDXGIAdapter_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIAdapter1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter1_Impl, const OFFSET: isize>() -> IDXGIAdapter1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc1(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self { base__: IDXGIAdapter_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc1: GetDesc1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter2_Impl: Sized + IDXGIAdapter1_Impl {
    fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIAdapter2 {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter2_Impl, const OFFSET: isize>() -> IDXGIAdapter2_Vtbl {
        unsafe extern "system" fn GetDesc2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc2(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self { base__: IDXGIAdapter1_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc2: GetDesc2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter3_Impl: Sized + IDXGIAdapter2_Impl {
    fn RegisterHardwareContentProtectionTeardownStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32);
    fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::Result<()>;
    fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::Result<()>;
    fn RegisterVideoMemoryBudgetChangeNotificationEvent(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIAdapter3 {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>() -> IDXGIAdapter3_Vtbl {
        unsafe extern "system" fn RegisterHardwareContentProtectionTeardownStatusEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterHardwareContentProtectionTeardownStatusEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterHardwareContentProtectionTeardownStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterHardwareContentProtectionTeardownStatus(::core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn QueryVideoMemoryInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryVideoMemoryInfo(::core::mem::transmute_copy(&nodeindex), ::core::mem::transmute_copy(&memorysegmentgroup), ::core::mem::transmute_copy(&pvideomemoryinfo)).into()
        }
        unsafe extern "system" fn SetVideoMemoryReservation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVideoMemoryReservation(::core::mem::transmute_copy(&nodeindex), ::core::mem::transmute_copy(&memorysegmentgroup), ::core::mem::transmute_copy(&reservation)).into()
        }
        unsafe extern "system" fn RegisterVideoMemoryBudgetChangeNotificationEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterVideoMemoryBudgetChangeNotificationEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterVideoMemoryBudgetChangeNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterVideoMemoryBudgetChangeNotification(::core::mem::transmute_copy(&dwcookie))
        }
        Self {
            base__: IDXGIAdapter2_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterHardwareContentProtectionTeardownStatusEvent: RegisterHardwareContentProtectionTeardownStatusEvent::<Identity, Impl, OFFSET>,
            UnregisterHardwareContentProtectionTeardownStatus: UnregisterHardwareContentProtectionTeardownStatus::<Identity, Impl, OFFSET>,
            QueryVideoMemoryInfo: QueryVideoMemoryInfo::<Identity, Impl, OFFSET>,
            SetVideoMemoryReservation: SetVideoMemoryReservation::<Identity, Impl, OFFSET>,
            RegisterVideoMemoryBudgetChangeNotificationEvent: RegisterVideoMemoryBudgetChangeNotificationEvent::<Identity, Impl, OFFSET>,
            UnregisterVideoMemoryBudgetChangeNotification: UnregisterVideoMemoryBudgetChangeNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIAdapter4_Impl: Sized + IDXGIAdapter3_Impl {
    fn GetDesc3(&self, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIAdapter4 {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIAdapter4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter4_Impl, const OFFSET: isize>() -> IDXGIAdapter4_Vtbl {
        unsafe extern "system" fn GetDesc3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIAdapter4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_ADAPTER_DESC3) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc3(::core::mem::transmute_copy(&pdesc)).into()
        }
        Self { base__: IDXGIAdapter3_Vtbl::new::<Identity, Impl, OFFSET>(), GetDesc3: GetDesc3::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIAdapter4 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIAdapter3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"implement\"`*"]
pub trait IDXGIDebug_Impl: Sized {
    fn ReportLiveObjects(&self, apiid: &::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDXGIDebug {}
impl IDXGIDebug_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug_Impl, const OFFSET: isize>() -> IDXGIDebug_Vtbl {
        unsafe extern "system" fn ReportLiveObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReportLiveObjects(::core::mem::transmute(&apiid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ReportLiveObjects: ReportLiveObjects::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDebug as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDebug1_Impl: Sized + IDXGIDebug_Impl {
    fn EnableLeakTrackingForThread(&self);
    fn DisableLeakTrackingForThread(&self);
    fn IsLeakTrackingEnabledForThread(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIDebug1 {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDebug1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: isize>() -> IDXGIDebug1_Vtbl {
        unsafe extern "system" fn EnableLeakTrackingForThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableLeakTrackingForThread()
        }
        unsafe extern "system" fn DisableLeakTrackingForThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableLeakTrackingForThread()
        }
        unsafe extern "system" fn IsLeakTrackingEnabledForThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDebug1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsLeakTrackingEnabledForThread()
        }
        Self {
            base__: IDXGIDebug_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnableLeakTrackingForThread: EnableLeakTrackingForThread::<Identity, Impl, OFFSET>,
            DisableLeakTrackingForThread: DisableLeakTrackingForThread::<Identity, Impl, OFFSET>,
            IsLeakTrackingEnabledForThread: IsLeakTrackingEnabledForThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDebug1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIDebug as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDecodeSwapChain_Impl: Sized {
    fn PresentBuffer(&self, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT;
    fn SetSourceRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetTargetRect(&self, prect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
    fn SetDestSize(&self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn GetSourceRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetTargetRect(&self) -> ::windows::core::Result<super::super::Foundation::RECT>;
    fn GetDestSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()>;
    fn SetColorSpace(&self, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::Result<()>;
    fn GetColorSpace(&self) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIDecodeSwapChain {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDecodeSwapChain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>() -> IDXGIDecodeSwapChain_Vtbl {
        unsafe extern "system" fn PresentBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffertopresent: u32, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PresentBuffer(::core::mem::transmute_copy(&buffertopresent), ::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn SetSourceRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourceRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetTargetRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTargetRect(::core::mem::transmute_copy(&prect)).into()
        }
        unsafe extern "system" fn SetDestSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDestSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSourceRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTargetRect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prect: *mut super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTargetRect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(prect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDestSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDestSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetColorSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorSpace(::core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn GetColorSpace<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDecodeSwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetColorSpace()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDXGIDecodeSwapChain as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice_Impl: Sized + IDXGIObject_Impl {
    fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter>;
    fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut ::core::option::Option<IDXGISurface>) -> ::windows::core::Result<()>;
    fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()>;
    fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()>;
    fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIDevice {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>() -> IDXGIDevice_Vtbl {
        unsafe extern "system" fn GetAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAdapter() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(padapter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *const DXGI_SURFACE_DESC, numsurfaces: u32, usage: DXGI_USAGE, psharedresource: *const DXGI_SHARED_RESOURCE, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSurface(::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&numsurfaces), ::core::mem::transmute_copy(&usage), ::core::mem::transmute_copy(&psharedresource), ::core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn QueryResourceResidency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresources: *const *mut ::core::ffi::c_void, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryResourceResidency(::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&presidencystatus), ::core::mem::transmute_copy(&numresources)).into()
        }
        unsafe extern "system" fn SetGPUThreadPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, priority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGPUThreadPriority(::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn GetGPUThreadPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGPUThreadPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, Impl, OFFSET>,
            CreateSurface: CreateSurface::<Identity, Impl, OFFSET>,
            QueryResourceResidency: QueryResourceResidency::<Identity, Impl, OFFSET>,
            SetGPUThreadPriority: SetGPUThreadPriority::<Identity, Impl, OFFSET>,
            GetGPUThreadPriority: GetGPUThreadPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice1_Impl: Sized + IDXGIDevice_Impl {
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIDevice1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice1_Impl, const OFFSET: isize>() -> IDXGIDevice1_Vtbl {
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaximumFrameLatency(::core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaximumFrameLatency() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmaxlatency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIDevice_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetMaximumFrameLatency: SetMaximumFrameLatency::<Identity, Impl, OFFSET>,
            GetMaximumFrameLatency: GetMaximumFrameLatency::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice2_Impl: Sized + IDXGIDevice1_Impl {
    fn OfferResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()>;
    fn ReclaimResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnqueueSetEvent(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIDevice2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: isize>() -> IDXGIDevice2_Vtbl {
        unsafe extern "system" fn OfferResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OfferResources(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&priority)).into()
        }
        unsafe extern "system" fn ReclaimResources<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, pdiscarded: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReclaimResources(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&pdiscarded)).into()
        }
        unsafe extern "system" fn EnqueueSetEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnqueueSetEvent(::core::mem::transmute_copy(&hevent)).into()
        }
        Self {
            base__: IDXGIDevice1_Vtbl::new::<Identity, Impl, OFFSET>(),
            OfferResources: OfferResources::<Identity, Impl, OFFSET>,
            ReclaimResources: ReclaimResources::<Identity, Impl, OFFSET>,
            EnqueueSetEvent: EnqueueSetEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice3_Impl: Sized + IDXGIDevice2_Impl {
    fn Trim(&self);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIDevice3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice3_Impl, const OFFSET: isize>() -> IDXGIDevice3_Vtbl {
        unsafe extern "system" fn Trim<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Trim()
        }
        Self { base__: IDXGIDevice2_Vtbl::new::<Identity, Impl, OFFSET>(), Trim: Trim::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIDevice4_Impl: Sized + IDXGIDevice3_Impl {
    fn OfferResources1(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::Result<()>;
    fn ReclaimResources1(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIDevice4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIDevice4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice4_Impl, const OFFSET: isize>() -> IDXGIDevice4_Vtbl {
        unsafe extern "system" fn OfferResources1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, priority: DXGI_OFFER_RESOURCE_PRIORITY, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OfferResources1(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&priority), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn ReclaimResources1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDevice4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numresources: u32, ppresources: *const *mut ::core::ffi::c_void, presults: *mut DXGI_RECLAIM_RESOURCE_RESULTS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReclaimResources1(::core::mem::transmute_copy(&numresources), ::core::mem::transmute_copy(&ppresources), ::core::mem::transmute_copy(&presults)).into()
        }
        Self {
            base__: IDXGIDevice3_Vtbl::new::<Identity, Impl, OFFSET>(),
            OfferResources1: OfferResources1::<Identity, Impl, OFFSET>,
            ReclaimResources1: ReclaimResources1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDevice4 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIDevice3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"implement\"`*"]
pub trait IDXGIDeviceSubObject_Impl: Sized + IDXGIObject_Impl {
    fn GetDevice(&self, riid: *const ::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDXGIDeviceSubObject {}
impl IDXGIDeviceSubObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDeviceSubObject_Impl, const OFFSET: isize>() -> IDXGIDeviceSubObject_Vtbl {
        unsafe extern "system" fn GetDevice<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDeviceSubObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppdevice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDevice(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppdevice)).into()
        }
        Self { base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(), GetDevice: GetDevice::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIDisplayControl_Impl: Sized {
    fn IsStereoEnabled(&self) -> super::super::Foundation::BOOL;
    fn SetStereoEnabled(&self, enabled: super::super::Foundation::BOOL);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIDisplayControl {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIDisplayControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDisplayControl_Impl, const OFFSET: isize>() -> IDXGIDisplayControl_Vtbl {
        unsafe extern "system" fn IsStereoEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsStereoEnabled()
        }
        unsafe extern "system" fn SetStereoEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIDisplayControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enabled: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStereoEnabled(::core::mem::transmute_copy(&enabled))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsStereoEnabled: IsStereoEnabled::<Identity, Impl, OFFSET>,
            SetStereoEnabled: SetStereoEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIDisplayControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory_Impl: Sized + IDXGIObject_Impl {
    fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter>;
    fn MakeWindowAssociation(&self, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::Result<()>;
    fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn CreateSwapChain(&self, pdevice: ::core::option::Option<&::windows::core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT;
    fn CreateSoftwareAdapter(&self, module: super::super::Foundation::HMODULE) -> ::windows::core::Result<IDXGIAdapter>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIFactory {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>() -> IDXGIFactory_Vtbl {
        unsafe extern "system" fn EnumAdapters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumAdapters(::core::mem::transmute_copy(&adapter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadapter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MakeWindowAssociation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MakeWindowAssociation(::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn GetWindowAssociation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwindowhandle: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWindowAssociation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwindowhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateSwapChain(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&ppswapchain))
        }
        unsafe extern "system" fn CreateSoftwareAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, module: super::super::Foundation::HMODULE, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSoftwareAdapter(::core::mem::transmute_copy(&module)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadapter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAdapters: EnumAdapters::<Identity, Impl, OFFSET>,
            MakeWindowAssociation: MakeWindowAssociation::<Identity, Impl, OFFSET>,
            GetWindowAssociation: GetWindowAssociation::<Identity, Impl, OFFSET>,
            CreateSwapChain: CreateSwapChain::<Identity, Impl, OFFSET>,
            CreateSoftwareAdapter: CreateSoftwareAdapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory1_Impl: Sized + IDXGIFactory_Impl {
    fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1>;
    fn IsCurrent(&self) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIFactory1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory1_Impl, const OFFSET: isize>() -> IDXGIFactory1_Vtbl {
        unsafe extern "system" fn EnumAdapters1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, ppadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumAdapters1(::core::mem::transmute_copy(&adapter)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppadapter, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsCurrent()
        }
        Self {
            base__: IDXGIFactory_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAdapters1: EnumAdapters1::<Identity, Impl, OFFSET>,
            IsCurrent: IsCurrent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory2_Impl: Sized + IDXGIFactory1_Impl {
    fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL;
    fn CreateSwapChainForHwnd(&self, pdevice: ::core::option::Option<&::windows::core::IUnknown>, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
    fn CreateSwapChainForCoreWindow(&self, pdevice: ::core::option::Option<&::windows::core::IUnknown>, pwindow: ::core::option::Option<&::windows::core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
    fn GetSharedResourceAdapterLuid(&self, hresource: super::super::Foundation::HANDLE) -> ::windows::core::Result<super::super::Foundation::LUID>;
    fn RegisterStereoStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> ::windows::core::Result<u32>;
    fn RegisterStereoStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterStereoStatus(&self, dwcookie: u32);
    fn RegisterOcclusionStatusWindow(&self, windowhandle: super::super::Foundation::HWND, wmsg: u32) -> ::windows::core::Result<u32>;
    fn RegisterOcclusionStatusEvent(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterOcclusionStatus(&self, dwcookie: u32);
    fn CreateSwapChainForComposition(&self, pdevice: ::core::option::Option<&::windows::core::IUnknown>, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIFactory2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>() -> IDXGIFactory2_Vtbl {
        unsafe extern "system" fn IsWindowedStereoEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsWindowedStereoEnabled()
        }
        unsafe extern "system" fn CreateSwapChainForHwnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hwnd: super::super::Foundation::HWND, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: *const DXGI_SWAP_CHAIN_FULLSCREEN_DESC, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSwapChainForHwnd(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pdesc), ::core::mem::transmute_copy(&pfullscreendesc), ::windows::core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSwapChainForCoreWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pwindow: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSwapChainForCoreWindow(::windows::core::from_raw_borrowed(&pdevice), ::windows::core::from_raw_borrowed(&pwindow), ::core::mem::transmute_copy(&pdesc), ::windows::core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSharedResourceAdapterLuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresource: super::super::Foundation::HANDLE, pluid: *mut super::super::Foundation::LUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSharedResourceAdapterLuid(::core::mem::transmute_copy(&hresource)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pluid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterStereoStatusWindow(::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&wmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterStereoStatusEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterStereoStatusEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterStereoStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterStereoStatus(::core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn RegisterOcclusionStatusWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, windowhandle: super::super::Foundation::HWND, wmsg: u32, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterOcclusionStatusWindow(::core::mem::transmute_copy(&windowhandle), ::core::mem::transmute_copy(&wmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterOcclusionStatusEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterOcclusionStatusEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterOcclusionStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterOcclusionStatus(::core::mem::transmute_copy(&dwcookie))
        }
        unsafe extern "system" fn CreateSwapChainForComposition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSwapChainForComposition(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&pdesc), ::windows::core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIFactory1_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IDXGIFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory3_Impl: Sized + IDXGIFactory2_Impl {
    fn GetCreationFlags(&self) -> u32;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIFactory3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory3_Impl, const OFFSET: isize>() -> IDXGIFactory3_Vtbl {
        unsafe extern "system" fn GetCreationFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCreationFlags()
        }
        Self { base__: IDXGIFactory2_Vtbl::new::<Identity, Impl, OFFSET>(), GetCreationFlags: GetCreationFlags::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory4_Impl: Sized + IDXGIFactory3_Impl {
    fn EnumAdapterByLuid(&self, adapterluid: &super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn EnumWarpAdapter(&self, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIFactory4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory4_Impl, const OFFSET: isize>() -> IDXGIFactory4_Vtbl {
        unsafe extern "system" fn EnumAdapterByLuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapterluid: super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAdapterByLuid(::core::mem::transmute(&adapterluid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn EnumWarpAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumWarpAdapter(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self {
            base__: IDXGIFactory3_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAdapterByLuid: EnumAdapterByLuid::<Identity, Impl, OFFSET>,
            EnumWarpAdapter: EnumWarpAdapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory4 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory5_Impl: Sized + IDXGIFactory4_Impl {
    fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIFactory5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory5_Impl, const OFFSET: isize>() -> IDXGIFactory5_Vtbl {
        unsafe extern "system" fn CheckFeatureSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckFeatureSupport(::core::mem::transmute_copy(&feature), ::core::mem::transmute_copy(&pfeaturesupportdata), ::core::mem::transmute_copy(&featuresupportdatasize)).into()
        }
        Self { base__: IDXGIFactory4_Vtbl::new::<Identity, Impl, OFFSET>(), CheckFeatureSupport: CheckFeatureSupport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory5 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory6_Impl: Sized + IDXGIFactory5_Impl {
    fn EnumAdapterByGpuPreference(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIFactory6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory6_Impl, const OFFSET: isize>() -> IDXGIFactory6_Vtbl {
        unsafe extern "system" fn EnumAdapterByGpuPreference<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAdapterByGpuPreference(::core::mem::transmute_copy(&adapter), ::core::mem::transmute_copy(&gpupreference), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        Self { base__: IDXGIFactory5_Vtbl::new::<Identity, Impl, OFFSET>(), EnumAdapterByGpuPreference: EnumAdapterByGpuPreference::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory6 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory4 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory5 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactory7_Impl: Sized + IDXGIFactory6_Impl {
    fn RegisterAdaptersChangedEvent(&self, hevent: super::super::Foundation::HANDLE) -> ::windows::core::Result<u32>;
    fn UnregisterAdaptersChangedEvent(&self, dwcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIFactory7 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactory7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory7_Impl, const OFFSET: isize>() -> IDXGIFactory7_Vtbl {
        unsafe extern "system" fn RegisterAdaptersChangedEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterAdaptersChangedEvent(::core::mem::transmute_copy(&hevent)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterAdaptersChangedEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactory7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterAdaptersChangedEvent(::core::mem::transmute_copy(&dwcookie)).into()
        }
        Self {
            base__: IDXGIFactory6_Vtbl::new::<Identity, Impl, OFFSET>(),
            RegisterAdaptersChangedEvent: RegisterAdaptersChangedEvent::<Identity, Impl, OFFSET>,
            UnregisterAdaptersChangedEvent: UnregisterAdaptersChangedEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactory7 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory4 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory5 as ::windows::core::ComInterface>::IID || iid == &<IDXGIFactory6 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIFactoryMedia_Impl: Sized {
    fn CreateSwapChainForCompositionSurfaceHandle(&self, pdevice: ::core::option::Option<&::windows::core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows::core::Result<IDXGISwapChain1>;
    fn CreateDecodeSwapChainForCompositionSurfaceHandle(&self, pdevice: ::core::option::Option<&::windows::core::IUnknown>, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: ::core::option::Option<&IDXGIResource>, prestricttooutput: ::core::option::Option<&IDXGIOutput>) -> ::windows::core::Result<IDXGIDecodeSwapChain>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIFactoryMedia {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIFactoryMedia_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactoryMedia_Impl, const OFFSET: isize>() -> IDXGIFactoryMedia_Vtbl {
        unsafe extern "system" fn CreateSwapChainForCompositionSurfaceHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSwapChainForCompositionSurfaceHandle(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hsurface), ::core::mem::transmute_copy(&pdesc), ::windows::core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDecodeSwapChainForCompositionSurfaceHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIFactoryMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, hsurface: super::super::Foundation::HANDLE, pdesc: *const DXGI_DECODE_SWAP_CHAIN_DESC, pyuvdecodebuffers: *mut ::core::ffi::c_void, prestricttooutput: *mut ::core::ffi::c_void, ppswapchain: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDecodeSwapChainForCompositionSurfaceHandle(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&hsurface), ::core::mem::transmute_copy(&pdesc), ::windows::core::from_raw_borrowed(&pyuvdecodebuffers), ::windows::core::from_raw_borrowed(&prestricttooutput)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppswapchain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateSwapChainForCompositionSurfaceHandle: CreateSwapChainForCompositionSurfaceHandle::<Identity, Impl, OFFSET>,
            CreateDecodeSwapChainForCompositionSurfaceHandle: CreateDecodeSwapChainForCompositionSurfaceHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIFactoryMedia as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIInfoQueue_Impl: Sized {
    fn SetMessageCountLimit(&self, producer: &::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::Result<()>;
    fn ClearStoredMessages(&self, producer: &::windows::core::GUID);
    fn GetMessage(&self, producer: &::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::Result<()>;
    fn GetNumStoredMessagesAllowedByRetrievalFilters(&self, producer: &::windows::core::GUID) -> u64;
    fn GetNumStoredMessages(&self, producer: &::windows::core::GUID) -> u64;
    fn GetNumMessagesDiscardedByMessageCountLimit(&self, producer: &::windows::core::GUID) -> u64;
    fn GetMessageCountLimit(&self, producer: &::windows::core::GUID) -> u64;
    fn GetNumMessagesAllowedByStorageFilter(&self, producer: &::windows::core::GUID) -> u64;
    fn GetNumMessagesDeniedByStorageFilter(&self, producer: &::windows::core::GUID) -> u64;
    fn AddStorageFilterEntries(&self, producer: &::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetStorageFilter(&self, producer: &::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearStorageFilter(&self, producer: &::windows::core::GUID);
    fn PushEmptyStorageFilter(&self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushDenyAllStorageFilter(&self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushCopyOfStorageFilter(&self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushStorageFilter(&self, producer: &::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopStorageFilter(&self, producer: &::windows::core::GUID);
    fn GetStorageFilterStackSize(&self, producer: &::windows::core::GUID) -> u32;
    fn AddRetrievalFilterEntries(&self, producer: &::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn GetRetrievalFilter(&self, producer: &::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::Result<()>;
    fn ClearRetrievalFilter(&self, producer: &::windows::core::GUID);
    fn PushEmptyRetrievalFilter(&self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushDenyAllRetrievalFilter(&self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushCopyOfRetrievalFilter(&self, producer: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn PushRetrievalFilter(&self, producer: &::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::Result<()>;
    fn PopRetrievalFilter(&self, producer: &::windows::core::GUID);
    fn GetRetrievalFilterStackSize(&self, producer: &::windows::core::GUID) -> u32;
    fn AddMessage(&self, producer: &::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn AddApplicationMessage(&self, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: &::windows::core::PCSTR) -> ::windows::core::Result<()>;
    fn SetBreakOnCategory(&self, producer: &::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnSeverity(&self, producer: &::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBreakOnID(&self, producer: &::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetBreakOnCategory(&self, producer: &::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL;
    fn GetBreakOnSeverity(&self, producer: &::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL;
    fn GetBreakOnID(&self, producer: &::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL;
    fn SetMuteDebugOutput(&self, producer: &::windows::core::GUID, bmute: super::super::Foundation::BOOL);
    fn GetMuteDebugOutput(&self, producer: &::windows::core::GUID) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIInfoQueue {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIInfoQueue_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>() -> IDXGIInfoQueue_Vtbl {
        unsafe extern "system" fn SetMessageCountLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messagecountlimit: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMessageCountLimit(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&messagecountlimit)).into()
        }
        unsafe extern "system" fn ClearStoredMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearStoredMessages(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, messageindex: u64, pmessage: *mut DXGI_INFO_QUEUE_MESSAGE, pmessagebytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMessage(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&messageindex), ::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pmessagebytelength)).into()
        }
        unsafe extern "system" fn GetNumStoredMessagesAllowedByRetrievalFilters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumStoredMessagesAllowedByRetrievalFilters(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumStoredMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumStoredMessages(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDiscardedByMessageCountLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumMessagesDiscardedByMessageCountLimit(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetMessageCountLimit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMessageCountLimit(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumMessagesAllowedByStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumMessagesAllowedByStorageFilter(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetNumMessagesDeniedByStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u64 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNumMessagesDeniedByStorageFilter(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn AddStorageFilterEntries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStorageFilterEntries(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStorageFilter(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearStorageFilter(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn PushEmptyStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushEmptyStorageFilter(::core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushDenyAllStorageFilter(::core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushCopyOfStorageFilter(::core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushStorageFilter(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopStorageFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopStorageFilter(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetStorageFilterStackSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStorageFilterStackSize(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn AddRetrievalFilterEntries<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRetrievalFilterEntries(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn GetRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *mut DXGI_INFO_QUEUE_FILTER, pfilterbytelength: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRetrievalFilter(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter), ::core::mem::transmute_copy(&pfilterbytelength)).into()
        }
        unsafe extern "system" fn ClearRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearRetrievalFilter(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn PushEmptyRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushEmptyRetrievalFilter(::core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushDenyAllRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushDenyAllRetrievalFilter(::core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushCopyOfRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushCopyOfRetrievalFilter(::core::mem::transmute(&producer)).into()
        }
        unsafe extern "system" fn PushRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, pfilter: *const DXGI_INFO_QUEUE_FILTER) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PushRetrievalFilter(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&pfilter)).into()
        }
        unsafe extern "system" fn PopRetrievalFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopRetrievalFilter(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn GetRetrievalFilterStackSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRetrievalFilterStackSize(::core::mem::transmute(&producer))
        }
        unsafe extern "system" fn AddMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, id: i32, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMessage(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&id), ::core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn AddApplicationMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, pdescription: ::windows::core::PCSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddApplicationMessage(::core::mem::transmute_copy(&severity), ::core::mem::transmute(&pdescription)).into()
        }
        unsafe extern "system" fn SetBreakOnCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBreakOnCategory(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&category), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBreakOnSeverity(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&severity), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn SetBreakOnID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32, benable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBreakOnID(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&benable)).into()
        }
        unsafe extern "system" fn GetBreakOnCategory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, category: DXGI_INFO_QUEUE_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakOnCategory(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&category))
        }
        unsafe extern "system" fn GetBreakOnSeverity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, severity: DXGI_INFO_QUEUE_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakOnSeverity(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&severity))
        }
        unsafe extern "system" fn GetBreakOnID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, id: i32) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBreakOnID(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&id))
        }
        unsafe extern "system" fn SetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID, bmute: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMuteDebugOutput(::core::mem::transmute(&producer), ::core::mem::transmute_copy(&bmute))
        }
        unsafe extern "system" fn GetMuteDebugOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIInfoQueue_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, producer: ::windows::core::GUID) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMuteDebugOutput(::core::mem::transmute(&producer))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDXGIInfoQueue as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"implement\"`*"]
pub trait IDXGIKeyedMutex_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn AcquireSync(&self, key: u64, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn ReleaseSync(&self, key: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDXGIKeyedMutex {}
impl IDXGIKeyedMutex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIKeyedMutex_Impl, const OFFSET: isize>() -> IDXGIKeyedMutex_Vtbl {
        unsafe extern "system" fn AcquireSync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireSync(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn ReleaseSync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIKeyedMutex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseSync(::core::mem::transmute_copy(&key)).into()
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            AcquireSync: AcquireSync::<Identity, Impl, OFFSET>,
            ReleaseSync: ReleaseSync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIKeyedMutex as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"implement\"`*"]
pub trait IDXGIObject_Impl: Sized {
    fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetPrivateDataInterface(&self, name: *const ::windows::core::GUID, punknown: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetParent(&self, riid: *const ::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDXGIObject {}
impl IDXGIObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>() -> IDXGIObject_Vtbl {
        unsafe extern "system" fn SetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateData(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&datasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn SetPrivateDataInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPrivateDataInterface(::core::mem::transmute_copy(&name), ::windows::core::from_raw_borrowed(&punknown)).into()
        }
        unsafe extern "system" fn GetPrivateData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrivateData(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdata)).into()
        }
        unsafe extern "system" fn GetParent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetParent(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparent)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPrivateData: SetPrivateData::<Identity, Impl, OFFSET>,
            SetPrivateDataInterface: SetPrivateDataInterface::<Identity, Impl, OFFSET>,
            GetPrivateData: GetPrivateData::<Identity, Impl, OFFSET>,
            GetParent: GetParent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput_Impl: Sized + IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()>;
    fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::Result<()>;
    fn FindClosestMatchingMode(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn WaitForVBlank(&self) -> ::windows::core::Result<()>;
    fn TakeOwnership(&self, pdevice: ::core::option::Option<&::windows::core::IUnknown>, exclusive: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ReleaseOwnership(&self);
    fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()>;
    fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()>;
    fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()>;
    fn SetDisplaySurface(&self, pscanoutsurface: ::core::option::Option<&IDXGISurface>) -> ::windows::core::Result<()>;
    fn GetDisplaySurfaceData(&self, pdestination: ::core::option::Option<&IDXGISurface>) -> ::windows::core::Result<()>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDXGIOutput {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>() -> IDXGIOutput_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetDisplayModeList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayModeList(::core::mem::transmute_copy(&enumformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pnummodes), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindClosestMatchingMode(::core::mem::transmute_copy(&pmodetomatch), ::core::mem::transmute_copy(&pclosestmatch), ::windows::core::from_raw_borrowed(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn WaitForVBlank<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WaitForVBlank().into()
        }
        unsafe extern "system" fn TakeOwnership<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, exclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TakeOwnership(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&exclusive)).into()
        }
        unsafe extern "system" fn ReleaseOwnership<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseOwnership()
        }
        unsafe extern "system" fn GetGammaControlCapabilities<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGammaControlCapabilities(::core::mem::transmute_copy(&pgammacaps)).into()
        }
        unsafe extern "system" fn SetGammaControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGammaControl(::core::mem::transmute_copy(&parray)).into()
        }
        unsafe extern "system" fn GetGammaControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGammaControl(::core::mem::transmute_copy(&parray)).into()
        }
        unsafe extern "system" fn SetDisplaySurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscanoutsurface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplaySurface(::windows::core::from_raw_borrowed(&pscanoutsurface)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplaySurfaceData(::windows::core::from_raw_borrowed(&pdestination)).into()
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrameStatistics(::core::mem::transmute_copy(&pstats)).into()
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IDXGIOutput as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput1_Impl: Sized + IDXGIOutput_Impl {
    fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::Result<()>;
    fn FindClosestMatchingMode1(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetDisplaySurfaceData1(&self, pdestination: ::core::option::Option<&IDXGIResource>) -> ::windows::core::Result<()>;
    fn DuplicateOutput(&self, pdevice: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDXGIOutput1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>() -> IDXGIOutput1_Vtbl {
        unsafe extern "system" fn GetDisplayModeList1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: *mut DXGI_MODE_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayModeList1(::core::mem::transmute_copy(&enumformat), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&pnummodes), ::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn FindClosestMatchingMode1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindClosestMatchingMode1(::core::mem::transmute_copy(&pmodetomatch), ::core::mem::transmute_copy(&pclosestmatch), ::windows::core::from_raw_borrowed(&pconcerneddevice)).into()
        }
        unsafe extern "system" fn GetDisplaySurfaceData1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestination: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplaySurfaceData1(::windows::core::from_raw_borrowed(&pdestination)).into()
        }
        unsafe extern "system" fn DuplicateOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DuplicateOutput(::windows::core::from_raw_borrowed(&pdevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutputduplication, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIOutput_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDisplayModeList1: GetDisplayModeList1::<Identity, Impl, OFFSET>,
            FindClosestMatchingMode1: FindClosestMatchingMode1::<Identity, Impl, OFFSET>,
            GetDisplaySurfaceData1: GetDisplaySurfaceData1::<Identity, Impl, OFFSET>,
            DuplicateOutput: DuplicateOutput::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput2_Impl: Sized + IDXGIOutput1_Impl {
    fn SupportsOverlays(&self) -> super::super::Foundation::BOOL;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDXGIOutput2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput2_Impl, const OFFSET: isize>() -> IDXGIOutput2_Vtbl {
        unsafe extern "system" fn SupportsOverlays<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SupportsOverlays()
        }
        Self { base__: IDXGIOutput1_Vtbl::new::<Identity, Impl, OFFSET>(), SupportsOverlays: SupportsOverlays::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput3_Impl: Sized + IDXGIOutput2_Impl {
    fn CheckOverlaySupport(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDXGIOutput3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput3_Impl, const OFFSET: isize>() -> IDXGIOutput3_Vtbl {
        unsafe extern "system" fn CheckOverlaySupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumformat: Common::DXGI_FORMAT, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckOverlaySupport(::core::mem::transmute_copy(&enumformat), ::windows::core::from_raw_borrowed(&pconcerneddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIOutput2_Vtbl::new::<Identity, Impl, OFFSET>(), CheckOverlaySupport: CheckOverlaySupport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput4_Impl: Sized + IDXGIOutput3_Impl {
    fn CheckOverlayColorSpaceSupport(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDXGIOutput4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput4_Impl, const OFFSET: isize>() -> IDXGIOutput4_Vtbl {
        unsafe extern "system" fn CheckOverlayColorSpaceSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckOverlayColorSpaceSupport(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&colorspace), ::windows::core::from_raw_borrowed(&pconcerneddevice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIOutput3_Vtbl::new::<Identity, Impl, OFFSET>(),
            CheckOverlayColorSpaceSupport: CheckOverlayColorSpaceSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput4 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput5_Impl: Sized + IDXGIOutput4_Impl {
    fn DuplicateOutput1(&self, pdevice: ::core::option::Option<&::windows::core::IUnknown>, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT) -> ::windows::core::Result<IDXGIOutputDuplication>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDXGIOutput5 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput5_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput5_Impl, const OFFSET: isize>() -> IDXGIOutput5_Vtbl {
        unsafe extern "system" fn DuplicateOutput1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput5_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdevice: *mut ::core::ffi::c_void, flags: u32, supportedformatscount: u32, psupportedformats: *const Common::DXGI_FORMAT, ppoutputduplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DuplicateOutput1(::windows::core::from_raw_borrowed(&pdevice), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&supportedformatscount), ::core::mem::transmute_copy(&psupportedformats)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutputduplication, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IDXGIOutput4_Vtbl::new::<Identity, Impl, OFFSET>(), DuplicateOutput1: DuplicateOutput1::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput5 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput4 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGIOutput6_Impl: Sized + IDXGIOutput5_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::core::Result<()>;
    fn CheckHardwareCompositionSupport(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDXGIOutput6 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGIOutput6_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput6_Impl, const OFFSET: isize>() -> IDXGIOutput6_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTPUT_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc1(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn CheckHardwareCompositionSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutput6_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckHardwareCompositionSupport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIOutput5_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc1: GetDesc1::<Identity, Impl, OFFSET>,
            CheckHardwareCompositionSupport: CheckHardwareCompositionSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIOutput6 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput4 as ::windows::core::ComInterface>::IID || iid == &<IDXGIOutput5 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGIOutputDuplication_Impl: Sized + IDXGIObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_OUTDUPL_DESC);
    fn AcquireNextFrame(&self, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut ::core::option::Option<IDXGIResource>) -> ::windows::core::Result<()>;
    fn GetFrameDirtyRects(&self, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetFrameMoveRects(&self, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetFramePointerShape(&self, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::Result<()>;
    fn MapDesktopSurface(&self) -> ::windows::core::Result<DXGI_MAPPED_RECT>;
    fn UnMapDesktopSurface(&self) -> ::windows::core::Result<()>;
    fn ReleaseFrame(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGIOutputDuplication {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGIOutputDuplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>() -> IDXGIOutputDuplication_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_OUTDUPL_DESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc))
        }
        unsafe extern "system" fn AcquireNextFrame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeoutinmilliseconds: u32, pframeinfo: *mut DXGI_OUTDUPL_FRAME_INFO, ppdesktopresource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireNextFrame(::core::mem::transmute_copy(&timeoutinmilliseconds), ::core::mem::transmute_copy(&pframeinfo), ::core::mem::transmute_copy(&ppdesktopresource)).into()
        }
        unsafe extern "system" fn GetFrameDirtyRects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dirtyrectsbuffersize: u32, pdirtyrectsbuffer: *mut super::super::Foundation::RECT, pdirtyrectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrameDirtyRects(::core::mem::transmute_copy(&dirtyrectsbuffersize), ::core::mem::transmute_copy(&pdirtyrectsbuffer), ::core::mem::transmute_copy(&pdirtyrectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFrameMoveRects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, moverectsbuffersize: u32, pmoverectbuffer: *mut DXGI_OUTDUPL_MOVE_RECT, pmoverectsbuffersizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrameMoveRects(::core::mem::transmute_copy(&moverectsbuffersize), ::core::mem::transmute_copy(&pmoverectbuffer), ::core::mem::transmute_copy(&pmoverectsbuffersizerequired)).into()
        }
        unsafe extern "system" fn GetFramePointerShape<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pointershapebuffersize: u32, ppointershapebuffer: *mut ::core::ffi::c_void, ppointershapebuffersizerequired: *mut u32, ppointershapeinfo: *mut DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFramePointerShape(::core::mem::transmute_copy(&pointershapebuffersize), ::core::mem::transmute_copy(&ppointershapebuffer), ::core::mem::transmute_copy(&ppointershapebuffersizerequired), ::core::mem::transmute_copy(&ppointershapeinfo)).into()
        }
        unsafe extern "system" fn MapDesktopSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MapDesktopSurface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plockedrect, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnMapDesktopSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnMapDesktopSurface().into()
        }
        unsafe extern "system" fn ReleaseFrame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIOutputDuplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseFrame().into()
        }
        Self {
            base__: IDXGIObject_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IDXGIOutputDuplication as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXGIResource_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn GetSharedHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn GetUsage(&self) -> ::windows::core::Result<DXGI_USAGE>;
    fn SetEvictionPriority(&self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::Result<()>;
    fn GetEvictionPriority(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXGIResource {}
#[cfg(feature = "Win32_Foundation")]
impl IDXGIResource_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>() -> IDXGIResource_Vtbl {
        unsafe extern "system" fn GetSharedHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psharedhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSharedHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psharedhandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pusage: *mut DXGI_USAGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUsage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pusage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEvictionPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEvictionPriority(::core::mem::transmute_copy(&evictionpriority)).into()
        }
        unsafe extern "system" fn GetEvictionPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevictionpriority: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEvictionPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pevictionpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSharedHandle: GetSharedHandle::<Identity, Impl, OFFSET>,
            GetUsage: GetUsage::<Identity, Impl, OFFSET>,
            SetEvictionPriority: SetEvictionPriority::<Identity, Impl, OFFSET>,
            GetEvictionPriority: GetEvictionPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIResource as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub trait IDXGIResource1_Impl: Sized + IDXGIResource_Impl {
    fn CreateSubresourceSurface(&self, index: u32) -> ::windows::core::Result<IDXGISurface2>;
    fn CreateSharedHandle(&self, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows::core::RuntimeName for IDXGIResource1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl IDXGIResource1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource1_Impl, const OFFSET: isize>() -> IDXGIResource1_Vtbl {
        unsafe extern "system" fn CreateSubresourceSurface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSubresourceSurface(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsurface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateSharedHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGIResource1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwaccess: u32, lpname: ::windows::core::PCWSTR, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSharedHandle(::core::mem::transmute_copy(&pattributes), ::core::mem::transmute_copy(&dwaccess), ::core::mem::transmute(&lpname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phandle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIResource_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateSubresourceSurface: CreateSubresourceSurface::<Identity, Impl, OFFSET>,
            CreateSharedHandle: CreateSharedHandle::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGIResource1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIResource as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IDXGISurface_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::Result<()>;
    fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()>;
    fn Unmap(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::windows::core::RuntimeName for IDXGISurface {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IDXGISurface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: isize>() -> IDXGISurface_Vtbl {
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn Map<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Map(::core::mem::transmute_copy(&plockedrect), ::core::mem::transmute_copy(&mapflags)).into()
        }
        unsafe extern "system" fn Unmap<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unmap().into()
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDesc: GetDesc::<Identity, Impl, OFFSET>,
            Map: Map::<Identity, Impl, OFFSET>,
            Unmap: Unmap::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface1_Impl: Sized + IDXGISurface_Impl {
    fn GetDC(&self, discard: super::super::Foundation::BOOL) -> ::windows::core::Result<super::Gdi::HDC>;
    fn ReleaseDC(&self, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDXGISurface1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface1_Impl, const OFFSET: isize>() -> IDXGISurface1_Vtbl {
        unsafe extern "system" fn GetDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discard: super::super::Foundation::BOOL, phdc: *mut super::Gdi::HDC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDC(::core::mem::transmute_copy(&discard)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phdc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseDC<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdirtyrect: *const super::super::Foundation::RECT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseDC(::core::mem::transmute_copy(&pdirtyrect)).into()
        }
        Self {
            base__: IDXGISurface_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDC: GetDC::<Identity, Impl, OFFSET>,
            ReleaseDC: ReleaseDC::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID || iid == &<IDXGISurface as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
pub trait IDXGISurface2_Impl: Sized + IDXGISurface1_Impl {
    fn GetResource(&self, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::windows::core::RuntimeName for IDXGISurface2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl IDXGISurface2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface2_Impl, const OFFSET: isize>() -> IDXGISurface2_Vtbl {
        unsafe extern "system" fn GetResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISurface2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppparentresource: *mut *mut ::core::ffi::c_void, psubresourceindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetResource(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppparentresource), ::core::mem::transmute_copy(&psubresourceindex)).into()
        }
        Self { base__: IDXGISurface1_Vtbl::new::<Identity, Impl, OFFSET>(), GetResource: GetResource::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISurface2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID || iid == &<IDXGISurface as ::windows::core::ComInterface>::IID || iid == &<IDXGISurface1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain_Impl: Sized + IDXGIDeviceSubObject_Impl {
    fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT;
    fn GetBuffer(&self, buffer: u32, riid: *const ::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetFullscreenState(&self, fullscreen: super::super::Foundation::BOOL, ptarget: ::core::option::Option<&IDXGIOutput>) -> ::windows::core::Result<()>;
    fn GetFullscreenState(&self, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut ::core::option::Option<IDXGIOutput>) -> ::windows::core::Result<()>;
    fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()>;
    fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()>;
    fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()>;
    fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput>;
    fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()>;
    fn GetLastPresentCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGISwapChain {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>() -> IDXGISwapChain_Vtbl {
        unsafe extern "system" fn Present<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Present(::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&flags))
        }
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffer: u32, riid: *const ::windows::core::GUID, ppsurface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBuffer(::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppsurface)).into()
        }
        unsafe extern "system" fn SetFullscreenState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullscreen: super::super::Foundation::BOOL, ptarget: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFullscreenState(::core::mem::transmute_copy(&fullscreen), ::windows::core::from_raw_borrowed(&ptarget)).into()
        }
        unsafe extern "system" fn GetFullscreenState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfullscreen: *mut super::super::Foundation::BOOL, pptarget: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFullscreenState(::core::mem::transmute_copy(&pfullscreen), ::core::mem::transmute_copy(&pptarget)).into()
        }
        unsafe extern "system" fn GetDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn ResizeBuffers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResizeBuffers(::core::mem::transmute_copy(&buffercount), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&newformat), ::core::mem::transmute_copy(&swapchainflags)).into()
        }
        unsafe extern "system" fn ResizeTarget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResizeTarget(::core::mem::transmute_copy(&pnewtargetparameters)).into()
        }
        unsafe extern "system" fn GetContainingOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContainingOutput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoutput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrameStatistics(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn GetLastPresentCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plastpresentcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLastPresentCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plastpresentcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGIDeviceSubObject_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IDXGISwapChain as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain1_Impl: Sized + IDXGISwapChain_Impl {
    fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::Result<()>;
    fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::Result<()>;
    fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND>;
    fn GetCoreWindow(&self, refiid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT;
    fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL;
    fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput>;
    fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()>;
    fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA>;
    fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()>;
    fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGISwapChain1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain1_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>() -> IDXGISwapChain1_Vtbl {
        unsafe extern "system" fn GetDesc1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDesc1(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetFullscreenDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFullscreenDesc(::core::mem::transmute_copy(&pdesc)).into()
        }
        unsafe extern "system" fn GetHwnd<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHwnd() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phwnd, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCoreWindow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, refiid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCoreWindow(::core::mem::transmute_copy(&refiid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn Present1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Present1(::core::mem::transmute_copy(&syncinterval), ::core::mem::transmute_copy(&presentflags), ::core::mem::transmute_copy(&ppresentparameters))
        }
        unsafe extern "system" fn IsTemporaryMonoSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsTemporaryMonoSupported()
        }
        unsafe extern "system" fn GetRestrictToOutput<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprestricttooutput: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRestrictToOutput() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprestricttooutput, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBackgroundColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *const DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBackgroundColor(::core::mem::transmute_copy(&pcolor)).into()
        }
        unsafe extern "system" fn GetBackgroundColor<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcolor: *mut DXGI_RGBA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBackgroundColor() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcolor, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRotation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRotation(::core::mem::transmute_copy(&rotation)).into()
        }
        unsafe extern "system" fn GetRotation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain1_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, protation: *mut Common::DXGI_MODE_ROTATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRotation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(protation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDXGISwapChain_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IDXGISwapChain1 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain2_Impl: Sized + IDXGISwapChain1_Impl {
    fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::core::Result<()>;
    fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()>;
    fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()>;
    fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32>;
    fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE;
    fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()>;
    fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGISwapChain2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>() -> IDXGISwapChain2_Vtbl {
        unsafe extern "system" fn SetSourceSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, width: u32, height: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourceSize(::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height)).into()
        }
        unsafe extern "system" fn GetSourceSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSourceSize(::core::mem::transmute_copy(&pwidth), ::core::mem::transmute_copy(&pheight)).into()
        }
        unsafe extern "system" fn SetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxlatency: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaximumFrameLatency(::core::mem::transmute_copy(&maxlatency)).into()
        }
        unsafe extern "system" fn GetMaximumFrameLatency<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmaxlatency: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaximumFrameLatency() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmaxlatency, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFrameLatencyWaitableObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrameLatencyWaitableObject()
        }
        unsafe extern "system" fn SetMatrixTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMatrixTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        unsafe extern "system" fn GetMatrixTransform<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMatrixTransform(::core::mem::transmute_copy(&pmatrix)).into()
        }
        Self {
            base__: IDXGISwapChain1_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IDXGISwapChain2 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain1 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain3_Impl: Sized + IDXGISwapChain2_Impl {
    fn GetCurrentBackBufferIndex(&self) -> u32;
    fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<u32>;
    fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()>;
    fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGISwapChain3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>() -> IDXGISwapChain3_Vtbl {
        unsafe extern "system" fn GetCurrentBackBufferIndex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentBackBufferIndex()
        }
        unsafe extern "system" fn CheckColorSpaceSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pcolorspacesupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CheckColorSpaceSupport(::core::mem::transmute_copy(&colorspace)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcolorspacesupport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetColorSpace1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetColorSpace1(::core::mem::transmute_copy(&colorspace)).into()
        }
        unsafe extern "system" fn ResizeBuffers1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResizeBuffers1(::core::mem::transmute_copy(&buffercount), ::core::mem::transmute_copy(&width), ::core::mem::transmute_copy(&height), ::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&swapchainflags), ::core::mem::transmute_copy(&pcreationnodemask), ::core::mem::transmute_copy(&pppresentqueue)).into()
        }
        Self {
            base__: IDXGISwapChain2_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCurrentBackBufferIndex: GetCurrentBackBufferIndex::<Identity, Impl, OFFSET>,
            CheckColorSpaceSupport: CheckColorSpaceSupport::<Identity, Impl, OFFSET>,
            SetColorSpace1: SetColorSpace1::<Identity, Impl, OFFSET>,
            ResizeBuffers1: ResizeBuffers1::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain3 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain1 as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub trait IDXGISwapChain4_Impl: Sized + IDXGISwapChain3_Impl {
    fn SetHDRMetaData(&self, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::windows::core::RuntimeName for IDXGISwapChain4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl IDXGISwapChain4_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain4_Impl, const OFFSET: isize>() -> IDXGISwapChain4_Vtbl {
        unsafe extern "system" fn SetHDRMetaData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChain4_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: DXGI_HDR_METADATA_TYPE, size: u32, pmetadata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHDRMetaData(::core::mem::transmute_copy(&r#type), ::core::mem::transmute_copy(&size), ::core::mem::transmute_copy(&pmetadata)).into()
        }
        Self { base__: IDXGISwapChain3_Vtbl::new::<Identity, Impl, OFFSET>(), SetHDRMetaData: SetHDRMetaData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChain4 as ::windows::core::ComInterface>::IID || iid == &<IDXGIObject as ::windows::core::ComInterface>::IID || iid == &<IDXGIDeviceSubObject as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain1 as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain2 as ::windows::core::ComInterface>::IID || iid == &<IDXGISwapChain3 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"implement\"`*"]
pub trait IDXGISwapChainMedia_Impl: Sized {
    fn GetFrameStatisticsMedia(&self, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows::core::Result<()>;
    fn SetPresentDuration(&self, duration: u32) -> ::windows::core::Result<()>;
    fn CheckPresentDurationSupport(&self, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDXGISwapChainMedia {}
impl IDXGISwapChainMedia_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>() -> IDXGISwapChainMedia_Vtbl {
        unsafe extern "system" fn GetFrameStatisticsMedia<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstats: *mut DXGI_FRAME_STATISTICS_MEDIA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFrameStatisticsMedia(::core::mem::transmute_copy(&pstats)).into()
        }
        unsafe extern "system" fn SetPresentDuration<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, duration: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPresentDuration(::core::mem::transmute_copy(&duration)).into()
        }
        unsafe extern "system" fn CheckPresentDurationSupport<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGISwapChainMedia_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desiredpresentduration: u32, pclosestsmallerpresentduration: *mut u32, pclosestlargerpresentduration: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckPresentDurationSupport(::core::mem::transmute_copy(&desiredpresentduration), ::core::mem::transmute_copy(&pclosestsmallerpresentduration), ::core::mem::transmute_copy(&pclosestlargerpresentduration)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFrameStatisticsMedia: GetFrameStatisticsMedia::<Identity, Impl, OFFSET>,
            SetPresentDuration: SetPresentDuration::<Identity, Impl, OFFSET>,
            CheckPresentDurationSupport: CheckPresentDurationSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGISwapChainMedia as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_Dxgi\"`, `\"implement\"`*"]
pub trait IDXGraphicsAnalysis_Impl: Sized {
    fn BeginCapture(&self);
    fn EndCapture(&self);
}
impl ::windows::core::RuntimeName for IDXGraphicsAnalysis {}
impl IDXGraphicsAnalysis_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: isize>() -> IDXGraphicsAnalysis_Vtbl {
        unsafe extern "system" fn BeginCapture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginCapture()
        }
        unsafe extern "system" fn EndCapture<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXGraphicsAnalysis_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCapture()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCapture: BeginCapture::<Identity, Impl, OFFSET>,
            EndCapture: EndCapture::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXGraphicsAnalysis as ::windows::core::ComInterface>::IID
    }
}
