windows_core::imp::define_interface!(IWbemPath, IWbemPath_Vtbl, 0x3bc15af2_736c_477e_9e51_238af8667dcc);
windows_core::imp::interface_hierarchy!(IWbemPath, windows_core::IUnknown);
impl IWbemPath {
    pub unsafe fn SetText<P1>(&self, umode: u32, pszpath: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetText)(windows_core::Interface::as_raw(self), umode, pszpath.param().abi()) }
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), lflags, pubufflength as _, core::mem::transmute(psztext)) }
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), urequestedinfo, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetServer<P0>(&self, name: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServer)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetServer(&self, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetServer)(windows_core::Interface::as_raw(self), punamebuflength as _, core::mem::transmute(pname)) }
    }
    pub unsafe fn GetNamespaceCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNamespaceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNamespaceAt<P1>(&self, uindex: u32, pszname: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNamespaceAt)(windows_core::Interface::as_raw(self), uindex, pszname.param().abi()) }
    }
    pub unsafe fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNamespaceAt)(windows_core::Interface::as_raw(self), uindex, punamebuflength as _, core::mem::transmute(pname)) }
    }
    pub unsafe fn RemoveNamespaceAt(&self, uindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveNamespaceAt)(windows_core::Interface::as_raw(self), uindex) }
    }
    pub unsafe fn RemoveAllNamespaces(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllNamespaces)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetScopeCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetScopeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetScope<P1>(&self, uindex: u32, pszclass: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScope)(windows_core::Interface::as_raw(self), uindex, pszclass.param().abi()) }
    }
    pub unsafe fn SetScopeFromText<P1>(&self, uindex: u32, psztext: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetScopeFromText)(windows_core::Interface::as_raw(self), uindex, psztext.param().abi()) }
    }
    pub unsafe fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: *mut Option<IWbemPathKeyList>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScope)(windows_core::Interface::as_raw(self), uindex, puclassnamebufsize as _, core::mem::transmute(pszclass), core::mem::transmute(pkeylist)) }
    }
    pub unsafe fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetScopeAsText)(windows_core::Interface::as_raw(self), uindex, putextbufsize as _, core::mem::transmute(psztext)) }
    }
    pub unsafe fn RemoveScope(&self, uindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveScope)(windows_core::Interface::as_raw(self), uindex) }
    }
    pub unsafe fn RemoveAllScopes(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllScopes)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetClassName<P0>(&self, name: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClassName)(windows_core::Interface::as_raw(self), name.param().abi()) }
    }
    pub unsafe fn GetClassName(&self, pubufflength: *mut u32, pszname: Option<windows_core::PWSTR>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetClassName)(windows_core::Interface::as_raw(self), pubufflength as _, pszname.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn GetKeyList(&self) -> windows_core::Result<IWbemPathKeyList> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetKeyList)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateClassPart<P1>(&self, lflags: i32, name: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateClassPart)(windows_core::Interface::as_raw(self), lflags, name.param().abi()) }
    }
    pub unsafe fn DeleteClassPart(&self, lflags: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteClassPart)(windows_core::Interface::as_raw(self), lflags) }
    }
    pub unsafe fn IsRelative<P0, P1>(&self, wszmachine: P0, wsznamespace: P1) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsRelative)(windows_core::Interface::as_raw(self), wszmachine.param().abi(), wsznamespace.param().abi()) }
    }
    pub unsafe fn IsRelativeOrChild<P0, P1>(&self, wszmachine: P0, wsznamespace: P1, lflags: i32) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsRelativeOrChild)(windows_core::Interface::as_raw(self), wszmachine.param().abi(), wsznamespace.param().abi(), lflags) }
    }
    pub unsafe fn IsLocal<P0>(&self, wszmachine: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsLocal)(windows_core::Interface::as_raw(self), wszmachine.param().abi()) }
    }
    pub unsafe fn IsSameClassName<P0>(&self, wszclass: P0) -> windows_core::BOOL
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsSameClassName)(windows_core::Interface::as_raw(self), wszclass.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPath_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
    pub SetServer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetNamespaceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNamespaceAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetNamespaceAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub RemoveNamespaceAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAllNamespaces: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetScopeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetScope: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetScopeFromText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetScope: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetScopeAsText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub RemoveScope: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub RemoveAllScopes: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetClassName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetKeyList: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateClassPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub DeleteClassPart: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub IsRelative: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::BOOL,
    pub IsRelativeOrChild: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32) -> windows_core::BOOL,
    pub IsLocal: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::BOOL,
    pub IsSameClassName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::BOOL,
}
pub trait IWbemPath_Impl: windows_core::IUnknownImpl {
    fn SetText(&self, umode: u32, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64>;
    fn SetServer(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetServer(&self, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetNamespaceCount(&self) -> windows_core::Result<u32>;
    fn SetNamespaceAt(&self, uindex: u32, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetNamespaceAt(&self, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RemoveNamespaceAt(&self, uindex: u32) -> windows_core::Result<()>;
    fn RemoveAllNamespaces(&self) -> windows_core::Result<()>;
    fn GetScopeCount(&self) -> windows_core::Result<u32>;
    fn SetScope(&self, uindex: u32, pszclass: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetScopeFromText(&self, uindex: u32, psztext: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetScope(&self, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: windows_core::OutRef<IWbemPathKeyList>) -> windows_core::Result<()>;
    fn GetScopeAsText(&self, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
    fn RemoveScope(&self, uindex: u32) -> windows_core::Result<()>;
    fn RemoveAllScopes(&self) -> windows_core::Result<()>;
    fn SetClassName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetClassName(&self, pubufflength: *mut u32, pszname: windows_core::PWSTR) -> windows_core::Result<()>;
    fn GetKeyList(&self) -> windows_core::Result<IWbemPathKeyList>;
    fn CreateClassPart(&self, lflags: i32, name: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn DeleteClassPart(&self, lflags: i32) -> windows_core::Result<()>;
    fn IsRelative(&self, wszmachine: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR) -> windows_core::BOOL;
    fn IsRelativeOrChild(&self, wszmachine: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR, lflags: i32) -> windows_core::BOOL;
    fn IsLocal(&self, wszmachine: &windows_core::PCWSTR) -> windows_core::BOOL;
    fn IsSameClassName(&self, wszclass: &windows_core::PCWSTR) -> windows_core::BOOL;
}
impl IWbemPath_Vtbl {
    pub const fn new<Identity: IWbemPath_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, umode: u32, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetText(this, core::mem::transmute_copy(&umode), core::mem::transmute(&pszpath)).into()
            }
        }
        unsafe extern "system" fn GetText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&psztext)).into()
            }
        }
        unsafe extern "system" fn GetInfo<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPath_Impl::GetInfo(this, core::mem::transmute_copy(&urequestedinfo)) {
                    Ok(ok__) => {
                        puresponse.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServer<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetServer(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn GetServer<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetServer(this, core::mem::transmute_copy(&punamebuflength), core::mem::transmute_copy(&pname)).into()
            }
        }
        unsafe extern "system" fn GetNamespaceCount<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPath_Impl::GetNamespaceCount(this) {
                    Ok(ok__) => {
                        pucount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNamespaceAt<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetNamespaceAt(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetNamespaceAt<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, punamebuflength: *mut u32, pname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetNamespaceAt(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&punamebuflength), core::mem::transmute_copy(&pname)).into()
            }
        }
        unsafe extern "system" fn RemoveNamespaceAt<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::RemoveNamespaceAt(this, core::mem::transmute_copy(&uindex)).into()
            }
        }
        unsafe extern "system" fn RemoveAllNamespaces<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::RemoveAllNamespaces(this).into()
            }
        }
        unsafe extern "system" fn GetScopeCount<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pucount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPath_Impl::GetScopeCount(this) {
                    Ok(ok__) => {
                        pucount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetScope<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, pszclass: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetScope(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&pszclass)).into()
            }
        }
        unsafe extern "system" fn SetScopeFromText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, psztext: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetScopeFromText(this, core::mem::transmute_copy(&uindex), core::mem::transmute(&psztext)).into()
            }
        }
        unsafe extern "system" fn GetScope<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, puclassnamebufsize: *mut u32, pszclass: windows_core::PWSTR, pkeylist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetScope(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&puclassnamebufsize), core::mem::transmute_copy(&pszclass), core::mem::transmute_copy(&pkeylist)).into()
            }
        }
        unsafe extern "system" fn GetScopeAsText<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32, putextbufsize: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetScopeAsText(this, core::mem::transmute_copy(&uindex), core::mem::transmute_copy(&putextbufsize), core::mem::transmute_copy(&psztext)).into()
            }
        }
        unsafe extern "system" fn RemoveScope<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::RemoveScope(this, core::mem::transmute_copy(&uindex)).into()
            }
        }
        unsafe extern "system" fn RemoveAllScopes<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::RemoveAllScopes(this).into()
            }
        }
        unsafe extern "system" fn SetClassName<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::SetClassName(this, core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn GetClassName<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pubufflength: *mut u32, pszname: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::GetClassName(this, core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetKeyList<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPath_Impl::GetKeyList(this) {
                    Ok(ok__) => {
                        pout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateClassPart<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, name: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::CreateClassPart(this, core::mem::transmute_copy(&lflags), core::mem::transmute(&name)).into()
            }
        }
        unsafe extern "system" fn DeleteClassPart<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::DeleteClassPart(this, core::mem::transmute_copy(&lflags)).into()
            }
        }
        unsafe extern "system" fn IsRelative<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::IsRelative(this, core::mem::transmute(&wszmachine), core::mem::transmute(&wsznamespace))
            }
        }
        unsafe extern "system" fn IsRelativeOrChild<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR, lflags: i32) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::IsRelativeOrChild(this, core::mem::transmute(&wszmachine), core::mem::transmute(&wsznamespace), core::mem::transmute_copy(&lflags))
            }
        }
        unsafe extern "system" fn IsLocal<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmachine: windows_core::PCWSTR) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::IsLocal(this, core::mem::transmute(&wszmachine))
            }
        }
        unsafe extern "system" fn IsSameClassName<Identity: IWbemPath_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszclass: windows_core::PCWSTR) -> windows_core::BOOL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPath_Impl::IsSameClassName(this, core::mem::transmute(&wszclass))
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetText: SetText::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
            SetServer: SetServer::<Identity, OFFSET>,
            GetServer: GetServer::<Identity, OFFSET>,
            GetNamespaceCount: GetNamespaceCount::<Identity, OFFSET>,
            SetNamespaceAt: SetNamespaceAt::<Identity, OFFSET>,
            GetNamespaceAt: GetNamespaceAt::<Identity, OFFSET>,
            RemoveNamespaceAt: RemoveNamespaceAt::<Identity, OFFSET>,
            RemoveAllNamespaces: RemoveAllNamespaces::<Identity, OFFSET>,
            GetScopeCount: GetScopeCount::<Identity, OFFSET>,
            SetScope: SetScope::<Identity, OFFSET>,
            SetScopeFromText: SetScopeFromText::<Identity, OFFSET>,
            GetScope: GetScope::<Identity, OFFSET>,
            GetScopeAsText: GetScopeAsText::<Identity, OFFSET>,
            RemoveScope: RemoveScope::<Identity, OFFSET>,
            RemoveAllScopes: RemoveAllScopes::<Identity, OFFSET>,
            SetClassName: SetClassName::<Identity, OFFSET>,
            GetClassName: GetClassName::<Identity, OFFSET>,
            GetKeyList: GetKeyList::<Identity, OFFSET>,
            CreateClassPart: CreateClassPart::<Identity, OFFSET>,
            DeleteClassPart: DeleteClassPart::<Identity, OFFSET>,
            IsRelative: IsRelative::<Identity, OFFSET>,
            IsRelativeOrChild: IsRelativeOrChild::<Identity, OFFSET>,
            IsLocal: IsLocal::<Identity, OFFSET>,
            IsSameClassName: IsSameClassName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPath as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemPath {}
