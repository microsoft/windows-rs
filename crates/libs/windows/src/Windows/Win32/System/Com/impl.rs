#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSinkImpl: Sized {
    fn Begin_OnDataChange();
    fn Finish_OnDataChange();
    fn Begin_OnViewChange();
    fn Finish_OnViewChange();
    fn Begin_OnRename();
    fn Finish_OnRename();
    fn Begin_OnSave();
    fn Finish_OnSave();
    fn Begin_OnClose();
    fn Finish_OnClose();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl AsyncIAdviseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAdviseSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIAdviseSinkVtbl {
        unsafe extern "system" fn Begin_OnDataChange<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_OnDataChange<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_OnViewChange<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_OnViewChange<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_OnRename<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_OnRename<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_OnSave<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_OnSave<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_OnClose<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_OnClose<Impl: AsyncIAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Begin_OnDataChange::<Impl, IMPL_OFFSET>,
            Finish_OnDataChange::<Impl, IMPL_OFFSET>,
            Begin_OnViewChange::<Impl, IMPL_OFFSET>,
            Finish_OnViewChange::<Impl, IMPL_OFFSET>,
            Begin_OnRename::<Impl, IMPL_OFFSET>,
            Finish_OnRename::<Impl, IMPL_OFFSET>,
            Begin_OnSave::<Impl, IMPL_OFFSET>,
            Finish_OnSave::<Impl, IMPL_OFFSET>,
            Begin_OnClose::<Impl, IMPL_OFFSET>,
            Finish_OnClose::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIAdviseSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSink2Impl: Sized + AsyncIAdviseSinkImpl {
    fn Begin_OnLinkSrcChange();
    fn Finish_OnLinkSrcChange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl AsyncIAdviseSink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIAdviseSink2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIAdviseSink2Vtbl {
        unsafe extern "system" fn Begin_OnLinkSrcChange<Impl: AsyncIAdviseSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_OnLinkSrcChange<Impl: AsyncIAdviseSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Begin_OnDataChange::<Impl, IMPL_OFFSET>,
            Finish_OnDataChange::<Impl, IMPL_OFFSET>,
            Begin_OnViewChange::<Impl, IMPL_OFFSET>,
            Finish_OnViewChange::<Impl, IMPL_OFFSET>,
            Begin_OnRename::<Impl, IMPL_OFFSET>,
            Finish_OnRename::<Impl, IMPL_OFFSET>,
            Begin_OnSave::<Impl, IMPL_OFFSET>,
            Finish_OnSave::<Impl, IMPL_OFFSET>,
            Begin_OnClose::<Impl, IMPL_OFFSET>,
            Finish_OnClose::<Impl, IMPL_OFFSET>,
            Begin_OnLinkSrcChange::<Impl, IMPL_OFFSET>,
            Finish_OnLinkSrcChange::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIAdviseSink2 as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIMultiQIImpl: Sized {
    fn Begin_QueryMultipleInterfaces();
    fn Finish_QueryMultipleInterfaces();
}
impl AsyncIMultiQIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIMultiQIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIMultiQIVtbl {
        unsafe extern "system" fn Begin_QueryMultipleInterfaces<Impl: AsyncIMultiQIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_QueryMultipleInterfaces<Impl: AsyncIMultiQIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin_QueryMultipleInterfaces::<Impl, IMPL_OFFSET>, Finish_QueryMultipleInterfaces::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIMultiQI as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIPipeByteImpl: Sized {
    fn Begin_Pull();
    fn Finish_Pull();
    fn Begin_Push();
    fn Finish_Push();
}
impl AsyncIPipeByteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIPipeByteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIPipeByteVtbl {
        unsafe extern "system" fn Begin_Pull<Impl: AsyncIPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Pull<Impl: AsyncIPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_Push<Impl: AsyncIPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Push<Impl: AsyncIPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin_Pull::<Impl, IMPL_OFFSET>, Finish_Pull::<Impl, IMPL_OFFSET>, Begin_Push::<Impl, IMPL_OFFSET>, Finish_Push::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIPipeByte as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIPipeDoubleImpl: Sized {
    fn Begin_Pull();
    fn Finish_Pull();
    fn Begin_Push();
    fn Finish_Push();
}
impl AsyncIPipeDoubleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIPipeDoubleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIPipeDoubleVtbl {
        unsafe extern "system" fn Begin_Pull<Impl: AsyncIPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Pull<Impl: AsyncIPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_Push<Impl: AsyncIPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Push<Impl: AsyncIPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin_Pull::<Impl, IMPL_OFFSET>, Finish_Pull::<Impl, IMPL_OFFSET>, Begin_Push::<Impl, IMPL_OFFSET>, Finish_Push::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIPipeDouble as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIPipeLongImpl: Sized {
    fn Begin_Pull();
    fn Finish_Pull();
    fn Begin_Push();
    fn Finish_Push();
}
impl AsyncIPipeLongVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIPipeLongImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIPipeLongVtbl {
        unsafe extern "system" fn Begin_Pull<Impl: AsyncIPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Pull<Impl: AsyncIPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_Push<Impl: AsyncIPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Push<Impl: AsyncIPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin_Pull::<Impl, IMPL_OFFSET>, Finish_Pull::<Impl, IMPL_OFFSET>, Begin_Push::<Impl, IMPL_OFFSET>, Finish_Push::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIPipeLong as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIUnknownImpl: Sized {
    fn Begin_QueryInterface();
    fn Finish_QueryInterface();
    fn Begin_AddRef();
    fn Finish_AddRef();
    fn Begin_Release();
    fn Finish_Release();
}
impl AsyncIUnknownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: AsyncIUnknownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> AsyncIUnknownVtbl {
        unsafe extern "system" fn Begin_QueryInterface<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_QueryInterface<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_AddRef<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_AddRef<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Begin_Release<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Finish_Release<Impl: AsyncIUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Begin_QueryInterface::<Impl, IMPL_OFFSET>, Finish_QueryInterface::<Impl, IMPL_OFFSET>, Begin_AddRef::<Impl, IMPL_OFFSET>, Finish_AddRef::<Impl, IMPL_OFFSET>, Begin_Release::<Impl, IMPL_OFFSET>, Finish_Release::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIUnknown as ::windows::core::Interface>::IID
    }
}
pub trait IActivationFilterImpl: Sized {
    fn HandleActivation();
}
impl IActivationFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IActivationFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IActivationFilterVtbl {
        unsafe extern "system" fn HandleActivation<Impl: IActivationFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwactivationtype: u32, rclsid: *const ::windows::core::GUID, preplacementclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, HandleActivation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationFilter as ::windows::core::Interface>::IID
    }
}
pub trait IAddrExclusionControlImpl: Sized {
    fn GetCurrentAddrExclusionList();
    fn UpdateAddrExclusionList();
}
impl IAddrExclusionControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddrExclusionControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAddrExclusionControlVtbl {
        unsafe extern "system" fn GetCurrentAddrExclusionList<Impl: IAddrExclusionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UpdateAddrExclusionList<Impl: IAddrExclusionControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penumerator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCurrentAddrExclusionList::<Impl, IMPL_OFFSET>, UpdateAddrExclusionList::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAddrExclusionControl as ::windows::core::Interface>::IID
    }
}
pub trait IAddrTrackingControlImpl: Sized {
    fn EnableCOMDynamicAddrTracking();
    fn DisableCOMDynamicAddrTracking();
}
impl IAddrTrackingControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAddrTrackingControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAddrTrackingControlVtbl {
        unsafe extern "system" fn EnableCOMDynamicAddrTracking<Impl: IAddrTrackingControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableCOMDynamicAddrTracking<Impl: IAddrTrackingControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnableCOMDynamicAddrTracking::<Impl, IMPL_OFFSET>, DisableCOMDynamicAddrTracking::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAddrTrackingControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSinkImpl: Sized {
    fn OnDataChange();
    fn OnViewChange();
    fn OnRename();
    fn OnSave();
    fn OnClose();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdviseSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdviseSinkVtbl {
        unsafe extern "system" fn OnDataChange<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnViewChange<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnRename<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSave<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnClose<Impl: IAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnDataChange::<Impl, IMPL_OFFSET>, OnViewChange::<Impl, IMPL_OFFSET>, OnRename::<Impl, IMPL_OFFSET>, OnSave::<Impl, IMPL_OFFSET>, OnClose::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdviseSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSink2Impl: Sized + IAdviseSinkImpl {
    fn OnLinkSrcChange();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSink2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAdviseSink2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAdviseSink2Vtbl {
        unsafe extern "system" fn OnLinkSrcChange<Impl: IAdviseSink2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnDataChange::<Impl, IMPL_OFFSET>, OnViewChange::<Impl, IMPL_OFFSET>, OnRename::<Impl, IMPL_OFFSET>, OnSave::<Impl, IMPL_OFFSET>, OnClose::<Impl, IMPL_OFFSET>, OnLinkSrcChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdviseSink2 as ::windows::core::Interface>::IID
    }
}
pub trait IAgileObjectImpl: Sized {}
impl IAgileObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAgileObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAgileObjectVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAgileObject as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncManagerImpl: Sized {
    fn CompleteCall();
    fn GetCallContext();
    fn GetState();
}
impl IAsyncManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncManagerVtbl {
        unsafe extern "system" fn CompleteCall<Impl: IAsyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCallContext<Impl: IAsyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetState<Impl: IAsyncManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstateflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CompleteCall::<Impl, IMPL_OFFSET>, GetCallContext::<Impl, IMPL_OFFSET>, GetState::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncManager as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncRpcChannelBufferImpl: Sized + IRpcChannelBuffer2Impl + IRpcChannelBufferImpl {
    fn Send();
    fn Receive();
    fn GetDestCtxEx();
}
impl IAsyncRpcChannelBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncRpcChannelBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncRpcChannelBufferVtbl {
        unsafe extern "system" fn Send<Impl: IAsyncRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: ::windows::core::RawPtr, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Receive<Impl: IAsyncRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDestCtxEx<Impl: IAsyncRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetBuffer::<Impl, IMPL_OFFSET>, SendReceive::<Impl, IMPL_OFFSET>, FreeBuffer::<Impl, IMPL_OFFSET>, GetDestCtx::<Impl, IMPL_OFFSET>, IsConnected::<Impl, IMPL_OFFSET>, GetProtocolVersion::<Impl, IMPL_OFFSET>, Send::<Impl, IMPL_OFFSET>, Receive::<Impl, IMPL_OFFSET>, GetDestCtxEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncRpcChannelBuffer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAuthenticateImpl: Sized {
    fn Authenticate();
}
#[cfg(feature = "Win32_Foundation")]
impl IAuthenticateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAuthenticateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAuthenticateVtbl {
        unsafe extern "system" fn Authenticate<Impl: IAuthenticateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Authenticate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAuthenticate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAuthenticateExImpl: Sized + IAuthenticateImpl {
    fn AuthenticateEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IAuthenticateExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAuthenticateExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAuthenticateExVtbl {
        unsafe extern "system" fn AuthenticateEx<Impl: IAuthenticateExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut super::super::Foundation::PWSTR, pszpassword: *mut super::super::Foundation::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Authenticate::<Impl, IMPL_OFFSET>, AuthenticateEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAuthenticateEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBindCtxImpl: Sized {
    fn RegisterObjectBound();
    fn RevokeObjectBound();
    fn ReleaseBoundObjects();
    fn SetBindOptions();
    fn GetBindOptions();
    fn GetRunningObjectTable();
    fn RegisterObjectParam();
    fn GetObjectParam();
    fn EnumObjectParam();
    fn RevokeObjectParam();
}
#[cfg(feature = "Win32_Foundation")]
impl IBindCtxVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindCtxImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindCtxVtbl {
        unsafe extern "system" fn RegisterObjectBound<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevokeObjectBound<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseBoundObjects<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBindOptions<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindopts: *const BIND_OPTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBindOptions<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRunningObjectTable<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterObjectParam<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkey: super::super::Foundation::PWSTR, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObjectParam<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkey: super::super::Foundation::PWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumObjectParam<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevokeObjectParam<Impl: IBindCtxImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkey: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            RegisterObjectBound::<Impl, IMPL_OFFSET>,
            RevokeObjectBound::<Impl, IMPL_OFFSET>,
            ReleaseBoundObjects::<Impl, IMPL_OFFSET>,
            SetBindOptions::<Impl, IMPL_OFFSET>,
            GetBindOptions::<Impl, IMPL_OFFSET>,
            GetRunningObjectTable::<Impl, IMPL_OFFSET>,
            RegisterObjectParam::<Impl, IMPL_OFFSET>,
            GetObjectParam::<Impl, IMPL_OFFSET>,
            EnumObjectParam::<Impl, IMPL_OFFSET>,
            RevokeObjectParam::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindCtx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBindHostImpl: Sized {
    fn CreateMoniker();
    fn MonikerBindToStorage();
    fn MonikerBindToObject();
}
#[cfg(feature = "Win32_Foundation")]
impl IBindHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindHostImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindHostVtbl {
        unsafe extern "system" fn CreateMoniker<Impl: IBindHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, ppmk: *mut ::windows::core::RawPtr, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MonikerBindToStorage<Impl: IBindHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MonikerBindToObject<Impl: IBindHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: ::windows::core::RawPtr, pbc: ::windows::core::RawPtr, pbsc: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateMoniker::<Impl, IMPL_OFFSET>, MonikerBindToStorage::<Impl, IMPL_OFFSET>, MonikerBindToObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallbackImpl: Sized {
    fn OnStartBinding();
    fn GetPriority();
    fn OnLowResource();
    fn OnProgress();
    fn OnStopBinding();
    fn GetBindInfo();
    fn OnDataAvailable();
    fn OnObjectAvailable();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IBindStatusCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindStatusCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindStatusCallbackVtbl {
        unsafe extern "system" fn OnStartBinding<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pib: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPriority<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLowResource<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnProgress<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnStopBinding<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, szerror: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBindInfo<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDataAvailable<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjectAvailable<Impl: IBindStatusCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnStartBinding::<Impl, IMPL_OFFSET>, GetPriority::<Impl, IMPL_OFFSET>, OnLowResource::<Impl, IMPL_OFFSET>, OnProgress::<Impl, IMPL_OFFSET>, OnStopBinding::<Impl, IMPL_OFFSET>, GetBindInfo::<Impl, IMPL_OFFSET>, OnDataAvailable::<Impl, IMPL_OFFSET>, OnObjectAvailable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindStatusCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallbackExImpl: Sized + IBindStatusCallbackImpl {
    fn GetBindInfoEx();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IBindStatusCallbackExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindStatusCallbackExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindStatusCallbackExVtbl {
        unsafe extern "system" fn GetBindInfoEx<Impl: IBindStatusCallbackExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            OnStartBinding::<Impl, IMPL_OFFSET>,
            GetPriority::<Impl, IMPL_OFFSET>,
            OnLowResource::<Impl, IMPL_OFFSET>,
            OnProgress::<Impl, IMPL_OFFSET>,
            OnStopBinding::<Impl, IMPL_OFFSET>,
            GetBindInfo::<Impl, IMPL_OFFSET>,
            OnDataAvailable::<Impl, IMPL_OFFSET>,
            OnObjectAvailable::<Impl, IMPL_OFFSET>,
            GetBindInfoEx::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindStatusCallbackEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IBindingImpl: Sized {
    fn Abort();
    fn Suspend();
    fn Resume();
    fn SetPriority();
    fn GetPriority();
    fn GetBindResult();
}
#[cfg(feature = "Win32_Foundation")]
impl IBindingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBindingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBindingVtbl {
        unsafe extern "system" fn Abort<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Suspend<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Resume<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPriority<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPriority<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBindResult<Impl: IBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut super::super::Foundation::PWSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Abort::<Impl, IMPL_OFFSET>, Suspend::<Impl, IMPL_OFFSET>, Resume::<Impl, IMPL_OFFSET>, SetPriority::<Impl, IMPL_OFFSET>, GetPriority::<Impl, IMPL_OFFSET>, GetBindResult::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBinding as ::windows::core::Interface>::IID
    }
}
pub trait IBlockingLockImpl: Sized {
    fn Lock();
    fn Unlock();
}
impl IBlockingLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockingLockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBlockingLockVtbl {
        unsafe extern "system" fn Lock<Impl: IBlockingLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unlock<Impl: IBlockingLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Lock::<Impl, IMPL_OFFSET>, Unlock::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBlockingLock as ::windows::core::Interface>::IID
    }
}
pub trait ICallFactoryImpl: Sized {
    fn CreateCall();
}
impl ICallFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICallFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICallFactoryVtbl {
        unsafe extern "system" fn CreateCall<Impl: ICallFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pctrlunk: *mut ::core::ffi::c_void, riid2: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateCall::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFactory as ::windows::core::Interface>::IID
    }
}
pub trait ICancelMethodCallsImpl: Sized {
    fn Cancel();
    fn TestCancel();
}
impl ICancelMethodCallsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICancelMethodCallsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICancelMethodCallsVtbl {
        unsafe extern "system" fn Cancel<Impl: ICancelMethodCallsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TestCancel<Impl: ICancelMethodCallsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, TestCancel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICancelMethodCalls as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICatInformationImpl: Sized {
    fn EnumCategories();
    fn GetCategoryDesc();
    fn EnumClassesOfCategories();
    fn IsClassOfCategories();
    fn EnumImplCategoriesOfClass();
    fn EnumReqCategoriesOfClass();
}
#[cfg(feature = "Win32_Foundation")]
impl ICatInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICatInformationVtbl {
        unsafe extern "system" fn EnumCategories<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCategoryDesc<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, lcid: u32, pszdesc: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumClassesOfCategories<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID, ppenumclsid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsClassOfCategories<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumImplCategoriesOfClass<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumReqCategoriesOfClass<Impl: ICatInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnumCategories::<Impl, IMPL_OFFSET>, GetCategoryDesc::<Impl, IMPL_OFFSET>, EnumClassesOfCategories::<Impl, IMPL_OFFSET>, IsClassOfCategories::<Impl, IMPL_OFFSET>, EnumImplCategoriesOfClass::<Impl, IMPL_OFFSET>, EnumReqCategoriesOfClass::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatInformation as ::windows::core::Interface>::IID
    }
}
pub trait ICatRegisterImpl: Sized {
    fn RegisterCategories();
    fn UnRegisterCategories();
    fn RegisterClassImplCategories();
    fn UnRegisterClassImplCategories();
    fn RegisterClassReqCategories();
    fn UnRegisterClassReqCategories();
}
impl ICatRegisterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatRegisterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICatRegisterVtbl {
        unsafe extern "system" fn RegisterCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnRegisterCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterClassImplCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnRegisterClassImplCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterClassReqCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnRegisterClassReqCategories<Impl: ICatRegisterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterCategories::<Impl, IMPL_OFFSET>, UnRegisterCategories::<Impl, IMPL_OFFSET>, RegisterClassImplCategories::<Impl, IMPL_OFFSET>, UnRegisterClassImplCategories::<Impl, IMPL_OFFSET>, RegisterClassReqCategories::<Impl, IMPL_OFFSET>, UnRegisterClassReqCategories::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatRegister as ::windows::core::Interface>::IID
    }
}
pub trait IChannelHookImpl: Sized {
    fn ClientGetSize();
    fn ClientFillBuffer();
    fn ClientNotify();
    fn ServerNotify();
    fn ServerGetSize();
    fn ServerFillBuffer();
}
impl IChannelHookVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IChannelHookImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IChannelHookVtbl {
        unsafe extern "system" fn ClientGetSize<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientFillBuffer<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientNotify<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerNotify<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerGetSize<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServerFillBuffer<Impl: IChannelHookImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ClientGetSize::<Impl, IMPL_OFFSET>, ClientFillBuffer::<Impl, IMPL_OFFSET>, ClientNotify::<Impl, IMPL_OFFSET>, ServerNotify::<Impl, IMPL_OFFSET>, ServerGetSize::<Impl, IMPL_OFFSET>, ServerFillBuffer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChannelHook as ::windows::core::Interface>::IID
    }
}
pub trait IClassActivatorImpl: Sized {
    fn GetClassObject();
}
impl IClassActivatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClassActivatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClassActivatorVtbl {
        unsafe extern "system" fn GetClassObject<Impl: IClassActivatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClassActivator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IClassFactoryImpl: Sized {
    fn CreateInstance();
    fn LockServer();
}
#[cfg(feature = "Win32_Foundation")]
impl IClassFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClassFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClassFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockServer<Impl: IClassFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>, LockServer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClassFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IClientSecurityImpl: Sized {
    fn QueryBlanket();
    fn SetBlanket();
    fn CopyProxy();
}
#[cfg(feature = "Win32_Foundation")]
impl IClientSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IClientSecurityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IClientSecurityVtbl {
        unsafe extern "system" fn QueryBlanket<Impl: IClientSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBlanket<Impl: IClientSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: super::super::Foundation::PWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyProxy<Impl: IClientSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryBlanket::<Impl, IMPL_OFFSET>, SetBlanket::<Impl, IMPL_OFFSET>, CopyProxy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClientSecurity as ::windows::core::Interface>::IID
    }
}
pub trait IComThreadingInfoImpl: Sized {
    fn GetCurrentApartmentType();
    fn GetCurrentThreadType();
    fn GetCurrentLogicalThreadId();
    fn SetCurrentLogicalThreadId();
}
impl IComThreadingInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComThreadingInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComThreadingInfoVtbl {
        unsafe extern "system" fn GetCurrentApartmentType<Impl: IComThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papttype: *mut APTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentThreadType<Impl: IComThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pthreadtype: *mut THDTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentLogicalThreadId<Impl: IComThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidlogicalthreadid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentLogicalThreadId<Impl: IComThreadingInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCurrentApartmentType::<Impl, IMPL_OFFSET>, GetCurrentThreadType::<Impl, IMPL_OFFSET>, GetCurrentLogicalThreadId::<Impl, IMPL_OFFSET>, SetCurrentLogicalThreadId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComThreadingInfo as ::windows::core::Interface>::IID
    }
}
pub trait IConnectionPointImpl: Sized {
    fn GetConnectionInterface();
    fn GetConnectionPointContainer();
    fn Advise();
    fn Unadvise();
    fn EnumConnections();
}
impl IConnectionPointVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionPointImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionPointVtbl {
        unsafe extern "system" fn GetConnectionInterface<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectionPointContainer<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcpc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Advise<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punksink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumConnections<Impl: IConnectionPointImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetConnectionInterface::<Impl, IMPL_OFFSET>, GetConnectionPointContainer::<Impl, IMPL_OFFSET>, Advise::<Impl, IMPL_OFFSET>, Unadvise::<Impl, IMPL_OFFSET>, EnumConnections::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionPoint as ::windows::core::Interface>::IID
    }
}
pub trait IConnectionPointContainerImpl: Sized {
    fn EnumConnectionPoints();
    fn FindConnectionPoint();
}
impl IConnectionPointContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IConnectionPointContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IConnectionPointContainerVtbl {
        unsafe extern "system" fn EnumConnectionPoints<Impl: IConnectionPointContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindConnectionPoint<Impl: IConnectionPointContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnumConnectionPoints::<Impl, IMPL_OFFSET>, FindConnectionPoint::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionPointContainer as ::windows::core::Interface>::IID
    }
}
pub trait IContextCallbackImpl: Sized {
    fn ContextCallback();
}
impl IContextCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextCallbackVtbl {
        unsafe extern "system" fn ContextCallback<Impl: IContextCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: ::windows::core::RawPtr, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ContextCallback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextCallback as ::windows::core::Interface>::IID
    }
}
pub trait IDataAdviseHolderImpl: Sized {
    fn Advise();
    fn Unadvise();
    fn EnumAdvise();
    fn SendOnDataChange();
}
impl IDataAdviseHolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataAdviseHolderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataAdviseHolderVtbl {
        unsafe extern "system" fn Advise<Impl: IDataAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, pfetc: *const FORMATETC, advf: u32, padvise: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unadvise<Impl: IDataAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAdvise<Impl: IDataAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendOnDataChange<Impl: IDataAdviseHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: ::windows::core::RawPtr, dwreserved: u32, advf: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Advise::<Impl, IMPL_OFFSET>, Unadvise::<Impl, IMPL_OFFSET>, EnumAdvise::<Impl, IMPL_OFFSET>, SendOnDataChange::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataAdviseHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDataObjectImpl: Sized {
    fn GetData();
    fn GetDataHere();
    fn QueryGetData();
    fn GetCanonicalFormatEtc();
    fn SetData();
    fn EnumFormatEtc();
    fn DAdvise();
    fn DUnadvise();
    fn EnumDAdvise();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IDataObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataObjectVtbl {
        unsafe extern "system" fn GetData<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataHere<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryGetData<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCanonicalFormatEtc<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFormatEtc<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DAdvise<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: ::windows::core::RawPtr, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DUnadvise<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDAdvise<Impl: IDataObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetData::<Impl, IMPL_OFFSET>, GetDataHere::<Impl, IMPL_OFFSET>, QueryGetData::<Impl, IMPL_OFFSET>, GetCanonicalFormatEtc::<Impl, IMPL_OFFSET>, SetData::<Impl, IMPL_OFFSET>, EnumFormatEtc::<Impl, IMPL_OFFSET>, DAdvise::<Impl, IMPL_OFFSET>, DUnadvise::<Impl, IMPL_OFFSET>, EnumDAdvise::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IDispatchImpl: Sized {
    fn GetTypeInfoCount();
    fn GetTypeInfo();
    fn GetIDsOfNames();
    fn Invoke();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IDispatchVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispatchImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispatchVtbl {
        unsafe extern "system" fn GetTypeInfoCount<Impl: IDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeInfo<Impl: IDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIDsOfNames<Impl: IDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invoke<Impl: IDispatchImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IEnumCATEGORYINFOImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumCATEGORYINFOVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumCATEGORYINFOImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumCATEGORYINFOVtbl {
        unsafe extern "system" fn Next<Impl: IEnumCATEGORYINFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumCATEGORYINFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumCATEGORYINFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumCATEGORYINFOImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCATEGORYINFO as ::windows::core::Interface>::IID
    }
}
pub trait IEnumConnectionPointsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumConnectionPointsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumConnectionPointsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumConnectionPointsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumConnectionPointsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32, ppcp: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumConnectionPointsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumConnectionPointsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumConnectionPointsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumConnectionPoints as ::windows::core::Interface>::IID
    }
}
pub trait IEnumConnectionsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumConnectionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumConnectionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumConnectionsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumConnectionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumConnections as ::windows::core::Interface>::IID
    }
}
pub trait IEnumFORMATETCImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumFORMATETCVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFORMATETCImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumFORMATETCVtbl {
        unsafe extern "system" fn Next<Impl: IEnumFORMATETCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumFORMATETCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumFORMATETCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumFORMATETCImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumFORMATETC as ::windows::core::Interface>::IID
    }
}
pub trait IEnumGUIDImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumGUIDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumGUIDImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumGUIDVtbl {
        unsafe extern "system" fn Next<Impl: IEnumGUIDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumGUIDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumGUIDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumGUIDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumGUID as ::windows::core::Interface>::IID
    }
}
pub trait IEnumMonikerImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumMonikerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumMonikerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumMonikerVtbl {
        unsafe extern "system" fn Next<Impl: IEnumMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMoniker as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSTATDATAImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumSTATDATAVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumSTATDATAImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumSTATDATAVtbl {
        unsafe extern "system" fn Next<Impl: IEnumSTATDATAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumSTATDATAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumSTATDATAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumSTATDATAImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSTATDATA as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumStringImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumStringVtbl {
        unsafe extern "system" fn Next<Impl: IEnumStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut super::super::Foundation::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumString as ::windows::core::Interface>::IID
    }
}
pub trait IEnumUnknownImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumUnknownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumUnknownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumUnknownVtbl {
        unsafe extern "system" fn Next<Impl: IEnumUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumUnknown as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IErrorInfoImpl: Sized {
    fn GetGUID();
    fn GetSource();
    fn GetDescription();
    fn GetHelpFile();
    fn GetHelpContext();
}
#[cfg(feature = "Win32_Foundation")]
impl IErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IErrorInfoVtbl {
        unsafe extern "system" fn GetGUID<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSource<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHelpFile<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHelpContext<Impl: IErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGUID::<Impl, IMPL_OFFSET>, GetSource::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, GetHelpFile::<Impl, IMPL_OFFSET>, GetHelpContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IErrorLogImpl: Sized {
    fn AddError();
}
#[cfg(feature = "Win32_Foundation")]
impl IErrorLogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IErrorLogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IErrorLogVtbl {
        unsafe extern "system" fn AddError<Impl: IErrorLogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: super::super::Foundation::PWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorLog as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IExternalConnectionImpl: Sized {
    fn AddConnection();
    fn ReleaseConnection();
}
#[cfg(feature = "Win32_Foundation")]
impl IExternalConnectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IExternalConnectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IExternalConnectionVtbl {
        unsafe extern "system" fn AddConnection<Impl: IExternalConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseConnection<Impl: IExternalConnectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddConnection::<Impl, IMPL_OFFSET>, ReleaseConnection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExternalConnection as ::windows::core::Interface>::IID
    }
}
pub trait IFastRundownImpl: Sized {}
impl IFastRundownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFastRundownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFastRundownVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFastRundown as ::windows::core::Interface>::IID
    }
}
pub trait IForegroundTransferImpl: Sized {
    fn AllowForegroundTransfer();
}
impl IForegroundTransferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IForegroundTransferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IForegroundTransferVtbl {
        unsafe extern "system" fn AllowForegroundTransfer<Impl: IForegroundTransferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AllowForegroundTransfer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForegroundTransfer as ::windows::core::Interface>::IID
    }
}
pub trait IGlobalInterfaceTableImpl: Sized {
    fn RegisterInterfaceInGlobal();
    fn RevokeInterfaceFromGlobal();
    fn GetInterfaceFromGlobal();
}
impl IGlobalInterfaceTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalInterfaceTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalInterfaceTableVtbl {
        unsafe extern "system" fn RegisterInterfaceInGlobal<Impl: IGlobalInterfaceTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevokeInterfaceFromGlobal<Impl: IGlobalInterfaceTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInterfaceFromGlobal<Impl: IGlobalInterfaceTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterInterfaceInGlobal::<Impl, IMPL_OFFSET>, RevokeInterfaceFromGlobal::<Impl, IMPL_OFFSET>, GetInterfaceFromGlobal::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalInterfaceTable as ::windows::core::Interface>::IID
    }
}
pub trait IGlobalOptionsImpl: Sized {
    fn Set();
    fn Query();
}
impl IGlobalOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGlobalOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGlobalOptionsVtbl {
        unsafe extern "system" fn Set<Impl: IGlobalOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Query<Impl: IGlobalOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Set::<Impl, IMPL_OFFSET>, Query::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalOptions as ::windows::core::Interface>::IID
    }
}
pub trait IInitializeSpyImpl: Sized {
    fn PreInitialize();
    fn PostInitialize();
    fn PreUninitialize();
    fn PostUninitialize();
}
impl IInitializeSpyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInitializeSpyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInitializeSpyVtbl {
        unsafe extern "system" fn PreInitialize<Impl: IInitializeSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostInitialize<Impl: IInitializeSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreUninitialize<Impl: IInitializeSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostUninitialize<Impl: IInitializeSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, PreInitialize::<Impl, IMPL_OFFSET>, PostInitialize::<Impl, IMPL_OFFSET>, PreUninitialize::<Impl, IMPL_OFFSET>, PostUninitialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitializeSpy as ::windows::core::Interface>::IID
    }
}
pub trait IInternalUnknownImpl: Sized {
    fn QueryInternalInterface();
}
impl IInternalUnknownVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IInternalUnknownImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IInternalUnknownVtbl {
        unsafe extern "system" fn QueryInternalInterface<Impl: IInternalUnknownImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryInternalInterface::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternalUnknown as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMachineGlobalObjectTableImpl: Sized {
    fn RegisterObject();
    fn GetObject();
    fn RevokeObject();
}
#[cfg(feature = "Win32_Foundation")]
impl IMachineGlobalObjectTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMachineGlobalObjectTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMachineGlobalObjectTableVtbl {
        unsafe extern "system" fn RegisterObject<Impl: IMachineGlobalObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: super::super::Foundation::PWSTR, object: *mut ::core::ffi::c_void, token: *mut *mut MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IMachineGlobalObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: super::super::Foundation::PWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevokeObject<Impl: IMachineGlobalObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterObject::<Impl, IMPL_OFFSET>, GetObject::<Impl, IMPL_OFFSET>, RevokeObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMachineGlobalObjectTable as ::windows::core::Interface>::IID
    }
}
pub trait IMallocImpl: Sized {
    fn Alloc();
    fn Realloc();
    fn Free();
    fn GetSize();
    fn DidAlloc();
    fn HeapMinimize();
}
impl IMallocVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMallocImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMallocVtbl {
        unsafe extern "system" fn Alloc<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Realloc<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Free<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSize<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DidAlloc<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HeapMinimize<Impl: IMallocImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Alloc::<Impl, IMPL_OFFSET>, Realloc::<Impl, IMPL_OFFSET>, Free::<Impl, IMPL_OFFSET>, GetSize::<Impl, IMPL_OFFSET>, DidAlloc::<Impl, IMPL_OFFSET>, HeapMinimize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMalloc as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMallocSpyImpl: Sized {
    fn PreAlloc();
    fn PostAlloc();
    fn PreFree();
    fn PostFree();
    fn PreRealloc();
    fn PostRealloc();
    fn PreGetSize();
    fn PostGetSize();
    fn PreDidAlloc();
    fn PostDidAlloc();
    fn PreHeapMinimize();
    fn PostHeapMinimize();
}
#[cfg(feature = "Win32_Foundation")]
impl IMallocSpyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMallocSpyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMallocSpyVtbl {
        unsafe extern "system" fn PreAlloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbrequest: usize) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostAlloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreFree<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostFree<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreRealloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostRealloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreGetSize<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostGetSize<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreDidAlloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostDidAlloc<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PreHeapMinimize<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostHeapMinimize<Impl: IMallocSpyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            PreAlloc::<Impl, IMPL_OFFSET>,
            PostAlloc::<Impl, IMPL_OFFSET>,
            PreFree::<Impl, IMPL_OFFSET>,
            PostFree::<Impl, IMPL_OFFSET>,
            PreRealloc::<Impl, IMPL_OFFSET>,
            PostRealloc::<Impl, IMPL_OFFSET>,
            PreGetSize::<Impl, IMPL_OFFSET>,
            PostGetSize::<Impl, IMPL_OFFSET>,
            PreDidAlloc::<Impl, IMPL_OFFSET>,
            PostDidAlloc::<Impl, IMPL_OFFSET>,
            PreHeapMinimize::<Impl, IMPL_OFFSET>,
            PostHeapMinimize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMallocSpy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMonikerImpl: Sized + IPersistStreamImpl + IPersistImpl {
    fn BindToObject();
    fn BindToStorage();
    fn Reduce();
    fn ComposeWith();
    fn Enum();
    fn IsEqual();
    fn Hash();
    fn IsRunning();
    fn GetTimeOfLastChange();
    fn Inverse();
    fn CommonPrefixWith();
    fn RelativePathTo();
    fn GetDisplayName();
    fn ParseDisplayName();
    fn IsSystemMoniker();
}
#[cfg(feature = "Win32_Foundation")]
impl IMonikerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMonikerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMonikerVtbl {
        unsafe extern "system" fn BindToObject<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindToStorage<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reduce<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, dwreducehowfar: u32, ppmktoleft: *mut ::windows::core::RawPtr, ppmkreduced: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ComposeWith<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkright: ::windows::core::RawPtr, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enum<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkothermoniker: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Hash<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhash: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRunning<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pmknewlyrunning: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeOfLastChange<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Inverse<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommonPrefixWith<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkother: ::windows::core::RawPtr, ppmkprefix: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RelativePathTo<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkother: ::windows::core::RawPtr, ppmkrelpath: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayName<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, ppszdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ParseDisplayName<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr, pmktoleft: ::windows::core::RawPtr, pszdisplayname: super::super::Foundation::PWSTR, pcheaten: *mut u32, ppmkout: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSystemMoniker<Impl: IMonikerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmksys: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetClassID::<Impl, IMPL_OFFSET>,
            IsDirty::<Impl, IMPL_OFFSET>,
            Load::<Impl, IMPL_OFFSET>,
            Save::<Impl, IMPL_OFFSET>,
            GetSizeMax::<Impl, IMPL_OFFSET>,
            BindToObject::<Impl, IMPL_OFFSET>,
            BindToStorage::<Impl, IMPL_OFFSET>,
            Reduce::<Impl, IMPL_OFFSET>,
            ComposeWith::<Impl, IMPL_OFFSET>,
            Enum::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
            Hash::<Impl, IMPL_OFFSET>,
            IsRunning::<Impl, IMPL_OFFSET>,
            GetTimeOfLastChange::<Impl, IMPL_OFFSET>,
            Inverse::<Impl, IMPL_OFFSET>,
            CommonPrefixWith::<Impl, IMPL_OFFSET>,
            RelativePathTo::<Impl, IMPL_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            ParseDisplayName::<Impl, IMPL_OFFSET>,
            IsSystemMoniker::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMoniker as ::windows::core::Interface>::IID
    }
}
pub trait IMultiQIImpl: Sized {
    fn QueryMultipleInterfaces();
}
impl IMultiQIVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiQIImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiQIVtbl {
        unsafe extern "system" fn QueryMultipleInterfaces<Impl: IMultiQIImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryMultipleInterfaces::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiQI as ::windows::core::Interface>::IID
    }
}
pub trait INoMarshalImpl: Sized {}
impl INoMarshalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INoMarshalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INoMarshalVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INoMarshal as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IOplockStorageImpl: Sized {
    fn CreateStorageEx();
    fn OpenStorageEx();
}
#[cfg(feature = "Win32_Foundation")]
impl IOplockStorageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IOplockStorageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IOplockStorageVtbl {
        unsafe extern "system" fn CreateStorageEx<Impl: IOplockStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenStorageEx<Impl: IOplockStorageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: super::super::Foundation::PWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateStorageEx::<Impl, IMPL_OFFSET>, OpenStorageEx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOplockStorage as ::windows::core::Interface>::IID
    }
}
pub trait IPSFactoryBufferImpl: Sized {
    fn CreateProxy();
    fn CreateStub();
}
impl IPSFactoryBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPSFactoryBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPSFactoryBufferVtbl {
        unsafe extern "system" fn CreateProxy<Impl: IPSFactoryBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppproxy: *mut ::windows::core::RawPtr, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateStub<Impl: IPSFactoryBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punkserver: *mut ::core::ffi::c_void, ppstub: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateProxy::<Impl, IMPL_OFFSET>, CreateStub::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPSFactoryBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IPersistImpl: Sized {
    fn GetClassID();
}
impl IPersistVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistVtbl {
        unsafe extern "system" fn GetClassID<Impl: IPersistImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersist as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistFileImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn SaveCompleted();
    fn GetCurFile();
}
#[cfg(feature = "Win32_Foundation")]
impl IPersistFileVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistFileImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistFileVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR, fremember: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveCompleted<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurFile<Impl: IPersistFileImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszfilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>, Load::<Impl, IMPL_OFFSET>, Save::<Impl, IMPL_OFFSET>, SaveCompleted::<Impl, IMPL_OFFSET>, GetCurFile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistFile as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistMemoryImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn GetSizeMax();
    fn InitNew();
}
#[cfg(feature = "Win32_Foundation")]
impl IPersistMemoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistMemoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistMemoryVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSizeMax<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitNew<Impl: IPersistMemoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>, Load::<Impl, IMPL_OFFSET>, Save::<Impl, IMPL_OFFSET>, GetSizeMax::<Impl, IMPL_OFFSET>, InitNew::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistMemory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistStreamImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn GetSizeMax();
}
#[cfg(feature = "Win32_Foundation")]
impl IPersistStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistStreamVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSizeMax<Impl: IPersistStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>, Load::<Impl, IMPL_OFFSET>, Save::<Impl, IMPL_OFFSET>, GetSizeMax::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistStreamInitImpl: Sized + IPersistImpl {
    fn IsDirty();
    fn Load();
    fn Save();
    fn GetSizeMax();
    fn InitNew();
}
#[cfg(feature = "Win32_Foundation")]
impl IPersistStreamInitVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPersistStreamInitImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPersistStreamInitVtbl {
        unsafe extern "system" fn IsDirty<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Load<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Save<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSizeMax<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitNew<Impl: IPersistStreamInitImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassID::<Impl, IMPL_OFFSET>, IsDirty::<Impl, IMPL_OFFSET>, Load::<Impl, IMPL_OFFSET>, Save::<Impl, IMPL_OFFSET>, GetSizeMax::<Impl, IMPL_OFFSET>, InitNew::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistStreamInit as ::windows::core::Interface>::IID
    }
}
pub trait IPipeByteImpl: Sized {
    fn Pull();
    fn Push();
}
impl IPipeByteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPipeByteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPipeByteVtbl {
        unsafe extern "system" fn Pull<Impl: IPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Push<Impl: IPipeByteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Pull::<Impl, IMPL_OFFSET>, Push::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPipeByte as ::windows::core::Interface>::IID
    }
}
pub trait IPipeDoubleImpl: Sized {
    fn Pull();
    fn Push();
}
impl IPipeDoubleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPipeDoubleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPipeDoubleVtbl {
        unsafe extern "system" fn Pull<Impl: IPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Push<Impl: IPipeDoubleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Pull::<Impl, IMPL_OFFSET>, Push::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPipeDouble as ::windows::core::Interface>::IID
    }
}
pub trait IPipeLongImpl: Sized {
    fn Pull();
    fn Push();
}
impl IPipeLongVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPipeLongImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPipeLongVtbl {
        unsafe extern "system" fn Pull<Impl: IPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Push<Impl: IPipeLongImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Pull::<Impl, IMPL_OFFSET>, Push::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPipeLong as ::windows::core::Interface>::IID
    }
}
pub trait IProcessInitControlImpl: Sized {
    fn ResetInitializerTimeout();
}
impl IProcessInitControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessInitControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessInitControlVtbl {
        unsafe extern "system" fn ResetInitializerTimeout<Impl: IProcessInitControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsecondsremaining: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ResetInitializerTimeout::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessInitControl as ::windows::core::Interface>::IID
    }
}
pub trait IProcessLockImpl: Sized {
    fn AddRefOnProcess();
    fn ReleaseRefOnProcess();
}
impl IProcessLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessLockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessLockVtbl {
        unsafe extern "system" fn AddRefOnProcess<Impl: IProcessLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseRefOnProcess<Impl: IProcessLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddRefOnProcess::<Impl, IMPL_OFFSET>, ReleaseRefOnProcess::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessLock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProgressNotifyImpl: Sized {
    fn OnProgress();
}
#[cfg(feature = "Win32_Foundation")]
impl IProgressNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressNotifyVtbl {
        unsafe extern "system" fn OnProgress<Impl: IProgressNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnProgress::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressNotify as ::windows::core::Interface>::IID
    }
}
pub trait IROTDataImpl: Sized {
    fn GetComparisonData();
}
impl IROTDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IROTDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IROTDataVtbl {
        unsafe extern "system" fn GetComparisonData<Impl: IROTDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetComparisonData::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IROTData as ::windows::core::Interface>::IID
    }
}
pub trait IReleaseMarshalBuffersImpl: Sized {
    fn ReleaseMarshalBuffer();
}
impl IReleaseMarshalBuffersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IReleaseMarshalBuffersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IReleaseMarshalBuffersVtbl {
        unsafe extern "system" fn ReleaseMarshalBuffer<Impl: IReleaseMarshalBuffersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ReleaseMarshalBuffer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReleaseMarshalBuffers as ::windows::core::Interface>::IID
    }
}
pub trait IRpcChannelBufferImpl: Sized {
    fn GetBuffer();
    fn SendReceive();
    fn FreeBuffer();
    fn GetDestCtx();
    fn IsConnected();
}
impl IRpcChannelBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcChannelBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRpcChannelBufferVtbl {
        unsafe extern "system" fn GetBuffer<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendReceive<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeBuffer<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDestCtx<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsConnected<Impl: IRpcChannelBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetBuffer::<Impl, IMPL_OFFSET>, SendReceive::<Impl, IMPL_OFFSET>, FreeBuffer::<Impl, IMPL_OFFSET>, GetDestCtx::<Impl, IMPL_OFFSET>, IsConnected::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcChannelBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IRpcChannelBuffer2Impl: Sized + IRpcChannelBufferImpl {
    fn GetProtocolVersion();
}
impl IRpcChannelBuffer2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcChannelBuffer2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRpcChannelBuffer2Vtbl {
        unsafe extern "system" fn GetProtocolVersion<Impl: IRpcChannelBuffer2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetBuffer::<Impl, IMPL_OFFSET>, SendReceive::<Impl, IMPL_OFFSET>, FreeBuffer::<Impl, IMPL_OFFSET>, GetDestCtx::<Impl, IMPL_OFFSET>, IsConnected::<Impl, IMPL_OFFSET>, GetProtocolVersion::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcChannelBuffer2 as ::windows::core::Interface>::IID
    }
}
pub trait IRpcChannelBuffer3Impl: Sized + IRpcChannelBuffer2Impl + IRpcChannelBufferImpl {
    fn Send();
    fn Receive();
    fn Cancel();
    fn GetCallContext();
    fn GetDestCtxEx();
    fn GetState();
    fn RegisterAsync();
}
impl IRpcChannelBuffer3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcChannelBuffer3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRpcChannelBuffer3Vtbl {
        unsafe extern "system" fn Send<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Receive<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCallContext<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDestCtxEx<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetState<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterAsync<Impl: IRpcChannelBuffer3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetBuffer::<Impl, IMPL_OFFSET>,
            SendReceive::<Impl, IMPL_OFFSET>,
            FreeBuffer::<Impl, IMPL_OFFSET>,
            GetDestCtx::<Impl, IMPL_OFFSET>,
            IsConnected::<Impl, IMPL_OFFSET>,
            GetProtocolVersion::<Impl, IMPL_OFFSET>,
            Send::<Impl, IMPL_OFFSET>,
            Receive::<Impl, IMPL_OFFSET>,
            Cancel::<Impl, IMPL_OFFSET>,
            GetCallContext::<Impl, IMPL_OFFSET>,
            GetDestCtxEx::<Impl, IMPL_OFFSET>,
            GetState::<Impl, IMPL_OFFSET>,
            RegisterAsync::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcChannelBuffer3 as ::windows::core::Interface>::IID
    }
}
pub trait IRpcHelperImpl: Sized {
    fn GetDCOMProtocolVersion();
    fn GetIIDFromOBJREF();
}
impl IRpcHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRpcHelperVtbl {
        unsafe extern "system" fn GetDCOMProtocolVersion<Impl: IRpcHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIIDFromOBJREF<Impl: IRpcHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDCOMProtocolVersion::<Impl, IMPL_OFFSET>, GetIIDFromOBJREF::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcHelper as ::windows::core::Interface>::IID
    }
}
pub trait IRpcOptionsImpl: Sized {
    fn Set();
    fn Query();
}
impl IRpcOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRpcOptionsVtbl {
        unsafe extern "system" fn Set<Impl: IRpcOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Query<Impl: IRpcOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Set::<Impl, IMPL_OFFSET>, Query::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcOptions as ::windows::core::Interface>::IID
    }
}
pub trait IRpcProxyBufferImpl: Sized {
    fn Connect();
    fn Disconnect();
}
impl IRpcProxyBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcProxyBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRpcProxyBufferVtbl {
        unsafe extern "system" fn Connect<Impl: IRpcProxyBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IRpcProxyBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Connect::<Impl, IMPL_OFFSET>, Disconnect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcProxyBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IRpcStubBufferImpl: Sized {
    fn Connect();
    fn Disconnect();
    fn Invoke();
    fn IsIIDSupported();
    fn CountRefs();
    fn DebugServerQueryInterface();
    fn DebugServerRelease();
}
impl IRpcStubBufferVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcStubBufferImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRpcStubBufferVtbl {
        unsafe extern "system" fn Connect<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Disconnect<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invoke<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsIIDSupported<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer> {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CountRefs<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DebugServerQueryInterface<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DebugServerRelease<Impl: IRpcStubBufferImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Connect::<Impl, IMPL_OFFSET>, Disconnect::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, IsIIDSupported::<Impl, IMPL_OFFSET>, CountRefs::<Impl, IMPL_OFFSET>, DebugServerQueryInterface::<Impl, IMPL_OFFSET>, DebugServerRelease::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcStubBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IRpcSyntaxNegotiateImpl: Sized {
    fn NegotiateSyntax();
}
impl IRpcSyntaxNegotiateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRpcSyntaxNegotiateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRpcSyntaxNegotiateVtbl {
        unsafe extern "system" fn NegotiateSyntax<Impl: IRpcSyntaxNegotiateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, NegotiateSyntax::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcSyntaxNegotiate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRunnableObjectImpl: Sized {
    fn GetRunningClass();
    fn Run();
    fn IsRunning();
    fn LockRunning();
    fn SetContainedObject();
}
#[cfg(feature = "Win32_Foundation")]
impl IRunnableObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunnableObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRunnableObjectVtbl {
        unsafe extern "system" fn GetRunningClass<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Run<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRunning<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockRunning<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContainedObject<Impl: IRunnableObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcontained: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetRunningClass::<Impl, IMPL_OFFSET>, Run::<Impl, IMPL_OFFSET>, IsRunning::<Impl, IMPL_OFFSET>, LockRunning::<Impl, IMPL_OFFSET>, SetContainedObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunnableObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRunningObjectTableImpl: Sized {
    fn Register();
    fn Revoke();
    fn IsRunning();
    fn GetObject();
    fn NoteChangeTime();
    fn GetTimeOfLastChange();
    fn EnumRunning();
}
#[cfg(feature = "Win32_Foundation")]
impl IRunningObjectTableVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRunningObjectTableImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRunningObjectTableVtbl {
        unsafe extern "system" fn Register<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: u32, punkobject: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, pdwregister: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Revoke<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregister: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRunning<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NoteChangeTime<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTimeOfLastChange<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkobjectname: ::windows::core::RawPtr, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumRunning<Impl: IRunningObjectTableImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenummoniker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Register::<Impl, IMPL_OFFSET>, Revoke::<Impl, IMPL_OFFSET>, IsRunning::<Impl, IMPL_OFFSET>, GetObject::<Impl, IMPL_OFFSET>, NoteChangeTime::<Impl, IMPL_OFFSET>, GetTimeOfLastChange::<Impl, IMPL_OFFSET>, EnumRunning::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunningObjectTable as ::windows::core::Interface>::IID
    }
}
pub trait ISequentialStreamImpl: Sized {
    fn Read();
    fn Write();
}
impl ISequentialStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISequentialStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISequentialStreamVtbl {
        unsafe extern "system" fn Read<Impl: ISequentialStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: ISequentialStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISequentialStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IServerSecurityImpl: Sized {
    fn QueryBlanket();
    fn ImpersonateClient();
    fn RevertToSelf();
    fn IsImpersonating();
}
#[cfg(feature = "Win32_Foundation")]
impl IServerSecurityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServerSecurityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServerSecurityVtbl {
        unsafe extern "system" fn QueryBlanket<Impl: IServerSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImpersonateClient<Impl: IServerSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RevertToSelf<Impl: IServerSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsImpersonating<Impl: IServerSecurityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryBlanket::<Impl, IMPL_OFFSET>, ImpersonateClient::<Impl, IMPL_OFFSET>, RevertToSelf::<Impl, IMPL_OFFSET>, IsImpersonating::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerSecurity as ::windows::core::Interface>::IID
    }
}
pub trait IServiceProviderImpl: Sized {
    fn QueryService();
}
impl IServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceProviderVtbl {
        unsafe extern "system" fn QueryService<Impl: IServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryService::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceProvider as ::windows::core::Interface>::IID
    }
}
pub trait IStdMarshalInfoImpl: Sized {
    fn GetClassForHandler();
}
impl IStdMarshalInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStdMarshalInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStdMarshalInfoVtbl {
        unsafe extern "system" fn GetClassForHandler<Impl: IStdMarshalInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClassForHandler::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStdMarshalInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IStreamImpl: Sized + ISequentialStreamImpl {
    fn Seek();
    fn SetSize();
    fn CopyTo();
    fn Commit();
    fn Revert();
    fn LockRegion();
    fn UnlockRegion();
    fn Stat();
    fn Clone();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamVtbl {
        unsafe extern "system" fn Seek<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSize<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyTo<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: ::windows::core::RawPtr, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: StructuredStorage::STGC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Revert<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockRegion<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnlockRegion<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stat<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IStreamImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstm: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Read::<Impl, IMPL_OFFSET>, Write::<Impl, IMPL_OFFSET>, Seek::<Impl, IMPL_OFFSET>, SetSize::<Impl, IMPL_OFFSET>, CopyTo::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, Revert::<Impl, IMPL_OFFSET>, LockRegion::<Impl, IMPL_OFFSET>, UnlockRegion::<Impl, IMPL_OFFSET>, Stat::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStream as ::windows::core::Interface>::IID
    }
}
pub trait ISupportErrorInfoImpl: Sized {
    fn InterfaceSupportsErrorInfo();
}
impl ISupportErrorInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISupportErrorInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISupportErrorInfoVtbl {
        unsafe extern "system" fn InterfaceSupportsErrorInfo<Impl: ISupportErrorInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, InterfaceSupportsErrorInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportErrorInfo as ::windows::core::Interface>::IID
    }
}
pub trait ISurrogateImpl: Sized {
    fn LoadDllServer();
    fn FreeSurrogate();
}
impl ISurrogateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurrogateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISurrogateVtbl {
        unsafe extern "system" fn LoadDllServer<Impl: ISurrogateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeSurrogate<Impl: ISurrogateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, LoadDllServer::<Impl, IMPL_OFFSET>, FreeSurrogate::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurrogate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISurrogateServiceImpl: Sized {
    fn Init();
    fn ApplicationLaunch();
    fn ApplicationFree();
    fn CatalogRefresh();
    fn ProcessShutdown();
}
#[cfg(feature = "Win32_Foundation")]
impl ISurrogateServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISurrogateServiceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISurrogateServiceVtbl {
        unsafe extern "system" fn Init<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidprocessid: *const ::windows::core::GUID, pprocesslock: ::windows::core::RawPtr, pfapplicationaware: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplicationLaunch<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplicationFree<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CatalogRefresh<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProcessShutdown<Impl: ISurrogateServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shutdowntype: ShutdownType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Init::<Impl, IMPL_OFFSET>, ApplicationLaunch::<Impl, IMPL_OFFSET>, ApplicationFree::<Impl, IMPL_OFFSET>, CatalogRefresh::<Impl, IMPL_OFFSET>, ProcessShutdown::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurrogateService as ::windows::core::Interface>::IID
    }
}
pub trait ISynchronizeImpl: Sized {
    fn Wait();
    fn Signal();
    fn Reset();
}
impl ISynchronizeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronizeVtbl {
        unsafe extern "system" fn Wait<Impl: ISynchronizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Signal<Impl: ISynchronizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: ISynchronizeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Wait::<Impl, IMPL_OFFSET>, Signal::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronize as ::windows::core::Interface>::IID
    }
}
pub trait ISynchronizeContainerImpl: Sized {
    fn AddSynchronize();
    fn WaitMultiple();
}
impl ISynchronizeContainerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeContainerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronizeContainerVtbl {
        unsafe extern "system" fn AddSynchronize<Impl: ISynchronizeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WaitMultiple<Impl: ISynchronizeContainerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddSynchronize::<Impl, IMPL_OFFSET>, WaitMultiple::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizeContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronizeEventImpl: Sized + ISynchronizeHandleImpl {
    fn SetEventHandle();
}
#[cfg(feature = "Win32_Foundation")]
impl ISynchronizeEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronizeEventVtbl {
        unsafe extern "system" fn SetEventHandle<Impl: ISynchronizeEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetHandle::<Impl, IMPL_OFFSET>, SetEventHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizeEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronizeHandleImpl: Sized {
    fn GetHandle();
}
#[cfg(feature = "Win32_Foundation")]
impl ISynchronizeHandleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeHandleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronizeHandleVtbl {
        unsafe extern "system" fn GetHandle<Impl: ISynchronizeHandleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetHandle::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizeHandle as ::windows::core::Interface>::IID
    }
}
pub trait ISynchronizeMutexImpl: Sized + ISynchronizeImpl {
    fn ReleaseMutex();
}
impl ISynchronizeMutexVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISynchronizeMutexImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISynchronizeMutexVtbl {
        unsafe extern "system" fn ReleaseMutex<Impl: ISynchronizeMutexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Wait::<Impl, IMPL_OFFSET>, Signal::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, ReleaseMutex::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizeMutex as ::windows::core::Interface>::IID
    }
}
pub trait ITimeAndNoticeControlImpl: Sized {
    fn SuppressChanges();
}
impl ITimeAndNoticeControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITimeAndNoticeControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITimeAndNoticeControlVtbl {
        unsafe extern "system" fn SuppressChanges<Impl: ITimeAndNoticeControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, res1: u32, res2: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SuppressChanges::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeAndNoticeControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ITypeCompImpl: Sized {
    fn Bind();
    fn BindType();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ITypeCompVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeCompImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeCompVtbl {
        unsafe extern "system" fn Bind<Impl: ITypeCompImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, lhashval: u32, wflags: u16, pptinfo: *mut ::windows::core::RawPtr, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindType<Impl: ITypeCompImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Bind::<Impl, IMPL_OFFSET>, BindType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeComp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ITypeInfoImpl: Sized {
    fn GetTypeAttr();
    fn GetTypeComp();
    fn GetFuncDesc();
    fn GetVarDesc();
    fn GetNames();
    fn GetRefTypeOfImplType();
    fn GetImplTypeFlags();
    fn GetIDsOfNames();
    fn Invoke();
    fn GetDocumentation();
    fn GetDllEntry();
    fn GetRefTypeInfo();
    fn AddressOfMember();
    fn CreateInstance();
    fn GetMops();
    fn GetContainingTypeLib();
    fn ReleaseTypeAttr();
    fn ReleaseFuncDesc();
    fn ReleaseVarDesc();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ITypeInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeInfoVtbl {
        unsafe extern "system" fn GetTypeAttr<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeComp<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFuncDesc<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVarDesc<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetNames<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRefTypeOfImplType<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, preftype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImplTypeFlags<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pimpltypeflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIDsOfNames<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgsznames: *const super::super::Foundation::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invoke<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentation<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDllEntry<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRefTypeInfo<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreftype: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddressOfMember<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstance<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMops<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, pbstrmops: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContainingTypeLib<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlib: *mut ::windows::core::RawPtr, pindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseTypeAttr<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeattr: *const TYPEATTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseFuncDesc<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuncdesc: *const FUNCDESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseVarDesc<Impl: ITypeInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardesc: *const VARDESC) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeAttr::<Impl, IMPL_OFFSET>,
            GetTypeComp::<Impl, IMPL_OFFSET>,
            GetFuncDesc::<Impl, IMPL_OFFSET>,
            GetVarDesc::<Impl, IMPL_OFFSET>,
            GetNames::<Impl, IMPL_OFFSET>,
            GetRefTypeOfImplType::<Impl, IMPL_OFFSET>,
            GetImplTypeFlags::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetDocumentation::<Impl, IMPL_OFFSET>,
            GetDllEntry::<Impl, IMPL_OFFSET>,
            GetRefTypeInfo::<Impl, IMPL_OFFSET>,
            AddressOfMember::<Impl, IMPL_OFFSET>,
            CreateInstance::<Impl, IMPL_OFFSET>,
            GetMops::<Impl, IMPL_OFFSET>,
            GetContainingTypeLib::<Impl, IMPL_OFFSET>,
            ReleaseTypeAttr::<Impl, IMPL_OFFSET>,
            ReleaseFuncDesc::<Impl, IMPL_OFFSET>,
            ReleaseVarDesc::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ITypeInfo2Impl: Sized + ITypeInfoImpl {
    fn GetTypeKind();
    fn GetTypeFlags();
    fn GetFuncIndexOfMemId();
    fn GetVarIndexOfMemId();
    fn GetCustData();
    fn GetFuncCustData();
    fn GetParamCustData();
    fn GetVarCustData();
    fn GetImplTypeCustData();
    fn GetDocumentation2();
    fn GetAllCustData();
    fn GetAllFuncCustData();
    fn GetAllParamCustData();
    fn GetAllVarCustData();
    fn GetAllImplTypeCustData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ITypeInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeInfo2Vtbl {
        unsafe extern "system" fn GetTypeKind<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypekind: *mut TYPEKIND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeFlags<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFuncIndexOfMemId<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVarIndexOfMemId<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFuncCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetParamCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVarCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetImplTypeCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentation2<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllFuncCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllParamCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllVarCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllImplTypeCustData<Impl: ITypeInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeAttr::<Impl, IMPL_OFFSET>,
            GetTypeComp::<Impl, IMPL_OFFSET>,
            GetFuncDesc::<Impl, IMPL_OFFSET>,
            GetVarDesc::<Impl, IMPL_OFFSET>,
            GetNames::<Impl, IMPL_OFFSET>,
            GetRefTypeOfImplType::<Impl, IMPL_OFFSET>,
            GetImplTypeFlags::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetDocumentation::<Impl, IMPL_OFFSET>,
            GetDllEntry::<Impl, IMPL_OFFSET>,
            GetRefTypeInfo::<Impl, IMPL_OFFSET>,
            AddressOfMember::<Impl, IMPL_OFFSET>,
            CreateInstance::<Impl, IMPL_OFFSET>,
            GetMops::<Impl, IMPL_OFFSET>,
            GetContainingTypeLib::<Impl, IMPL_OFFSET>,
            ReleaseTypeAttr::<Impl, IMPL_OFFSET>,
            ReleaseFuncDesc::<Impl, IMPL_OFFSET>,
            ReleaseVarDesc::<Impl, IMPL_OFFSET>,
            GetTypeKind::<Impl, IMPL_OFFSET>,
            GetTypeFlags::<Impl, IMPL_OFFSET>,
            GetFuncIndexOfMemId::<Impl, IMPL_OFFSET>,
            GetVarIndexOfMemId::<Impl, IMPL_OFFSET>,
            GetCustData::<Impl, IMPL_OFFSET>,
            GetFuncCustData::<Impl, IMPL_OFFSET>,
            GetParamCustData::<Impl, IMPL_OFFSET>,
            GetVarCustData::<Impl, IMPL_OFFSET>,
            GetImplTypeCustData::<Impl, IMPL_OFFSET>,
            GetDocumentation2::<Impl, IMPL_OFFSET>,
            GetAllCustData::<Impl, IMPL_OFFSET>,
            GetAllFuncCustData::<Impl, IMPL_OFFSET>,
            GetAllParamCustData::<Impl, IMPL_OFFSET>,
            GetAllVarCustData::<Impl, IMPL_OFFSET>,
            GetAllImplTypeCustData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITypeLibImpl: Sized {
    fn GetTypeInfoCount();
    fn GetTypeInfo();
    fn GetTypeInfoType();
    fn GetTypeInfoOfGuid();
    fn GetLibAttr();
    fn GetTypeComp();
    fn GetDocumentation();
    fn IsName();
    fn FindName();
    fn ReleaseTLibAttr();
}
#[cfg(feature = "Win32_Foundation")]
impl ITypeLibVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeLibImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeLibVtbl {
        unsafe extern "system" fn GetTypeInfoCount<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeInfo<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeInfoType<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeInfoOfGuid<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pptinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLibAttr<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTypeComp<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptcomp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentation<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsName<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindName<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznamebuf: super::super::Foundation::PWSTR, lhashval: u32, pptinfo: *mut ::windows::core::RawPtr, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseTLibAttr<Impl: ITypeLibImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptlibattr: *const TLIBATTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetTypeInfoType::<Impl, IMPL_OFFSET>,
            GetTypeInfoOfGuid::<Impl, IMPL_OFFSET>,
            GetLibAttr::<Impl, IMPL_OFFSET>,
            GetTypeComp::<Impl, IMPL_OFFSET>,
            GetDocumentation::<Impl, IMPL_OFFSET>,
            IsName::<Impl, IMPL_OFFSET>,
            FindName::<Impl, IMPL_OFFSET>,
            ReleaseTLibAttr::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeLib as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ITypeLib2Impl: Sized + ITypeLibImpl {
    fn GetCustData();
    fn GetLibStatistics();
    fn GetDocumentation2();
    fn GetAllCustData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ITypeLib2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeLib2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeLib2Vtbl {
        unsafe extern "system" fn GetCustData<Impl: ITypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLibStatistics<Impl: ITypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDocumentation2<Impl: ITypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAllCustData<Impl: ITypeLib2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetTypeInfoType::<Impl, IMPL_OFFSET>,
            GetTypeInfoOfGuid::<Impl, IMPL_OFFSET>,
            GetLibAttr::<Impl, IMPL_OFFSET>,
            GetTypeComp::<Impl, IMPL_OFFSET>,
            GetDocumentation::<Impl, IMPL_OFFSET>,
            IsName::<Impl, IMPL_OFFSET>,
            FindName::<Impl, IMPL_OFFSET>,
            ReleaseTLibAttr::<Impl, IMPL_OFFSET>,
            GetCustData::<Impl, IMPL_OFFSET>,
            GetLibStatistics::<Impl, IMPL_OFFSET>,
            GetDocumentation2::<Impl, IMPL_OFFSET>,
            GetAllCustData::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeLib2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITypeLibRegistrationImpl: Sized {
    fn GetGuid();
    fn GetVersion();
    fn GetLcid();
    fn GetWin32Path();
    fn GetWin64Path();
    fn GetDisplayName();
    fn GetFlags();
    fn GetHelpDir();
}
#[cfg(feature = "Win32_Foundation")]
impl ITypeLibRegistrationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeLibRegistrationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeLibRegistrationVtbl {
        unsafe extern "system" fn GetGuid<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetVersion<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLcid<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWin32Path<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwin32path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWin64Path<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwin64path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayName<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFlags<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHelpDir<Impl: ITypeLibRegistrationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phelpdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetGuid::<Impl, IMPL_OFFSET>, GetVersion::<Impl, IMPL_OFFSET>, GetLcid::<Impl, IMPL_OFFSET>, GetWin32Path::<Impl, IMPL_OFFSET>, GetWin64Path::<Impl, IMPL_OFFSET>, GetDisplayName::<Impl, IMPL_OFFSET>, GetFlags::<Impl, IMPL_OFFSET>, GetHelpDir::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeLibRegistration as ::windows::core::Interface>::IID
    }
}
pub trait ITypeLibRegistrationReaderImpl: Sized {
    fn EnumTypeLibRegistrations();
}
impl ITypeLibRegistrationReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITypeLibRegistrationReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITypeLibRegistrationReaderVtbl {
        unsafe extern "system" fn EnumTypeLibRegistrations<Impl: ITypeLibRegistrationReaderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, EnumTypeLibRegistrations::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeLibRegistrationReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUriImpl: Sized {
    fn GetPropertyBSTR();
    fn GetPropertyLength();
    fn GetPropertyDWORD();
    fn HasProperty();
    fn GetAbsoluteUri();
    fn GetAuthority();
    fn GetDisplayUri();
    fn GetDomain();
    fn GetExtension();
    fn GetFragment();
    fn GetHost();
    fn GetPassword();
    fn GetPath();
    fn GetPathAndQuery();
    fn GetQuery();
    fn GetRawUri();
    fn GetSchemeName();
    fn GetUserInfo();
    fn GetUserName();
    fn GetHostType();
    fn GetPort();
    fn GetScheme();
    fn GetZone();
    fn GetProperties();
    fn IsEqual();
}
#[cfg(feature = "Win32_Foundation")]
impl IUriVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriVtbl {
        unsafe extern "system" fn GetPropertyBSTR<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyLength<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyDWORD<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasProperty<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAbsoluteUri<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrabsoluteuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAuthority<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayUri<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplaystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDomain<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetExtension<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextension: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFragment<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfragment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHost<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhost: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPassword<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpassword: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPath<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPathAndQuery<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathandquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuery<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRawUri<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrawuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSchemeName<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrschemename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserInfo<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruserinfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserName<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHostType<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhosttype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPort<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetScheme<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscheme: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetZone<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperties<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEqual<Impl: IUriImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: ::windows::core::RawPtr, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetPropertyBSTR::<Impl, IMPL_OFFSET>,
            GetPropertyLength::<Impl, IMPL_OFFSET>,
            GetPropertyDWORD::<Impl, IMPL_OFFSET>,
            HasProperty::<Impl, IMPL_OFFSET>,
            GetAbsoluteUri::<Impl, IMPL_OFFSET>,
            GetAuthority::<Impl, IMPL_OFFSET>,
            GetDisplayUri::<Impl, IMPL_OFFSET>,
            GetDomain::<Impl, IMPL_OFFSET>,
            GetExtension::<Impl, IMPL_OFFSET>,
            GetFragment::<Impl, IMPL_OFFSET>,
            GetHost::<Impl, IMPL_OFFSET>,
            GetPassword::<Impl, IMPL_OFFSET>,
            GetPath::<Impl, IMPL_OFFSET>,
            GetPathAndQuery::<Impl, IMPL_OFFSET>,
            GetQuery::<Impl, IMPL_OFFSET>,
            GetRawUri::<Impl, IMPL_OFFSET>,
            GetSchemeName::<Impl, IMPL_OFFSET>,
            GetUserInfo::<Impl, IMPL_OFFSET>,
            GetUserName::<Impl, IMPL_OFFSET>,
            GetHostType::<Impl, IMPL_OFFSET>,
            GetPort::<Impl, IMPL_OFFSET>,
            GetScheme::<Impl, IMPL_OFFSET>,
            GetZone::<Impl, IMPL_OFFSET>,
            GetProperties::<Impl, IMPL_OFFSET>,
            IsEqual::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUri as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUriBuilderImpl: Sized {
    fn CreateUriSimple();
    fn CreateUri();
    fn CreateUriWithFlags();
    fn GetIUri();
    fn SetIUri();
    fn GetFragment();
    fn GetHost();
    fn GetPassword();
    fn GetPath();
    fn GetPort();
    fn GetQuery();
    fn GetSchemeName();
    fn GetUserName();
    fn SetFragment();
    fn SetHost();
    fn SetPassword();
    fn SetPath();
    fn SetPort();
    fn SetQuery();
    fn SetSchemeName();
    fn SetUserName();
    fn RemoveProperties();
    fn HasBeenModified();
}
#[cfg(feature = "Win32_Foundation")]
impl IUriBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUriBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUriBuilderVtbl {
        unsafe extern "system" fn CreateUriSimple<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateUri<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateUriWithFlags<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIUri<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiuri: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIUri<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piuri: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFragment<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHost<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPassword<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPath<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPort<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQuery<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSchemeName<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserName<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFragment<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetHost<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPassword<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPath<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPort<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQuery<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSchemeName<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserName<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveProperties<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpropertymask: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HasBeenModified<Impl: IUriBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmodified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            CreateUriSimple::<Impl, IMPL_OFFSET>,
            CreateUri::<Impl, IMPL_OFFSET>,
            CreateUriWithFlags::<Impl, IMPL_OFFSET>,
            GetIUri::<Impl, IMPL_OFFSET>,
            SetIUri::<Impl, IMPL_OFFSET>,
            GetFragment::<Impl, IMPL_OFFSET>,
            GetHost::<Impl, IMPL_OFFSET>,
            GetPassword::<Impl, IMPL_OFFSET>,
            GetPath::<Impl, IMPL_OFFSET>,
            GetPort::<Impl, IMPL_OFFSET>,
            GetQuery::<Impl, IMPL_OFFSET>,
            GetSchemeName::<Impl, IMPL_OFFSET>,
            GetUserName::<Impl, IMPL_OFFSET>,
            SetFragment::<Impl, IMPL_OFFSET>,
            SetHost::<Impl, IMPL_OFFSET>,
            SetPassword::<Impl, IMPL_OFFSET>,
            SetPath::<Impl, IMPL_OFFSET>,
            SetPort::<Impl, IMPL_OFFSET>,
            SetQuery::<Impl, IMPL_OFFSET>,
            SetSchemeName::<Impl, IMPL_OFFSET>,
            SetUserName::<Impl, IMPL_OFFSET>,
            RemoveProperties::<Impl, IMPL_OFFSET>,
            HasBeenModified::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriBuilder as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUrlMonImpl: Sized {
    fn AsyncGetClassBits();
}
#[cfg(feature = "Win32_Foundation")]
impl IUrlMonVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUrlMonImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUrlMonVtbl {
        unsafe extern "system" fn AsyncGetClassBits<Impl: IUrlMonImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, psztype: super::super::Foundation::PWSTR, pszext: super::super::Foundation::PWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: super::super::Foundation::PWSTR, pbc: ::windows::core::RawPtr, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AsyncGetClassBits::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUrlMon as ::windows::core::Interface>::IID
    }
}
pub trait IWaitMultipleImpl: Sized {
    fn WaitMultiple();
    fn AddSynchronize();
}
impl IWaitMultipleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWaitMultipleImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWaitMultipleVtbl {
        unsafe extern "system" fn WaitMultiple<Impl: IWaitMultipleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32, psync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSynchronize<Impl: IWaitMultipleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psync: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, WaitMultiple::<Impl, IMPL_OFFSET>, AddSynchronize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWaitMultiple as ::windows::core::Interface>::IID
    }
}
