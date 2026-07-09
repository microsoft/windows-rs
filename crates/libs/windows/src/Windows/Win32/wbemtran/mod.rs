windows_core::imp::define_interface!(IWbemAddressResolution, IWbemAddressResolution_Vtbl, 0xf7ce2e12_8c90_11d1_9e7b_00c04fc324a8);
windows_core::imp::interface_hierarchy!(IWbemAddressResolution, windows_core::IUnknown);
impl IWbemAddressResolution {
    pub unsafe fn Resolve<P0>(&self, wsznamespacepath: P0, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Resolve)(windows_core::Interface::as_raw(self), wsznamespacepath.param().abi(), core::mem::transmute(wszaddresstype), pdwaddresslength as _, pabbinaryaddress as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemAddressResolution_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Resolve: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PWSTR, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
pub trait IWbemAddressResolution_Impl: windows_core::IUnknownImpl {
    fn Resolve(&self, wsznamespacepath: &windows_core::PCWSTR, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::Result<()>;
}
impl IWbemAddressResolution_Vtbl {
    pub const fn new<Identity: IWbemAddressResolution_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Resolve<Identity: IWbemAddressResolution_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznamespacepath: windows_core::PCWSTR, wszaddresstype: windows_core::PWSTR, pdwaddresslength: *mut u32, pabbinaryaddress: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemAddressResolution_Impl::Resolve(this, core::mem::transmute(&wsznamespacepath), core::mem::transmute_copy(&wszaddresstype), core::mem::transmute_copy(&pdwaddresslength), core::mem::transmute_copy(&pabbinaryaddress)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Resolve: Resolve::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemAddressResolution as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemAddressResolution {}
windows_core::imp::define_interface!(IWbemClientConnectionTransport, IWbemClientConnectionTransport_Vtbl, 0xa889c72a_fcc1_4a9e_af61_ed071333fb5b);
windows_core::imp::interface_hierarchy!(IWbemClientConnectionTransport, windows_core::IUnknown);
impl IWbemClientConnectionTransport {
    #[cfg(feature = "Win32_wbemcli")]
    pub unsafe fn Open<P8, T>(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: P8, pcallres: *mut Option<super::wbemcli::IWbemCallResult>) -> windows_core::Result<T>
    where
        P8: windows_core::Param<super::wbemcli::IWbemContext>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).Open)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(straddresstype), dwbinaryaddresslength, abbinaryaddress, core::mem::transmute_copy(strobject), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), lflags, pctx.param().abi(), &T::IID, &mut result__, core::mem::transmute(pcallres)).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "Win32_wbemcli")]
    pub unsafe fn OpenAsync<P8, P10>(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: P8, riid: *const windows_core::GUID, presponsehandler: P10) -> windows_core::HRESULT
    where
        P8: windows_core::Param<super::wbemcli::IWbemContext>,
        P10: windows_core::Param<super::wbemcli::IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).OpenAsync)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(straddresstype), dwbinaryaddresslength, abbinaryaddress, core::mem::transmute_copy(strobject), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), lflags, pctx.param().abi(), riid, presponsehandler.param().abi()) }
    }
    #[cfg(feature = "Win32_wbemcli")]
    pub unsafe fn Cancel<P1>(&self, lflags: i32, phandler: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wbemcli::IWbemObjectSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self), lflags, phandler.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientConnectionTransport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_wbemcli")]
    pub Open: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wbemcli"))]
    Open: usize,
    #[cfg(feature = "Win32_wbemcli")]
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wbemcli"))]
    OpenAsync: usize,
    #[cfg(feature = "Win32_wbemcli")]
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wbemcli"))]
    Cancel: usize,
}
#[cfg(feature = "Win32_wbemcli")]
pub trait IWbemClientConnectionTransport_Impl: windows_core::IUnknownImpl {
    fn Open(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<super::wbemcli::IWbemContext>, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void, pcallres: windows_core::OutRef<super::wbemcli::IWbemCallResult>) -> windows_core::Result<()>;
    fn OpenAsync(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lflags: i32, pctx: windows_core::Ref<super::wbemcli::IWbemContext>, riid: *const windows_core::GUID, presponsehandler: windows_core::Ref<super::wbemcli::IWbemObjectSink>) -> windows_core::Result<()>;
    fn Cancel(&self, lflags: i32, phandler: windows_core::Ref<super::wbemcli::IWbemObjectSink>) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_wbemcli")]
impl IWbemClientConnectionTransport_Vtbl {
    pub const fn new<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Open<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: *mut core::ffi::c_void, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void, pcallres: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClientConnectionTransport_Impl::Open(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strobject), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface), core::mem::transmute_copy(&pcallres)).into()
            }
        }
        unsafe extern "system" fn OpenAsync<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: *mut core::ffi::c_void, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strobject: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, presponsehandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClientConnectionTransport_Impl::OpenAsync(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strobject), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&presponsehandler)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IWbemClientConnectionTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lflags: i32, phandler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemClientConnectionTransport_Impl::Cancel(this, core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&phandler)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, OFFSET>,
            OpenAsync: OpenAsync::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClientConnectionTransport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_wbemcli")]
