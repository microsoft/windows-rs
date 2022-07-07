#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSink_Impl: Sized {
    fn Begin_OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn Finish_OnDataChange(&self);
    fn Begin_OnViewChange(&self, dwaspect: u32, lindex: i32);
    fn Finish_OnViewChange(&self);
    fn Begin_OnRename(&self, pmk: &::core::option::Option<IMoniker>);
    fn Finish_OnRename(&self);
    fn Begin_OnSave(&self);
    fn Finish_OnSave(&self);
    fn Begin_OnClose(&self);
    fn Finish_OnClose(&self);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for AsyncIAdviseSink {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl AsyncIAdviseSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>() -> AsyncIAdviseSink_Vtbl {
        unsafe extern "system" fn Begin_OnDataChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_OnDataChange(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pstgmed))
        }
        unsafe extern "system" fn Finish_OnDataChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_OnDataChange()
        }
        unsafe extern "system" fn Begin_OnViewChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_OnViewChange(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&lindex))
        }
        unsafe extern "system" fn Finish_OnViewChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_OnViewChange()
        }
        unsafe extern "system" fn Begin_OnRename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_OnRename(::core::mem::transmute(&pmk))
        }
        unsafe extern "system" fn Finish_OnRename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_OnRename()
        }
        unsafe extern "system" fn Begin_OnSave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_OnSave()
        }
        unsafe extern "system" fn Finish_OnSave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_OnSave()
        }
        unsafe extern "system" fn Begin_OnClose<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_OnClose()
        }
        unsafe extern "system" fn Finish_OnClose<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_OnClose()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_OnDataChange: Begin_OnDataChange::<Identity, Impl, OFFSET>,
            Finish_OnDataChange: Finish_OnDataChange::<Identity, Impl, OFFSET>,
            Begin_OnViewChange: Begin_OnViewChange::<Identity, Impl, OFFSET>,
            Finish_OnViewChange: Finish_OnViewChange::<Identity, Impl, OFFSET>,
            Begin_OnRename: Begin_OnRename::<Identity, Impl, OFFSET>,
            Finish_OnRename: Finish_OnRename::<Identity, Impl, OFFSET>,
            Begin_OnSave: Begin_OnSave::<Identity, Impl, OFFSET>,
            Finish_OnSave: Finish_OnSave::<Identity, Impl, OFFSET>,
            Begin_OnClose: Begin_OnClose::<Identity, Impl, OFFSET>,
            Finish_OnClose: Finish_OnClose::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIAdviseSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait AsyncIAdviseSink2_Impl: Sized + AsyncIAdviseSink_Impl {
    fn Begin_OnLinkSrcChange(&self, pmk: &::core::option::Option<IMoniker>);
    fn Finish_OnLinkSrcChange(&self);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for AsyncIAdviseSink2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl AsyncIAdviseSink2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink2_Impl, const OFFSET: isize>() -> AsyncIAdviseSink2_Vtbl {
        unsafe extern "system" fn Begin_OnLinkSrcChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_OnLinkSrcChange(::core::mem::transmute(&pmk))
        }
        unsafe extern "system" fn Finish_OnLinkSrcChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIAdviseSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_OnLinkSrcChange()
        }
        Self {
            base__: AsyncIAdviseSink_Vtbl::new::<Identity, Impl, OFFSET>(),
            Begin_OnLinkSrcChange: Begin_OnLinkSrcChange::<Identity, Impl, OFFSET>,
            Finish_OnLinkSrcChange: Finish_OnLinkSrcChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIAdviseSink2 as ::windows::core::Interface>::IID || iid == &<AsyncIAdviseSink as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIMultiQI_Impl: Sized {
    fn Begin_QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()>;
    fn Finish_QueryMultipleInterfaces(&self, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for AsyncIMultiQI {}
impl AsyncIMultiQI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIMultiQI_Impl, const OFFSET: isize>() -> AsyncIMultiQI_Vtbl {
        unsafe extern "system" fn Begin_QueryMultipleInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIMultiQI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_QueryMultipleInterfaces(::core::mem::transmute_copy(&cmqis), ::core::mem::transmute_copy(&pmqis)).into()
        }
        unsafe extern "system" fn Finish_QueryMultipleInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIMultiQI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_QueryMultipleInterfaces(::core::mem::transmute_copy(&pmqis)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_QueryMultipleInterfaces: Begin_QueryMultipleInterfaces::<Identity, Impl, OFFSET>,
            Finish_QueryMultipleInterfaces: Finish_QueryMultipleInterfaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIMultiQI as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIPipeByte_Impl: Sized {
    fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()>;
    fn Finish_Pull(&self, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::Result<()>;
    fn Begin_Push(&self, buf: *const u8, csent: u32) -> ::windows::core::Result<()>;
    fn Finish_Push(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for AsyncIPipeByte {}
impl AsyncIPipeByte_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: isize>() -> AsyncIPipeByte_Vtbl {
        unsafe extern "system" fn Begin_Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Pull(::core::mem::transmute_copy(&crequest)).into()
        }
        unsafe extern "system" fn Finish_Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut u8, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Pull(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Begin_Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Push(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into()
        }
        unsafe extern "system" fn Finish_Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeByte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Push().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, Impl, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, Impl, OFFSET>,
            Begin_Push: Begin_Push::<Identity, Impl, OFFSET>,
            Finish_Push: Finish_Push::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIPipeByte as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIPipeDouble_Impl: Sized {
    fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()>;
    fn Finish_Pull(&self, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::Result<()>;
    fn Begin_Push(&self, buf: *const f64, csent: u32) -> ::windows::core::Result<()>;
    fn Finish_Push(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for AsyncIPipeDouble {}
impl AsyncIPipeDouble_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: isize>() -> AsyncIPipeDouble_Vtbl {
        unsafe extern "system" fn Begin_Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Pull(::core::mem::transmute_copy(&crequest)).into()
        }
        unsafe extern "system" fn Finish_Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut f64, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Pull(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Begin_Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Push(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into()
        }
        unsafe extern "system" fn Finish_Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeDouble_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Push().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, Impl, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, Impl, OFFSET>,
            Begin_Push: Begin_Push::<Identity, Impl, OFFSET>,
            Finish_Push: Finish_Push::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIPipeDouble as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIPipeLong_Impl: Sized {
    fn Begin_Pull(&self, crequest: u32) -> ::windows::core::Result<()>;
    fn Finish_Pull(&self, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::Result<()>;
    fn Begin_Push(&self, buf: *const i32, csent: u32) -> ::windows::core::Result<()>;
    fn Finish_Push(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for AsyncIPipeLong {}
impl AsyncIPipeLong_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: isize>() -> AsyncIPipeLong_Vtbl {
        unsafe extern "system" fn Begin_Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crequest: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Pull(::core::mem::transmute_copy(&crequest)).into()
        }
        unsafe extern "system" fn Finish_Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut i32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Pull(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Begin_Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Push(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into()
        }
        unsafe extern "system" fn Finish_Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIPipeLong_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Push().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_Pull: Begin_Pull::<Identity, Impl, OFFSET>,
            Finish_Pull: Finish_Pull::<Identity, Impl, OFFSET>,
            Begin_Push: Begin_Push::<Identity, Impl, OFFSET>,
            Finish_Push: Finish_Push::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIPipeLong as ::windows::core::Interface>::IID
    }
}
pub trait AsyncIUnknown_Impl: Sized {
    fn Begin_QueryInterface(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Finish_QueryInterface(&self, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Begin_AddRef(&self) -> ::windows::core::Result<()>;
    fn Finish_AddRef(&self) -> u32;
    fn Begin_Release(&self) -> ::windows::core::Result<()>;
    fn Finish_Release(&self) -> u32;
}
impl ::windows::core::RuntimeName for AsyncIUnknown {}
impl AsyncIUnknown_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: isize>() -> AsyncIUnknown_Vtbl {
        unsafe extern "system" fn Begin_QueryInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_QueryInterface(::core::mem::transmute_copy(&riid)).into()
        }
        unsafe extern "system" fn Finish_QueryInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_QueryInterface(::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn Begin_AddRef<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_AddRef().into()
        }
        unsafe extern "system" fn Finish_AddRef<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_AddRef()
        }
        unsafe extern "system" fn Begin_Release<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Begin_Release().into()
        }
        unsafe extern "system" fn Finish_Release<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: AsyncIUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Finish_Release()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Begin_QueryInterface: Begin_QueryInterface::<Identity, Impl, OFFSET>,
            Finish_QueryInterface: Finish_QueryInterface::<Identity, Impl, OFFSET>,
            Begin_AddRef: Begin_AddRef::<Identity, Impl, OFFSET>,
            Finish_AddRef: Finish_AddRef::<Identity, Impl, OFFSET>,
            Begin_Release: Begin_Release::<Identity, Impl, OFFSET>,
            Finish_Release: Finish_Release::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<AsyncIUnknown as ::windows::core::Interface>::IID
    }
}
pub trait IActivationFilter_Impl: Sized {
    fn HandleActivation(&self, dwactivationtype: u32, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IActivationFilter {}
impl IActivationFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivationFilter_Impl, const OFFSET: isize>() -> IActivationFilter_Vtbl {
        unsafe extern "system" fn HandleActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IActivationFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwactivationtype: u32, rclsid: *const ::windows::core::GUID, preplacementclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HandleActivation(::core::mem::transmute_copy(&dwactivationtype), ::core::mem::transmute_copy(&rclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preplacementclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), HandleActivation: HandleActivation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IActivationFilter as ::windows::core::Interface>::IID
    }
}
pub trait IAddrExclusionControl_Impl: Sized {
    fn GetCurrentAddrExclusionList(&self, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn UpdateAddrExclusionList(&self, penumerator: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAddrExclusionControl {}
impl IAddrExclusionControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAddrExclusionControl_Impl, const OFFSET: isize>() -> IAddrExclusionControl_Vtbl {
        unsafe extern "system" fn GetCurrentAddrExclusionList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAddrExclusionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentAddrExclusionList(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppenumerator)).into()
        }
        unsafe extern "system" fn UpdateAddrExclusionList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAddrExclusionControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, penumerator: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateAddrExclusionList(::core::mem::transmute(&penumerator)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCurrentAddrExclusionList: GetCurrentAddrExclusionList::<Identity, Impl, OFFSET>,
            UpdateAddrExclusionList: UpdateAddrExclusionList::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAddrExclusionControl as ::windows::core::Interface>::IID
    }
}
pub trait IAddrTrackingControl_Impl: Sized {
    fn EnableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()>;
    fn DisableCOMDynamicAddrTracking(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAddrTrackingControl {}
impl IAddrTrackingControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAddrTrackingControl_Impl, const OFFSET: isize>() -> IAddrTrackingControl_Vtbl {
        unsafe extern "system" fn EnableCOMDynamicAddrTracking<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAddrTrackingControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableCOMDynamicAddrTracking().into()
        }
        unsafe extern "system" fn DisableCOMDynamicAddrTracking<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAddrTrackingControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableCOMDynamicAddrTracking().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnableCOMDynamicAddrTracking: EnableCOMDynamicAddrTracking::<Identity, Impl, OFFSET>,
            DisableCOMDynamicAddrTracking: DisableCOMDynamicAddrTracking::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAddrTrackingControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSink_Impl: Sized {
    fn OnDataChange(&self, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM);
    fn OnViewChange(&self, dwaspect: u32, lindex: i32);
    fn OnRename(&self, pmk: &::core::option::Option<IMoniker>);
    fn OnSave(&self);
    fn OnClose(&self);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IAdviseSink {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSink_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: isize>() -> IAdviseSink_Vtbl {
        unsafe extern "system" fn OnDataChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataChange(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pstgmed))
        }
        unsafe extern "system" fn OnViewChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaspect: u32, lindex: i32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnViewChange(::core::mem::transmute_copy(&dwaspect), ::core::mem::transmute_copy(&lindex))
        }
        unsafe extern "system" fn OnRename<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnRename(::core::mem::transmute(&pmk))
        }
        unsafe extern "system" fn OnSave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSave()
        }
        unsafe extern "system" fn OnClose<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSink_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnClose()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnDataChange: OnDataChange::<Identity, Impl, OFFSET>,
            OnViewChange: OnViewChange::<Identity, Impl, OFFSET>,
            OnRename: OnRename::<Identity, Impl, OFFSET>,
            OnSave: OnSave::<Identity, Impl, OFFSET>,
            OnClose: OnClose::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdviseSink as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IAdviseSink2_Impl: Sized + IAdviseSink_Impl {
    fn OnLinkSrcChange(&self, pmk: &::core::option::Option<IMoniker>);
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IAdviseSink2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IAdviseSink2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSink2_Impl, const OFFSET: isize>() -> IAdviseSink2_Vtbl {
        unsafe extern "system" fn OnLinkSrcChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAdviseSink2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLinkSrcChange(::core::mem::transmute(&pmk))
        }
        Self { base__: IAdviseSink_Vtbl::new::<Identity, Impl, OFFSET>(), OnLinkSrcChange: OnLinkSrcChange::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAdviseSink2 as ::windows::core::Interface>::IID || iid == &<IAdviseSink as ::windows::core::Interface>::IID
    }
}
pub trait IAgileObject_Impl: Sized {}
impl ::windows::core::RuntimeName for IAgileObject {}
impl IAgileObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAgileObject_Impl, const OFFSET: isize>() -> IAgileObject_Vtbl {
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAgileObject as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncManager_Impl: Sized {
    fn CompleteCall(&self, result: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetCallContext(&self, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetState(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IAsyncManager {}
impl IAsyncManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncManager_Impl, const OFFSET: isize>() -> IAsyncManager_Vtbl {
        unsafe extern "system" fn CompleteCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CompleteCall(::core::mem::transmute_copy(&result)).into()
        }
        unsafe extern "system" fn GetCallContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCallContext(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pinterface)).into()
        }
        unsafe extern "system" fn GetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstateflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstateflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CompleteCall: CompleteCall::<Identity, Impl, OFFSET>,
            GetCallContext: GetCallContext::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncManager as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncRpcChannelBuffer_Impl: Sized + IRpcChannelBuffer_Impl + IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, psync: &::core::option::Option<ISynchronize>, pulstatus: *mut u32) -> ::windows::core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncRpcChannelBuffer {}
impl IAsyncRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>() -> IAsyncRpcChannelBuffer_Vtbl {
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, psync: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute(&psync), ::core::mem::transmute_copy(&pulstatus)).into()
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Receive(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&pulstatus)).into()
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDestCtxEx(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&pdwdestcontext), ::core::mem::transmute_copy(&ppvdestcontext)).into()
        }
        Self {
            base__: IRpcChannelBuffer2_Vtbl::new::<Identity, Impl, OFFSET>(),
            Send: Send::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncRpcChannelBuffer as ::windows::core::Interface>::IID || iid == &<IRpcChannelBuffer as ::windows::core::Interface>::IID || iid == &<IRpcChannelBuffer2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAuthenticate_Impl: Sized {
    fn Authenticate(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAuthenticate {}
#[cfg(feature = "Win32_Foundation")]
impl IAuthenticate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticate_Impl, const OFFSET: isize>() -> IAuthenticate_Vtbl {
        unsafe extern "system" fn Authenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Authenticate(::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Authenticate: Authenticate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAuthenticate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAuthenticateEx_Impl: Sized + IAuthenticate_Impl {
    fn AuthenticateEx(&self, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAuthenticateEx {}
#[cfg(feature = "Win32_Foundation")]
impl IAuthenticateEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticateEx_Impl, const OFFSET: isize>() -> IAuthenticateEx_Vtbl {
        unsafe extern "system" fn AuthenticateEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticateEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND, pszusername: *mut ::windows::core::PWSTR, pszpassword: *mut ::windows::core::PWSTR, pauthinfo: *const AUTHENTICATEINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AuthenticateEx(::core::mem::transmute_copy(&phwnd), ::core::mem::transmute_copy(&pszusername), ::core::mem::transmute_copy(&pszpassword), ::core::mem::transmute_copy(&pauthinfo)).into()
        }
        Self { base__: IAuthenticate_Vtbl::new::<Identity, Impl, OFFSET>(), AuthenticateEx: AuthenticateEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAuthenticateEx as ::windows::core::Interface>::IID || iid == &<IAuthenticate as ::windows::core::Interface>::IID
    }
}
pub trait IBindCtx_Impl: Sized {
    fn RegisterObjectBound(&self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RevokeObjectBound(&self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn ReleaseBoundObjects(&self) -> ::windows::core::Result<()>;
    fn SetBindOptions(&self, pbindopts: *const BIND_OPTS) -> ::windows::core::Result<()>;
    fn GetBindOptions(&self, pbindopts: *mut BIND_OPTS) -> ::windows::core::Result<()>;
    fn GetRunningObjectTable(&self) -> ::windows::core::Result<IRunningObjectTable>;
    fn RegisterObjectParam(&self, pszkey: &::windows::core::PCWSTR, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetObjectParam(&self, pszkey: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn EnumObjectParam(&self) -> ::windows::core::Result<IEnumString>;
    fn RevokeObjectParam(&self, pszkey: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBindCtx {}
impl IBindCtx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>() -> IBindCtx_Vtbl {
        unsafe extern "system" fn RegisterObjectBound<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterObjectBound(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn RevokeObjectBound<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevokeObjectBound(::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn ReleaseBoundObjects<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseBoundObjects().into()
        }
        unsafe extern "system" fn SetBindOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindopts: *const BIND_OPTS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBindOptions(::core::mem::transmute_copy(&pbindopts)).into()
        }
        unsafe extern "system" fn GetBindOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbindopts: *mut BIND_OPTS) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindOptions(::core::mem::transmute_copy(&pbindopts)).into()
        }
        unsafe extern "system" fn GetRunningObjectTable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRunningObjectTable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterObjectParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkey: ::windows::core::PCWSTR, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterObjectParam(::core::mem::transmute(&pszkey), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetObjectParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkey: ::windows::core::PCWSTR, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObjectParam(::core::mem::transmute(&pszkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumObjectParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumObjectParam() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeObjectParam<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindCtx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszkey: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevokeObjectParam(::core::mem::transmute(&pszkey)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterObjectBound: RegisterObjectBound::<Identity, Impl, OFFSET>,
            RevokeObjectBound: RevokeObjectBound::<Identity, Impl, OFFSET>,
            ReleaseBoundObjects: ReleaseBoundObjects::<Identity, Impl, OFFSET>,
            SetBindOptions: SetBindOptions::<Identity, Impl, OFFSET>,
            GetBindOptions: GetBindOptions::<Identity, Impl, OFFSET>,
            GetRunningObjectTable: GetRunningObjectTable::<Identity, Impl, OFFSET>,
            RegisterObjectParam: RegisterObjectParam::<Identity, Impl, OFFSET>,
            GetObjectParam: GetObjectParam::<Identity, Impl, OFFSET>,
            EnumObjectParam: EnumObjectParam::<Identity, Impl, OFFSET>,
            RevokeObjectParam: RevokeObjectParam::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindCtx as ::windows::core::Interface>::IID
    }
}
pub trait IBindHost_Impl: Sized {
    fn CreateMoniker(&self, szname: &::windows::core::PCWSTR, pbc: &::core::option::Option<IBindCtx>, ppmk: *mut ::core::option::Option<IMoniker>, dwreserved: u32) -> ::windows::core::Result<()>;
    fn MonikerBindToStorage(&self, pmk: &::core::option::Option<IMoniker>, pbc: &::core::option::Option<IBindCtx>, pbsc: &::core::option::Option<IBindStatusCallback>, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn MonikerBindToObject(&self, pmk: &::core::option::Option<IMoniker>, pbc: &::core::option::Option<IBindCtx>, pbsc: &::core::option::Option<IBindStatusCallback>, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBindHost {}
impl IBindHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindHost_Impl, const OFFSET: isize>() -> IBindHost_Vtbl {
        unsafe extern "system" fn CreateMoniker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, pbc: *mut ::core::ffi::c_void, ppmk: *mut *mut ::core::ffi::c_void, dwreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateMoniker(::core::mem::transmute(&szname), ::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&ppmk), ::core::mem::transmute_copy(&dwreserved)).into()
        }
        unsafe extern "system" fn MonikerBindToStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pbsc: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MonikerBindToStorage(::core::mem::transmute(&pmk), ::core::mem::transmute(&pbc), ::core::mem::transmute(&pbsc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn MonikerBindToObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmk: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pbsc: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MonikerBindToObject(::core::mem::transmute(&pmk), ::core::mem::transmute(&pbc), ::core::mem::transmute(&pbsc), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateMoniker: CreateMoniker::<Identity, Impl, OFFSET>,
            MonikerBindToStorage: MonikerBindToStorage::<Identity, Impl, OFFSET>,
            MonikerBindToObject: MonikerBindToObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindHost as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallback_Impl: Sized {
    fn OnStartBinding(&self, dwreserved: u32, pib: &::core::option::Option<IBinding>) -> ::windows::core::Result<()>;
    fn GetPriority(&self) -> ::windows::core::Result<i32>;
    fn OnLowResource(&self, reserved: u32) -> ::windows::core::Result<()>;
    fn OnProgress(&self, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnStopBinding(&self, hresult: ::windows::core::HRESULT, szerror: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetBindInfo(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::Result<()>;
    fn OnDataAvailable(&self, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::Result<()>;
    fn OnObjectAvailable(&self, riid: *const ::windows::core::GUID, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IBindStatusCallback {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IBindStatusCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: isize>() -> IBindStatusCallback_Vtbl {
        unsafe extern "system" fn OnStartBinding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwreserved: u32, pib: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStartBinding(::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute(&pib)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLowResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, reserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLowResource(::core::mem::transmute_copy(&reserved)).into()
        }
        unsafe extern "system" fn OnProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulprogress: u32, ulprogressmax: u32, ulstatuscode: u32, szstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnProgress(::core::mem::transmute_copy(&ulprogress), ::core::mem::transmute_copy(&ulprogressmax), ::core::mem::transmute_copy(&ulstatuscode), ::core::mem::transmute(&szstatustext)).into()
        }
        unsafe extern "system" fn OnStopBinding<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hresult: ::windows::core::HRESULT, szerror: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStopBinding(::core::mem::transmute_copy(&hresult), ::core::mem::transmute(&szerror)).into()
        }
        unsafe extern "system" fn GetBindInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindInfo(::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo)).into()
        }
        unsafe extern "system" fn OnDataAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbscf: u32, dwsize: u32, pformatetc: *const FORMATETC, pstgmed: *const STGMEDIUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataAvailable(::core::mem::transmute_copy(&grfbscf), ::core::mem::transmute_copy(&dwsize), ::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pstgmed)).into()
        }
        unsafe extern "system" fn OnObjectAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectAvailable(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punk)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnStartBinding: OnStartBinding::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            OnLowResource: OnLowResource::<Identity, Impl, OFFSET>,
            OnProgress: OnProgress::<Identity, Impl, OFFSET>,
            OnStopBinding: OnStopBinding::<Identity, Impl, OFFSET>,
            GetBindInfo: GetBindInfo::<Identity, Impl, OFFSET>,
            OnDataAvailable: OnDataAvailable::<Identity, Impl, OFFSET>,
            OnObjectAvailable: OnObjectAvailable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindStatusCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IBindStatusCallbackEx_Impl: Sized + IBindStatusCallback_Impl {
    fn GetBindInfoEx(&self, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IBindStatusCallbackEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage"))]
impl IBindStatusCallbackEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallbackEx_Impl, const OFFSET: isize>() -> IBindStatusCallbackEx_Vtbl {
        unsafe extern "system" fn GetBindInfoEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBindStatusCallbackEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfbindf: *mut u32, pbindinfo: *mut BINDINFO, grfbindf2: *mut u32, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindInfoEx(::core::mem::transmute_copy(&grfbindf), ::core::mem::transmute_copy(&pbindinfo), ::core::mem::transmute_copy(&grfbindf2), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self { base__: IBindStatusCallback_Vtbl::new::<Identity, Impl, OFFSET>(), GetBindInfoEx: GetBindInfoEx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBindStatusCallbackEx as ::windows::core::Interface>::IID || iid == &<IBindStatusCallback as ::windows::core::Interface>::IID
    }
}
pub trait IBinding_Impl: Sized {
    fn Abort(&self) -> ::windows::core::Result<()>;
    fn Suspend(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
    fn SetPriority(&self, npriority: i32) -> ::windows::core::Result<()>;
    fn GetPriority(&self) -> ::windows::core::Result<i32>;
    fn GetBindResult(&self, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows::core::PWSTR, pdwreserved: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBinding {}
impl IBinding_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: isize>() -> IBinding_Vtbl {
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn Suspend<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Suspend().into()
        }
        unsafe extern "system" fn Resume<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Resume().into()
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, npriority: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPriority(::core::mem::transmute_copy(&npriority)).into()
        }
        unsafe extern "system" fn GetPriority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnpriority: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPriority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnpriority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsidprotocol: *mut ::windows::core::GUID, pdwresult: *mut u32, pszresult: *mut ::windows::core::PWSTR, pdwreserved: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindResult(::core::mem::transmute_copy(&pclsidprotocol), ::core::mem::transmute_copy(&pdwresult), ::core::mem::transmute_copy(&pszresult), ::core::mem::transmute_copy(&pdwreserved)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Abort: Abort::<Identity, Impl, OFFSET>,
            Suspend: Suspend::<Identity, Impl, OFFSET>,
            Resume: Resume::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
            GetPriority: GetPriority::<Identity, Impl, OFFSET>,
            GetBindResult: GetBindResult::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBinding as ::windows::core::Interface>::IID
    }
}
pub trait IBlockingLock_Impl: Sized {
    fn Lock(&self, dwtimeout: u32) -> ::windows::core::Result<()>;
    fn Unlock(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBlockingLock {}
impl IBlockingLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBlockingLock_Impl, const OFFSET: isize>() -> IBlockingLock_Vtbl {
        unsafe extern "system" fn Lock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBlockingLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Lock(::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn Unlock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IBlockingLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unlock().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Lock: Lock::<Identity, Impl, OFFSET>, Unlock: Unlock::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBlockingLock as ::windows::core::Interface>::IID
    }
}
pub trait ICallFactory_Impl: Sized {
    fn CreateCall(&self, riid: *const ::windows::core::GUID, pctrlunk: &::core::option::Option<::windows::core::IUnknown>, riid2: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for ICallFactory {}
impl ICallFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFactory_Impl, const OFFSET: isize>() -> ICallFactory_Vtbl {
        unsafe extern "system" fn CreateCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICallFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pctrlunk: *mut ::core::ffi::c_void, riid2: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateCall(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&pctrlunk), ::core::mem::transmute_copy(&riid2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppv, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateCall: CreateCall::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICallFactory as ::windows::core::Interface>::IID
    }
}
pub trait ICancelMethodCalls_Impl: Sized {
    fn Cancel(&self, ulseconds: u32) -> ::windows::core::Result<()>;
    fn TestCancel(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICancelMethodCalls {}
impl ICancelMethodCalls_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICancelMethodCalls_Impl, const OFFSET: isize>() -> ICancelMethodCalls_Vtbl {
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICancelMethodCalls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel(::core::mem::transmute_copy(&ulseconds)).into()
        }
        unsafe extern "system" fn TestCancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICancelMethodCalls_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TestCancel().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            TestCancel: TestCancel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICancelMethodCalls as ::windows::core::Interface>::IID
    }
}
pub trait ICatInformation_Impl: Sized {
    fn EnumCategories(&self, lcid: u32) -> ::windows::core::Result<IEnumCATEGORYINFO>;
    fn GetCategoryDesc(&self, rcatid: *const ::windows::core::GUID, lcid: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn EnumClassesOfCategories(&self, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID>;
    fn IsClassOfCategories(&self, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn EnumImplCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID>;
    fn EnumReqCategoriesOfClass(&self, rclsid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumGUID>;
}
impl ::windows::core::RuntimeName for ICatInformation {}
impl ICatInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: isize>() -> ICatInformation_Vtbl {
        unsafe extern "system" fn EnumCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcid: u32, ppenumcategoryinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumCategories(::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcategoryinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategoryDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rcatid: *const ::windows::core::GUID, lcid: u32, pszdesc: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCategoryDesc(::core::mem::transmute_copy(&rcatid), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumClassesOfCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID, ppenumclsid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumClassesOfCategories(::core::mem::transmute_copy(&cimplemented), ::core::mem::transmute_copy(&rgcatidimpl), ::core::mem::transmute_copy(&crequired), ::core::mem::transmute_copy(&rgcatidreq)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsClassOfCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, cimplemented: u32, rgcatidimpl: *const ::windows::core::GUID, crequired: u32, rgcatidreq: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsClassOfCategories(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&cimplemented), ::core::mem::transmute_copy(&rgcatidimpl), ::core::mem::transmute_copy(&crequired), ::core::mem::transmute_copy(&rgcatidreq)).into()
        }
        unsafe extern "system" fn EnumImplCategoriesOfClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumImplCategoriesOfClass(::core::mem::transmute_copy(&rclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcatid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumReqCategoriesOfClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ppenumcatid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumReqCategoriesOfClass(::core::mem::transmute_copy(&rclsid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcatid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumCategories: EnumCategories::<Identity, Impl, OFFSET>,
            GetCategoryDesc: GetCategoryDesc::<Identity, Impl, OFFSET>,
            EnumClassesOfCategories: EnumClassesOfCategories::<Identity, Impl, OFFSET>,
            IsClassOfCategories: IsClassOfCategories::<Identity, Impl, OFFSET>,
            EnumImplCategoriesOfClass: EnumImplCategoriesOfClass::<Identity, Impl, OFFSET>,
            EnumReqCategoriesOfClass: EnumReqCategoriesOfClass::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatInformation as ::windows::core::Interface>::IID
    }
}
pub trait ICatRegister_Impl: Sized {
    fn RegisterCategories(&self, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::Result<()>;
    fn UnRegisterCategories(&self, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnRegisterClassImplCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn UnRegisterClassReqCategories(&self, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICatRegister {}
impl ICatRegister_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: isize>() -> ICatRegister_Vtbl {
        unsafe extern "system" fn RegisterCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccategories: u32, rgcategoryinfo: *const CATEGORYINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterCategories(::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcategoryinfo)).into()
        }
        unsafe extern "system" fn UnRegisterCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnRegisterCategories(::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into()
        }
        unsafe extern "system" fn RegisterClassImplCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterClassImplCategories(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into()
        }
        unsafe extern "system" fn UnRegisterClassImplCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnRegisterClassImplCategories(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into()
        }
        unsafe extern "system" fn RegisterClassReqCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterClassReqCategories(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into()
        }
        unsafe extern "system" fn UnRegisterClassReqCategories<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatRegister_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, ccategories: u32, rgcatid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnRegisterClassReqCategories(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&ccategories), ::core::mem::transmute_copy(&rgcatid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterCategories: RegisterCategories::<Identity, Impl, OFFSET>,
            UnRegisterCategories: UnRegisterCategories::<Identity, Impl, OFFSET>,
            RegisterClassImplCategories: RegisterClassImplCategories::<Identity, Impl, OFFSET>,
            UnRegisterClassImplCategories: UnRegisterClassImplCategories::<Identity, Impl, OFFSET>,
            RegisterClassReqCategories: RegisterClassReqCategories::<Identity, Impl, OFFSET>,
            UnRegisterClassReqCategories: UnRegisterClassReqCategories::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatRegister as ::windows::core::Interface>::IID
    }
}
pub trait IChannelHook_Impl: Sized {
    fn ClientGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32);
    fn ClientFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void);
    fn ClientNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT);
    fn ServerNotify(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32);
    fn ServerGetSize(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32);
    fn ServerFillBuffer(&self, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT);
}
impl ::windows::core::RuntimeName for IChannelHook {}
impl IChannelHook_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: isize>() -> IChannelHook_Vtbl {
        unsafe extern "system" fn ClientGetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClientGetSize(::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pdatasize))
        }
        unsafe extern "system" fn ClientFillBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClientFillBuffer(::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdatabuffer))
        }
        unsafe extern "system" fn ClientNotify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32, hrfault: ::windows::core::HRESULT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClientNotify(::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pdatabuffer), ::core::mem::transmute_copy(&ldatarep), ::core::mem::transmute_copy(&hrfault))
        }
        unsafe extern "system" fn ServerNotify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, cbdatasize: u32, pdatabuffer: *const ::core::ffi::c_void, ldatarep: u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ServerNotify(::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&cbdatasize), ::core::mem::transmute_copy(&pdatabuffer), ::core::mem::transmute_copy(&ldatarep))
        }
        unsafe extern "system" fn ServerGetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, hrfault: ::windows::core::HRESULT, pdatasize: *mut u32) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ServerGetSize(::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&hrfault), ::core::mem::transmute_copy(&pdatasize))
        }
        unsafe extern "system" fn ServerFillBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IChannelHook_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uextent: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pdatasize: *mut u32, pdatabuffer: *const ::core::ffi::c_void, hrfault: ::windows::core::HRESULT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ServerFillBuffer(::core::mem::transmute_copy(&uextent), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pdatasize), ::core::mem::transmute_copy(&pdatabuffer), ::core::mem::transmute_copy(&hrfault))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ClientGetSize: ClientGetSize::<Identity, Impl, OFFSET>,
            ClientFillBuffer: ClientFillBuffer::<Identity, Impl, OFFSET>,
            ClientNotify: ClientNotify::<Identity, Impl, OFFSET>,
            ServerNotify: ServerNotify::<Identity, Impl, OFFSET>,
            ServerGetSize: ServerGetSize::<Identity, Impl, OFFSET>,
            ServerFillBuffer: ServerFillBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IChannelHook as ::windows::core::Interface>::IID
    }
}
pub trait IClassActivator_Impl: Sized {
    fn GetClassObject(&self, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IClassActivator {}
impl IClassActivator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClassActivator_Impl, const OFFSET: isize>() -> IClassActivator_Vtbl {
        unsafe extern "system" fn GetClassObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClassActivator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, dwclasscontext: u32, locale: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassObject(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&dwclasscontext), ::core::mem::transmute_copy(&locale), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetClassObject: GetClassObject::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClassActivator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IClassFactory_Impl: Sized {
    fn CreateInstance(&self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn LockServer(&self, flock: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IClassFactory {}
#[cfg(feature = "Win32_Foundation")]
impl IClassFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClassFactory_Impl, const OFFSET: isize>() -> IClassFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClassFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        unsafe extern "system" fn LockServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClassFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockServer(::core::mem::transmute_copy(&flock)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            LockServer: LockServer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClassFactory as ::windows::core::Interface>::IID
    }
}
pub trait IClientSecurity_Impl: Sized {
    fn QueryBlanket(&self, pproxy: &::core::option::Option<::windows::core::IUnknown>, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()>;
    fn SetBlanket(&self, pproxy: &::core::option::Option<::windows::core::IUnknown>, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: &::windows::core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::Result<()>;
    fn CopyProxy(&self, pproxy: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for IClientSecurity {}
impl IClientSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClientSecurity_Impl, const OFFSET: isize>() -> IClientSecurity_Vtbl {
        unsafe extern "system" fn QueryBlanket<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClientSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut RPC_C_AUTHN_LEVEL, pimplevel: *mut RPC_C_IMP_LEVEL, pauthinfo: *mut *mut ::core::ffi::c_void, pcapabilites: *mut EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryBlanket(::core::mem::transmute(&pproxy), ::core::mem::transmute_copy(&pauthnsvc), ::core::mem::transmute_copy(&pauthzsvc), ::core::mem::transmute_copy(&pserverprincname), ::core::mem::transmute_copy(&pauthnlevel), ::core::mem::transmute_copy(&pimplevel), ::core::mem::transmute_copy(&pauthinfo), ::core::mem::transmute_copy(&pcapabilites)).into()
        }
        unsafe extern "system" fn SetBlanket<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClientSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, dwauthnsvc: u32, dwauthzsvc: u32, pserverprincname: ::windows::core::PCWSTR, dwauthnlevel: RPC_C_AUTHN_LEVEL, dwimplevel: RPC_C_IMP_LEVEL, pauthinfo: *const ::core::ffi::c_void, dwcapabilities: EOLE_AUTHENTICATION_CAPABILITIES) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBlanket(::core::mem::transmute(&pproxy), ::core::mem::transmute_copy(&dwauthnsvc), ::core::mem::transmute_copy(&dwauthzsvc), ::core::mem::transmute(&pserverprincname), ::core::mem::transmute_copy(&dwauthnlevel), ::core::mem::transmute_copy(&dwimplevel), ::core::mem::transmute_copy(&pauthinfo), ::core::mem::transmute_copy(&dwcapabilities)).into()
        }
        unsafe extern "system" fn CopyProxy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IClientSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproxy: *mut ::core::ffi::c_void, ppcopy: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CopyProxy(::core::mem::transmute(&pproxy)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcopy, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryBlanket: QueryBlanket::<Identity, Impl, OFFSET>,
            SetBlanket: SetBlanket::<Identity, Impl, OFFSET>,
            CopyProxy: CopyProxy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IClientSecurity as ::windows::core::Interface>::IID
    }
}
pub trait IComThreadingInfo_Impl: Sized {
    fn GetCurrentApartmentType(&self) -> ::windows::core::Result<APTTYPE>;
    fn GetCurrentThreadType(&self) -> ::windows::core::Result<THDTYPE>;
    fn GetCurrentLogicalThreadId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetCurrentLogicalThreadId(&self, rguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComThreadingInfo {}
impl IComThreadingInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: isize>() -> IComThreadingInfo_Vtbl {
        unsafe extern "system" fn GetCurrentApartmentType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, papttype: *mut APTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentApartmentType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(papttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentThreadType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pthreadtype: *mut THDTYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentThreadType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pthreadtype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentLogicalThreadId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidlogicalthreadid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurrentLogicalThreadId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguidlogicalthreadid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentLogicalThreadId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadingInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentLogicalThreadId(::core::mem::transmute_copy(&rguid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCurrentApartmentType: GetCurrentApartmentType::<Identity, Impl, OFFSET>,
            GetCurrentThreadType: GetCurrentThreadType::<Identity, Impl, OFFSET>,
            GetCurrentLogicalThreadId: GetCurrentLogicalThreadId::<Identity, Impl, OFFSET>,
            SetCurrentLogicalThreadId: SetCurrentLogicalThreadId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComThreadingInfo as ::windows::core::Interface>::IID
    }
}
pub trait IConnectionPoint_Impl: Sized {
    fn GetConnectionInterface(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetConnectionPointContainer(&self) -> ::windows::core::Result<IConnectionPointContainer>;
    fn Advise(&self, punksink: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<u32>;
    fn Unadvise(&self, dwcookie: u32) -> ::windows::core::Result<()>;
    fn EnumConnections(&self) -> ::windows::core::Result<IEnumConnections>;
}
impl ::windows::core::RuntimeName for IConnectionPoint {}
impl IConnectionPoint_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: isize>() -> IConnectionPoint_Vtbl {
        unsafe extern "system" fn GetConnectionInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectionInterface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionPointContainer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcpc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectionPointContainer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcpc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punksink: *mut ::core::ffi::c_void, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Advise(::core::mem::transmute(&punksink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn EnumConnections<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnectionPoint_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumConnections() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetConnectionInterface: GetConnectionInterface::<Identity, Impl, OFFSET>,
            GetConnectionPointContainer: GetConnectionPointContainer::<Identity, Impl, OFFSET>,
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            EnumConnections: EnumConnections::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionPoint as ::windows::core::Interface>::IID
    }
}
pub trait IConnectionPointContainer_Impl: Sized {
    fn EnumConnectionPoints(&self) -> ::windows::core::Result<IEnumConnectionPoints>;
    fn FindConnectionPoint(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<IConnectionPoint>;
}
impl ::windows::core::RuntimeName for IConnectionPointContainer {}
impl IConnectionPointContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnectionPointContainer_Impl, const OFFSET: isize>() -> IConnectionPointContainer_Vtbl {
        unsafe extern "system" fn EnumConnectionPoints<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnectionPointContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumConnectionPoints() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindConnectionPoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IConnectionPointContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppcp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindConnectionPoint(::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            EnumConnectionPoints: EnumConnectionPoints::<Identity, Impl, OFFSET>,
            FindConnectionPoint: FindConnectionPoint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IConnectionPointContainer as ::windows::core::Interface>::IID
    }
}
pub trait IContextCallback_Impl: Sized {
    fn ContextCallback(&self, pfncallback: &PFNCONTEXTCALL, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IContextCallback {}
impl IContextCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextCallback_Impl, const OFFSET: isize>() -> IContextCallback_Vtbl {
        unsafe extern "system" fn ContextCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfncallback: *mut ::core::ffi::c_void, pparam: *const ComCallData, riid: *const ::windows::core::GUID, imethod: i32, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ContextCallback(::core::mem::transmute(&pfncallback), ::core::mem::transmute_copy(&pparam), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&imethod), ::core::mem::transmute(&punk)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ContextCallback: ContextCallback::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextCallback as ::windows::core::Interface>::IID
    }
}
pub trait IDataAdviseHolder_Impl: Sized {
    fn Advise(&self, pdataobject: &::core::option::Option<IDataObject>, pfetc: *const FORMATETC, advf: u32, padvise: &::core::option::Option<IAdviseSink>) -> ::windows::core::Result<u32>;
    fn Unadvise(&self, dwconnection: u32) -> ::windows::core::Result<()>;
    fn EnumAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA>;
    fn SendOnDataChange(&self, pdataobject: &::core::option::Option<IDataObject>, dwreserved: u32, advf: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDataAdviseHolder {}
impl IDataAdviseHolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: isize>() -> IDataAdviseHolder_Vtbl {
        unsafe extern "system" fn Advise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, pfetc: *const FORMATETC, advf: u32, padvise: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Advise(::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&pfetc), ::core::mem::transmute_copy(&advf), ::core::mem::transmute(&padvise)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unadvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unadvise(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumAdvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumAdvise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumadvise, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendOnDataChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataAdviseHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, dwreserved: u32, advf: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendOnDataChange(::core::mem::transmute(&pdataobject), ::core::mem::transmute_copy(&dwreserved), ::core::mem::transmute_copy(&advf)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Advise: Advise::<Identity, Impl, OFFSET>,
            Unadvise: Unadvise::<Identity, Impl, OFFSET>,
            EnumAdvise: EnumAdvise::<Identity, Impl, OFFSET>,
            SendOnDataChange: SendOnDataChange::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataAdviseHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDataObject_Impl: Sized {
    fn GetData(&self, pformatetcin: *const FORMATETC) -> ::windows::core::Result<STGMEDIUM>;
    fn GetDataHere(&self, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::Result<()>;
    fn QueryGetData(&self, pformatetc: *const FORMATETC) -> ::windows::core::Result<()>;
    fn GetCanonicalFormatEtc(&self, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::core::HRESULT;
    fn SetData(&self, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn EnumFormatEtc(&self, dwdirection: u32) -> ::windows::core::Result<IEnumFORMATETC>;
    fn DAdvise(&self, pformatetc: *const FORMATETC, advf: u32, padvsink: &::core::option::Option<IAdviseSink>) -> ::windows::core::Result<u32>;
    fn DUnadvise(&self, dwconnection: u32) -> ::windows::core::Result<()>;
    fn EnumDAdvise(&self) -> ::windows::core::Result<IEnumSTATDATA>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl ::windows::core::RuntimeName for IDataObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com_StructuredStorage"))]
impl IDataObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>() -> IDataObject_Vtbl {
        unsafe extern "system" fn GetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetcin: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetData(::core::mem::transmute_copy(&pformatetcin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmedium, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataHere<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *mut STGMEDIUM) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataHere(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium)).into()
        }
        unsafe extern "system" fn QueryGetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryGetData(::core::mem::transmute_copy(&pformatetc)).into()
        }
        unsafe extern "system" fn GetCanonicalFormatEtc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatectin: *const FORMATETC, pformatetcout: *mut FORMATETC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCanonicalFormatEtc(::core::mem::transmute_copy(&pformatectin), ::core::mem::transmute_copy(&pformatetcout))
        }
        unsafe extern "system" fn SetData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, pmedium: *const STGMEDIUM, frelease: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&pmedium), ::core::mem::transmute_copy(&frelease)).into()
        }
        unsafe extern "system" fn EnumFormatEtc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdirection: u32, ppenumformatetc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFormatEtc(::core::mem::transmute_copy(&dwdirection)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumformatetc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DAdvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pformatetc: *const FORMATETC, advf: u32, padvsink: *mut ::core::ffi::c_void, pdwconnection: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DAdvise(::core::mem::transmute_copy(&pformatetc), ::core::mem::transmute_copy(&advf), ::core::mem::transmute(&padvsink)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwconnection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DUnadvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwconnection: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DUnadvise(::core::mem::transmute_copy(&dwconnection)).into()
        }
        unsafe extern "system" fn EnumDAdvise<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDataObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumadvise: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDAdvise() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumadvise, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetData: GetData::<Identity, Impl, OFFSET>,
            GetDataHere: GetDataHere::<Identity, Impl, OFFSET>,
            QueryGetData: QueryGetData::<Identity, Impl, OFFSET>,
            GetCanonicalFormatEtc: GetCanonicalFormatEtc::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
            EnumFormatEtc: EnumFormatEtc::<Identity, Impl, OFFSET>,
            DAdvise: DAdvise::<Identity, Impl, OFFSET>,
            DUnadvise: DUnadvise::<Identity, Impl, OFFSET>,
            EnumDAdvise: EnumDAdvise::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IDispatch_Impl: Sized {
    fn GetTypeInfoCount(&self) -> ::windows::core::Result<u32>;
    fn GetTypeInfo(&self, itinfo: u32, lcid: u32) -> ::windows::core::Result<ITypeInfo>;
    fn GetIDsOfNames(&self, riid: *const ::windows::core::GUID, rgsznames: *const ::windows::core::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::Result<()>;
    fn Invoke(&self, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IDispatch {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IDispatch_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: isize>() -> IDispatch_Vtbl {
        unsafe extern "system" fn GetTypeInfoCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctinfo: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeInfoCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pctinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, itinfo: u32, lcid: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeInfo(::core::mem::transmute_copy(&itinfo), ::core::mem::transmute_copy(&lcid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, rgsznames: *const ::windows::core::PWSTR, cnames: u32, lcid: u32, rgdispid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIDsOfNames(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&rgdispid)).into()
        }
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispatch_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dispidmember: i32, riid: *const ::windows::core::GUID, lcid: u32, wflags: u16, pdispparams: *const DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::core::mem::transmute_copy(&dispidmember), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IEnumCATEGORYINFO_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumCATEGORYINFO>;
}
impl ::windows::core::RuntimeName for IEnumCATEGORYINFO {}
impl IEnumCATEGORYINFO_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: isize>() -> IEnumCATEGORYINFO_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut CATEGORYINFO, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumCATEGORYINFO_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumCATEGORYINFO as ::windows::core::Interface>::IID
    }
}
pub trait IEnumConnectionPoints_Impl: Sized {
    fn Next(&self, cconnections: u32, ppcp: *mut ::core::option::Option<IConnectionPoint>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumConnectionPoints>;
}
impl ::windows::core::RuntimeName for IEnumConnectionPoints {}
impl IEnumConnectionPoints_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: isize>() -> IEnumConnectionPoints_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32, ppcp: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&cconnections), ::core::mem::transmute_copy(&ppcp), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&cconnections)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnectionPoints_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumConnectionPoints as ::windows::core::Interface>::IID
    }
}
pub trait IEnumConnections_Impl: Sized {
    fn Next(&self, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, cconnections: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumConnections>;
}
impl ::windows::core::RuntimeName for IEnumConnections {}
impl IEnumConnections_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: isize>() -> IEnumConnections_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32, rgcd: *mut CONNECTDATA, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&cconnections), ::core::mem::transmute_copy(&rgcd), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cconnections: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&cconnections)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumConnections as ::windows::core::Interface>::IID
    }
}
pub trait IEnumFORMATETC_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumFORMATETC>;
}
impl ::windows::core::RuntimeName for IEnumFORMATETC {}
impl IEnumFORMATETC_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: isize>() -> IEnumFORMATETC_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut FORMATETC, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumFORMATETC_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumFORMATETC as ::windows::core::Interface>::IID
    }
}
pub trait IEnumGUID_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumGUID>;
}
impl ::windows::core::RuntimeName for IEnumGUID {}
impl IEnumGUID_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: isize>() -> IEnumGUID_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::GUID, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumGUID_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumGUID as ::windows::core::Interface>::IID
    }
}
pub trait IEnumMoniker_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IMoniker>, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumMoniker>;
}
impl ::windows::core::RuntimeName for IEnumMoniker {}
impl IEnumMoniker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: isize>() -> IEnumMoniker_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumMoniker as ::windows::core::Interface>::IID
    }
}
pub trait IEnumSTATDATA_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumSTATDATA>;
}
impl ::windows::core::RuntimeName for IEnumSTATDATA {}
impl IEnumSTATDATA_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: isize>() -> IEnumSTATDATA_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut STATDATA, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumSTATDATA_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumSTATDATA as ::windows::core::Interface>::IID
    }
}
pub trait IEnumString_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::windows::core::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT;
    fn Skip(&self, celt: u32) -> ::windows::core::HRESULT;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumString>;
}
impl ::windows::core::RuntimeName for IEnumString {}
impl IEnumString_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: isize>() -> IEnumString_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::PWSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched))
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt))
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumString as ::windows::core::Interface>::IID
    }
}
pub trait IEnumUnknown_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<::windows::core::IUnknown>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumUnknown>;
}
impl ::windows::core::RuntimeName for IEnumUnknown {}
impl IEnumUnknown_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: isize>() -> IEnumUnknown_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumUnknown as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IErrorInfo_Impl: Sized {
    fn GetGUID(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetSource(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDescription(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetHelpFile(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetHelpContext(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IErrorInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: isize>() -> IErrorInfo_Vtbl {
        unsafe extern "system" fn GetGUID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGUID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSource() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHelpFile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrhelpfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhelpcontext: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHelpContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhelpcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetGUID: GetGUID::<Identity, Impl, OFFSET>,
            GetSource: GetSource::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            GetHelpFile: GetHelpFile::<Identity, Impl, OFFSET>,
            GetHelpContext: GetHelpContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IErrorLog_Impl: Sized {
    fn AddError(&self, pszpropname: &::windows::core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IErrorLog {}
#[cfg(feature = "Win32_Foundation")]
impl IErrorLog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorLog_Impl, const OFFSET: isize>() -> IErrorLog_Vtbl {
        unsafe extern "system" fn AddError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IErrorLog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, pexcepinfo: *const EXCEPINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddError(::core::mem::transmute(&pszpropname), ::core::mem::transmute_copy(&pexcepinfo)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AddError: AddError::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IErrorLog as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IExternalConnection_Impl: Sized {
    fn AddConnection(&self, extconn: u32, reserved: u32) -> u32;
    fn ReleaseConnection(&self, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IExternalConnection {}
#[cfg(feature = "Win32_Foundation")]
impl IExternalConnection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExternalConnection_Impl, const OFFSET: isize>() -> IExternalConnection_Vtbl {
        unsafe extern "system" fn AddConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExternalConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddConnection(::core::mem::transmute_copy(&extconn), ::core::mem::transmute_copy(&reserved))
        }
        unsafe extern "system" fn ReleaseConnection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IExternalConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, extconn: u32, reserved: u32, flastreleasecloses: super::super::Foundation::BOOL) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseConnection(::core::mem::transmute_copy(&extconn), ::core::mem::transmute_copy(&reserved), ::core::mem::transmute_copy(&flastreleasecloses))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddConnection: AddConnection::<Identity, Impl, OFFSET>,
            ReleaseConnection: ReleaseConnection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IExternalConnection as ::windows::core::Interface>::IID
    }
}
pub trait IFastRundown_Impl: Sized {}
impl ::windows::core::RuntimeName for IFastRundown {}
impl IFastRundown_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFastRundown_Impl, const OFFSET: isize>() -> IFastRundown_Vtbl {
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFastRundown as ::windows::core::Interface>::IID
    }
}
pub trait IForegroundTransfer_Impl: Sized {
    fn AllowForegroundTransfer(&self, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IForegroundTransfer {}
impl IForegroundTransfer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IForegroundTransfer_Impl, const OFFSET: isize>() -> IForegroundTransfer_Vtbl {
        unsafe extern "system" fn AllowForegroundTransfer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IForegroundTransfer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllowForegroundTransfer(::core::mem::transmute_copy(&lpvreserved)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AllowForegroundTransfer: AllowForegroundTransfer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IForegroundTransfer as ::windows::core::Interface>::IID
    }
}
pub trait IGlobalInterfaceTable_Impl: Sized {
    fn RegisterInterfaceInGlobal(&self, punk: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
    fn RevokeInterfaceFromGlobal(&self, dwcookie: u32) -> ::windows::core::Result<()>;
    fn GetInterfaceFromGlobal(&self, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IGlobalInterfaceTable {}
impl IGlobalInterfaceTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGlobalInterfaceTable_Impl, const OFFSET: isize>() -> IGlobalInterfaceTable_Vtbl {
        unsafe extern "system" fn RegisterInterfaceInGlobal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, pdwcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterInterfaceInGlobal(::core::mem::transmute(&punk), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RevokeInterfaceFromGlobal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevokeInterfaceFromGlobal(::core::mem::transmute_copy(&dwcookie)).into()
        }
        unsafe extern "system" fn GetInterfaceFromGlobal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGlobalInterfaceTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcookie: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInterfaceFromGlobal(::core::mem::transmute_copy(&dwcookie), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterInterfaceInGlobal: RegisterInterfaceInGlobal::<Identity, Impl, OFFSET>,
            RevokeInterfaceFromGlobal: RevokeInterfaceFromGlobal::<Identity, Impl, OFFSET>,
            GetInterfaceFromGlobal: GetInterfaceFromGlobal::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalInterfaceTable as ::windows::core::Interface>::IID
    }
}
pub trait IGlobalOptions_Impl: Sized {
    fn Set(&self, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()>;
    fn Query(&self, dwproperty: GLOBALOPT_PROPERTIES) -> ::windows::core::Result<usize>;
}
impl ::windows::core::RuntimeName for IGlobalOptions {}
impl IGlobalOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGlobalOptions_Impl, const OFFSET: isize>() -> IGlobalOptions_Vtbl {
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGlobalOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set(::core::mem::transmute_copy(&dwproperty), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGlobalOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwproperty: GLOBALOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Query(::core::mem::transmute_copy(&dwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Set: Set::<Identity, Impl, OFFSET>, Query: Query::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGlobalOptions as ::windows::core::Interface>::IID
    }
}
pub trait IInitializeSpy_Impl: Sized {
    fn PreInitialize(&self, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()>;
    fn PostInitialize(&self, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()>;
    fn PreUninitialize(&self, dwcurthreadaptrefs: u32) -> ::windows::core::Result<()>;
    fn PostUninitialize(&self, dwnewthreadaptrefs: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInitializeSpy {}
impl IInitializeSpy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: isize>() -> IInitializeSpy_Vtbl {
        unsafe extern "system" fn PreInitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcoinit: u32, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreInitialize(::core::mem::transmute_copy(&dwcoinit), ::core::mem::transmute_copy(&dwcurthreadaptrefs)).into()
        }
        unsafe extern "system" fn PostInitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrcoinit: ::windows::core::HRESULT, dwcoinit: u32, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostInitialize(::core::mem::transmute_copy(&hrcoinit), ::core::mem::transmute_copy(&dwcoinit), ::core::mem::transmute_copy(&dwnewthreadaptrefs)).into()
        }
        unsafe extern "system" fn PreUninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcurthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreUninitialize(::core::mem::transmute_copy(&dwcurthreadaptrefs)).into()
        }
        unsafe extern "system" fn PostUninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInitializeSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnewthreadaptrefs: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostUninitialize(::core::mem::transmute_copy(&dwnewthreadaptrefs)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PreInitialize: PreInitialize::<Identity, Impl, OFFSET>,
            PostInitialize: PostInitialize::<Identity, Impl, OFFSET>,
            PreUninitialize: PreUninitialize::<Identity, Impl, OFFSET>,
            PostUninitialize: PostUninitialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInitializeSpy as ::windows::core::Interface>::IID
    }
}
pub trait IInternalUnknown_Impl: Sized {
    fn QueryInternalInterface(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IInternalUnknown {}
impl IInternalUnknown_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternalUnknown_Impl, const OFFSET: isize>() -> IInternalUnknown_Vtbl {
        unsafe extern "system" fn QueryInternalInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IInternalUnknown_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryInternalInterface(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), QueryInternalInterface: QueryInternalInterface::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IInternalUnknown as ::windows::core::Interface>::IID
    }
}
pub trait IMachineGlobalObjectTable_Impl: Sized {
    fn RegisterObject(&self, clsid: *const ::windows::core::GUID, identifier: &::windows::core::PCWSTR, object: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<*mut MachineGlobalObjectTableRegistrationToken__>;
    fn GetObject(&self, clsid: *const ::windows::core::GUID, identifier: &::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn RevokeObject(&self, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMachineGlobalObjectTable {}
impl IMachineGlobalObjectTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMachineGlobalObjectTable_Impl, const OFFSET: isize>() -> IMachineGlobalObjectTable_Vtbl {
        unsafe extern "system" fn RegisterObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: ::windows::core::PCWSTR, object: *mut ::core::ffi::c_void, token: *mut *mut MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterObject(::core::mem::transmute_copy(&clsid), ::core::mem::transmute(&identifier), ::core::mem::transmute(&object)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(token, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID, identifier: ::windows::core::PCWSTR, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObject(::core::mem::transmute_copy(&clsid), ::core::mem::transmute(&identifier), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn RevokeObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMachineGlobalObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, token: *const MachineGlobalObjectTableRegistrationToken__) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevokeObject(::core::mem::transmute_copy(&token)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterObject: RegisterObject::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            RevokeObject: RevokeObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMachineGlobalObjectTable as ::windows::core::Interface>::IID
    }
}
pub trait IMalloc_Impl: Sized {
    fn Alloc(&self, cb: usize) -> *mut ::core::ffi::c_void;
    fn Realloc(&self, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void;
    fn Free(&self, pv: *const ::core::ffi::c_void);
    fn GetSize(&self, pv: *const ::core::ffi::c_void) -> usize;
    fn DidAlloc(&self, pv: *const ::core::ffi::c_void) -> i32;
    fn HeapMinimize(&self);
}
impl ::windows::core::RuntimeName for IMalloc {}
impl IMalloc_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: isize>() -> IMalloc_Vtbl {
        unsafe extern "system" fn Alloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Alloc(::core::mem::transmute_copy(&cb))
        }
        unsafe extern "system" fn Realloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: usize) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Realloc(::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb))
        }
        unsafe extern "system" fn Free<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Free(::core::mem::transmute_copy(&pv))
        }
        unsafe extern "system" fn GetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSize(::core::mem::transmute_copy(&pv))
        }
        unsafe extern "system" fn DidAlloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DidAlloc(::core::mem::transmute_copy(&pv))
        }
        unsafe extern "system" fn HeapMinimize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMalloc_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HeapMinimize()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Alloc: Alloc::<Identity, Impl, OFFSET>,
            Realloc: Realloc::<Identity, Impl, OFFSET>,
            Free: Free::<Identity, Impl, OFFSET>,
            GetSize: GetSize::<Identity, Impl, OFFSET>,
            DidAlloc: DidAlloc::<Identity, Impl, OFFSET>,
            HeapMinimize: HeapMinimize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMalloc as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMallocSpy_Impl: Sized {
    fn PreAlloc(&self, cbrequest: usize) -> usize;
    fn PostAlloc(&self, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn PreFree(&self, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void;
    fn PostFree(&self, fspyed: super::super::Foundation::BOOL);
    fn PreRealloc(&self, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize;
    fn PostRealloc(&self, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void;
    fn PreGetSize(&self, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void;
    fn PostGetSize(&self, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize;
    fn PreDidAlloc(&self, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void;
    fn PostDidAlloc(&self, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32;
    fn PreHeapMinimize(&self);
    fn PostHeapMinimize(&self);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMallocSpy {}
#[cfg(feature = "Win32_Foundation")]
impl IMallocSpy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>() -> IMallocSpy_Vtbl {
        unsafe extern "system" fn PreAlloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbrequest: usize) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreAlloc(::core::mem::transmute_copy(&cbrequest))
        }
        unsafe extern "system" fn PostAlloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostAlloc(::core::mem::transmute_copy(&pactual))
        }
        unsafe extern "system" fn PreFree<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreFree(::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PostFree<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostFree(::core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PreRealloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, cbrequest: usize, ppnewrequest: *mut *mut ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreRealloc(::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&cbrequest), ::core::mem::transmute_copy(&ppnewrequest), ::core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PostRealloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactual: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostRealloc(::core::mem::transmute_copy(&pactual), ::core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PreGetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreGetSize(::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PostGetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cbactual: usize, fspyed: super::super::Foundation::BOOL) -> usize {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostGetSize(::core::mem::transmute_copy(&cbactual), ::core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PreDidAlloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL) -> *mut ::core::ffi::c_void {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreDidAlloc(::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&fspyed))
        }
        unsafe extern "system" fn PostDidAlloc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequest: *const ::core::ffi::c_void, fspyed: super::super::Foundation::BOOL, factual: i32) -> i32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostDidAlloc(::core::mem::transmute_copy(&prequest), ::core::mem::transmute_copy(&fspyed), ::core::mem::transmute_copy(&factual))
        }
        unsafe extern "system" fn PreHeapMinimize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PreHeapMinimize()
        }
        unsafe extern "system" fn PostHeapMinimize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMallocSpy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostHeapMinimize()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PreAlloc: PreAlloc::<Identity, Impl, OFFSET>,
            PostAlloc: PostAlloc::<Identity, Impl, OFFSET>,
            PreFree: PreFree::<Identity, Impl, OFFSET>,
            PostFree: PostFree::<Identity, Impl, OFFSET>,
            PreRealloc: PreRealloc::<Identity, Impl, OFFSET>,
            PostRealloc: PostRealloc::<Identity, Impl, OFFSET>,
            PreGetSize: PreGetSize::<Identity, Impl, OFFSET>,
            PostGetSize: PostGetSize::<Identity, Impl, OFFSET>,
            PreDidAlloc: PreDidAlloc::<Identity, Impl, OFFSET>,
            PostDidAlloc: PostDidAlloc::<Identity, Impl, OFFSET>,
            PreHeapMinimize: PreHeapMinimize::<Identity, Impl, OFFSET>,
            PostHeapMinimize: PostHeapMinimize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMallocSpy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMoniker_Impl: Sized + IPersist_Impl + IPersistStream_Impl {
    fn BindToObject(&self, pbc: &::core::option::Option<IBindCtx>, pmktoleft: &::core::option::Option<IMoniker>, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn BindToStorage(&self, pbc: &::core::option::Option<IBindCtx>, pmktoleft: &::core::option::Option<IMoniker>, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Reduce(&self, pbc: &::core::option::Option<IBindCtx>, dwreducehowfar: u32, ppmktoleft: *mut ::core::option::Option<IMoniker>, ppmkreduced: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()>;
    fn ComposeWith(&self, pmkright: &::core::option::Option<IMoniker>, fonlyifnotgeneric: super::super::Foundation::BOOL) -> ::windows::core::Result<IMoniker>;
    fn Enum(&self, fforward: super::super::Foundation::BOOL) -> ::windows::core::Result<IEnumMoniker>;
    fn IsEqual(&self, pmkothermoniker: &::core::option::Option<IMoniker>) -> ::windows::core::Result<()>;
    fn Hash(&self) -> ::windows::core::Result<u32>;
    fn IsRunning(&self, pbc: &::core::option::Option<IBindCtx>, pmktoleft: &::core::option::Option<IMoniker>, pmknewlyrunning: &::core::option::Option<IMoniker>) -> ::windows::core::Result<()>;
    fn GetTimeOfLastChange(&self, pbc: &::core::option::Option<IBindCtx>, pmktoleft: &::core::option::Option<IMoniker>) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn Inverse(&self) -> ::windows::core::Result<IMoniker>;
    fn CommonPrefixWith(&self, pmkother: &::core::option::Option<IMoniker>) -> ::windows::core::Result<IMoniker>;
    fn RelativePathTo(&self, pmkother: &::core::option::Option<IMoniker>) -> ::windows::core::Result<IMoniker>;
    fn GetDisplayName(&self, pbc: &::core::option::Option<IBindCtx>, pmktoleft: &::core::option::Option<IMoniker>) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn ParseDisplayName(&self, pbc: &::core::option::Option<IBindCtx>, pmktoleft: &::core::option::Option<IMoniker>, pszdisplayname: &::windows::core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut ::core::option::Option<IMoniker>) -> ::windows::core::Result<()>;
    fn IsSystemMoniker(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMoniker {}
#[cfg(feature = "Win32_Foundation")]
impl IMoniker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>() -> IMoniker_Vtbl {
        unsafe extern "system" fn BindToObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, riidresult: *const ::windows::core::GUID, ppvresult: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToObject(::core::mem::transmute(&pbc), ::core::mem::transmute(&pmktoleft), ::core::mem::transmute_copy(&riidresult), ::core::mem::transmute_copy(&ppvresult)).into()
        }
        unsafe extern "system" fn BindToStorage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToStorage(::core::mem::transmute(&pbc), ::core::mem::transmute(&pmktoleft), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn Reduce<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, dwreducehowfar: u32, ppmktoleft: *mut *mut ::core::ffi::c_void, ppmkreduced: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reduce(::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&dwreducehowfar), ::core::mem::transmute_copy(&ppmktoleft), ::core::mem::transmute_copy(&ppmkreduced)).into()
        }
        unsafe extern "system" fn ComposeWith<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkright: *mut ::core::ffi::c_void, fonlyifnotgeneric: super::super::Foundation::BOOL, ppmkcomposite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ComposeWith(::core::mem::transmute(&pmkright), ::core::mem::transmute_copy(&fonlyifnotgeneric)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmkcomposite, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fforward: super::super::Foundation::BOOL, ppenummoniker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enum(::core::mem::transmute_copy(&fforward)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenummoniker, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkothermoniker: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsEqual(::core::mem::transmute(&pmkothermoniker)).into()
        }
        unsafe extern "system" fn Hash<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhash: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Hash() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhash, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRunning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, pmknewlyrunning: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsRunning(::core::mem::transmute(&pbc), ::core::mem::transmute(&pmktoleft), ::core::mem::transmute(&pmknewlyrunning)).into()
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTimeOfLastChange(::core::mem::transmute(&pbc), ::core::mem::transmute(&pmktoleft)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiletime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Inverse<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Inverse() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommonPrefixWith<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkother: *mut ::core::ffi::c_void, ppmkprefix: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommonPrefixWith(::core::mem::transmute(&pmkother)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmkprefix, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RelativePathTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkother: *mut ::core::ffi::c_void, ppmkrelpath: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RelativePathTo(::core::mem::transmute(&pmkother)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmkrelpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, ppszdisplayname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayName(::core::mem::transmute(&pbc), ::core::mem::transmute(&pmktoleft)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszdisplayname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParseDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void, pmktoleft: *mut ::core::ffi::c_void, pszdisplayname: ::windows::core::PCWSTR, pcheaten: *mut u32, ppmkout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ParseDisplayName(::core::mem::transmute(&pbc), ::core::mem::transmute(&pmktoleft), ::core::mem::transmute(&pszdisplayname), ::core::mem::transmute_copy(&pcheaten), ::core::mem::transmute_copy(&ppmkout)).into()
        }
        unsafe extern "system" fn IsSystemMoniker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMoniker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmksys: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSystemMoniker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmksys, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPersistStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            BindToObject: BindToObject::<Identity, Impl, OFFSET>,
            BindToStorage: BindToStorage::<Identity, Impl, OFFSET>,
            Reduce: Reduce::<Identity, Impl, OFFSET>,
            ComposeWith: ComposeWith::<Identity, Impl, OFFSET>,
            Enum: Enum::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
            Hash: Hash::<Identity, Impl, OFFSET>,
            IsRunning: IsRunning::<Identity, Impl, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, Impl, OFFSET>,
            Inverse: Inverse::<Identity, Impl, OFFSET>,
            CommonPrefixWith: CommonPrefixWith::<Identity, Impl, OFFSET>,
            RelativePathTo: RelativePathTo::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            ParseDisplayName: ParseDisplayName::<Identity, Impl, OFFSET>,
            IsSystemMoniker: IsSystemMoniker::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMoniker as ::windows::core::Interface>::IID || iid == &<IPersist as ::windows::core::Interface>::IID || iid == &<IPersistStream as ::windows::core::Interface>::IID
    }
}
pub trait IMultiQI_Impl: Sized {
    fn QueryMultipleInterfaces(&self, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMultiQI {}
impl IMultiQI_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiQI_Impl, const OFFSET: isize>() -> IMultiQI_Vtbl {
        unsafe extern "system" fn QueryMultipleInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiQI_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmqis: u32, pmqis: *mut MULTI_QI) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryMultipleInterfaces(::core::mem::transmute_copy(&cmqis), ::core::mem::transmute_copy(&pmqis)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), QueryMultipleInterfaces: QueryMultipleInterfaces::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiQI as ::windows::core::Interface>::IID
    }
}
pub trait INoMarshal_Impl: Sized {}
impl ::windows::core::RuntimeName for INoMarshal {}
impl INoMarshal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INoMarshal_Impl, const OFFSET: isize>() -> INoMarshal_Vtbl {
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INoMarshal as ::windows::core::Interface>::IID
    }
}
pub trait IOplockStorage_Impl: Sized {
    fn CreateStorageEx(&self, pwcsname: &::windows::core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn OpenStorageEx(&self, pwcsname: &::windows::core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IOplockStorage {}
impl IOplockStorage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOplockStorage_Impl, const OFFSET: isize>() -> IOplockStorage_Vtbl {
        unsafe extern "system" fn CreateStorageEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOplockStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateStorageEx(::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&stgfmt), ::core::mem::transmute_copy(&grfattrs), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstgopen)).into()
        }
        unsafe extern "system" fn OpenStorageEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IOplockStorage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwcsname: ::windows::core::PCWSTR, grfmode: u32, stgfmt: u32, grfattrs: u32, riid: *const ::windows::core::GUID, ppstgopen: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenStorageEx(::core::mem::transmute(&pwcsname), ::core::mem::transmute_copy(&grfmode), ::core::mem::transmute_copy(&stgfmt), ::core::mem::transmute_copy(&grfattrs), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppstgopen)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateStorageEx: CreateStorageEx::<Identity, Impl, OFFSET>,
            OpenStorageEx: OpenStorageEx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IOplockStorage as ::windows::core::Interface>::IID
    }
}
pub trait IPSFactoryBuffer_Impl: Sized {
    fn CreateProxy(&self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, ppproxy: *mut ::core::option::Option<IRpcProxyBuffer>, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateStub(&self, riid: *const ::windows::core::GUID, punkserver: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<IRpcStubBuffer>;
}
impl ::windows::core::RuntimeName for IPSFactoryBuffer {}
impl IPSFactoryBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPSFactoryBuffer_Impl, const OFFSET: isize>() -> IPSFactoryBuffer_Vtbl {
        unsafe extern "system" fn CreateProxy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPSFactoryBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppproxy: *mut *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateProxy(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppproxy), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateStub<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPSFactoryBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, punkserver: *mut ::core::ffi::c_void, ppstub: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateStub(::core::mem::transmute_copy(&riid), ::core::mem::transmute(&punkserver)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstub, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateProxy: CreateProxy::<Identity, Impl, OFFSET>,
            CreateStub: CreateStub::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPSFactoryBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IPersist_Impl: Sized {
    fn GetClassID(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IPersist {}
impl IPersist_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersist_Impl, const OFFSET: isize>() -> IPersist_Vtbl {
        unsafe extern "system" fn GetClassID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersist_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclassid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClassID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclassid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetClassID: GetClassID::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersist as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistFile_Impl: Sized + IPersist_Impl {
    fn IsDirty(&self) -> ::windows::core::HRESULT;
    fn Load(&self, pszfilename: &::windows::core::PCWSTR, dwmode: u32) -> ::windows::core::Result<()>;
    fn Save(&self, pszfilename: &::windows::core::PCWSTR, fremember: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SaveCompleted(&self, pszfilename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetCurFile(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPersistFile {}
#[cfg(feature = "Win32_Foundation")]
impl IPersistFile_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: isize>() -> IPersistFile_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDirty()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, dwmode: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&dwmode)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR, fremember: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute(&pszfilename), ::core::mem::transmute_copy(&fremember)).into()
        }
        unsafe extern "system" fn SaveCompleted<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfilename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveCompleted(::core::mem::transmute(&pszfilename)).into()
        }
        unsafe extern "system" fn GetCurFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistFile_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszfilename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCurFile() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszfilename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            SaveCompleted: SaveCompleted::<Identity, Impl, OFFSET>,
            GetCurFile: GetCurFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistFile as ::windows::core::Interface>::IID || iid == &<IPersist as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistMemory_Impl: Sized + IPersist_Impl {
    fn IsDirty(&self) -> ::windows::core::HRESULT;
    fn Load(&self, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::Result<()>;
    fn Save(&self, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows::core::Result<()>;
    fn GetSizeMax(&self) -> ::windows::core::Result<u32>;
    fn InitNew(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPersistMemory {}
#[cfg(feature = "Win32_Foundation")]
impl IPersistMemory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: isize>() -> IPersistMemory_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDirty()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *const ::core::ffi::c_void, cbsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute_copy(&pmem), ::core::mem::transmute_copy(&cbsize)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmem: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL, cbsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute_copy(&pmem), ::core::mem::transmute_copy(&fcleardirty), ::core::mem::transmute_copy(&cbsize)).into()
        }
        unsafe extern "system" fn GetSizeMax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSizeMax() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitNew<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistMemory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitNew().into()
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, Impl, OFFSET>,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistMemory as ::windows::core::Interface>::IID || iid == &<IPersist as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistStream_Impl: Sized + IPersist_Impl {
    fn IsDirty(&self) -> ::windows::core::HRESULT;
    fn Load(&self, pstm: &::core::option::Option<IStream>) -> ::windows::core::Result<()>;
    fn Save(&self, pstm: &::core::option::Option<IStream>, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSizeMax(&self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPersistStream {}
#[cfg(feature = "Win32_Foundation")]
impl IPersistStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: isize>() -> IPersistStream_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDirty()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute(&pstm)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute(&pstm), ::core::mem::transmute_copy(&fcleardirty)).into()
        }
        unsafe extern "system" fn GetSizeMax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSizeMax() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistStream as ::windows::core::Interface>::IID || iid == &<IPersist as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPersistStreamInit_Impl: Sized + IPersist_Impl {
    fn IsDirty(&self) -> ::windows::core::HRESULT;
    fn Load(&self, pstm: &::core::option::Option<IStream>) -> ::windows::core::Result<()>;
    fn Save(&self, pstm: &::core::option::Option<IStream>, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSizeMax(&self) -> ::windows::core::Result<u64>;
    fn InitNew(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IPersistStreamInit {}
#[cfg(feature = "Win32_Foundation")]
impl IPersistStreamInit_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: isize>() -> IPersistStreamInit_Vtbl {
        unsafe extern "system" fn IsDirty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsDirty()
        }
        unsafe extern "system" fn Load<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Load(::core::mem::transmute(&pstm)).into()
        }
        unsafe extern "system" fn Save<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, fcleardirty: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Save(::core::mem::transmute(&pstm), ::core::mem::transmute_copy(&fcleardirty)).into()
        }
        unsafe extern "system" fn GetSizeMax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcbsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSizeMax() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcbsize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitNew<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPersistStreamInit_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitNew().into()
        }
        Self {
            base__: IPersist_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsDirty: IsDirty::<Identity, Impl, OFFSET>,
            Load: Load::<Identity, Impl, OFFSET>,
            Save: Save::<Identity, Impl, OFFSET>,
            GetSizeMax: GetSizeMax::<Identity, Impl, OFFSET>,
            InitNew: InitNew::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPersistStreamInit as ::windows::core::Interface>::IID || iid == &<IPersist as ::windows::core::Interface>::IID
    }
}
pub trait IPipeByte_Impl: Sized {
    fn Pull(&self, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::Result<()>;
    fn Push(&self, buf: *const u8, csent: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPipeByte {}
impl IPipeByte_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPipeByte_Impl, const OFFSET: isize>() -> IPipeByte_Vtbl {
        unsafe extern "system" fn Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPipeByte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut u8, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pull(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&crequest), ::core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPipeByte_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const u8, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Push(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, Impl, OFFSET>, Push: Push::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPipeByte as ::windows::core::Interface>::IID
    }
}
pub trait IPipeDouble_Impl: Sized {
    fn Pull(&self, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::Result<()>;
    fn Push(&self, buf: *const f64, csent: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPipeDouble {}
impl IPipeDouble_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPipeDouble_Impl, const OFFSET: isize>() -> IPipeDouble_Vtbl {
        unsafe extern "system" fn Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPipeDouble_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut f64, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pull(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&crequest), ::core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPipeDouble_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const f64, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Push(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, Impl, OFFSET>, Push: Push::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPipeDouble as ::windows::core::Interface>::IID
    }
}
pub trait IPipeLong_Impl: Sized {
    fn Pull(&self, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::Result<()>;
    fn Push(&self, buf: *const i32, csent: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPipeLong {}
impl IPipeLong_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPipeLong_Impl, const OFFSET: isize>() -> IPipeLong_Vtbl {
        unsafe extern "system" fn Pull<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPipeLong_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *mut i32, crequest: u32, pcreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Pull(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&crequest), ::core::mem::transmute_copy(&pcreturned)).into()
        }
        unsafe extern "system" fn Push<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPipeLong_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, buf: *const i32, csent: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Push(::core::mem::transmute_copy(&buf), ::core::mem::transmute_copy(&csent)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Pull: Pull::<Identity, Impl, OFFSET>, Push: Push::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPipeLong as ::windows::core::Interface>::IID
    }
}
pub trait IProcessInitControl_Impl: Sized {
    fn ResetInitializerTimeout(&self, dwsecondsremaining: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IProcessInitControl {}
impl IProcessInitControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitControl_Impl, const OFFSET: isize>() -> IProcessInitControl_Vtbl {
        unsafe extern "system" fn ResetInitializerTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsecondsremaining: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetInitializerTimeout(::core::mem::transmute_copy(&dwsecondsremaining)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ResetInitializerTimeout: ResetInitializerTimeout::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessInitControl as ::windows::core::Interface>::IID
    }
}
pub trait IProcessLock_Impl: Sized {
    fn AddRefOnProcess(&self) -> u32;
    fn ReleaseRefOnProcess(&self) -> u32;
}
impl ::windows::core::RuntimeName for IProcessLock {}
impl IProcessLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProcessLock_Impl, const OFFSET: isize>() -> IProcessLock_Vtbl {
        unsafe extern "system" fn AddRefOnProcess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProcessLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRefOnProcess()
        }
        unsafe extern "system" fn ReleaseRefOnProcess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProcessLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseRefOnProcess()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddRefOnProcess: AddRefOnProcess::<Identity, Impl, OFFSET>,
            ReleaseRefOnProcess: ReleaseRefOnProcess::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessLock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProgressNotify_Impl: Sized {
    fn OnProgress(&self, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IProgressNotify {}
#[cfg(feature = "Win32_Foundation")]
impl IProgressNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProgressNotify_Impl, const OFFSET: isize>() -> IProgressNotify_Vtbl {
        unsafe extern "system" fn OnProgress<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProgressNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwprogresscurrent: u32, dwprogressmaximum: u32, faccurate: super::super::Foundation::BOOL, fowner: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnProgress(::core::mem::transmute_copy(&dwprogresscurrent), ::core::mem::transmute_copy(&dwprogressmaximum), ::core::mem::transmute_copy(&faccurate), ::core::mem::transmute_copy(&fowner)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnProgress: OnProgress::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressNotify as ::windows::core::Interface>::IID
    }
}
pub trait IROTData_Impl: Sized {
    fn GetComparisonData(&self, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IROTData {}
impl IROTData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IROTData_Impl, const OFFSET: isize>() -> IROTData_Vtbl {
        unsafe extern "system" fn GetComparisonData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IROTData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdata: *mut u8, cbmax: u32, pcbdata: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetComparisonData(::core::mem::transmute_copy(&pbdata), ::core::mem::transmute_copy(&cbmax), ::core::mem::transmute_copy(&pcbdata)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetComparisonData: GetComparisonData::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IROTData as ::windows::core::Interface>::IID
    }
}
pub trait IReleaseMarshalBuffers_Impl: Sized {
    fn ReleaseMarshalBuffer(&self, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IReleaseMarshalBuffers {}
impl IReleaseMarshalBuffers_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReleaseMarshalBuffers_Impl, const OFFSET: isize>() -> IReleaseMarshalBuffers_Vtbl {
        unsafe extern "system" fn ReleaseMarshalBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IReleaseMarshalBuffers_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, dwflags: u32, pchnl: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseMarshalBuffer(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pchnl)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ReleaseMarshalBuffer: ReleaseMarshalBuffer::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IReleaseMarshalBuffers as ::windows::core::Interface>::IID
    }
}
pub trait IRpcChannelBuffer_Impl: Sized {
    fn GetBuffer(&self, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SendReceive(&self, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::Result<()>;
    fn FreeBuffer(&self, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()>;
    fn GetDestCtx(&self, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsConnected(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRpcChannelBuffer {}
impl IRpcChannelBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: isize>() -> IRpcChannelBuffer_Vtbl {
        unsafe extern "system" fn GetBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBuffer(::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&riid)).into()
        }
        unsafe extern "system" fn SendReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE, pstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendReceive(::core::mem::transmute_copy(&pmessage), ::core::mem::transmute_copy(&pstatus)).into()
        }
        unsafe extern "system" fn FreeBuffer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessage: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeBuffer(::core::mem::transmute_copy(&pmessage)).into()
        }
        unsafe extern "system" fn GetDestCtx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDestCtx(::core::mem::transmute_copy(&pdwdestcontext), ::core::mem::transmute_copy(&ppvdestcontext)).into()
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsConnected().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetBuffer: GetBuffer::<Identity, Impl, OFFSET>,
            SendReceive: SendReceive::<Identity, Impl, OFFSET>,
            FreeBuffer: FreeBuffer::<Identity, Impl, OFFSET>,
            GetDestCtx: GetDestCtx::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcChannelBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IRpcChannelBuffer2_Impl: Sized + IRpcChannelBuffer_Impl {
    fn GetProtocolVersion(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IRpcChannelBuffer2 {}
impl IRpcChannelBuffer2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer2_Impl, const OFFSET: isize>() -> IRpcChannelBuffer2_Vtbl {
        unsafe extern "system" fn GetProtocolVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProtocolVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IRpcChannelBuffer_Vtbl::new::<Identity, Impl, OFFSET>(), GetProtocolVersion: GetProtocolVersion::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcChannelBuffer2 as ::windows::core::Interface>::IID || iid == &<IRpcChannelBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IRpcChannelBuffer3_Impl: Sized + IRpcChannelBuffer_Impl + IRpcChannelBuffer2_Impl {
    fn Send(&self, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::Result<()>;
    fn Receive(&self, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::Result<()>;
    fn Cancel(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()>;
    fn GetCallContext(&self, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetDestCtxEx(&self, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetState(&self, pmsg: *const RPCOLEMESSAGE) -> ::windows::core::Result<u32>;
    fn RegisterAsync(&self, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: &::core::option::Option<IAsyncManager>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRpcChannelBuffer3 {}
impl IRpcChannelBuffer3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: isize>() -> IRpcChannelBuffer3_Vtbl {
        unsafe extern "system" fn Send<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Send(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&pulstatus)).into()
        }
        unsafe extern "system" fn Receive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, ulsize: u32, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Receive(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&ulsize), ::core::mem::transmute_copy(&pulstatus)).into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel(::core::mem::transmute_copy(&pmsg)).into()
        }
        unsafe extern "system" fn GetCallContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, riid: *const ::windows::core::GUID, pinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCallContext(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pinterface)).into()
        }
        unsafe extern "system" fn GetDestCtxEx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pdwdestcontext: *mut u32, ppvdestcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDestCtxEx(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute_copy(&pdwdestcontext), ::core::mem::transmute_copy(&ppvdestcontext)).into()
        }
        unsafe extern "system" fn GetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *const RPCOLEMESSAGE, pstate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetState(::core::mem::transmute_copy(&pmsg)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterAsync<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcChannelBuffer3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE, pasyncmgr: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterAsync(::core::mem::transmute_copy(&pmsg), ::core::mem::transmute(&pasyncmgr)).into()
        }
        Self {
            base__: IRpcChannelBuffer2_Vtbl::new::<Identity, Impl, OFFSET>(),
            Send: Send::<Identity, Impl, OFFSET>,
            Receive: Receive::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            GetCallContext: GetCallContext::<Identity, Impl, OFFSET>,
            GetDestCtxEx: GetDestCtxEx::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            RegisterAsync: RegisterAsync::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcChannelBuffer3 as ::windows::core::Interface>::IID || iid == &<IRpcChannelBuffer as ::windows::core::Interface>::IID || iid == &<IRpcChannelBuffer2 as ::windows::core::Interface>::IID
    }
}
pub trait IRpcHelper_Impl: Sized {
    fn GetDCOMProtocolVersion(&self) -> ::windows::core::Result<u32>;
    fn GetIIDFromOBJREF(&self, pobjref: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut ::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IRpcHelper {}
impl IRpcHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcHelper_Impl, const OFFSET: isize>() -> IRpcHelper_Vtbl {
        unsafe extern "system" fn GetDCOMProtocolVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomversion: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDCOMProtocolVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcomversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIIDFromOBJREF<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobjref: *const ::core::ffi::c_void, piid: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIIDFromOBJREF(::core::mem::transmute_copy(&pobjref)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(piid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDCOMProtocolVersion: GetDCOMProtocolVersion::<Identity, Impl, OFFSET>,
            GetIIDFromOBJREF: GetIIDFromOBJREF::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcHelper as ::windows::core::Interface>::IID
    }
}
pub trait IRpcOptions_Impl: Sized {
    fn Set(&self, pprx: &::core::option::Option<::windows::core::IUnknown>, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::Result<()>;
    fn Query(&self, pprx: &::core::option::Option<::windows::core::IUnknown>, dwproperty: RPCOPT_PROPERTIES) -> ::windows::core::Result<usize>;
}
impl ::windows::core::RuntimeName for IRpcOptions {}
impl IRpcOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcOptions_Impl, const OFFSET: isize>() -> IRpcOptions_Vtbl {
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, dwvalue: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set(::core::mem::transmute(&pprx), ::core::mem::transmute_copy(&dwproperty), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprx: *mut ::core::ffi::c_void, dwproperty: RPCOPT_PROPERTIES, pdwvalue: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Query(::core::mem::transmute(&pprx), ::core::mem::transmute_copy(&dwproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Set: Set::<Identity, Impl, OFFSET>, Query: Query::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcOptions as ::windows::core::Interface>::IID
    }
}
pub trait IRpcProxyBuffer_Impl: Sized {
    fn Connect(&self, prpcchannelbuffer: &::core::option::Option<IRpcChannelBuffer>) -> ::windows::core::Result<()>;
    fn Disconnect(&self);
}
impl ::windows::core::RuntimeName for IRpcProxyBuffer {}
impl IRpcProxyBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcProxyBuffer_Impl, const OFFSET: isize>() -> IRpcProxyBuffer_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcProxyBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prpcchannelbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect(::core::mem::transmute(&prpcchannelbuffer)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcProxyBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcProxyBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IRpcStubBuffer_Impl: Sized {
    fn Connect(&self, punkserver: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Disconnect(&self);
    fn Invoke(&self, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: &::core::option::Option<IRpcChannelBuffer>) -> ::windows::core::Result<()>;
    fn IsIIDSupported(&self, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer>;
    fn CountRefs(&self) -> u32;
    fn DebugServerQueryInterface(&self, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DebugServerRelease(&self, pv: *const ::core::ffi::c_void);
}
impl ::windows::core::RuntimeName for IRpcStubBuffer {}
impl IRpcStubBuffer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: isize>() -> IRpcStubBuffer_Vtbl {
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkserver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Connect(::core::mem::transmute(&punkserver)).into()
        }
        unsafe extern "system" fn Disconnect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Disconnect()
        }
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, _prpcmsg: *mut RPCOLEMESSAGE, _prpcchannelbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::core::mem::transmute_copy(&_prpcmsg), ::core::mem::transmute(&_prpcchannelbuffer)).into()
        }
        unsafe extern "system" fn IsIIDSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::core::option::Option<IRpcStubBuffer> {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsIIDSupported(::core::mem::transmute_copy(&riid))
        }
        unsafe extern "system" fn CountRefs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CountRefs()
        }
        unsafe extern "system" fn DebugServerQueryInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DebugServerQueryInterface(::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn DebugServerRelease<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcStubBuffer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DebugServerRelease(::core::mem::transmute_copy(&pv))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Connect: Connect::<Identity, Impl, OFFSET>,
            Disconnect: Disconnect::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
            IsIIDSupported: IsIIDSupported::<Identity, Impl, OFFSET>,
            CountRefs: CountRefs::<Identity, Impl, OFFSET>,
            DebugServerQueryInterface: DebugServerQueryInterface::<Identity, Impl, OFFSET>,
            DebugServerRelease: DebugServerRelease::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcStubBuffer as ::windows::core::Interface>::IID
    }
}
pub trait IRpcSyntaxNegotiate_Impl: Sized {
    fn NegotiateSyntax(&self, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IRpcSyntaxNegotiate {}
impl IRpcSyntaxNegotiate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcSyntaxNegotiate_Impl, const OFFSET: isize>() -> IRpcSyntaxNegotiate_Vtbl {
        unsafe extern "system" fn NegotiateSyntax<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRpcSyntaxNegotiate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsg: *mut RPCOLEMESSAGE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NegotiateSyntax(::core::mem::transmute_copy(&pmsg)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), NegotiateSyntax: NegotiateSyntax::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRpcSyntaxNegotiate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRunnableObject_Impl: Sized {
    fn GetRunningClass(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn Run(&self, pbc: &::core::option::Option<IBindCtx>) -> ::windows::core::Result<()>;
    fn IsRunning(&self) -> super::super::Foundation::BOOL;
    fn LockRunning(&self, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetContainedObject(&self, fcontained: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRunnableObject {}
#[cfg(feature = "Win32_Foundation")]
impl IRunnableObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: isize>() -> IRunnableObject_Vtbl {
        unsafe extern "system" fn GetRunningClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRunningClass() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Run<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbc: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Run(::core::mem::transmute(&pbc)).into()
        }
        unsafe extern "system" fn IsRunning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsRunning()
        }
        unsafe extern "system" fn LockRunning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flock: super::super::Foundation::BOOL, flastunlockcloses: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockRunning(::core::mem::transmute_copy(&flock), ::core::mem::transmute_copy(&flastunlockcloses)).into()
        }
        unsafe extern "system" fn SetContainedObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunnableObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fcontained: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContainedObject(::core::mem::transmute_copy(&fcontained)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetRunningClass: GetRunningClass::<Identity, Impl, OFFSET>,
            Run: Run::<Identity, Impl, OFFSET>,
            IsRunning: IsRunning::<Identity, Impl, OFFSET>,
            LockRunning: LockRunning::<Identity, Impl, OFFSET>,
            SetContainedObject: SetContainedObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunnableObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRunningObjectTable_Impl: Sized {
    fn Register(&self, grfflags: ROT_FLAGS, punkobject: &::core::option::Option<::windows::core::IUnknown>, pmkobjectname: &::core::option::Option<IMoniker>) -> ::windows::core::Result<u32>;
    fn Revoke(&self, dwregister: u32) -> ::windows::core::Result<()>;
    fn IsRunning(&self, pmkobjectname: &::core::option::Option<IMoniker>) -> ::windows::core::Result<()>;
    fn GetObject(&self, pmkobjectname: &::core::option::Option<IMoniker>) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn NoteChangeTime(&self, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::Result<()>;
    fn GetTimeOfLastChange(&self, pmkobjectname: &::core::option::Option<IMoniker>) -> ::windows::core::Result<super::super::Foundation::FILETIME>;
    fn EnumRunning(&self) -> ::windows::core::Result<IEnumMoniker>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRunningObjectTable {}
#[cfg(feature = "Win32_Foundation")]
impl IRunningObjectTable_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: isize>() -> IRunningObjectTable_Vtbl {
        unsafe extern "system" fn Register<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfflags: ROT_FLAGS, punkobject: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void, pdwregister: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Register(::core::mem::transmute_copy(&grfflags), ::core::mem::transmute(&punkobject), ::core::mem::transmute(&pmkobjectname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwregister, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Revoke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregister: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Revoke(::core::mem::transmute_copy(&dwregister)).into()
        }
        unsafe extern "system" fn IsRunning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsRunning(::core::mem::transmute(&pmkobjectname)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void, ppunkobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetObject(::core::mem::transmute(&pmkobjectname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NoteChangeTime<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwregister: u32, pfiletime: *const super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NoteChangeTime(::core::mem::transmute_copy(&dwregister), ::core::mem::transmute_copy(&pfiletime)).into()
        }
        unsafe extern "system" fn GetTimeOfLastChange<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmkobjectname: *mut ::core::ffi::c_void, pfiletime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTimeOfLastChange(::core::mem::transmute(&pmkobjectname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfiletime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumRunning<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRunningObjectTable_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenummoniker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumRunning() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenummoniker, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Register: Register::<Identity, Impl, OFFSET>,
            Revoke: Revoke::<Identity, Impl, OFFSET>,
            IsRunning: IsRunning::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            NoteChangeTime: NoteChangeTime::<Identity, Impl, OFFSET>,
            GetTimeOfLastChange: GetTimeOfLastChange::<Identity, Impl, OFFSET>,
            EnumRunning: EnumRunning::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRunningObjectTable as ::windows::core::Interface>::IID
    }
}
pub trait ISequentialStream_Impl: Sized {
    fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT;
    fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT;
}
impl ::windows::core::RuntimeName for ISequentialStream {}
impl ISequentialStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISequentialStream_Impl, const OFFSET: isize>() -> ISequentialStream_Vtbl {
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISequentialStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread))
        }
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISequentialStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Write(::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbwritten))
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, Impl, OFFSET>, Write: Write::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISequentialStream as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IServerSecurity_Impl: Sized {
    fn QueryBlanket(&self, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::Result<()>;
    fn ImpersonateClient(&self) -> ::windows::core::Result<()>;
    fn RevertToSelf(&self) -> ::windows::core::Result<()>;
    fn IsImpersonating(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IServerSecurity {}
#[cfg(feature = "Win32_Foundation")]
impl IServerSecurity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: isize>() -> IServerSecurity_Vtbl {
        unsafe extern "system" fn QueryBlanket<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthnsvc: *mut u32, pauthzsvc: *mut u32, pserverprincname: *mut *mut u16, pauthnlevel: *mut u32, pimplevel: *mut u32, pprivs: *mut *mut ::core::ffi::c_void, pcapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryBlanket(::core::mem::transmute_copy(&pauthnsvc), ::core::mem::transmute_copy(&pauthzsvc), ::core::mem::transmute_copy(&pserverprincname), ::core::mem::transmute_copy(&pauthnlevel), ::core::mem::transmute_copy(&pimplevel), ::core::mem::transmute_copy(&pprivs), ::core::mem::transmute_copy(&pcapabilities)).into()
        }
        unsafe extern "system" fn ImpersonateClient<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImpersonateClient().into()
        }
        unsafe extern "system" fn RevertToSelf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RevertToSelf().into()
        }
        unsafe extern "system" fn IsImpersonating<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServerSecurity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsImpersonating()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryBlanket: QueryBlanket::<Identity, Impl, OFFSET>,
            ImpersonateClient: ImpersonateClient::<Identity, Impl, OFFSET>,
            RevertToSelf: RevertToSelf::<Identity, Impl, OFFSET>,
            IsImpersonating: IsImpersonating::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServerSecurity as ::windows::core::Interface>::IID
    }
}
pub trait IServiceProvider_Impl: Sized {
    fn QueryService(&self, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceProvider {}
impl IServiceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceProvider_Impl, const OFFSET: isize>() -> IServiceProvider_Vtbl {
        unsafe extern "system" fn QueryService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryService(::core::mem::transmute_copy(&guidservice), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), QueryService: QueryService::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceProvider as ::windows::core::Interface>::IID
    }
}
pub trait IStdMarshalInfo_Impl: Sized {
    fn GetClassForHandler(&self, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IStdMarshalInfo {}
impl IStdMarshalInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStdMarshalInfo_Impl, const OFFSET: isize>() -> IStdMarshalInfo_Vtbl {
        unsafe extern "system" fn GetClassForHandler<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStdMarshalInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwdestcontext: u32, pvdestcontext: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassForHandler(::core::mem::transmute_copy(&dwdestcontext), ::core::mem::transmute_copy(&pvdestcontext), ::core::mem::transmute_copy(&pclsid)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetClassForHandler: GetClassForHandler::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStdMarshalInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IStream_Impl: Sized + ISequentialStream_Impl {
    fn Seek(&self, dlibmove: i64, dworigin: STREAM_SEEK) -> ::windows::core::Result<u64>;
    fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()>;
    fn CopyTo(&self, pstm: &::core::option::Option<IStream>, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::Result<()>;
    fn Commit(&self, grfcommitflags: STGC) -> ::windows::core::Result<()>;
    fn Revert(&self) -> ::windows::core::Result<()>;
    fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()>;
    fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()>;
    fn Stat(&self, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IStream>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IStream {}
#[cfg(feature = "Win32_Foundation")]
impl IStream_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>() -> IStream_Vtbl {
        unsafe extern "system" fn Seek<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dlibmove: i64, dworigin: STREAM_SEEK, plibnewposition: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Seek(::core::mem::transmute_copy(&dlibmove), ::core::mem::transmute_copy(&dworigin)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plibnewposition, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, libnewsize: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSize(::core::mem::transmute_copy(&libnewsize)).into()
        }
        unsafe extern "system" fn CopyTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstm: *mut ::core::ffi::c_void, cb: u64, pcbread: *mut u64, pcbwritten: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyTo(::core::mem::transmute(&pstm), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&pcbread), ::core::mem::transmute_copy(&pcbwritten)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, grfcommitflags: STGC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute_copy(&grfcommitflags)).into()
        }
        unsafe extern "system" fn Revert<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Revert().into()
        }
        unsafe extern "system" fn LockRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockRegion(::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&dwlocktype)).into()
        }
        unsafe extern "system" fn UnlockRegion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnlockRegion(::core::mem::transmute_copy(&liboffset), ::core::mem::transmute_copy(&cb), ::core::mem::transmute_copy(&dwlocktype)).into()
        }
        unsafe extern "system" fn Stat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatstg: *mut STATSTG, grfstatflag: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stat(::core::mem::transmute_copy(&pstatstg), ::core::mem::transmute_copy(&grfstatflag)).into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IStream_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppstm: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppstm, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ISequentialStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            Seek: Seek::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            CopyTo: CopyTo::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Revert: Revert::<Identity, Impl, OFFSET>,
            LockRegion: LockRegion::<Identity, Impl, OFFSET>,
            UnlockRegion: UnlockRegion::<Identity, Impl, OFFSET>,
            Stat: Stat::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStream as ::windows::core::Interface>::IID || iid == &<ISequentialStream as ::windows::core::Interface>::IID
    }
}
pub trait ISupportErrorInfo_Impl: Sized {
    fn InterfaceSupportsErrorInfo(&self, riid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISupportErrorInfo {}
impl ISupportErrorInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISupportErrorInfo_Impl, const OFFSET: isize>() -> ISupportErrorInfo_Vtbl {
        unsafe extern "system" fn InterfaceSupportsErrorInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISupportErrorInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InterfaceSupportsErrorInfo(::core::mem::transmute_copy(&riid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            InterfaceSupportsErrorInfo: InterfaceSupportsErrorInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISupportErrorInfo as ::windows::core::Interface>::IID
    }
}
pub trait ISurrogate_Impl: Sized {
    fn LoadDllServer(&self, clsid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn FreeSurrogate(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISurrogate {}
impl ISurrogate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISurrogate_Impl, const OFFSET: isize>() -> ISurrogate_Vtbl {
        unsafe extern "system" fn LoadDllServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISurrogate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LoadDllServer(::core::mem::transmute_copy(&clsid)).into()
        }
        unsafe extern "system" fn FreeSurrogate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISurrogate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeSurrogate().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            LoadDllServer: LoadDllServer::<Identity, Impl, OFFSET>,
            FreeSurrogate: FreeSurrogate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurrogate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISurrogateService_Impl: Sized {
    fn Init(&self, rguidprocessid: *const ::windows::core::GUID, pprocesslock: &::core::option::Option<IProcessLock>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ApplicationLaunch(&self, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::Result<()>;
    fn ApplicationFree(&self, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn CatalogRefresh(&self, ulreserved: u32) -> ::windows::core::Result<()>;
    fn ProcessShutdown(&self, shutdowntype: ShutdownType) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISurrogateService {}
#[cfg(feature = "Win32_Foundation")]
impl ISurrogateService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: isize>() -> ISurrogateService_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidprocessid: *const ::windows::core::GUID, pprocesslock: *mut ::core::ffi::c_void, pfapplicationaware: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Init(::core::mem::transmute_copy(&rguidprocessid), ::core::mem::transmute(&pprocesslock)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfapplicationaware, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationLaunch<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID, apptype: ApplicationType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplicationLaunch(::core::mem::transmute_copy(&rguidapplid), ::core::mem::transmute_copy(&apptype)).into()
        }
        unsafe extern "system" fn ApplicationFree<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rguidapplid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplicationFree(::core::mem::transmute_copy(&rguidapplid)).into()
        }
        unsafe extern "system" fn CatalogRefresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulreserved: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CatalogRefresh(::core::mem::transmute_copy(&ulreserved)).into()
        }
        unsafe extern "system" fn ProcessShutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISurrogateService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, shutdowntype: ShutdownType) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessShutdown(::core::mem::transmute_copy(&shutdowntype)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            ApplicationLaunch: ApplicationLaunch::<Identity, Impl, OFFSET>,
            ApplicationFree: ApplicationFree::<Identity, Impl, OFFSET>,
            CatalogRefresh: CatalogRefresh::<Identity, Impl, OFFSET>,
            ProcessShutdown: ProcessShutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISurrogateService as ::windows::core::Interface>::IID
    }
}
pub trait ISynchronize_Impl: Sized {
    fn Wait(&self, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::Result<()>;
    fn Signal(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISynchronize {}
impl ISynchronize_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronize_Impl, const OFFSET: isize>() -> ISynchronize_Vtbl {
        unsafe extern "system" fn Wait<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwmilliseconds: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Wait(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwmilliseconds)).into()
        }
        unsafe extern "system" fn Signal<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Signal().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronize_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Wait: Wait::<Identity, Impl, OFFSET>,
            Signal: Signal::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronize as ::windows::core::Interface>::IID
    }
}
pub trait ISynchronizeContainer_Impl: Sized {
    fn AddSynchronize(&self, psync: &::core::option::Option<ISynchronize>) -> ::windows::core::Result<()>;
    fn WaitMultiple(&self, dwflags: u32, dwtimeout: u32) -> ::windows::core::Result<ISynchronize>;
}
impl ::windows::core::RuntimeName for ISynchronizeContainer {}
impl ISynchronizeContainer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizeContainer_Impl, const OFFSET: isize>() -> ISynchronizeContainer_Vtbl {
        unsafe extern "system" fn AddSynchronize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psync: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSynchronize(::core::mem::transmute(&psync)).into()
        }
        unsafe extern "system" fn WaitMultiple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizeContainer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, dwtimeout: u32, ppsync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitMultiple(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&dwtimeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsync, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddSynchronize: AddSynchronize::<Identity, Impl, OFFSET>,
            WaitMultiple: WaitMultiple::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizeContainer as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronizeEvent_Impl: Sized + ISynchronizeHandle_Impl {
    fn SetEventHandle(&self, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISynchronizeEvent {}
#[cfg(feature = "Win32_Foundation")]
impl ISynchronizeEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizeEvent_Impl, const OFFSET: isize>() -> ISynchronizeEvent_Vtbl {
        unsafe extern "system" fn SetEventHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizeEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ph: *const super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventHandle(::core::mem::transmute_copy(&ph)).into()
        }
        Self { base__: ISynchronizeHandle_Vtbl::new::<Identity, Impl, OFFSET>(), SetEventHandle: SetEventHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizeEvent as ::windows::core::Interface>::IID || iid == &<ISynchronizeHandle as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISynchronizeHandle_Impl: Sized {
    fn GetHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISynchronizeHandle {}
#[cfg(feature = "Win32_Foundation")]
impl ISynchronizeHandle_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizeHandle_Impl, const OFFSET: isize>() -> ISynchronizeHandle_Vtbl {
        unsafe extern "system" fn GetHandle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizeHandle_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ph: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHandle() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ph, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetHandle: GetHandle::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizeHandle as ::windows::core::Interface>::IID
    }
}
pub trait ISynchronizeMutex_Impl: Sized + ISynchronize_Impl {
    fn ReleaseMutex(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISynchronizeMutex {}
impl ISynchronizeMutex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizeMutex_Impl, const OFFSET: isize>() -> ISynchronizeMutex_Vtbl {
        unsafe extern "system" fn ReleaseMutex<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISynchronizeMutex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseMutex().into()
        }
        Self { base__: ISynchronize_Vtbl::new::<Identity, Impl, OFFSET>(), ReleaseMutex: ReleaseMutex::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISynchronizeMutex as ::windows::core::Interface>::IID || iid == &<ISynchronize as ::windows::core::Interface>::IID
    }
}
pub trait ITimeAndNoticeControl_Impl: Sized {
    fn SuppressChanges(&self, res1: u32, res2: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITimeAndNoticeControl {}
impl ITimeAndNoticeControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITimeAndNoticeControl_Impl, const OFFSET: isize>() -> ITimeAndNoticeControl_Vtbl {
        unsafe extern "system" fn SuppressChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITimeAndNoticeControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, res1: u32, res2: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SuppressChanges(::core::mem::transmute_copy(&res1), ::core::mem::transmute_copy(&res2)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SuppressChanges: SuppressChanges::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITimeAndNoticeControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ITypeComp_Impl: Sized {
    fn Bind(&self, szname: &::windows::core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut ::core::option::Option<ITypeInfo>, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::Result<()>;
    fn BindType(&self, szname: &::windows::core::PCWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, pptcomp: *mut ::core::option::Option<ITypeComp>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITypeComp {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ITypeComp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeComp_Impl, const OFFSET: isize>() -> ITypeComp_Vtbl {
        unsafe extern "system" fn Bind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeComp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, lhashval: u32, wflags: u16, pptinfo: *mut *mut ::core::ffi::c_void, pdesckind: *mut DESCKIND, pbindptr: *mut BINDPTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Bind(::core::mem::transmute(&szname), ::core::mem::transmute_copy(&lhashval), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pptinfo), ::core::mem::transmute_copy(&pdesckind), ::core::mem::transmute_copy(&pbindptr)).into()
        }
        unsafe extern "system" fn BindType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeComp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szname: ::windows::core::PCWSTR, lhashval: u32, pptinfo: *mut *mut ::core::ffi::c_void, pptcomp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindType(::core::mem::transmute(&szname), ::core::mem::transmute_copy(&lhashval), ::core::mem::transmute_copy(&pptinfo), ::core::mem::transmute_copy(&pptcomp)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Bind: Bind::<Identity, Impl, OFFSET>,
            BindType: BindType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeComp as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ITypeInfo_Impl: Sized {
    fn GetTypeAttr(&self) -> ::windows::core::Result<*mut TYPEATTR>;
    fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp>;
    fn GetFuncDesc(&self, index: u32) -> ::windows::core::Result<*mut FUNCDESC>;
    fn GetVarDesc(&self, index: u32) -> ::windows::core::Result<*mut VARDESC>;
    fn GetNames(&self, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::Result<()>;
    fn GetRefTypeOfImplType(&self, index: u32) -> ::windows::core::Result<u32>;
    fn GetImplTypeFlags(&self, index: u32) -> ::windows::core::Result<i32>;
    fn GetIDsOfNames(&self, rgsznames: *const ::windows::core::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::Result<()>;
    fn Invoke(&self, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::Result<()>;
    fn GetDocumentation(&self, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDllEntry(&self, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::Result<()>;
    fn GetRefTypeInfo(&self, hreftype: u32) -> ::windows::core::Result<ITypeInfo>;
    fn AddressOfMember(&self, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn CreateInstance(&self, punkouter: &::core::option::Option<::windows::core::IUnknown>, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetMops(&self, memid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetContainingTypeLib(&self, pptlib: *mut ::core::option::Option<ITypeLib>, pindex: *mut u32) -> ::windows::core::Result<()>;
    fn ReleaseTypeAttr(&self, ptypeattr: *const TYPEATTR);
    fn ReleaseFuncDesc(&self, pfuncdesc: *const FUNCDESC);
    fn ReleaseVarDesc(&self, pvardesc: *const VARDESC);
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITypeInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ITypeInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>() -> ITypeInfo_Vtbl {
        unsafe extern "system" fn GetTypeAttr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeattr: *mut *mut TYPEATTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeAttr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptypeattr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeComp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptcomp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeComp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptcomp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFuncDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppfuncdesc: *mut *mut FUNCDESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFuncDesc(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfuncdesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ppvardesc: *mut *mut VARDESC) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVarDesc(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvardesc, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, rgbstrnames: *mut super::super::Foundation::BSTR, cmaxnames: u32, pcnames: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetNames(::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&rgbstrnames), ::core::mem::transmute_copy(&cmaxnames), ::core::mem::transmute_copy(&pcnames)).into()
        }
        unsafe extern "system" fn GetRefTypeOfImplType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, preftype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRefTypeOfImplType(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(preftype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplTypeFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pimpltypeflags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImplTypeFlags(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pimpltypeflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIDsOfNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgsznames: *const ::windows::core::PWSTR, cnames: u32, pmemid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIDsOfNames(::core::mem::transmute_copy(&rgsznames), ::core::mem::transmute_copy(&cnames), ::core::mem::transmute_copy(&pmemid)).into()
        }
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvinstance: *const ::core::ffi::c_void, memid: i32, wflags: u16, pdispparams: *mut DISPPARAMS, pvarresult: *mut VARIANT, pexcepinfo: *mut EXCEPINFO, puargerr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke(::core::mem::transmute_copy(&pvinstance), ::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&wflags), ::core::mem::transmute_copy(&pdispparams), ::core::mem::transmute_copy(&pvarresult), ::core::mem::transmute_copy(&pexcepinfo), ::core::mem::transmute_copy(&puargerr)).into()
        }
        unsafe extern "system" fn GetDocumentation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDocumentation(::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrdocstring), ::core::mem::transmute_copy(&pdwhelpcontext), ::core::mem::transmute_copy(&pbstrhelpfile)).into()
        }
        unsafe extern "system" fn GetDllEntry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pbstrdllname: *mut super::super::Foundation::BSTR, pbstrname: *mut super::super::Foundation::BSTR, pwordinal: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDllEntry(::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind), ::core::mem::transmute_copy(&pbstrdllname), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pwordinal)).into()
        }
        unsafe extern "system" fn GetRefTypeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hreftype: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRefTypeInfo(::core::mem::transmute_copy(&hreftype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddressOfMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddressOfMember(::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkouter: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::core::mem::transmute(&punkouter), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobj)).into()
        }
        unsafe extern "system" fn GetMops<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, pbstrmops: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMops(::core::mem::transmute_copy(&memid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmops, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContainingTypeLib<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlib: *mut *mut ::core::ffi::c_void, pindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContainingTypeLib(::core::mem::transmute_copy(&pptlib), ::core::mem::transmute_copy(&pindex)).into()
        }
        unsafe extern "system" fn ReleaseTypeAttr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeattr: *const TYPEATTR) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseTypeAttr(::core::mem::transmute_copy(&ptypeattr))
        }
        unsafe extern "system" fn ReleaseFuncDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfuncdesc: *const FUNCDESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseFuncDesc(::core::mem::transmute_copy(&pfuncdesc))
        }
        unsafe extern "system" fn ReleaseVarDesc<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardesc: *const VARDESC) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseVarDesc(::core::mem::transmute_copy(&pvardesc))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTypeAttr: GetTypeAttr::<Identity, Impl, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, Impl, OFFSET>,
            GetFuncDesc: GetFuncDesc::<Identity, Impl, OFFSET>,
            GetVarDesc: GetVarDesc::<Identity, Impl, OFFSET>,
            GetNames: GetNames::<Identity, Impl, OFFSET>,
            GetRefTypeOfImplType: GetRefTypeOfImplType::<Identity, Impl, OFFSET>,
            GetImplTypeFlags: GetImplTypeFlags::<Identity, Impl, OFFSET>,
            GetIDsOfNames: GetIDsOfNames::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, Impl, OFFSET>,
            GetDllEntry: GetDllEntry::<Identity, Impl, OFFSET>,
            GetRefTypeInfo: GetRefTypeInfo::<Identity, Impl, OFFSET>,
            AddressOfMember: AddressOfMember::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            GetMops: GetMops::<Identity, Impl, OFFSET>,
            GetContainingTypeLib: GetContainingTypeLib::<Identity, Impl, OFFSET>,
            ReleaseTypeAttr: ReleaseTypeAttr::<Identity, Impl, OFFSET>,
            ReleaseFuncDesc: ReleaseFuncDesc::<Identity, Impl, OFFSET>,
            ReleaseVarDesc: ReleaseVarDesc::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ITypeInfo2_Impl: Sized + ITypeInfo_Impl {
    fn GetTypeKind(&self) -> ::windows::core::Result<TYPEKIND>;
    fn GetTypeFlags(&self) -> ::windows::core::Result<u32>;
    fn GetFuncIndexOfMemId(&self, memid: i32, invkind: INVOKEKIND) -> ::windows::core::Result<u32>;
    fn GetVarIndexOfMemId(&self, memid: i32) -> ::windows::core::Result<u32>;
    fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT>;
    fn GetFuncCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT>;
    fn GetParamCustData(&self, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT>;
    fn GetVarCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT>;
    fn GetImplTypeCustData(&self, index: u32, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT>;
    fn GetDocumentation2(&self, memid: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA>;
    fn GetAllFuncCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA>;
    fn GetAllParamCustData(&self, indexfunc: u32, indexparam: u32) -> ::windows::core::Result<CUSTDATA>;
    fn GetAllVarCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA>;
    fn GetAllImplTypeCustData(&self, index: u32) -> ::windows::core::Result<CUSTDATA>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITypeInfo2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ITypeInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>() -> ITypeInfo2_Vtbl {
        unsafe extern "system" fn GetTypeKind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypekind: *mut TYPEKIND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeKind() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptypekind, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptypeflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFuncIndexOfMemId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, invkind: INVOKEKIND, pfuncindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFuncIndexOfMemId(::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&invkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfuncindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarIndexOfMemId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, pvarindex: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVarIndexOfMemId(::core::mem::transmute_copy(&memid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustData(::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFuncCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFuncCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParamCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParamCustData(::core::mem::transmute_copy(&indexfunc), ::core::mem::transmute_copy(&indexparam), ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVarCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVarCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImplTypeCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetImplTypeCustData(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentation2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, memid: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDocumentation2(::core::mem::transmute_copy(&memid), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrhelpstring), ::core::mem::transmute_copy(&pdwhelpstringcontext), ::core::mem::transmute_copy(&pbstrhelpstringdll)).into()
        }
        unsafe extern "system" fn GetAllCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllCustData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllFuncCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllFuncCustData(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllParamCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexfunc: u32, indexparam: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllParamCustData(::core::mem::transmute_copy(&indexfunc), ::core::mem::transmute_copy(&indexparam)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllVarCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllVarCustData(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllImplTypeCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllImplTypeCustData(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITypeInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTypeKind: GetTypeKind::<Identity, Impl, OFFSET>,
            GetTypeFlags: GetTypeFlags::<Identity, Impl, OFFSET>,
            GetFuncIndexOfMemId: GetFuncIndexOfMemId::<Identity, Impl, OFFSET>,
            GetVarIndexOfMemId: GetVarIndexOfMemId::<Identity, Impl, OFFSET>,
            GetCustData: GetCustData::<Identity, Impl, OFFSET>,
            GetFuncCustData: GetFuncCustData::<Identity, Impl, OFFSET>,
            GetParamCustData: GetParamCustData::<Identity, Impl, OFFSET>,
            GetVarCustData: GetVarCustData::<Identity, Impl, OFFSET>,
            GetImplTypeCustData: GetImplTypeCustData::<Identity, Impl, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, Impl, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, Impl, OFFSET>,
            GetAllFuncCustData: GetAllFuncCustData::<Identity, Impl, OFFSET>,
            GetAllParamCustData: GetAllParamCustData::<Identity, Impl, OFFSET>,
            GetAllVarCustData: GetAllVarCustData::<Identity, Impl, OFFSET>,
            GetAllImplTypeCustData: GetAllImplTypeCustData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeInfo2 as ::windows::core::Interface>::IID || iid == &<ITypeInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITypeLib_Impl: Sized {
    fn GetTypeInfoCount(&self) -> u32;
    fn GetTypeInfo(&self, index: u32) -> ::windows::core::Result<ITypeInfo>;
    fn GetTypeInfoType(&self, index: u32) -> ::windows::core::Result<TYPEKIND>;
    fn GetTypeInfoOfGuid(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<ITypeInfo>;
    fn GetLibAttr(&self) -> ::windows::core::Result<*mut TLIBATTR>;
    fn GetTypeComp(&self) -> ::windows::core::Result<ITypeComp>;
    fn GetDocumentation(&self, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsName(&self, sznamebuf: &::windows::core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FindName(&self, sznamebuf: &::windows::core::PWSTR, lhashval: u32, pptinfo: *mut ::core::option::Option<ITypeInfo>, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::Result<()>;
    fn ReleaseTLibAttr(&self, ptlibattr: *const TLIBATTR);
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITypeLib {}
#[cfg(feature = "Win32_Foundation")]
impl ITypeLib_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>() -> ITypeLib_Vtbl {
        unsafe extern "system" fn GetTypeInfoCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTypeInfoCount()
        }
        unsafe extern "system" fn GetTypeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeInfo(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfoType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, ptkind: *mut TYPEKIND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeInfoType(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptkind, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeInfoOfGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pptinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeInfoOfGuid(::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLibAttr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptlibattr: *mut *mut TLIBATTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLibAttr() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptlibattr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypeComp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptcomp: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypeComp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptcomp, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDocumentation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pbstrname: *mut super::super::Foundation::BSTR, pbstrdocstring: *mut super::super::Foundation::BSTR, pdwhelpcontext: *mut u32, pbstrhelpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDocumentation(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&pbstrname), ::core::mem::transmute_copy(&pbstrdocstring), ::core::mem::transmute_copy(&pdwhelpcontext), ::core::mem::transmute_copy(&pbstrhelpfile)).into()
        }
        unsafe extern "system" fn IsName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pfname: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsName(::core::mem::transmute(&sznamebuf), ::core::mem::transmute_copy(&lhashval), ::core::mem::transmute_copy(&pfname)).into()
        }
        unsafe extern "system" fn FindName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznamebuf: ::windows::core::PWSTR, lhashval: u32, pptinfo: *mut *mut ::core::ffi::c_void, rgmemid: *mut i32, pcfound: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindName(::core::mem::transmute(&sznamebuf), ::core::mem::transmute_copy(&lhashval), ::core::mem::transmute_copy(&pptinfo), ::core::mem::transmute_copy(&rgmemid), ::core::mem::transmute_copy(&pcfound)).into()
        }
        unsafe extern "system" fn ReleaseTLibAttr<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptlibattr: *const TLIBATTR) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseTLibAttr(::core::mem::transmute_copy(&ptlibattr))
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetTypeInfoCount: GetTypeInfoCount::<Identity, Impl, OFFSET>,
            GetTypeInfo: GetTypeInfo::<Identity, Impl, OFFSET>,
            GetTypeInfoType: GetTypeInfoType::<Identity, Impl, OFFSET>,
            GetTypeInfoOfGuid: GetTypeInfoOfGuid::<Identity, Impl, OFFSET>,
            GetLibAttr: GetLibAttr::<Identity, Impl, OFFSET>,
            GetTypeComp: GetTypeComp::<Identity, Impl, OFFSET>,
            GetDocumentation: GetDocumentation::<Identity, Impl, OFFSET>,
            IsName: IsName::<Identity, Impl, OFFSET>,
            FindName: FindName::<Identity, Impl, OFFSET>,
            ReleaseTLibAttr: ReleaseTLibAttr::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeLib as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait ITypeLib2_Impl: Sized + ITypeLib_Impl {
    fn GetCustData(&self, guid: *const ::windows::core::GUID) -> ::windows::core::Result<VARIANT>;
    fn GetLibStatistics(&self, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::Result<()>;
    fn GetDocumentation2(&self, index: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetAllCustData(&self) -> ::windows::core::Result<CUSTDATA>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITypeLib2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ITypeLib2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: isize>() -> ITypeLib2_Vtbl {
        unsafe extern "system" fn GetCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: *const ::windows::core::GUID, pvarval: *mut VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCustData(::core::mem::transmute_copy(&guid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLibStatistics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcuniquenames: *mut u32, pcchuniquenames: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLibStatistics(::core::mem::transmute_copy(&pcuniquenames), ::core::mem::transmute_copy(&pcchuniquenames)).into()
        }
        unsafe extern "system" fn GetDocumentation2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, lcid: u32, pbstrhelpstring: *mut super::super::Foundation::BSTR, pdwhelpstringcontext: *mut u32, pbstrhelpstringdll: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDocumentation2(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&lcid), ::core::mem::transmute_copy(&pbstrhelpstring), ::core::mem::transmute_copy(&pdwhelpstringcontext), ::core::mem::transmute_copy(&pbstrhelpstringdll)).into()
        }
        unsafe extern "system" fn GetAllCustData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLib2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcustdata: *mut CUSTDATA) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllCustData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcustdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ITypeLib_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCustData: GetCustData::<Identity, Impl, OFFSET>,
            GetLibStatistics: GetLibStatistics::<Identity, Impl, OFFSET>,
            GetDocumentation2: GetDocumentation2::<Identity, Impl, OFFSET>,
            GetAllCustData: GetAllCustData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeLib2 as ::windows::core::Interface>::IID || iid == &<ITypeLib as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ITypeLibRegistration_Impl: Sized {
    fn GetGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetVersion(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetLcid(&self) -> ::windows::core::Result<u32>;
    fn GetWin32Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetWin64Path(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFlags(&self) -> ::windows::core::Result<u32>;
    fn GetHelpDir(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ITypeLibRegistration {}
#[cfg(feature = "Win32_Foundation")]
impl ITypeLibRegistration_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: isize>() -> ITypeLibRegistration_Vtbl {
        unsafe extern "system" fn GetGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pversion: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLcid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcid: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLcid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWin32Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwin32path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWin32Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwin32path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWin64Path<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwin64path: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWin64Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwin64path, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdisplayname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFlags() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpDir<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistration_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phelpdir: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHelpDir() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phelpdir, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetGuid: GetGuid::<Identity, Impl, OFFSET>,
            GetVersion: GetVersion::<Identity, Impl, OFFSET>,
            GetLcid: GetLcid::<Identity, Impl, OFFSET>,
            GetWin32Path: GetWin32Path::<Identity, Impl, OFFSET>,
            GetWin64Path: GetWin64Path::<Identity, Impl, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            GetFlags: GetFlags::<Identity, Impl, OFFSET>,
            GetHelpDir: GetHelpDir::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeLibRegistration as ::windows::core::Interface>::IID
    }
}
pub trait ITypeLibRegistrationReader_Impl: Sized {
    fn EnumTypeLibRegistrations(&self) -> ::windows::core::Result<IEnumUnknown>;
}
impl ::windows::core::RuntimeName for ITypeLibRegistrationReader {}
impl ITypeLibRegistrationReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistrationReader_Impl, const OFFSET: isize>() -> ITypeLibRegistrationReader_Vtbl {
        unsafe extern "system" fn EnumTypeLibRegistrations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITypeLibRegistrationReader_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumTypeLibRegistrations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumunknown, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), EnumTypeLibRegistrations: EnumTypeLibRegistrations::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITypeLibRegistrationReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUri_Impl: Sized {
    fn GetPropertyBSTR(&self, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetPropertyLength(&self, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn GetPropertyDWORD(&self, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn HasProperty(&self, uriprop: Uri_PROPERTY) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetAbsoluteUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetAuthority(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDisplayUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDomain(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetExtension(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetFragment(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetHost(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPassword(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPathAndQuery(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetQuery(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRawUri(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSchemeName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetUserInfo(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetUserName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetHostType(&self) -> ::windows::core::Result<u32>;
    fn GetPort(&self) -> ::windows::core::Result<u32>;
    fn GetScheme(&self) -> ::windows::core::Result<u32>;
    fn GetZone(&self) -> ::windows::core::Result<u32>;
    fn GetProperties(&self) -> ::windows::core::Result<u32>;
    fn IsEqual(&self, puri: &::core::option::Option<IUri>) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUri {}
#[cfg(feature = "Win32_Foundation")]
impl IUri_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>() -> IUri_Vtbl {
        unsafe extern "system" fn GetPropertyBSTR<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pbstrproperty: *mut super::super::Foundation::BSTR, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyBSTR(::core::mem::transmute_copy(&uriprop), ::core::mem::transmute_copy(&pbstrproperty), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetPropertyLength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pcchproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyLength(::core::mem::transmute_copy(&uriprop), ::core::mem::transmute_copy(&pcchproperty), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn GetPropertyDWORD<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pdwproperty: *mut u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPropertyDWORD(::core::mem::transmute_copy(&uriprop), ::core::mem::transmute_copy(&pdwproperty), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn HasProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uriprop: Uri_PROPERTY, pfhasproperty: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasProperty(::core::mem::transmute_copy(&uriprop)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfhasproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAbsoluteUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrabsoluteuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAbsoluteUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrabsoluteuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthority<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrauthority: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAuthority() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrauthority, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdisplaystring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdisplaystring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomain<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdomain: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDomain() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdomain, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtension<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrextension: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetExtension() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrextension, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFragment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfragment: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFragment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfragment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrhost: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHost() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrhost, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPassword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpassword: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPassword() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpassword, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpath, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPathAndQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpathandquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPathAndQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpathandquery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrquery: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrquery, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRawUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrrawuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRawUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrrawuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrschemename: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSchemeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrschemename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstruserinfo: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstruserinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwhosttype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHostType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwhosttype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPort() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScheme<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwscheme: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScheme() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwscheme, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetZone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwzone: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetZone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwzone, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwflags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEqual<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUri_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puri: *mut ::core::ffi::c_void, pfequal: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsEqual(::core::mem::transmute(&puri)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfequal, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPropertyBSTR: GetPropertyBSTR::<Identity, Impl, OFFSET>,
            GetPropertyLength: GetPropertyLength::<Identity, Impl, OFFSET>,
            GetPropertyDWORD: GetPropertyDWORD::<Identity, Impl, OFFSET>,
            HasProperty: HasProperty::<Identity, Impl, OFFSET>,
            GetAbsoluteUri: GetAbsoluteUri::<Identity, Impl, OFFSET>,
            GetAuthority: GetAuthority::<Identity, Impl, OFFSET>,
            GetDisplayUri: GetDisplayUri::<Identity, Impl, OFFSET>,
            GetDomain: GetDomain::<Identity, Impl, OFFSET>,
            GetExtension: GetExtension::<Identity, Impl, OFFSET>,
            GetFragment: GetFragment::<Identity, Impl, OFFSET>,
            GetHost: GetHost::<Identity, Impl, OFFSET>,
            GetPassword: GetPassword::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetPathAndQuery: GetPathAndQuery::<Identity, Impl, OFFSET>,
            GetQuery: GetQuery::<Identity, Impl, OFFSET>,
            GetRawUri: GetRawUri::<Identity, Impl, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, Impl, OFFSET>,
            GetUserInfo: GetUserInfo::<Identity, Impl, OFFSET>,
            GetUserName: GetUserName::<Identity, Impl, OFFSET>,
            GetHostType: GetHostType::<Identity, Impl, OFFSET>,
            GetPort: GetPort::<Identity, Impl, OFFSET>,
            GetScheme: GetScheme::<Identity, Impl, OFFSET>,
            GetZone: GetZone::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            IsEqual: IsEqual::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUri as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IUriBuilder_Impl: Sized {
    fn CreateUriSimple(&self, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri>;
    fn CreateUri(&self, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri>;
    fn CreateUriWithFlags(&self, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize) -> ::windows::core::Result<IUri>;
    fn GetIUri(&self) -> ::windows::core::Result<IUri>;
    fn SetIUri(&self, piuri: &::core::option::Option<IUri>) -> ::windows::core::Result<()>;
    fn GetFragment(&self, pcchfragment: *mut u32, ppwzfragment: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetHost(&self, pcchhost: *mut u32, ppwzhost: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetPassword(&self, pcchpassword: *mut u32, ppwzpassword: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetPath(&self, pcchpath: *mut u32, ppwzpath: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetPort(&self, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::Result<()>;
    fn GetQuery(&self, pcchquery: *mut u32, ppwzquery: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetSchemeName(&self, pcchschemename: *mut u32, ppwzschemename: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn GetUserName(&self, pcchusername: *mut u32, ppwzusername: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn SetFragment(&self, pwznewvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetHost(&self, pwznewvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetPassword(&self, pwznewvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetPath(&self, pwznewvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetPort(&self, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows::core::Result<()>;
    fn SetQuery(&self, pwznewvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetSchemeName(&self, pwznewvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SetUserName(&self, pwznewvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn RemoveProperties(&self, dwpropertymask: u32) -> ::windows::core::Result<()>;
    fn HasBeenModified(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IUriBuilder {}
#[cfg(feature = "Win32_Foundation")]
impl IUriBuilder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>() -> IUriBuilder_Vtbl {
        unsafe extern "system" fn CreateUriSimple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateUriSimple(::core::mem::transmute_copy(&dwallowencodingpropertymask), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateUri(::core::mem::transmute_copy(&dwcreateflags), ::core::mem::transmute_copy(&dwallowencodingpropertymask), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateUriWithFlags<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreateflags: u32, dwuribuilderflags: u32, dwallowencodingpropertymask: u32, dwreserved: usize, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateUriWithFlags(::core::mem::transmute_copy(&dwcreateflags), ::core::mem::transmute_copy(&dwuribuilderflags), ::core::mem::transmute_copy(&dwallowencodingpropertymask), ::core::mem::transmute_copy(&dwreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiuri: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIUri() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIUri<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piuri: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIUri(::core::mem::transmute(&piuri)).into()
        }
        unsafe extern "system" fn GetFragment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchfragment: *mut u32, ppwzfragment: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFragment(::core::mem::transmute_copy(&pcchfragment), ::core::mem::transmute_copy(&ppwzfragment)).into()
        }
        unsafe extern "system" fn GetHost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchhost: *mut u32, ppwzhost: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHost(::core::mem::transmute_copy(&pcchhost), ::core::mem::transmute_copy(&ppwzhost)).into()
        }
        unsafe extern "system" fn GetPassword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchpassword: *mut u32, ppwzpassword: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPassword(::core::mem::transmute_copy(&pcchpassword), ::core::mem::transmute_copy(&ppwzpassword)).into()
        }
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchpath: *mut u32, ppwzpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPath(::core::mem::transmute_copy(&pcchpath), ::core::mem::transmute_copy(&ppwzpath)).into()
        }
        unsafe extern "system" fn GetPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfhasport: *mut super::super::Foundation::BOOL, pdwport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPort(::core::mem::transmute_copy(&pfhasport), ::core::mem::transmute_copy(&pdwport)).into()
        }
        unsafe extern "system" fn GetQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchquery: *mut u32, ppwzquery: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetQuery(::core::mem::transmute_copy(&pcchquery), ::core::mem::transmute_copy(&ppwzquery)).into()
        }
        unsafe extern "system" fn GetSchemeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchschemename: *mut u32, ppwzschemename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSchemeName(::core::mem::transmute_copy(&pcchschemename), ::core::mem::transmute_copy(&ppwzschemename)).into()
        }
        unsafe extern "system" fn GetUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchusername: *mut u32, ppwzusername: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUserName(::core::mem::transmute_copy(&pcchusername), ::core::mem::transmute_copy(&ppwzusername)).into()
        }
        unsafe extern "system" fn SetFragment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFragment(::core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetHost<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHost(::core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPassword(::core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPath(::core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fhasport: super::super::Foundation::BOOL, dwnewvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPort(::core::mem::transmute_copy(&fhasport), ::core::mem::transmute_copy(&dwnewvalue)).into()
        }
        unsafe extern "system" fn SetQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQuery(::core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetSchemeName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSchemeName(::core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn SetUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwznewvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserName(::core::mem::transmute(&pwznewvalue)).into()
        }
        unsafe extern "system" fn RemoveProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpropertymask: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveProperties(::core::mem::transmute_copy(&dwpropertymask)).into()
        }
        unsafe extern "system" fn HasBeenModified<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUriBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfmodified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HasBeenModified() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmodified, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateUriSimple: CreateUriSimple::<Identity, Impl, OFFSET>,
            CreateUri: CreateUri::<Identity, Impl, OFFSET>,
            CreateUriWithFlags: CreateUriWithFlags::<Identity, Impl, OFFSET>,
            GetIUri: GetIUri::<Identity, Impl, OFFSET>,
            SetIUri: SetIUri::<Identity, Impl, OFFSET>,
            GetFragment: GetFragment::<Identity, Impl, OFFSET>,
            GetHost: GetHost::<Identity, Impl, OFFSET>,
            GetPassword: GetPassword::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetPort: GetPort::<Identity, Impl, OFFSET>,
            GetQuery: GetQuery::<Identity, Impl, OFFSET>,
            GetSchemeName: GetSchemeName::<Identity, Impl, OFFSET>,
            GetUserName: GetUserName::<Identity, Impl, OFFSET>,
            SetFragment: SetFragment::<Identity, Impl, OFFSET>,
            SetHost: SetHost::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
            SetQuery: SetQuery::<Identity, Impl, OFFSET>,
            SetSchemeName: SetSchemeName::<Identity, Impl, OFFSET>,
            SetUserName: SetUserName::<Identity, Impl, OFFSET>,
            RemoveProperties: RemoveProperties::<Identity, Impl, OFFSET>,
            HasBeenModified: HasBeenModified::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUriBuilder as ::windows::core::Interface>::IID
    }
}
pub trait IUrlMon_Impl: Sized {
    fn AsyncGetClassBits(&self, rclsid: *const ::windows::core::GUID, psztype: &::windows::core::PCWSTR, pszext: &::windows::core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: &::windows::core::PCWSTR, pbc: &::core::option::Option<IBindCtx>, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IUrlMon {}
impl IUrlMon_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlMon_Impl, const OFFSET: isize>() -> IUrlMon_Vtbl {
        unsafe extern "system" fn AsyncGetClassBits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IUrlMon_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, psztype: ::windows::core::PCWSTR, pszext: ::windows::core::PCWSTR, dwfileversionms: u32, dwfileversionls: u32, pszcodebase: ::windows::core::PCWSTR, pbc: *mut ::core::ffi::c_void, dwclasscontext: u32, riid: *const ::windows::core::GUID, flags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncGetClassBits(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute(&psztype), ::core::mem::transmute(&pszext), ::core::mem::transmute_copy(&dwfileversionms), ::core::mem::transmute_copy(&dwfileversionls), ::core::mem::transmute(&pszcodebase), ::core::mem::transmute(&pbc), ::core::mem::transmute_copy(&dwclasscontext), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AsyncGetClassBits: AsyncGetClassBits::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUrlMon as ::windows::core::Interface>::IID
    }
}
pub trait IWaitMultiple_Impl: Sized {
    fn WaitMultiple(&self, timeout: u32) -> ::windows::core::Result<ISynchronize>;
    fn AddSynchronize(&self, psync: &::core::option::Option<ISynchronize>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IWaitMultiple {}
impl IWaitMultiple_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWaitMultiple_Impl, const OFFSET: isize>() -> IWaitMultiple_Vtbl {
        unsafe extern "system" fn WaitMultiple<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWaitMultiple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, timeout: u32, psync: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WaitMultiple(::core::mem::transmute_copy(&timeout)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psync, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSynchronize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWaitMultiple_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psync: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSynchronize(::core::mem::transmute(&psync)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            WaitMultiple: WaitMultiple::<Identity, Impl, OFFSET>,
            AddSynchronize: AddSynchronize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWaitMultiple as ::windows::core::Interface>::IID
    }
}