windows_core::imp::define_interface!(IWbemPathKeyList, IWbemPathKeyList_Vtbl, 0x9ae62877_7544_4bb0_aa26_a13824659ed6);
windows_core::imp::interface_hierarchy!(IWbemPathKeyList, windows_core::IUnknown);
impl IWbemPathKeyList {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetKey<P0>(&self, wszname: P0, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetKey)(windows_core::Interface::as_raw(self), wszname.param().abi(), uflags, ucimtype, pkeyval) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn SetKey2<P0>(&self, wszname: P0, uflags: u32, ucimtype: u32, pkeyval: *const super::oaidl::VARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetKey2)(windows_core::Interface::as_raw(self), wszname.param().abi(), uflags, ucimtype, core::mem::transmute(pkeyval)) }
    }
    pub unsafe fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: Option<windows_core::PWSTR>, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKey)(windows_core::Interface::as_raw(self), ukeyix, uflags, punamebufsize as _, pszkeyname.unwrap_or(core::mem::zeroed()) as _, pukeyvalbufsize as _, pkeyval as _, puapparentcimtype as _) }
    }
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: Option<windows_core::PWSTR>, pkeyvalue: *mut super::oaidl::VARIANT, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetKey2)(windows_core::Interface::as_raw(self), ukeyix, uflags, punamebufsize as _, pszkeyname.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(pkeyvalue), puapparentcimtype as _) }
    }
    pub unsafe fn RemoveKey<P0>(&self, wszname: P0, uflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveKey)(windows_core::Interface::as_raw(self), wszname.param().abi(), uflags) }
    }
    pub unsafe fn RemoveAllKeys(&self, uflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllKeys)(windows_core::Interface::as_raw(self), uflags) }
    }
    #[cfg(feature = "Win32_rpcndr")]
    pub unsafe fn MakeSingleton(&self, bset: super::rpcndr::boolean) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MakeSingleton)(windows_core::Interface::as_raw(self), bset) }
    }
    pub unsafe fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInfo)(windows_core::Interface::as_raw(self), urequestedinfo, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), lflags, pubufflength as _, core::mem::transmute(psztext)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemPathKeyList_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *const core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub SetKey2: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *const super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    SetKey2: usize,
    pub GetKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, windows_core::PWSTR, *mut u32, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetKey2: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u32, windows_core::PWSTR, *mut super::oaidl::VARIANT, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_oaidl", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetKey2: usize,
    pub RemoveKey: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub RemoveAllKeys: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_rpcndr")]
    pub MakeSingleton: unsafe extern "system" fn(*mut core::ffi::c_void, super::rpcndr::boolean) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpcndr"))]
    MakeSingleton: usize,
    pub GetInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u64) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut u32, windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait IWbemPathKeyList_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn SetKey(&self, wszname: &windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn SetKey2(&self, wszname: &windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn GetKey(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::Result<()>;
    fn GetKey2(&self, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pkeyvalue: *mut super::oaidl::VARIANT, puapparentcimtype: *mut u32) -> windows_core::Result<()>;
    fn RemoveKey(&self, wszname: &windows_core::PCWSTR, uflags: u32) -> windows_core::Result<()>;
    fn RemoveAllKeys(&self, uflags: u32) -> windows_core::Result<()>;
    fn MakeSingleton(&self, bset: super::rpcndr::boolean) -> windows_core::Result<()>;
    fn GetInfo(&self, urequestedinfo: u32) -> windows_core::Result<u64>;
    fn GetText(&self, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl IWbemPathKeyList_Vtbl {
    pub const fn new<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pukeycount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPathKeyList_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pukeycount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetKey<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::SetKey(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ucimtype), core::mem::transmute_copy(&pkeyval)).into()
            }
        }
        unsafe extern "system" fn SetKey2<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32, ucimtype: u32, pkeyval: *const super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::SetKey2(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&ucimtype), core::mem::transmute_copy(&pkeyval)).into()
            }
        }
        unsafe extern "system" fn GetKey<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pukeyvalbufsize: *mut u32, pkeyval: *mut core::ffi::c_void, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::GetKey(this, core::mem::transmute_copy(&ukeyix), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&punamebufsize), core::mem::transmute_copy(&pszkeyname), core::mem::transmute_copy(&pukeyvalbufsize), core::mem::transmute_copy(&pkeyval), core::mem::transmute_copy(&puapparentcimtype)).into()
            }
        }
        unsafe extern "system" fn GetKey2<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ukeyix: u32, uflags: u32, punamebufsize: *mut u32, pszkeyname: windows_core::PWSTR, pkeyvalue: *mut super::oaidl::VARIANT, puapparentcimtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::GetKey2(this, core::mem::transmute_copy(&ukeyix), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&punamebufsize), core::mem::transmute_copy(&pszkeyname), core::mem::transmute_copy(&pkeyvalue), core::mem::transmute_copy(&puapparentcimtype)).into()
            }
        }
        unsafe extern "system" fn RemoveKey<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszname: windows_core::PCWSTR, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::RemoveKey(this, core::mem::transmute(&wszname), core::mem::transmute_copy(&uflags)).into()
            }
        }
        unsafe extern "system" fn RemoveAllKeys<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::RemoveAllKeys(this, core::mem::transmute_copy(&uflags)).into()
            }
        }
        unsafe extern "system" fn MakeSingleton<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bset: super::rpcndr::boolean) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::MakeSingleton(this, core::mem::transmute_copy(&bset)).into()
            }
        }
        unsafe extern "system" fn GetInfo<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, urequestedinfo: u32, puresponse: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemPathKeyList_Impl::GetInfo(this, core::mem::transmute_copy(&urequestedinfo)) {
                    Ok(ok__) => {
                        puresponse.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetText<Identity: IWbemPathKeyList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, pubufflength: *mut u32, psztext: windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemPathKeyList_Impl::GetText(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pubufflength), core::mem::transmute_copy(&psztext)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            SetKey: SetKey::<Identity, OFFSET>,
            SetKey2: SetKey2::<Identity, OFFSET>,
            GetKey: GetKey::<Identity, OFFSET>,
            GetKey2: GetKey2::<Identity, OFFSET>,
            RemoveKey: RemoveKey::<Identity, OFFSET>,
            RemoveAllKeys: RemoveAllKeys::<Identity, OFFSET>,
            MakeSingleton: MakeSingleton::<Identity, OFFSET>,
            GetInfo: GetInfo::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemPathKeyList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_oaidl", feature = "Win32_rpcndr", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for IWbemPathKeyList {}
windows_core::imp::define_interface!(IWbemQuery, IWbemQuery_Vtbl, 0x81166f58_dd98_11d3_a120_00105a1f515a);
windows_core::imp::interface_hierarchy!(IWbemQuery, windows_core::IUnknown);
impl IWbemQuery {
    pub unsafe fn Empty(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Empty)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetLanguageFeatures)(windows_core::Interface::as_raw(self), uflags, uarraysize, pufeatures) }
    }
    pub unsafe fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TestLanguageFeatures)(windows_core::Interface::as_raw(self), uflags, uarraysize as _, pufeatures as _) }
    }
    pub unsafe fn Parse<P0, P1>(&self, pszlang: P0, pszquery: P1, uflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Parse)(windows_core::Interface::as_raw(self), pszlang.param().abi(), pszquery.param().abi(), uflags) }
    }
    pub unsafe fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAnalysis)(windows_core::Interface::as_raw(self), uanalysistype, uflags, panalysis as _) }
    }
    pub unsafe fn FreeMemory(&self, pmem: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FreeMemory)(windows_core::Interface::as_raw(self), pmem) }
    }
    pub unsafe fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetQueryInfo)(windows_core::Interface::as_raw(self), uanalysistype, uinfoid, ubufsize, pdestbuf as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemQuery_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Empty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetLanguageFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u32) -> windows_core::HRESULT,
    pub TestLanguageFeatures: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut u32) -> windows_core::HRESULT,
    pub Parse: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetAnalysis: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FreeMemory: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
    pub GetQueryInfo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemQuery_Impl: windows_core::IUnknownImpl {
    fn Empty(&self) -> windows_core::Result<()>;
    fn SetLanguageFeatures(&self, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::Result<()>;
    fn TestLanguageFeatures(&self, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::Result<()>;
    fn Parse(&self, pszlang: &windows_core::PCWSTR, pszquery: &windows_core::PCWSTR, uflags: u32) -> windows_core::Result<()>;
    fn GetAnalysis(&self, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn FreeMemory(&self, pmem: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetQueryInfo(&self, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IWbemQuery_Vtbl {
    pub const fn new<Identity: IWbemQuery_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Empty<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::Empty(this).into()
            }
        }
        unsafe extern "system" fn SetLanguageFeatures<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, uarraysize: u32, pufeatures: *const u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::SetLanguageFeatures(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&uarraysize), core::mem::transmute_copy(&pufeatures)).into()
            }
        }
        unsafe extern "system" fn TestLanguageFeatures<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uflags: u32, uarraysize: *mut u32, pufeatures: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::TestLanguageFeatures(this, core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&uarraysize), core::mem::transmute_copy(&pufeatures)).into()
            }
        }
        unsafe extern "system" fn Parse<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlang: windows_core::PCWSTR, pszquery: windows_core::PCWSTR, uflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::Parse(this, core::mem::transmute(&pszlang), core::mem::transmute(&pszquery), core::mem::transmute_copy(&uflags)).into()
            }
        }
        unsafe extern "system" fn GetAnalysis<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uanalysistype: u32, uflags: u32, panalysis: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::GetAnalysis(this, core::mem::transmute_copy(&uanalysistype), core::mem::transmute_copy(&uflags), core::mem::transmute_copy(&panalysis)).into()
            }
        }
        unsafe extern "system" fn FreeMemory<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmem: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::FreeMemory(this, core::mem::transmute_copy(&pmem)).into()
            }
        }
        unsafe extern "system" fn GetQueryInfo<Identity: IWbemQuery_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uanalysistype: u32, uinfoid: u32, ubufsize: u32, pdestbuf: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemQuery_Impl::GetQueryInfo(this, core::mem::transmute_copy(&uanalysistype), core::mem::transmute_copy(&uinfoid), core::mem::transmute_copy(&ubufsize), core::mem::transmute_copy(&pdestbuf)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Empty: Empty::<Identity, OFFSET>,
            SetLanguageFeatures: SetLanguageFeatures::<Identity, OFFSET>,
            TestLanguageFeatures: TestLanguageFeatures::<Identity, OFFSET>,
            Parse: Parse::<Identity, OFFSET>,
            GetAnalysis: GetAnalysis::<Identity, OFFSET>,
            FreeMemory: FreeMemory::<Identity, OFFSET>,
            GetQueryInfo: GetQueryInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemQuery as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemQuery {}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SWbemAnalysisMatrix {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_pszProperty: windows_core::PCWSTR,
    pub m_uPropertyType: u32,
    pub m_uEntries: u32,
    pub m_pValues: *mut *mut core::ffi::c_void,
    pub m_pbTruthTable: *mut windows_core::BOOL,
}
impl Default for SWbemAnalysisMatrix {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SWbemAnalysisMatrixList {
    pub m_uVersion: u32,
    pub m_uMatrixType: u32,
    pub m_uNumMatrices: u32,
    pub m_pMatrices: *mut SWbemAnalysisMatrix,
}
impl Default for SWbemAnalysisMatrixList {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SWbemAssocQueryInf {
    pub m_uVersion: u32,
    pub m_uAnalysisType: u32,
    pub m_uFeatureMask: u32,
    pub m_pPath: core::mem::ManuallyDrop<Option<IWbemPath>>,
    pub m_pszPath: windows_core::PWSTR,
    pub m_pszQueryText: windows_core::PWSTR,
    pub m_pszResultClass: windows_core::PWSTR,
    pub m_pszAssocClass: windows_core::PWSTR,
    pub m_pszRole: windows_core::PWSTR,
    pub m_pszResultRole: windows_core::PWSTR,
    pub m_pszRequiredQualifier: windows_core::PWSTR,
    pub m_pszRequiredAssocQualifier: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SWbemQueryQualifiedName {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNameListSize: u32,
    pub m_ppszNameList: *mut windows_core::PCWSTR,
    pub m_bArraysUsed: windows_core::BOOL,
    pub m_pbArrayElUsed: *mut windows_core::BOOL,
    pub m_puArrayIndex: *mut u32,
}
impl Default for SWbemQueryQualifiedName {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SWbemRpnConst {
    pub m_pszStrVal: windows_core::PCWSTR,
    pub m_bBoolVal: windows_core::BOOL,
    pub m_lLongVal: i32,
    pub m_uLongVal: u32,
    pub m_dblVal: f64,
    pub m_lVal64: i64,
    pub m_uVal64: i64,
}
impl Default for SWbemRpnConst {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SWbemRpnEncodedQuery {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uParsedFeatureMask: u64,
    pub m_uDetectedArraySize: u32,
    pub m_puDetectedFeatures: *mut u32,
    pub m_uSelectListSize: u32,
    pub m_ppSelectList: *mut *mut SWbemQueryQualifiedName,
    pub m_uFromTargetType: u32,
    pub m_pszOptionalFromPath: windows_core::PCWSTR,
    pub m_uFromListSize: u32,
    pub m_ppszFromList: *mut windows_core::PCWSTR,
    pub m_uWhereClauseSize: u32,
    pub m_ppRpnWhereClause: *mut *mut SWbemRpnQueryToken,
    pub m_dblWithinPolling: f64,
    pub m_dblWithinWindow: f64,
    pub m_uOrderByListSize: u32,
    pub m_ppszOrderByList: *mut windows_core::PCWSTR,
    pub m_uOrderDirectionEl: *mut u32,
}
impl Default for SWbemRpnEncodedQuery {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SWbemRpnQueryToken {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uSubexpressionShape: u32,
    pub m_uOperator: u32,
    pub m_pRightIdent: *mut SWbemQueryQualifiedName,
    pub m_pLeftIdent: *mut SWbemQueryQualifiedName,
    pub m_uConstApparentType: u32,
    pub m_Const: SWbemRpnConst,
    pub m_uConst2ApparentType: u32,
    pub m_Const2: SWbemRpnConst,
    pub m_pszRightFunc: windows_core::PCWSTR,
    pub m_pszLeftFunc: windows_core::PCWSTR,
}
impl Default for SWbemRpnQueryToken {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SWbemRpnTokenList {
    pub m_uVersion: u32,
    pub m_uTokenType: u32,
    pub m_uNumTokens: u32,
}
pub const WBEMPATH_COMPRESSED: tag_WBEM_GET_TEXT_FLAGS = 1;
pub const WBEMPATH_CREATE_ACCEPT_ABSOLUTE: tag_WBEM_PATH_CREATE_FLAG = 2;
pub const WBEMPATH_CREATE_ACCEPT_ALL: tag_WBEM_PATH_CREATE_FLAG = 4;
pub const WBEMPATH_CREATE_ACCEPT_RELATIVE: tag_WBEM_PATH_CREATE_FLAG = 1;
pub const WBEMPATH_GET_NAMESPACE_ONLY: tag_WBEM_GET_TEXT_FLAGS = 16;
pub const WBEMPATH_GET_ORIGINAL: tag_WBEM_GET_TEXT_FLAGS = 32;
pub const WBEMPATH_GET_RELATIVE_ONLY: tag_WBEM_GET_TEXT_FLAGS = 2;
pub const WBEMPATH_GET_SERVER_AND_NAMESPACE_ONLY: tag_WBEM_GET_TEXT_FLAGS = 8;
pub const WBEMPATH_GET_SERVER_TOO: tag_WBEM_GET_TEXT_FLAGS = 4;
pub const WBEMPATH_INFO_ANON_LOCAL_MACHINE: tag_WBEM_PATH_STATUS_FLAG = 1;
pub const WBEMPATH_INFO_CIM_COMPLIANT: tag_WBEM_PATH_STATUS_FLAG = 2048;
pub const WBEMPATH_INFO_CONTAINS_SINGLETON: tag_WBEM_PATH_STATUS_FLAG = 256;
pub const WBEMPATH_INFO_HAS_IMPLIED_KEY: tag_WBEM_PATH_STATUS_FLAG = 128;
pub const WBEMPATH_INFO_HAS_MACHINE_NAME: tag_WBEM_PATH_STATUS_FLAG = 2;
pub const WBEMPATH_INFO_HAS_SUBSCOPES: tag_WBEM_PATH_STATUS_FLAG = 16;
pub const WBEMPATH_INFO_HAS_V2_REF_PATHS: tag_WBEM_PATH_STATUS_FLAG = 64;
pub const WBEMPATH_INFO_IS_CLASS_REF: tag_WBEM_PATH_STATUS_FLAG = 4;
pub const WBEMPATH_INFO_IS_COMPOUND: tag_WBEM_PATH_STATUS_FLAG = 32;
pub const WBEMPATH_INFO_IS_INST_REF: tag_WBEM_PATH_STATUS_FLAG = 8;
pub const WBEMPATH_INFO_IS_PARENT: tag_WBEM_PATH_STATUS_FLAG = 8192;
pub const WBEMPATH_INFO_IS_SINGLETON: tag_WBEM_PATH_STATUS_FLAG = 4096;
pub const WBEMPATH_INFO_NATIVE_PATH: tag_WBEM_PATH_STATUS_FLAG = 32768;
pub const WBEMPATH_INFO_PATH_HAD_SERVER: tag_WBEM_PATH_STATUS_FLAG = 131072;
pub const WBEMPATH_INFO_SERVER_NAMESPACE_ONLY: tag_WBEM_PATH_STATUS_FLAG = 16384;
pub const WBEMPATH_INFO_V1_COMPLIANT: tag_WBEM_PATH_STATUS_FLAG = 512;
pub const WBEMPATH_INFO_V2_COMPLIANT: tag_WBEM_PATH_STATUS_FLAG = 1024;
pub const WBEMPATH_INFO_WMI_PATH: tag_WBEM_PATH_STATUS_FLAG = 65536;
pub const WBEMPATH_QUOTEDTEXT: tag_WBEM_GET_KEY_FLAGS = 2;
pub const WBEMPATH_TEXT: tag_WBEM_GET_KEY_FLAGS = 1;
pub const WBEMPATH_TREAT_SINGLE_IDENT_AS_NS: tag_WBEM_PATH_CREATE_FLAG = 8;
pub const WMIQ_ANALYSIS_ASSOC_QUERY: WMIQ_ANALYSIS_TYPE = 2;
pub const WMIQ_ANALYSIS_PROP_ANALYSIS_MATRIX: WMIQ_ANALYSIS_TYPE = 3;
pub const WMIQ_ANALYSIS_QUERY_TEXT: WMIQ_ANALYSIS_TYPE = 4;
pub const WMIQ_ANALYSIS_RESERVED: WMIQ_ANALYSIS_TYPE = 134217728;
pub const WMIQ_ANALYSIS_RPN_SEQUENCE: WMIQ_ANALYSIS_TYPE = 1;
pub type WMIQ_ANALYSIS_TYPE = i32;
pub const WMIQ_ASSOCQ_ASSOCCLASS: WMIQ_ASSOCQ_FLAGS = 8;
pub const WMIQ_ASSOCQ_ASSOCIATORS: WMIQ_ASSOCQ_FLAGS = 1;
pub const WMIQ_ASSOCQ_CLASSDEFSONLY: WMIQ_ASSOCQ_FLAGS = 256;
pub const WMIQ_ASSOCQ_CLASSREFSONLY: WMIQ_ASSOCQ_FLAGS = 2048;
pub type WMIQ_ASSOCQ_FLAGS = i32;
pub const WMIQ_ASSOCQ_KEYSONLY: WMIQ_ASSOCQ_FLAGS = 512;
pub const WMIQ_ASSOCQ_REFERENCES: WMIQ_ASSOCQ_FLAGS = 2;
pub const WMIQ_ASSOCQ_REQUIREDASSOCQUALIFIER: WMIQ_ASSOCQ_FLAGS = 128;
pub const WMIQ_ASSOCQ_REQUIREDQUALIFIER: WMIQ_ASSOCQ_FLAGS = 64;
pub const WMIQ_ASSOCQ_RESULTCLASS: WMIQ_ASSOCQ_FLAGS = 4;
pub const WMIQ_ASSOCQ_RESULTROLE: WMIQ_ASSOCQ_FLAGS = 32;
pub const WMIQ_ASSOCQ_ROLE: WMIQ_ASSOCQ_FLAGS = 16;
pub const WMIQ_ASSOCQ_SCHEMAONLY: WMIQ_ASSOCQ_FLAGS = 1024;
pub type WMIQ_LANGUAGE_FEATURES = i32;
pub const WMIQ_LF10_COMPEX_SUBEXPRESSIONS: WMIQ_LANGUAGE_FEATURES = 10;
pub const WMIQ_LF11_ALIASING: WMIQ_LANGUAGE_FEATURES = 11;
pub const WMIQ_LF12_GROUP_BY_HAVING: WMIQ_LANGUAGE_FEATURES = 12;
pub const WMIQ_LF13_WMI_WITHIN: WMIQ_LANGUAGE_FEATURES = 13;
pub const WMIQ_LF14_SQL_WRITE_OPERATIONS: WMIQ_LANGUAGE_FEATURES = 14;
pub const WMIQ_LF15_GO: WMIQ_LANGUAGE_FEATURES = 15;
pub const WMIQ_LF16_SINGLE_LEVEL_TRANSACTIONS: WMIQ_LANGUAGE_FEATURES = 16;
pub const WMIQ_LF17_QUALIFIED_NAMES: WMIQ_LANGUAGE_FEATURES = 17;
pub const WMIQ_LF18_ASSOCIATONS: WMIQ_LANGUAGE_FEATURES = 18;
pub const WMIQ_LF19_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = 19;
pub const WMIQ_LF1_BASIC_SELECT: WMIQ_LANGUAGE_FEATURES = 1;
pub const WMIQ_LF20_EXTENDED_SYSTEM_PROPERTIES: WMIQ_LANGUAGE_FEATURES = 20;
pub const WMIQ_LF21_SQL89_JOINS: WMIQ_LANGUAGE_FEATURES = 21;
pub const WMIQ_LF22_SQL92_JOINS: WMIQ_LANGUAGE_FEATURES = 22;
pub const WMIQ_LF23_SUBSELECTS: WMIQ_LANGUAGE_FEATURES = 23;
pub const WMIQ_LF24_UMI_EXTENSIONS: WMIQ_LANGUAGE_FEATURES = 24;
pub const WMIQ_LF25_DATEPART: WMIQ_LANGUAGE_FEATURES = 25;
pub const WMIQ_LF26_LIKE: WMIQ_LANGUAGE_FEATURES = 26;
pub const WMIQ_LF27_CIM_TEMPORAL_CONSTRUCTS: WMIQ_LANGUAGE_FEATURES = 27;
pub const WMIQ_LF28_STANDARD_AGGREGATES: WMIQ_LANGUAGE_FEATURES = 28;
pub const WMIQ_LF29_MULTI_LEVEL_ORDER_BY: WMIQ_LANGUAGE_FEATURES = 29;
pub const WMIQ_LF2_CLASS_NAME_IN_QUERY: WMIQ_LANGUAGE_FEATURES = 2;
pub const WMIQ_LF30_WMI_PRAGMAS: WMIQ_LANGUAGE_FEATURES = 30;
pub const WMIQ_LF31_QUALIFIER_TESTS: WMIQ_LANGUAGE_FEATURES = 31;
pub const WMIQ_LF32_SP_EXECUTE: WMIQ_LANGUAGE_FEATURES = 32;
pub const WMIQ_LF33_ARRAY_ACCESS: WMIQ_LANGUAGE_FEATURES = 33;
pub const WMIQ_LF34_UNION: WMIQ_LANGUAGE_FEATURES = 34;
pub const WMIQ_LF35_COMPLEX_SELECT_TARGET: WMIQ_LANGUAGE_FEATURES = 35;
pub const WMIQ_LF36_REFERENCE_TESTS: WMIQ_LANGUAGE_FEATURES = 36;
pub const WMIQ_LF37_SELECT_INTO: WMIQ_LANGUAGE_FEATURES = 37;
pub const WMIQ_LF38_BASIC_DATETIME_TESTS: WMIQ_LANGUAGE_FEATURES = 38;
pub const WMIQ_LF39_COUNT_COLUMN: WMIQ_LANGUAGE_FEATURES = 39;
pub const WMIQ_LF3_STRING_CASE_FUNCTIONS: WMIQ_LANGUAGE_FEATURES = 3;
pub const WMIQ_LF40_BETWEEN: WMIQ_LANGUAGE_FEATURES = 40;
pub const WMIQ_LF4_PROP_TO_PROP_TESTS: WMIQ_LANGUAGE_FEATURES = 4;
pub const WMIQ_LF5_COUNT_STAR: WMIQ_LANGUAGE_FEATURES = 5;
pub const WMIQ_LF6_ORDER_BY: WMIQ_LANGUAGE_FEATURES = 6;
pub const WMIQ_LF7_DISTINCT: WMIQ_LANGUAGE_FEATURES = 7;
pub const WMIQ_LF8_ISA: WMIQ_LANGUAGE_FEATURES = 8;
pub const WMIQ_LF9_THIS: WMIQ_LANGUAGE_FEATURES = 9;
pub const WMIQ_LF_LAST: WMIQ_LANGUAGE_FEATURES = 40;
pub const WMIQ_RPNF_ARRAY_ACCESS_USED: WMIQ_RPNF_FEATURE = 8192;
pub const WMIQ_RPNF_COUNT_STAR: WMIQ_RPNF_FEATURE = 64;
pub const WMIQ_RPNF_EQUALITY_TESTS_ONLY: WMIQ_RPNF_FEATURE = 32;
pub type WMIQ_RPNF_FEATURE = i32;
pub const WMIQ_RPNF_FEATURE_SELECT_STAR: WMIQ_RPNF_FEATURE = 16;
pub const WMIQ_RPNF_GROUP_BY_HAVING: WMIQ_RPNF_FEATURE = 4096;
pub const WMIQ_RPNF_ISA_USED: WMIQ_RPNF_FEATURE = 2048;
pub const WMIQ_RPNF_ORDER_BY: WMIQ_RPNF_FEATURE = 1024;
pub const WMIQ_RPNF_PROJECTION: WMIQ_RPNF_FEATURE = 8;
pub const WMIQ_RPNF_PROP_TO_PROP_TESTS: WMIQ_RPNF_FEATURE = 512;
pub const WMIQ_RPNF_QUALIFIED_NAMES_USED: WMIQ_RPNF_FEATURE = 128;
pub const WMIQ_RPNF_QUERY_IS_CONJUNCTIVE: WMIQ_RPNF_FEATURE = 2;
pub const WMIQ_RPNF_QUERY_IS_DISJUNCTIVE: WMIQ_RPNF_FEATURE = 4;
pub const WMIQ_RPNF_SYSPROP_CLASS_USED: WMIQ_RPNF_FEATURE = 256;
pub const WMIQ_RPNF_WHERE_CLAUSE_PRESENT: WMIQ_RPNF_FEATURE = 1;
pub const WMIQ_RPN_CONST: WMIQ_RPN_TOKEN_FLAGS = 8;
pub const WMIQ_RPN_CONST2: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_FROM_CLASS_LIST: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_FROM_MULTIPLE: WMIQ_RPN_TOKEN_FLAGS = 8;
pub const WMIQ_RPN_FROM_PATH: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_FROM_UNARY: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_GET_EXPR_SHAPE: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_GET_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 3;
pub const WMIQ_RPN_GET_RELOP: WMIQ_RPN_TOKEN_FLAGS = 5;
pub const WMIQ_RPN_GET_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_GET_TOKEN_TYPE: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_LEFT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 32;
pub const WMIQ_RPN_LEFT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_NEXT_TOKEN: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_OP_EQ: WMIQ_RPN_TOKEN_FLAGS = 1;
pub const WMIQ_RPN_OP_GE: WMIQ_RPN_TOKEN_FLAGS = 3;
pub const WMIQ_RPN_OP_GT: WMIQ_RPN_TOKEN_FLAGS = 6;
pub const WMIQ_RPN_OP_ISA: WMIQ_RPN_TOKEN_FLAGS = 8;
pub const WMIQ_RPN_OP_ISNOTA: WMIQ_RPN_TOKEN_FLAGS = 9;
pub const WMIQ_RPN_OP_ISNOTNULL: WMIQ_RPN_TOKEN_FLAGS = 11;
pub const WMIQ_RPN_OP_ISNULL: WMIQ_RPN_TOKEN_FLAGS = 10;
pub const WMIQ_RPN_OP_LE: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_OP_LIKE: WMIQ_RPN_TOKEN_FLAGS = 7;
pub const WMIQ_RPN_OP_LT: WMIQ_RPN_TOKEN_FLAGS = 5;
pub const WMIQ_RPN_OP_NE: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_OP_UNDEFINED: WMIQ_RPN_TOKEN_FLAGS = 0;
pub const WMIQ_RPN_RELOP: WMIQ_RPN_TOKEN_FLAGS = 16;
pub const WMIQ_RPN_RIGHT_FUNCTION: WMIQ_RPN_TOKEN_FLAGS = 64;
pub const WMIQ_RPN_RIGHT_PROPERTY_NAME: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_TOKEN_AND: WMIQ_RPN_TOKEN_FLAGS = 2;
pub const WMIQ_RPN_TOKEN_EXPRESSION: WMIQ_RPN_TOKEN_FLAGS = 1;
pub type WMIQ_RPN_TOKEN_FLAGS = i32;
pub const WMIQ_RPN_TOKEN_NOT: WMIQ_RPN_TOKEN_FLAGS = 4;
pub const WMIQ_RPN_TOKEN_OR: WMIQ_RPN_TOKEN_FLAGS = 3;
pub const WbemDefPath: windows_core::GUID = windows_core::GUID::from_u128(0xcf4cc405_e2c5_4ddd_b3ce_5e7582d8c9fa);
pub const WbemQuery: windows_core::GUID = windows_core::GUID::from_u128(0xeac8a024_21e2_4523_ad73_a71a0aa2f56a);
pub type tag_WBEM_GET_KEY_FLAGS = i32;
pub type tag_WBEM_GET_TEXT_FLAGS = i32;
pub type tag_WBEM_PATH_CREATE_FLAG = i32;
pub type tag_WBEM_PATH_STATUS_FLAG = i32;
