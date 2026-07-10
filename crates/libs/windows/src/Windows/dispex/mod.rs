pub const DISPATCH_CONSTRUCT: u32 = 16384;
pub const DISPID_STARTENUM: i32 = -1;
windows_core::imp::define_interface!(ICanHandleException, ICanHandleException_Vtbl, 0xc5598e60_b307_11d1_b27d_006008c3fbfb);
windows_core::imp::interface_hierarchy!(ICanHandleException, windows_core::IUnknown);
impl ICanHandleException {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CanHandleException(&self, pexcepinfo: *const super::oaidl::EXCEPINFO, pvar: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CanHandleException)(windows_core::Interface::as_raw(self), core::mem::transmute(pexcepinfo), core::mem::transmute(pvar)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICanHandleException_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub CanHandleException: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::EXCEPINFO, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    CanHandleException: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ICanHandleException_Impl: windows_core::IUnknownImpl {
    fn CanHandleException(&self, pexcepinfo: *const super::oaidl::EXCEPINFO, pvar: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ICanHandleException_Vtbl {
    pub const fn new<Identity: ICanHandleException_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CanHandleException<Identity: ICanHandleException_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pexcepinfo: *const super::oaidl::EXCEPINFO, pvar: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ICanHandleException {}
windows_core::imp::define_interface!(IDispError, IDispError_Vtbl, 0xa6ef9861_c720_11d0_9337_00a0c90dcaa9);
windows_core::imp::interface_hierarchy!(IDispError, windows_core::IUnknown);
impl IDispError {
    pub unsafe fn QueryErrorInfo(&self, guiderrortype: windows_core::GUID) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryErrorInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(guiderrortype), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNext(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHresult(&self) -> windows_core::Result<windows_core::HRESULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHresult)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSource(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSource)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetHelpInfo(&self, pbstrfilename: *mut windows_core::BSTR, pdwcontext: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetHelpInfo)(windows_core::Interface::as_raw(self), core::mem::transmute(pbstrfilename), pdwcontext as _) }
    }
    pub unsafe fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
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
    pub GetHelpInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IDispError_Impl: windows_core::IUnknownImpl {
    fn QueryErrorInfo(&self, guiderrortype: &windows_core::GUID) -> windows_core::Result<IDispError>;
    fn GetNext(&self) -> windows_core::Result<IDispError>;
    fn GetHresult(&self) -> windows_core::Result<windows_core::HRESULT>;
    fn GetSource(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetHelpInfo(&self, pbstrfilename: *mut windows_core::BSTR, pdwcontext: *mut u32) -> windows_core::Result<()>;
    fn GetDescription(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IDispError_Vtbl {
    pub const fn new<Identity: IDispError_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn QueryErrorInfo<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guiderrortype: windows_core::GUID, ppde: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispError_Impl::QueryErrorInfo(this, core::mem::transmute(&guiderrortype)) {
                    Ok(ok__) => {
                        ppde.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNext<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppde: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispError_Impl::GetNext(this) {
                    Ok(ok__) => {
                        ppde.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHresult<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispError_Impl::GetHresult(this) {
                    Ok(ok__) => {
                        phr.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSource<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispError_Impl::GetSource(this) {
                    Ok(ok__) => {
                        pbstrsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHelpInfo<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void, pdwcontext: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispError_Impl::GetHelpInfo(this, core::mem::transmute_copy(&pbstrfilename), core::mem::transmute_copy(&pdwcontext)).into()
            }
        }
        unsafe extern "system" fn GetDescription<Identity: IDispError_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispError_Impl::GetDescription(this) {
                    Ok(ok__) => {
                        pbstrdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
impl windows_core::RuntimeName for IDispError {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDispatchEx, IDispatchEx_Vtbl, 0xa6ef9860_c720_11d0_9337_00a0c90dcaa9);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDispatchEx {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDispatchEx, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IDispatchEx {
    pub unsafe fn GetDispID(&self, bstrname: &windows_core::BSTR, grfdex: u32) -> windows_core::Result<super::oaidl::DISPID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDispID)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), grfdex, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "servprov", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn InvokeEx<P6>(&self, id: super::oaidl::DISPID, lcid: super::winnt::LCID, wflags: u16, pdp: *const super::oaidl::DISPPARAMS, pvarres: Option<*mut super::oaidl::VARIANT>, pei: Option<*mut super::oaidl::EXCEPINFO>, pspcaller: P6) -> windows_core::HRESULT
    where
        P6: windows_core::Param<super::servprov::IServiceProvider>,
    {
        unsafe { (windows_core::Interface::vtable(self).InvokeEx)(windows_core::Interface::as_raw(self), id, lcid, wflags, pdp, pvarres.unwrap_or(core::mem::zeroed()) as _, pei.unwrap_or(core::mem::zeroed()) as _, pspcaller.param().abi()) }
    }
    pub unsafe fn DeleteMemberByName(&self, bstrname: &windows_core::BSTR, grfdex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteMemberByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(bstrname), grfdex) }
    }
    pub unsafe fn DeleteMemberByDispID(&self, id: super::oaidl::DISPID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteMemberByDispID)(windows_core::Interface::as_raw(self), id) }
    }
    pub unsafe fn GetMemberProperties(&self, id: super::oaidl::DISPID, grfdexfetch: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMemberProperties)(windows_core::Interface::as_raw(self), id, grfdexfetch, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMemberName(&self, id: super::oaidl::DISPID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMemberName)(windows_core::Interface::as_raw(self), id, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetNextDispID(&self, grfdex: u32, id: super::oaidl::DISPID) -> windows_core::Result<super::oaidl::DISPID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextDispID)(windows_core::Interface::as_raw(self), grfdex, id, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNameSpaceParent(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNameSpaceParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDispatchEx_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub GetDispID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut super::oaidl::DISPID) -> windows_core::HRESULT,
    #[cfg(all(feature = "servprov", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub InvokeEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, super::winnt::LCID, u16, *const super::oaidl::DISPPARAMS, *mut super::oaidl::VARIANT, *mut super::oaidl::EXCEPINFO, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "servprov", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    InvokeEx: usize,
    pub DeleteMemberByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeleteMemberByDispID: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID) -> windows_core::HRESULT,
    pub GetMemberProperties: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, u32, *mut u32) -> windows_core::HRESULT,
    pub GetMemberName: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::DISPID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextDispID: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::oaidl::DISPID, *mut super::oaidl::DISPID) -> windows_core::HRESULT,
    pub GetNameSpaceParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "servprov", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDispatchEx_Impl: super::oaidl::IDispatch_Impl {
    fn GetDispID(&self, bstrname: &windows_core::BSTR, grfdex: u32) -> windows_core::Result<super::oaidl::DISPID>;
    fn InvokeEx(&self, id: super::oaidl::DISPID, lcid: super::winnt::LCID, wflags: u16, pdp: *const super::oaidl::DISPPARAMS, pvarres: *mut super::oaidl::VARIANT, pei: *mut super::oaidl::EXCEPINFO, pspcaller: windows_core::Ref<super::servprov::IServiceProvider>) -> windows_core::Result<()>;
    fn DeleteMemberByName(&self, bstrname: &windows_core::BSTR, grfdex: u32) -> windows_core::Result<()>;
    fn DeleteMemberByDispID(&self, id: super::oaidl::DISPID) -> windows_core::Result<()>;
    fn GetMemberProperties(&self, id: super::oaidl::DISPID, grfdexfetch: u32) -> windows_core::Result<u32>;
    fn GetMemberName(&self, id: super::oaidl::DISPID) -> windows_core::Result<windows_core::BSTR>;
    fn GetNextDispID(&self, grfdex: u32, id: super::oaidl::DISPID) -> windows_core::Result<super::oaidl::DISPID>;
    fn GetNameSpaceParent(&self) -> windows_core::Result<windows_core::IUnknown>;
}
#[cfg(all(feature = "oaidl", feature = "servprov", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDispatchEx_Vtbl {
    pub const fn new<Identity: IDispatchEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDispID<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, grfdex: u32, pid: *mut super::oaidl::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatchEx_Impl::GetDispID(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&grfdex)) {
                    Ok(ok__) => {
                        pid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InvokeEx<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::oaidl::DISPID, lcid: super::winnt::LCID, wflags: u16, pdp: *const super::oaidl::DISPPARAMS, pvarres: *mut super::oaidl::VARIANT, pei: *mut super::oaidl::EXCEPINFO, pspcaller: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::InvokeEx(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&lcid), core::mem::transmute_copy(&wflags), core::mem::transmute_copy(&pdp), core::mem::transmute_copy(&pvarres), core::mem::transmute_copy(&pei), core::mem::transmute_copy(&pspcaller)).into()
            }
        }
        unsafe extern "system" fn DeleteMemberByName<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bstrname: *mut core::ffi::c_void, grfdex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::DeleteMemberByName(this, core::mem::transmute(&bstrname), core::mem::transmute_copy(&grfdex)).into()
            }
        }
        unsafe extern "system" fn DeleteMemberByDispID<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::oaidl::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDispatchEx_Impl::DeleteMemberByDispID(this, core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn GetMemberProperties<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::oaidl::DISPID, grfdexfetch: u32, pgrfdex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatchEx_Impl::GetMemberProperties(this, core::mem::transmute_copy(&id), core::mem::transmute_copy(&grfdexfetch)) {
                    Ok(ok__) => {
                        pgrfdex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMemberName<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: super::oaidl::DISPID, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatchEx_Impl::GetMemberName(this, core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextDispID<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, grfdex: u32, id: super::oaidl::DISPID, pid: *mut super::oaidl::DISPID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatchEx_Impl::GetNextDispID(this, core::mem::transmute_copy(&grfdex), core::mem::transmute_copy(&id)) {
                    Ok(ok__) => {
                        pid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNameSpaceParent<Identity: IDispatchEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDispatchEx_Impl::GetNameSpaceParent(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IDispatchEx as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "servprov", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDispatchEx {}
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
    #[cfg(feature = "wtypes")]
    pub unsafe fn GetCurrentSourceContext(&self, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentSourceContext)(windows_core::Interface::as_raw(self), pdwcontext as _, pfexecutingglobalcode as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProvideRuntimeContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub GetCurrentSourceContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut usize, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    GetCurrentSourceContext: usize,
}
#[cfg(feature = "wtypes")]
pub trait IProvideRuntimeContext_Impl: windows_core::IUnknownImpl {
    fn GetCurrentSourceContext(&self, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IProvideRuntimeContext_Vtbl {
    pub const fn new<Identity: IProvideRuntimeContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentSourceContext<Identity: IProvideRuntimeContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcontext: *mut usize, pfexecutingglobalcode: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
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
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IProvideRuntimeContext {}
windows_core::imp::define_interface!(IVariantChangeType, IVariantChangeType_Vtbl, 0xa6ef9862_c720_11d0_9337_00a0c90dcaa9);
windows_core::imp::interface_hierarchy!(IVariantChangeType, windows_core::IUnknown);
impl IVariantChangeType {
    #[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn ChangeType(&self, pvardst: *mut super::oaidl::VARIANT, pvarsrc: *const super::oaidl::VARIANT, lcid: super::winnt::LCID, vtnew: super::wtypes::VARTYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ChangeType)(windows_core::Interface::as_raw(self), core::mem::transmute(pvardst), core::mem::transmute(pvarsrc), lcid, vtnew) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariantChangeType_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub ChangeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT, *const super::oaidl::VARIANT, super::winnt::LCID, super::wtypes::VARTYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    ChangeType: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IVariantChangeType_Impl: windows_core::IUnknownImpl {
    fn ChangeType(&self, pvardst: *mut super::oaidl::VARIANT, pvarsrc: *const super::oaidl::VARIANT, lcid: super::winnt::LCID, vtnew: super::wtypes::VARTYPE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IVariantChangeType_Vtbl {
    pub const fn new<Identity: IVariantChangeType_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ChangeType<Identity: IVariantChangeType_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvardst: *mut super::oaidl::VARIANT, pvarsrc: *const super::oaidl::VARIANT, lcid: super::winnt::LCID, vtnew: super::wtypes::VARTYPE) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IVariantChangeType {}
pub const SID_GetCaller: windows_core::GUID = windows_core::GUID::from_u128(0x4717cc40_bcb9_11d0_9336_00a0c90dcaa9);
pub const SID_ProvideRuntimeContext: windows_core::GUID = windows_core::GUID::from_u128(0x74a5040c_dd0c_48f0_ac85_194c3259180a);
pub const SID_VariantConversion: windows_core::GUID = windows_core::GUID::from_u128(0x1f101481_bccd_11d0_9336_00a0c90dcaa9);
pub const fdexEnumAll: u32 = 2;
pub const fdexEnumDefault: u32 = 1;
pub const fdexNameCaseInsensitive: u32 = 8;
pub const fdexNameCaseSensitive: u32 = 1;
pub const fdexNameEnsure: u32 = 2;
pub const fdexNameImplicit: u32 = 4;
pub const fdexNameInternal: u32 = 16;
pub const fdexNameNoDynamicProperties: u32 = 32;
pub const fdexPropCanCall: u32 = 256;
pub const fdexPropCanConstruct: u32 = 1024;
pub const fdexPropCanGet: u32 = 1;
pub const fdexPropCanPut: u32 = 4;
pub const fdexPropCanPutRef: u32 = 16;
pub const fdexPropCanSourceEvents: u32 = 4096;
pub const fdexPropCannotCall: u32 = 512;
pub const fdexPropCannotConstruct: u32 = 2048;
pub const fdexPropCannotGet: u32 = 2;
pub const fdexPropCannotPut: u32 = 8;
pub const fdexPropCannotPutRef: u32 = 32;
pub const fdexPropCannotSourceEvents: u32 = 8192;
pub const fdexPropDynamicType: u32 = 128;
pub const fdexPropNoSideEffects: u32 = 64;
pub const grfdexPropAll: u32 = 16383;
pub const grfdexPropCanAll: u32 = 5397;
pub const grfdexPropCannotAll: u32 = 10794;
pub const grfdexPropExtraAll: u32 = 192;
