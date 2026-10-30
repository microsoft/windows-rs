pub const DISPATCH_CONSTRUCT: i32 = 16384;
pub const DISPID_STARTENUM: i32 = -1;
pub const DISPID_THIS: i32 = -613;
windows_core::imp::define_interface!(ICanHandleException, ICanHandleException_Vtbl, 0xc5598e60_b307_11d1_b27d_006008c3fbfb);
windows_core::imp::interface_hierarchy!(ICanHandleException, windows_core::IUnknown);
impl ICanHandleException {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CanHandleException(&self, pexcepinfo: *const super::EXCEPINFO, pvar: *const super::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CanHandleException)(windows_core::Interface::as_raw(self), pexcepinfo, pvar) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanHandleException_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub CanHandleException: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::EXCEPINFO, *const super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    CanHandleException: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICanHandleException_Impl: windows_core::IUnknownImpl {
    fn CanHandleException(&self, pexcepinfo: *const super::EXCEPINFO, pvar: *const super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl ICanHandleException_Vtbl {
    pub const fn new<Identity: ICanHandleException_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanHandleException<Identity: ICanHandleException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexcepinfo: *const super::EXCEPINFO, pvar: *const super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ICanHandleException_Impl::CanHandleException(this, core::mem::transmute_copy(&pexcepinfo), core::mem::transmute_copy(&pvar)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CanHandleException: CanHandleException::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ICanHandleException as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICanHandleException {}
windows_core::imp::define_interface!(IDispError, IDispError_Vtbl, 0xa6ef9861_c720_11d0_9337_00a0c90dcaa9);
windows_core::imp::interface_hierarchy!(IDispError, windows_core::IUnknown);
impl IDispError {
    pub unsafe fn QueryErrorInfo(&self, guiderrortype: windows_core::GUID, ppde: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).QueryErrorInfo)(windows_core::Interface::as_raw(self), guiderrortype, core::mem::transmute(ppde)) }
    }
    pub unsafe fn GetNext(&self, ppde: *mut Option<Self>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNext)(windows_core::Interface::as_raw(self), core::mem::transmute(ppde)) }
    }
    pub unsafe fn GetHresult(&self, phr: *mut windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHresult)(windows_core::Interface::as_raw(self), phr as _) }
    }
    pub unsafe fn GetSource(&self, pbstrsource: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrsource)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetHelpInfo(&self, pbstrfilename: *mut windows_core::BSTR, pdwcontext: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHelpInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrfilename), pdwcontext as _) }
    }
    pub unsafe fn GetDescription(&self, pbstrdescription: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrdescription)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDispError_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub QueryErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHresult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetHelpInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetHelpInfo: usize,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IDispError_Impl: windows_core::IUnknownImpl {
    fn QueryErrorInfo(&self, guiderrortype: &windows_core::GUID, ppde: windows_core::OutRef<IDispError>) -> windows_core::Result<()>;
    fn GetNext(&self, ppde: windows_core::OutRef<IDispError>) -> windows_core::Result<()>;
    fn GetHresult(&self, phr: *mut windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetSource(&self, pbstrsource: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetHelpInfo(&self, pbstrfilename: *mut windows_core::BSTR, pdwcontext: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetDescription(&self, pbstrdescription: *mut windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IDispError_Vtbl {
    pub const fn new<Identity: IDispError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryErrorInfo<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guiderrortype: windows_core::GUID, ppde: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispError_Impl::QueryErrorInfo(this, core::mem::transmute(&guiderrortype), core::mem::transmute_copy(&ppde)).into()
            }
        }
        unsafe extern "system" fn GetNext<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppde: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispError_Impl::GetNext(this, core::mem::transmute_copy(&ppde)).into()
            }
        }
        unsafe extern "system" fn GetHresult<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispError_Impl::GetHresult(this, core::mem::transmute_copy(&phr)).into()
            }
        }
        unsafe extern "system" fn GetSource<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispError_Impl::GetSource(this, core::mem::transmute_copy(&pbstrsource)).into()
            }
        }
        unsafe extern "system" fn GetHelpInfo<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void, pdwcontext: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispError_Impl::GetHelpInfo(this, core::mem::transmute_copy(&pbstrfilename), core::mem::transmute_copy(&pdwcontext)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispError_Impl::GetDescription(this, core::mem::transmute_copy(&pbstrdescription)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryErrorInfo: QueryErrorInfo::<Identity, OFFSET>,
            GetNext: GetNext::<Identity, OFFSET>,
            GetHresult: GetHresult::<Identity, OFFSET>,
            GetSource: GetSource::<Identity, OFFSET>,
            GetHelpInfo: GetHelpInfo::<Identity, OFFSET>,
            GetDescription: GetDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispError as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDispError {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDispatchEx, IDispatchEx_Vtbl, 0xa6ef9860_c720_11d0_9337_00a0c90dcaa9);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDispatchEx {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDispatchEx, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDispatchEx {
    #[cfg(feature = "shared")]
    pub unsafe fn GetDispID(&self, bstrname: &windows_core::BSTR, grfdex: super::DWORD, pid: *mut super::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDispID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), grfdex, pid as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "servprov", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InvokeEx<P6>(&self, id: super::DISPID, lcid: super::LCID, wflags: super::WORD, pdp: *const super::DISPPARAMS, pvarres: Option<*mut super::VARIANT>, pei: Option<*mut super::EXCEPINFO>, pspcaller: P6) -> windows_core::HRESULT
    where
        P6: windows_core::Param<super::IServiceProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).InvokeEx)(windows_core::Interface::as_raw(self), id, lcid, wflags, pdp, pvarres.unwrap_or(core::mem::zeroed()) as _, pei.unwrap_or(core::mem::zeroed()) as _, pspcaller.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteMemberByName(&self, bstrname: &windows_core::BSTR, grfdex: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteMemberByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), grfdex) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn DeleteMemberByDispID(&self, id: super::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteMemberByDispID)(windows_core::Interface::as_raw(self), id) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMemberProperties(&self, id: super::DISPID, grfdexfetch: super::DWORD, pgrfdex: *mut super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMemberProperties)(windows_core::Interface::as_raw(self), id, grfdexfetch, pgrfdex as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMemberName(&self, id: super::DISPID, pbstrname: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetMemberName)(windows_core::Interface::as_raw(self), id, core::mem::transmute(pbstrname)) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetNextDispID(&self, grfdex: super::DWORD, id: super::DISPID, pid: *mut super::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNextDispID)(windows_core::Interface::as_raw(self), grfdex, id, pid as _) }
    }
    pub unsafe fn GetNameSpaceParent(&self, ppunk: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNameSpaceParent)(windows_core::Interface::as_raw(self), core::mem::transmute(ppunk)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDispatchEx_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub GetDispID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD, *mut super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDispID: usize,
    #[cfg(all(feature = "minwindef", feature = "servprov", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub InvokeEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, super::LCID, super::WORD, *const super::DISPPARAMS, *mut super::VARIANT, *mut super::EXCEPINFO, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "servprov", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    InvokeEx: usize,
    #[cfg(feature = "shared")]
    pub DeleteMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteMemberByName: usize,
    #[cfg(feature = "shared")]
    pub DeleteMemberByDispID: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    DeleteMemberByDispID: usize,
    #[cfg(feature = "shared")]
    pub GetMemberProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, super::DWORD, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMemberProperties: usize,
    #[cfg(feature = "shared")]
    pub GetMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, super::DISPID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMemberName: usize,
    #[cfg(feature = "shared")]
    pub GetNextDispID: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::DISPID, *mut super::DISPID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetNextDispID: usize,
    pub GetNameSpaceParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "servprov", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDispatchEx_Impl: super::IDispatch_Impl {
    fn GetDispID(&self, bstrname: &windows_core::BSTR, grfdex: super::DWORD, pid: *mut super::DISPID) -> windows_core::Result<()>;
    fn InvokeEx(&self, id: super::DISPID, lcid: super::LCID, wflags: super::WORD, pdp: *const super::DISPPARAMS, pvarres: *mut super::VARIANT, pei: *mut super::EXCEPINFO, pspcaller: windows_core::Ref<super::IServiceProvider>) -> windows_core::Result<()>;
    fn DeleteMemberByName(&self, bstrname: &windows_core::BSTR, grfdex: super::DWORD) -> windows_core::Result<()>;
    fn DeleteMemberByDispID(&self, id: super::DISPID) -> windows_core::Result<()>;
    fn GetMemberProperties(&self, id: super::DISPID, grfdexfetch: super::DWORD, pgrfdex: *mut super::DWORD) -> windows_core::Result<()>;
    fn GetMemberName(&self, id: super::DISPID, pbstrname: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetNextDispID(&self, grfdex: super::DWORD, id: super::DISPID, pid: *mut super::DISPID) -> windows_core::Result<()>;
    fn GetNameSpaceParent(&self, ppunk: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "servprov", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDispatchEx_Vtbl {
    pub const fn new<Identity: IDispatchEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDispID<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, grfdex: super::DWORD, pid: *mut super::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::GetDispID(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&grfdex), core::mem::transmute_copy(&pid)).into()
            }
        }
        unsafe extern "system" fn InvokeEx<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::DISPID, lcid: super::LCID, wflags: super::WORD, pdp: *const super::DISPPARAMS, pvarres: *mut super::VARIANT, pei: *mut super::EXCEPINFO, pspcaller: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::InvokeEx(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdp), core::mem::transmute_copy(&pvarres), core::mem::transmute_copy(&pei), core::mem::transmute_copy(&pspcaller)).into()
            }
        }
        unsafe extern "system" fn DeleteMemberByName<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, grfdex: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::DeleteMemberByName(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&grfdex)).into()
            }
        }
        unsafe extern "system" fn DeleteMemberByDispID<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::DeleteMemberByDispID(this, core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn GetMemberProperties<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::DISPID, grfdexfetch: super::DWORD, pgrfdex: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::GetMemberProperties(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&grfdexfetch), core::mem::transmute_copy(&pgrfdex)).into()
            }
        }
        unsafe extern "system" fn GetMemberName<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::DISPID, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::GetMemberName(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&pbstrname)).into()
            }
        }
        unsafe extern "system" fn GetNextDispID<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfdex: super::DWORD, id: super::DISPID, pid: *mut super::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::GetNextDispID(this, core::mem::transmute_copy(&grfdex), core::mem::transmute_copy(&id), core::mem::transmute_copy(&pid)).into()
            }
        }
        unsafe extern "system" fn GetNameSpaceParent<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::GetNameSpaceParent(this, core::mem::transmute_copy(&ppunk)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            GetDispID: GetDispID::<Identity, OFFSET>,
            InvokeEx: InvokeEx::<Identity, OFFSET>,
            DeleteMemberByName: DeleteMemberByName::<Identity, OFFSET>,
            DeleteMemberByDispID: DeleteMemberByDispID::<Identity, OFFSET>,
            GetMemberProperties: GetMemberProperties::<Identity, OFFSET>,
            GetMemberName: GetMemberName::<Identity, OFFSET>,
            GetNextDispID: GetNextDispID::<Identity, OFFSET>,
            GetNameSpaceParent: GetNameSpaceParent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDispatchEx as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "servprov", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDispatchEx {}
pub const IID_ICanHandleException: windows_core::GUID = windows_core::GUID::from_u128(0xc5598e60_b307_11d1_b27d_006008c3fbfb);
pub const IID_IDispError: windows_core::GUID = windows_core::GUID::from_u128(0xa6ef9861_c720_11d0_9337_00a0c90dcaa9);
pub const IID_IDispatchEx: windows_core::GUID = windows_core::GUID::from_u128(0xa6ef9860_c720_11d0_9337_00a0c90dcaa9);
pub const IID_IObjectIdentity: windows_core::GUID = windows_core::GUID::from_u128(0xca04b7e6_0d21_11d1_8cc5_00c04fc2b085);
pub const IID_IProvideRuntimeContext: windows_core::GUID = windows_core::GUID::from_u128(0x10e2414a_ec59_49d2_bc51_5add2c36febc);
pub const IID_IVariantChangeType: windows_core::GUID = windows_core::GUID::from_u128(0xa6ef9862_c720_11d0_9337_00a0c90dcaa9);
windows_core::imp::define_interface!(IObjectIdentity, IObjectIdentity_Vtbl, 0xca04b7e6_0d21_11d1_8cc5_00c04fc2b085);
windows_core::imp::interface_hierarchy!(IObjectIdentity, windows_core::IUnknown);
impl IObjectIdentity {
    pub unsafe fn IsEqualObject<P0>(&self, punk: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsEqualObject)(windows_core::Interface::as_raw(self), punk.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectIdentity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsEqualObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IObjectIdentity_Impl: windows_core::IUnknownImpl {
    fn IsEqualObject(&self, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IObjectIdentity_Vtbl {
    pub const fn new<Identity: IObjectIdentity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsEqualObject<Identity: IObjectIdentity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IObjectIdentity_Impl::IsEqualObject(this, core::mem::transmute_copy(&punk)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsEqualObject: IsEqualObject::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectIdentity as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjectIdentity {}
windows_core::imp::define_interface!(IProvideRuntimeContext, IProvideRuntimeContext_Vtbl, 0x10e2414a_ec59_49d2_bc51_5add2c36febc);
windows_core::imp::interface_hierarchy!(IProvideRuntimeContext, windows_core::IUnknown);
impl IProvideRuntimeContext {
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn GetCurrentSourceContext(&self, pdwcontext: *mut super::DWORD_PTR, pfexecutingglobalcode: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentSourceContext)(windows_core::Interface::as_raw(self), pdwcontext as _, pfexecutingglobalcode as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideRuntimeContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub GetCurrentSourceContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD_PTR, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    GetCurrentSourceContext: usize,
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub trait IProvideRuntimeContext_Impl: windows_core::IUnknownImpl {
    fn GetCurrentSourceContext(&self, pdwcontext: *mut super::DWORD_PTR, pfexecutingglobalcode: *mut super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
impl IProvideRuntimeContext_Vtbl {
    pub const fn new<Identity: IProvideRuntimeContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentSourceContext<Identity: IProvideRuntimeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcontext: *mut super::DWORD_PTR, pfexecutingglobalcode: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProvideRuntimeContext_Impl::GetCurrentSourceContext(this, core::mem::transmute_copy(&pdwcontext), core::mem::transmute_copy(&pfexecutingglobalcode)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetCurrentSourceContext: GetCurrentSourceContext::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProvideRuntimeContext as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "wtypes"))]
impl windows_core::RuntimeName for IProvideRuntimeContext {}
windows_core::imp::define_interface!(IVariantChangeType, IVariantChangeType_Vtbl, 0xa6ef9862_c720_11d0_9337_00a0c90dcaa9);
windows_core::imp::interface_hierarchy!(IVariantChangeType, windows_core::IUnknown);
impl IVariantChangeType {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ChangeType(&self, pvardst: Option<*mut super::VARIANT>, pvarsrc: Option<*const super::VARIANT>, lcid: super::LCID, vtnew: super::VARTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangeType)(windows_core::Interface::as_raw(self), pvardst.unwrap_or(core::mem::zeroed()) as _, pvarsrc.unwrap_or(core::mem::zeroed()) as _, lcid, vtnew) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariantChangeType_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT, *const super::VARIANT, super::LCID, super::VARTYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ChangeType: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IVariantChangeType_Impl: windows_core::IUnknownImpl {
    fn ChangeType(&self, pvardst: *mut super::VARIANT, pvarsrc: *const super::VARIANT, lcid: super::LCID, vtnew: super::VARTYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IVariantChangeType_Vtbl {
    pub const fn new<Identity: IVariantChangeType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ChangeType<Identity: IVariantChangeType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardst: *mut super::VARIANT, pvarsrc: *const super::VARIANT, lcid: super::LCID, vtnew: super::VARTYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVariantChangeType_Impl::ChangeType(this, core::mem::transmute_copy(&pvardst), core::mem::transmute_copy(&pvarsrc), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&vtnew)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ChangeType: ChangeType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVariantChangeType as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IVariantChangeType {}
pub const SID_GetCaller: windows_core::GUID = windows_core::GUID::from_u128(0x4717cc40_bcb9_11d0_9336_00a0c90dcaa9);
pub const SID_ProvideRuntimeContext: windows_core::GUID = windows_core::GUID::from_u128(0x74a5040c_dd0c_48f0_ac85_194c3259180a);
pub const SID_VariantConversion: windows_core::GUID = windows_core::GUID::from_u128(0x1f101481_bccd_11d0_9336_00a0c90dcaa9);
pub const fdexEnumAll: i32 = 2;
pub const fdexEnumDefault: i32 = 1;
pub const fdexNameCaseInsensitive: i32 = 8;
pub const fdexNameCaseSensitive: i32 = 1;
pub const fdexNameEnsure: i32 = 2;
pub const fdexNameImplicit: i32 = 4;
pub const fdexNameInternal: i32 = 16;
pub const fdexNameNoDynamicProperties: i32 = 32;
pub const fdexPropCanCall: i32 = 256;
pub const fdexPropCanConstruct: i32 = 1024;
pub const fdexPropCanGet: i32 = 1;
pub const fdexPropCanPut: i32 = 4;
pub const fdexPropCanPutRef: i32 = 16;
pub const fdexPropCanSourceEvents: i32 = 4096;
pub const fdexPropCannotCall: i32 = 512;
pub const fdexPropCannotConstruct: i32 = 2048;
pub const fdexPropCannotGet: i32 = 2;
pub const fdexPropCannotPut: i32 = 8;
pub const fdexPropCannotPutRef: i32 = 32;
pub const fdexPropCannotSourceEvents: i32 = 8192;
pub const fdexPropDynamicType: i32 = 128;
pub const fdexPropNoSideEffects: i32 = 64;
pub const grfdexPropAll: i32 = 16383;
pub const grfdexPropCanAll: i32 = 5397;
pub const grfdexPropCannotAll: i32 = 10794;
pub const grfdexPropExtraAll: i32 = 192;