impl windows_core::RuntimeName for IWbemClientConnectionTransport {}
windows_core::imp::define_interface!(IWbemClientTransport, IWbemClientTransport_Vtbl, 0xf7ce2e11_8c90_11d1_9e7b_00c04fc324a8);
windows_core::imp::interface_hierarchy!(IWbemClientTransport, windows_core::IUnknown);
impl IWbemClientTransport {
    #[cfg(feature = "Win32_wbemcli")]
    pub unsafe fn ConnectServer<P9>(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: P9) -> windows_core::Result<super::wbemcli::IWbemServices>
    where
        P9: windows_core::Param<super::wbemcli::IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectServer)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(straddresstype), dwbinaryaddresslength, abbinaryaddress, core::mem::transmute_copy(strnetworkresource), core::mem::transmute_copy(struser), core::mem::transmute_copy(strpassword), core::mem::transmute_copy(strlocale), lsecurityflags, core::mem::transmute_copy(strauthority), pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemClientTransport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_wbemcli")]
    pub ConnectServer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const u8, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wbemcli"))]
    ConnectServer: usize,
}
#[cfg(feature = "Win32_wbemcli")]
pub trait IWbemClientTransport_Impl: windows_core::IUnknownImpl {
    fn ConnectServer(&self, straddresstype: &windows_core::BSTR, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: &windows_core::BSTR, struser: &windows_core::BSTR, strpassword: &windows_core::BSTR, strlocale: &windows_core::BSTR, lsecurityflags: i32, strauthority: &windows_core::BSTR, pctx: windows_core::Ref<super::wbemcli::IWbemContext>) -> windows_core::Result<super::wbemcli::IWbemServices>;
}
#[cfg(feature = "Win32_wbemcli")]
impl IWbemClientTransport_Vtbl {
    pub const fn new<Identity: IWbemClientTransport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectServer<Identity: IWbemClientTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, straddresstype: *mut core::ffi::c_void, dwbinaryaddresslength: u32, abbinaryaddress: *const u8, strnetworkresource: *mut core::ffi::c_void, struser: *mut core::ffi::c_void, strpassword: *mut core::ffi::c_void, strlocale: *mut core::ffi::c_void, lsecurityflags: i32, strauthority: *mut core::ffi::c_void, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemClientTransport_Impl::ConnectServer(this, core::mem::transmute(&straddresstype), core::mem::transmute_copy(&dwbinaryaddresslength), core::mem::transmute_copy(&abbinaryaddress), core::mem::transmute(&strnetworkresource), core::mem::transmute(&struser), core::mem::transmute(&strpassword), core::mem::transmute(&strlocale), core::mem::transmute_copy(&lsecurityflags), core::mem::transmute(&strauthority), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectServer: ConnectServer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemClientTransport as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_wbemcli")]
impl windows_core::RuntimeName for IWbemClientTransport {}
windows_core::imp::define_interface!(IWbemConnectorLogin, IWbemConnectorLogin_Vtbl, 0xd8ec9cb1_b135_4f10_8b1b_c7188bb0d186);
windows_core::imp::interface_hierarchy!(IWbemConnectorLogin, windows_core::IUnknown);
impl IWbemConnectorLogin {
    #[cfg(feature = "Win32_wbemcli")]
    pub unsafe fn ConnectorLogin<P0, P1, P3, T>(&self, wsznetworkresource: P0, wszpreferredlocale: P1, lflags: i32, pctx: P3) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::wbemcli::IWbemContext>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ConnectorLogin)(windows_core::Interface::as_raw(self), wsznetworkresource.param().abi(), wszpreferredlocale.param().abi(), lflags, pctx.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConnectorLogin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_wbemcli")]
    pub ConnectorLogin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wbemcli"))]
    ConnectorLogin: usize,
}
#[cfg(feature = "Win32_wbemcli")]
pub trait IWbemConnectorLogin_Impl: windows_core::IUnknownImpl {
    fn ConnectorLogin(&self, wsznetworkresource: &windows_core::PCWSTR, wszpreferredlocale: &windows_core::PCWSTR, lflags: i32, pctx: windows_core::Ref<super::wbemcli::IWbemContext>, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_wbemcli")]
impl IWbemConnectorLogin_Vtbl {
    pub const fn new<Identity: IWbemConnectorLogin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ConnectorLogin<Identity: IWbemConnectorLogin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszpreferredlocale: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, riid: *const windows_core::GUID, pinterface: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConnectorLogin_Impl::ConnectorLogin(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&pinterface)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectorLogin: ConnectorLogin::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConnectorLogin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_wbemcli")]
impl windows_core::RuntimeName for IWbemConnectorLogin {}
windows_core::imp::define_interface!(IWbemConstructClassObject, IWbemConstructClassObject_Vtbl, 0x9ef76194_70d5_11d1_ad90_00c04fd8fdff);
windows_core::imp::interface_hierarchy!(IWbemConstructClassObject, windows_core::IUnknown);
impl IWbemConstructClassObject {
    pub unsafe fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInheritanceChain)(windows_core::Interface::as_raw(self), lnumantecedents, awszantecedents) }
    }
    pub unsafe fn SetPropertyOrigin<P0>(&self, wszpropertyname: P0, loriginindex: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPropertyOrigin)(windows_core::Interface::as_raw(self), wszpropertyname.param().abi(), loriginindex) }
    }
    pub unsafe fn SetMethodOrigin<P0>(&self, wszmethodname: P0, loriginindex: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMethodOrigin)(windows_core::Interface::as_raw(self), wszmethodname.param().abi(), loriginindex) }
    }
    pub unsafe fn SetServerNamespace<P0, P1>(&self, wszserver: P0, wsznamespace: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServerNamespace)(windows_core::Interface::as_raw(self), wszserver.param().abi(), wsznamespace.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemConstructClassObject_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetInheritanceChain: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPropertyOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub SetMethodOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub SetServerNamespace: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IWbemConstructClassObject_Impl: windows_core::IUnknownImpl {
    fn SetInheritanceChain(&self, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPropertyOrigin(&self, wszpropertyname: &windows_core::PCWSTR, loriginindex: i32) -> windows_core::Result<()>;
    fn SetMethodOrigin(&self, wszmethodname: &windows_core::PCWSTR, loriginindex: i32) -> windows_core::Result<()>;
    fn SetServerNamespace(&self, wszserver: &windows_core::PCWSTR, wsznamespace: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IWbemConstructClassObject_Vtbl {
    pub const fn new<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetInheritanceChain<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lnumantecedents: i32, awszantecedents: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConstructClassObject_Impl::SetInheritanceChain(this, core::mem::transmute_copy(&lnumantecedents), core::mem::transmute_copy(&awszantecedents)).into()
            }
        }
        unsafe extern "system" fn SetPropertyOrigin<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpropertyname: windows_core::PCWSTR, loriginindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConstructClassObject_Impl::SetPropertyOrigin(this, core::mem::transmute(&wszpropertyname), core::mem::transmute_copy(&loriginindex)).into()
            }
        }
        unsafe extern "system" fn SetMethodOrigin<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszmethodname: windows_core::PCWSTR, loriginindex: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConstructClassObject_Impl::SetMethodOrigin(this, core::mem::transmute(&wszmethodname), core::mem::transmute_copy(&loriginindex)).into()
            }
        }
        unsafe extern "system" fn SetServerNamespace<Identity: IWbemConstructClassObject_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszserver: windows_core::PCWSTR, wsznamespace: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemConstructClassObject_Impl::SetServerNamespace(this, core::mem::transmute(&wszserver), core::mem::transmute(&wsznamespace)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetInheritanceChain: SetInheritanceChain::<Identity, OFFSET>,
            SetPropertyOrigin: SetPropertyOrigin::<Identity, OFFSET>,
            SetMethodOrigin: SetMethodOrigin::<Identity, OFFSET>,
            SetServerNamespace: SetServerNamespace::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemConstructClassObject as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemConstructClassObject {}
windows_core::imp::define_interface!(IWbemLevel1Login, IWbemLevel1Login_Vtbl, 0xf309ad18_d86a_11d0_a075_00c04fb68820);
windows_core::imp::interface_hierarchy!(IWbemLevel1Login, windows_core::IUnknown);
impl IWbemLevel1Login {
    pub unsafe fn EstablishPosition<P0>(&self, wszlocalelist: P0, dwnumlocales: u32) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EstablishPosition)(windows_core::Interface::as_raw(self), wszlocalelist.param().abi(), dwnumlocales, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RequestChallenge<P0, P1>(&self, wsznetworkresource: P0, wszuser: P1) -> windows_core::Result<u8>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestChallenge)(windows_core::Interface::as_raw(self), wsznetworkresource.param().abi(), wszuser.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_wbemcli")]
    pub unsafe fn WBEMLogin<P0, P3>(&self, wszpreferredlocale: P0, accesstoken: *const u8, lflags: i32, pctx: P3) -> windows_core::Result<super::wbemcli::IWbemServices>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::wbemcli::IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WBEMLogin)(windows_core::Interface::as_raw(self), wszpreferredlocale.param().abi(), accesstoken, lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wbemcli")]
    pub unsafe fn NTLMLogin<P0, P1, P3>(&self, wsznetworkresource: P0, wszpreferredlocale: P1, lflags: i32, pctx: P3) -> windows_core::Result<super::wbemcli::IWbemServices>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::wbemcli::IWbemContext>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NTLMLogin)(windows_core::Interface::as_raw(self), wsznetworkresource.param().abi(), wszpreferredlocale.param().abi(), lflags, pctx.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemLevel1Login_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub EstablishPosition: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub RequestChallenge: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *mut u8) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wbemcli")]
    pub WBEMLogin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wbemcli"))]
    WBEMLogin: usize,
    #[cfg(feature = "Win32_wbemcli")]
    pub NTLMLogin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wbemcli"))]
    NTLMLogin: usize,
}
#[cfg(feature = "Win32_wbemcli")]
pub trait IWbemLevel1Login_Impl: windows_core::IUnknownImpl {
    fn EstablishPosition(&self, wszlocalelist: &windows_core::PCWSTR, dwnumlocales: u32) -> windows_core::Result<u32>;
    fn RequestChallenge(&self, wsznetworkresource: &windows_core::PCWSTR, wszuser: &windows_core::PCWSTR) -> windows_core::Result<u8>;
    fn WBEMLogin(&self, wszpreferredlocale: &windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: windows_core::Ref<super::wbemcli::IWbemContext>) -> windows_core::Result<super::wbemcli::IWbemServices>;
    fn NTLMLogin(&self, wsznetworkresource: &windows_core::PCWSTR, wszpreferredlocale: &windows_core::PCWSTR, lflags: i32, pctx: windows_core::Ref<super::wbemcli::IWbemContext>) -> windows_core::Result<super::wbemcli::IWbemServices>;
}
#[cfg(feature = "Win32_wbemcli")]
impl IWbemLevel1Login_Vtbl {
    pub const fn new<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EstablishPosition<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlocalelist: windows_core::PCWSTR, dwnumlocales: u32, reserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLevel1Login_Impl::EstablishPosition(this, core::mem::transmute(&wszlocalelist), core::mem::transmute_copy(&dwnumlocales)) {
                    Ok(ok__) => {
                        reserved.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestChallenge<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszuser: windows_core::PCWSTR, nonce: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLevel1Login_Impl::RequestChallenge(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszuser)) {
                    Ok(ok__) => {
                        nonce.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WBEMLogin<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpreferredlocale: windows_core::PCWSTR, accesstoken: *const u8, lflags: i32, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLevel1Login_Impl::WBEMLogin(this, core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&accesstoken), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NTLMLogin<Identity: IWbemLevel1Login_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wsznetworkresource: windows_core::PCWSTR, wszpreferredlocale: windows_core::PCWSTR, lflags: i32, pctx: *mut core::ffi::c_void, ppnamespace: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWbemLevel1Login_Impl::NTLMLogin(this, core::mem::transmute(&wsznetworkresource), core::mem::transmute(&wszpreferredlocale), core::mem::transmute_copy(&lflags), core::mem::transmute_copy(&pctx)) {
                    Ok(ok__) => {
                        ppnamespace.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            EstablishPosition: EstablishPosition::<Identity, OFFSET>,
            RequestChallenge: RequestChallenge::<Identity, OFFSET>,
            WBEMLogin: WBEMLogin::<Identity, OFFSET>,
            NTLMLogin: NTLMLogin::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemLevel1Login as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_wbemcli")]
impl windows_core::RuntimeName for IWbemLevel1Login {}
windows_core::imp::define_interface!(IWbemTransport, IWbemTransport_Vtbl, 0x553fe584_2156_11d0_b6ae_00aa003240c7);
windows_core::imp::interface_hierarchy!(IWbemTransport, windows_core::IUnknown);
impl IWbemTransport {
    pub unsafe fn Initialize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWbemTransport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWbemTransport_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self) -> windows_core::Result<()>;
}
impl IWbemTransport_Vtbl {
    pub const fn new<Identity: IWbemTransport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IWbemTransport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWbemTransport_Impl::Initialize(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWbemTransport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWbemTransport {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WBEM_128BITS(pub *mut u8);
impl WBEM_128BITS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for WBEM_128BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WBEM_AUTHENTICATION_METHOD_MASK: WBEM_LOGIN_TYPE = 15;
pub const WBEM_FLAG_INPROC_LOGIN: WBEM_LOGIN_TYPE = 0;
pub const WBEM_FLAG_LOCAL_LOGIN: WBEM_LOGIN_TYPE = 1;
pub const WBEM_FLAG_REMOTE_LOGIN: WBEM_LOGIN_TYPE = 2;
pub const WBEM_FLAG_USE_MULTIPLE_CHALLENGES: WBEM_LOGIN_TYPE = 16;
pub type WBEM_LOGIN_TYPE = i32;
pub const WbemDCOMTransport: windows_core::GUID = windows_core::GUID::from_u128(0xf7ce2e13_8c90_11d1_9e7b_00c04fc324a8);
pub const WbemLevel1Login: windows_core::GUID = windows_core::GUID::from_u128(0x8bc3f05e_d86b_11d0_a075_00c04fb68820);
pub const WbemLocalAddrRes: windows_core::GUID = windows_core::GUID::from_u128(0xa1044801_8f7e_11d1_9e7c_00c04fc324a8);
pub const WbemUninitializedClassObject: windows_core::GUID = windows_core::GUID::from_u128(0x7a0227f6_7108_11d1_ad90_00c04fd8fdff);
