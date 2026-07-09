#[cfg(feature = "Win32_wsdxml")]
#[inline]
pub unsafe fn WSDCreateDeviceProxy<P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2) -> windows_core::Result<IWSDDeviceProxy>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy(pszdeviceid : windows_core::PCWSTR, pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceProxy(pszdeviceid.param().abi(), pszlocalid.param().abi(), pcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
#[inline]
pub unsafe fn WSDCreateDeviceProxy2<P0, P1, P2>(pszdeviceid: P0, pszlocalid: P1, pcontext: P2, pconfigparams: Option<&[super::wsdbase::WSD_CONFIG_PARAM]>) -> windows_core::Result<IWSDDeviceProxy>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxy2(pszdeviceid : windows_core::PCWSTR, pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const super::wsdbase::WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceProxy2(pszdeviceid.param().abi(), pszlocalid.param().abi(), pcontext.param().abi(), core::mem::transmute(pconfigparams.map_or(core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
#[inline]
pub unsafe fn WSDCreateDeviceProxyAdvanced<P0, P1, P2, P3>(pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3) -> windows_core::Result<IWSDDeviceProxy>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::wsdbase::IWSDAddress>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceProxyAdvanced(pszdeviceid : windows_core::PCWSTR, pdeviceaddress : *mut core::ffi::c_void, pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdeviceproxy : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceProxyAdvanced(pszdeviceid.param().abi(), pdeviceaddress.param().abi(), pszlocalid.param().abi(), pcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IWSDAsyncCallback, IWSDAsyncCallback_Vtbl, 0xa63e109d_ce72_49e2_ba98_e845f5ee1666);
windows_core::imp::interface_hierarchy!(IWSDAsyncCallback, windows_core::IUnknown);
impl IWSDAsyncCallback {
    pub unsafe fn AsyncOperationComplete<P0, P1>(&self, pasyncresult: P0, pasyncstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAsyncResult>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AsyncOperationComplete)(windows_core::Interface::as_raw(self), pasyncresult.param().abi(), pasyncstate.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AsyncOperationComplete: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDAsyncCallback_Impl: windows_core::IUnknownImpl {
    fn AsyncOperationComplete(&self, pasyncresult: windows_core::Ref<IWSDAsyncResult>, pasyncstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IWSDAsyncCallback_Vtbl {
    pub const fn new<Identity: IWSDAsyncCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AsyncOperationComplete<Identity: IWSDAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void, pasyncstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncCallback_Impl::AsyncOperationComplete(this, core::mem::transmute_copy(&pasyncresult), core::mem::transmute_copy(&pasyncstate)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAsyncCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDAsyncCallback {}
windows_core::imp::define_interface!(IWSDAsyncResult, IWSDAsyncResult_Vtbl, 0x11a9852a_8dd8_423e_b537_9356db4fbfb8);
windows_core::imp::interface_hierarchy!(IWSDAsyncResult, windows_core::IUnknown);
impl IWSDAsyncResult {
    pub unsafe fn SetCallback<P0, P1>(&self, pcallback: P0, pasyncstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCallback)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pasyncstate.param().abi()) }
    }
    #[cfg(feature = "Win32_winnt")]
    pub unsafe fn SetWaitHandle(&self, hwaithandle: super::winnt::HANDLE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWaitHandle)(windows_core::Interface::as_raw(self), hwaithandle) }
    }
    pub unsafe fn HasCompleted(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HasCompleted)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetAsyncState(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAsyncState)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Abort(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Abort)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetEvent(&self, pevent: *mut super::wsdtypes::WSD_EVENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEvent)(windows_core::Interface::as_raw(self), core::mem::transmute(pevent)) }
    }
    pub unsafe fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndpointProxy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDAsyncResult_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_winnt")]
    pub SetWaitHandle: unsafe extern "system" fn(*mut core::ffi::c_void, super::winnt::HANDLE) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_winnt"))]
    SetWaitHandle: usize,
    pub HasCompleted: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAsyncState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Abort: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wsdtypes::WSD_EVENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetEvent: usize,
    pub GetEndpointProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
pub trait IWSDAsyncResult_Impl: windows_core::IUnknownImpl {
    fn SetCallback(&self, pcallback: windows_core::Ref<IWSDAsyncCallback>, pasyncstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetWaitHandle(&self, hwaithandle: super::winnt::HANDLE) -> windows_core::Result<()>;
    fn HasCompleted(&self) -> windows_core::Result<()>;
    fn GetAsyncState(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn GetEvent(&self, pevent: *mut super::wsdtypes::WSD_EVENT) -> windows_core::Result<()>;
    fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy>;
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl IWSDAsyncResult_Vtbl {
    pub const fn new<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCallback<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pasyncstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::SetCallback(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pasyncstate)).into()
            }
        }
        unsafe extern "system" fn SetWaitHandle<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwaithandle: super::winnt::HANDLE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::SetWaitHandle(this, core::mem::transmute_copy(&hwaithandle)).into()
            }
        }
        unsafe extern "system" fn HasCompleted<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::HasCompleted(this).into()
            }
        }
        unsafe extern "system" fn GetAsyncState<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasyncstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDAsyncResult_Impl::GetAsyncState(this) {
                    Ok(ok__) => {
                        ppasyncstate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Abort<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::Abort(this).into()
            }
        }
        unsafe extern "system" fn GetEvent<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut super::wsdtypes::WSD_EVENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDAsyncResult_Impl::GetEvent(this, core::mem::transmute_copy(&pevent)).into()
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppendpoint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDAsyncResult_Impl::GetEndpointProxy(this) {
                    Ok(ok__) => {
                        ppendpoint.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCallback: SetCallback::<Identity, OFFSET>,
            SetWaitHandle: SetWaitHandle::<Identity, OFFSET>,
            HasCompleted: HasCompleted::<Identity, OFFSET>,
            GetAsyncState: GetAsyncState::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
            GetEvent: GetEvent::<Identity, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAsyncResult as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDAsyncResult {}
windows_core::imp::define_interface!(IWSDDeviceProxy, IWSDDeviceProxy_Vtbl, 0xeee0c031_c578_4c0e_9a3b_973c35f409db);
windows_core::imp::interface_hierarchy!(IWSDDeviceProxy, windows_core::IUnknown);
impl IWSDDeviceProxy {
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
    pub unsafe fn Init<P0, P1, P2, P3, P4>(&self, pszdeviceid: P0, pdeviceaddress: P1, pszlocalid: P2, pcontext: P3, psponsor: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::wsdbase::IWSDAddress>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::wsdxml::IWSDXMLContext>,
        P4: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pszdeviceid.param().abi(), pdeviceaddress.param().abi(), pszlocalid.param().abi(), pcontext.param().abi(), psponsor.param().abi()) }
    }
    pub unsafe fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginGetMetadata)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndGetMetadata)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetHostMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_HOST_METADATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHostMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetThisModelMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_THIS_MODEL_METADATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThisModelMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetThisDeviceMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_THIS_DEVICE_METADATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetThisDeviceMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetAllMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_METADATA_SECTION_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetServiceProxyById<P0>(&self, pszserviceid: P0) -> windows_core::Result<IWSDServiceProxy>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceProxyById)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wsdxmldom")]
    pub unsafe fn GetServiceProxyByType(&self, ptype: *const super::wsdxmldom::WSDXML_NAME) -> windows_core::Result<IWSDServiceProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceProxyByType)(windows_core::Interface::as_raw(self), ptype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndpointProxy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceProxy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml")))]
    Init: usize,
    pub BeginGetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EndGetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetHostMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_HOST_METADATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetHostMetadata: usize,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetThisModelMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_THIS_MODEL_METADATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetThisModelMetadata: usize,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetThisDeviceMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_THIS_DEVICE_METADATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetThisDeviceMetadata: usize,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetAllMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetAllMetadata: usize,
    pub GetServiceProxyById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wsdxmldom")]
    pub GetServiceProxyByType: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdxmldom::WSDXML_NAME, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdxmldom"))]
    GetServiceProxyByType: usize,
    pub GetEndpointProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
pub trait IWSDDeviceProxy_Impl: windows_core::IUnknownImpl {
    fn Init(&self, pszdeviceid: &windows_core::PCWSTR, pdeviceaddress: windows_core::Ref<super::wsdbase::IWSDAddress>, pszlocalid: &windows_core::PCWSTR, pcontext: windows_core::Ref<super::wsdxml::IWSDXMLContext>, psponsor: windows_core::Ref<IWSDDeviceProxy>) -> windows_core::Result<()>;
    fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&self, presult: windows_core::Ref<IWSDAsyncResult>) -> windows_core::Result<()>;
    fn GetHostMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_HOST_METADATA>;
    fn GetThisModelMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_THIS_MODEL_METADATA>;
    fn GetThisDeviceMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_THIS_DEVICE_METADATA>;
    fn GetAllMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_METADATA_SECTION_LIST>;
    fn GetServiceProxyById(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<IWSDServiceProxy>;
    fn GetServiceProxyByType(&self, ptype: *const super::wsdxmldom::WSDXML_NAME) -> windows_core::Result<IWSDServiceProxy>;
    fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy>;
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl IWSDDeviceProxy_Vtbl {
    pub const fn new<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdeviceid: windows_core::PCWSTR, pdeviceaddress: *mut core::ffi::c_void, pszlocalid: windows_core::PCWSTR, pcontext: *mut core::ffi::c_void, psponsor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceProxy_Impl::Init(this, core::mem::transmute(&pszdeviceid), core::mem::transmute_copy(&pdeviceaddress), core::mem::transmute(&pszlocalid), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&psponsor)).into()
            }
        }
        unsafe extern "system" fn BeginGetMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::BeginGetMetadata(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceProxy_Impl::EndGetMetadata(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn GetHostMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphostmetadata: *mut *mut super::wsdtypes::WSD_HOST_METADATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetHostMetadata(this) {
                    Ok(ok__) => {
                        pphostmetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThisModelMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanufacturermetadata: *mut *mut super::wsdtypes::WSD_THIS_MODEL_METADATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetThisModelMetadata(this) {
                    Ok(ok__) => {
                        ppmanufacturermetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetThisDeviceMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppthisdevicemetadata: *mut *mut super::wsdtypes::WSD_THIS_DEVICE_METADATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetThisDeviceMetadata(this) {
                    Ok(ok__) => {
                        ppthisdevicemetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmetadata: *mut *mut super::wsdtypes::WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetAllMetadata(this) {
                    Ok(ok__) => {
                        ppmetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetServiceProxyById<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, ppserviceproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetServiceProxyById(this, core::mem::transmute(&pszserviceid)) {
                    Ok(ok__) => {
                        ppserviceproxy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetServiceProxyByType<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *const super::wsdxmldom::WSDXML_NAME, ppserviceproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetServiceProxyByType(this, core::mem::transmute_copy(&ptype)) {
                    Ok(ok__) => {
                        ppserviceproxy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceProxy_Impl::GetEndpointProxy(this) {
                    Ok(ok__) => {
                        ppproxy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            BeginGetMetadata: BeginGetMetadata::<Identity, OFFSET>,
            EndGetMetadata: EndGetMetadata::<Identity, OFFSET>,
            GetHostMetadata: GetHostMetadata::<Identity, OFFSET>,
            GetThisModelMetadata: GetThisModelMetadata::<Identity, OFFSET>,
            GetThisDeviceMetadata: GetThisDeviceMetadata::<Identity, OFFSET>,
            GetAllMetadata: GetAllMetadata::<Identity, OFFSET>,
            GetServiceProxyById: GetServiceProxyById::<Identity, OFFSET>,
            GetServiceProxyByType: GetServiceProxyByType::<Identity, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDDeviceProxy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDDeviceProxy {}
windows_core::imp::define_interface!(IWSDEndpointProxy, IWSDEndpointProxy_Vtbl, 0x1860d430_b24c_4975_9f90_dbb39baa24ec);
windows_core::imp::interface_hierarchy!(IWSDEndpointProxy, windows_core::IUnknown);
impl IWSDEndpointProxy {
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn SendOneWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendOneWayRequest)(windows_core::Interface::as_raw(self), pbody, poperation) }
    }
    #[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn SendTwoWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION, presponsecontext: Option<*const super::wsdtypes::WSD_SYNCHRONOUS_RESPONSE_CONTEXT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendTwoWayRequest)(windows_core::Interface::as_raw(self), pbody, poperation, presponsecontext.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn SendTwoWayRequestAsync<P2, P3>(&self, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION, pasyncstate: P2, pcallback: P3) -> windows_core::Result<IWSDAsyncResult>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
        P3: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SendTwoWayRequestAsync)(windows_core::Interface::as_raw(self), pbody, poperation, pasyncstate.param().abi(), pcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AbortAsyncOperation<P0>(&self, pasyncresult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).AbortAsyncOperation)(windows_core::Interface::as_raw(self), pasyncresult.param().abi()) }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn ProcessFault(&self, pfault: *const super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ProcessFault)(windows_core::Interface::as_raw(self), pfault) }
    }
    pub unsafe fn GetErrorInfo(&self) -> windows_core::Result<windows_core::PCWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetErrorInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetFaultInfo(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_SOAP_FAULT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFaultInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEndpointProxy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub SendOneWayRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    SendOneWayRequest: usize,
    #[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub SendTwoWayRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, *const super::wsdtypes::WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_winnt", feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    SendTwoWayRequest: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub SendTwoWayRequestAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    SendTwoWayRequestAsync: usize,
    pub AbortAsyncOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub ProcessFault: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    ProcessFault: usize,
    pub GetErrorInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetFaultInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetFaultInfo: usize,
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
pub trait IWSDEndpointProxy_Impl: windows_core::IUnknownImpl {
    fn SendOneWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION) -> windows_core::Result<()>;
    fn SendTwoWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION, presponsecontext: *const super::wsdtypes::WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> windows_core::Result<()>;
    fn SendTwoWayRequestAsync(&self, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pcallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn AbortAsyncOperation(&self, pasyncresult: windows_core::Ref<IWSDAsyncResult>) -> windows_core::Result<()>;
    fn ProcessFault(&self, pfault: *const super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::Result<()>;
    fn GetErrorInfo(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetFaultInfo(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_SOAP_FAULT>;
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl IWSDEndpointProxy_Vtbl {
    pub const fn new<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendOneWayRequest<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEndpointProxy_Impl::SendOneWayRequest(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation)).into()
            }
        }
        unsafe extern "system" fn SendTwoWayRequest<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION, presponsecontext: *const super::wsdtypes::WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEndpointProxy_Impl::SendTwoWayRequest(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&presponsecontext)).into()
            }
        }
        unsafe extern "system" fn SendTwoWayRequestAsync<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION, pasyncstate: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDEndpointProxy_Impl::SendTwoWayRequestAsync(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pcallback)) {
                    Ok(ok__) => {
                        presult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AbortAsyncOperation<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEndpointProxy_Impl::AbortAsyncOperation(this, core::mem::transmute_copy(&pasyncresult)).into()
            }
        }
        unsafe extern "system" fn ProcessFault<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfault: *const super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEndpointProxy_Impl::ProcessFault(this, core::mem::transmute_copy(&pfault)).into()
            }
        }
        unsafe extern "system" fn GetErrorInfo<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszerrorinfo: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDEndpointProxy_Impl::GetErrorInfo(this) {
                    Ok(ok__) => {
                        ppszerrorinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFaultInfo<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfault: *mut *mut super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDEndpointProxy_Impl::GetFaultInfo(this) {
                    Ok(ok__) => {
                        ppfault.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendOneWayRequest: SendOneWayRequest::<Identity, OFFSET>,
            SendTwoWayRequest: SendTwoWayRequest::<Identity, OFFSET>,
            SendTwoWayRequestAsync: SendTwoWayRequestAsync::<Identity, OFFSET>,
            AbortAsyncOperation: AbortAsyncOperation::<Identity, OFFSET>,
            ProcessFault: ProcessFault::<Identity, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, OFFSET>,
            GetFaultInfo: GetFaultInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDEndpointProxy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDEndpointProxy {}
windows_core::imp::define_interface!(IWSDEventingStatus, IWSDEventingStatus_Vtbl, 0x49b17f52_637a_407a_ae99_fbe82a4d38c0);
windows_core::imp::interface_hierarchy!(IWSDEventingStatus, windows_core::IUnknown);
impl IWSDEventingStatus {
    pub unsafe fn SubscriptionRenewed<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubscriptionRenewed)(windows_core::Interface::as_raw(self), pszsubscriptionaction.param().abi());
        }
    }
    pub unsafe fn SubscriptionRenewalFailed<P0>(&self, pszsubscriptionaction: P0, hr: windows_core::HRESULT)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubscriptionRenewalFailed)(windows_core::Interface::as_raw(self), pszsubscriptionaction.param().abi(), hr);
        }
    }
    pub unsafe fn SubscriptionEnded<P0>(&self, pszsubscriptionaction: P0)
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            (windows_core::Interface::vtable(self).SubscriptionEnded)(windows_core::Interface::as_raw(self), pszsubscriptionaction.param().abi());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDEventingStatus_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SubscriptionRenewed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
    pub SubscriptionRenewalFailed: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::HRESULT),
    pub SubscriptionEnded: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR),
}
pub trait IWSDEventingStatus_Impl: windows_core::IUnknownImpl {
    fn SubscriptionRenewed(&self, pszsubscriptionaction: &windows_core::PCWSTR);
    fn SubscriptionRenewalFailed(&self, pszsubscriptionaction: &windows_core::PCWSTR, hr: windows_core::HRESULT);
    fn SubscriptionEnded(&self, pszsubscriptionaction: &windows_core::PCWSTR);
}
impl IWSDEventingStatus_Vtbl {
    pub const fn new<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubscriptionRenewed<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEventingStatus_Impl::SubscriptionRenewed(this, core::mem::transmute(&pszsubscriptionaction));
            }
        }
        unsafe extern "system" fn SubscriptionRenewalFailed<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR, hr: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEventingStatus_Impl::SubscriptionRenewalFailed(this, core::mem::transmute(&pszsubscriptionaction), core::mem::transmute_copy(&hr));
            }
        }
        unsafe extern "system" fn SubscriptionEnded<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDEventingStatus_Impl::SubscriptionEnded(this, core::mem::transmute(&pszsubscriptionaction));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SubscriptionRenewed: SubscriptionRenewed::<Identity, OFFSET>,
            SubscriptionRenewalFailed: SubscriptionRenewalFailed::<Identity, OFFSET>,
            SubscriptionEnded: SubscriptionEnded::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDEventingStatus as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDEventingStatus {}
windows_core::imp::define_interface!(IWSDMetadataExchange, IWSDMetadataExchange_Vtbl, 0x06996d57_1d67_4928_9307_3d7833fdb846);
windows_core::imp::interface_hierarchy!(IWSDMetadataExchange, windows_core::IUnknown);
impl IWSDMetadataExchange {
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_METADATA_SECTION_LIST> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDMetadataExchange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetMetadata: usize,
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
pub trait IWSDMetadataExchange_Impl: windows_core::IUnknownImpl {
    fn GetMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_METADATA_SECTION_LIST>;
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl IWSDMetadataExchange_Vtbl {
    pub const fn new<Identity: IWSDMetadataExchange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetadata<Identity: IWSDMetadataExchange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataout: *mut *mut super::wsdtypes::WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDMetadataExchange_Impl::GetMetadata(this) {
                    Ok(ok__) => {
                        metadataout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMetadata: GetMetadata::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDMetadataExchange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDMetadataExchange {}
windows_core::imp::define_interface!(IWSDServiceProxy, IWSDServiceProxy_Vtbl, 0xd4c7fb9c_03ab_4175_9d67_094fafebf487);
impl core::ops::Deref for IWSDServiceProxy {
    type Target = IWSDMetadataExchange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDServiceProxy, windows_core::IUnknown, IWSDMetadataExchange);
impl IWSDServiceProxy {
    pub unsafe fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginGetMetadata)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn EndGetMetadata<P0>(&self, presult: P0) -> windows_core::Result<*mut super::wsdtypes::WSD_METADATA_SECTION_LIST>
    where
        P0: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndGetMetadata)(windows_core::Interface::as_raw(self), presult.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetServiceMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_SERVICE_METADATA> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetServiceMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn SubscribeToOperation<P1>(&self, poperation: *const super::wsdtypes::WSD_OPERATION, punknown: P1, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<*mut super::wsdxmldom::WSDXML_ELEMENT>
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SubscribeToOperation)(windows_core::Interface::as_raw(self), poperation, punknown.param().abi(), pany, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn UnsubscribeToOperation(&self, poperation: *const super::wsdtypes::WSD_OPERATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnsubscribeToOperation)(windows_core::Interface::as_raw(self), poperation) }
    }
    pub unsafe fn SetEventingStatusCallback<P0>(&self, pstatus: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IWSDEventingStatus>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEventingStatusCallback)(windows_core::Interface::as_raw(self), pstatus.param().abi()) }
    }
    pub unsafe fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEndpointProxy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxy_Vtbl {
    pub base__: IWSDMetadataExchange_Vtbl,
    pub BeginGetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub EndGetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    EndGetMetadata: usize,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub GetServiceMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_SERVICE_METADATA) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    GetServiceMetadata: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub SubscribeToOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, *mut core::ffi::c_void, *const super::wsdxmldom::WSDXML_ELEMENT, *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    SubscribeToOperation: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub UnsubscribeToOperation: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    UnsubscribeToOperation: usize,
    pub SetEventingStatusCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetEndpointProxy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
pub trait IWSDServiceProxy_Impl: IWSDMetadataExchange_Impl {
    fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&self, presult: windows_core::Ref<IWSDAsyncResult>) -> windows_core::Result<*mut super::wsdtypes::WSD_METADATA_SECTION_LIST>;
    fn GetServiceMetadata(&self) -> windows_core::Result<*mut super::wsdtypes::WSD_SERVICE_METADATA>;
    fn SubscribeToOperation(&self, poperation: *const super::wsdtypes::WSD_OPERATION, punknown: windows_core::Ref<windows_core::IUnknown>, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<*mut super::wsdxmldom::WSDXML_ELEMENT>;
    fn UnsubscribeToOperation(&self, poperation: *const super::wsdtypes::WSD_OPERATION) -> windows_core::Result<()>;
    fn SetEventingStatusCallback(&self, pstatus: windows_core::Ref<IWSDEventingStatus>) -> windows_core::Result<()>;
    fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy>;
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl IWSDServiceProxy_Vtbl {
    pub const fn new<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginGetMetadata<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxy_Impl::BeginGetMetadata(this) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, ppmetadata: *mut *mut super::wsdtypes::WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxy_Impl::EndGetMetadata(this, core::mem::transmute_copy(&presult)) {
                    Ok(ok__) => {
                        ppmetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetServiceMetadata<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservicemetadata: *mut *mut super::wsdtypes::WSD_SERVICE_METADATA) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxy_Impl::GetServiceMetadata(this) {
                    Ok(ok__) => {
                        ppservicemetadata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SubscribeToOperation<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION, punknown: *mut core::ffi::c_void, pany: *const super::wsdxmldom::WSDXML_ELEMENT, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxy_Impl::SubscribeToOperation(this, core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&pany)) {
                    Ok(ok__) => {
                        ppany.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnsubscribeToOperation<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxy_Impl::UnsubscribeToOperation(this, core::mem::transmute_copy(&poperation)).into()
            }
        }
        unsafe extern "system" fn SetEventingStatusCallback<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxy_Impl::SetEventingStatusCallback(this, core::mem::transmute_copy(&pstatus)).into()
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxy_Impl::GetEndpointProxy(this) {
                    Ok(ok__) => {
                        ppproxy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWSDMetadataExchange_Vtbl::new::<Identity, OFFSET>(),
            BeginGetMetadata: BeginGetMetadata::<Identity, OFFSET>,
            EndGetMetadata: EndGetMetadata::<Identity, OFFSET>,
            GetServiceMetadata: GetServiceMetadata::<Identity, OFFSET>,
            SubscribeToOperation: SubscribeToOperation::<Identity, OFFSET>,
            UnsubscribeToOperation: UnsubscribeToOperation::<Identity, OFFSET>,
            SetEventingStatusCallback: SetEventingStatusCallback::<Identity, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDServiceProxy as windows_core::Interface>::IID || iid == &<IWSDMetadataExchange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDServiceProxy {}
windows_core::imp::define_interface!(IWSDServiceProxyEventing, IWSDServiceProxyEventing_Vtbl, 0xf9279d6d_1012_4a94_b8cc_fd35d2202bfe);
impl core::ops::Deref for IWSDServiceProxyEventing {
    type Target = IWSDServiceProxy;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IWSDServiceProxyEventing, windows_core::IUnknown, IWSDMetadataExchange, IWSDServiceProxy);
impl IWSDServiceProxyEventing {
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub unsafe fn SubscribeToMultipleOperations<P2>(&self, poperations: &[super::wsdtypes::WSD_OPERATION], punknown: P2, pexpires: Option<*const super::wsdtypes::WSD_EVENTING_EXPIRES>, pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), punknown.param().abi(), pexpires.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, ppexpires as _, ppany as _) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub unsafe fn BeginSubscribeToMultipleOperations<P2, P5, P6>(&self, poperations: &[super::wsdtypes::WSD_OPERATION], punknown: P2, pexpires: Option<*const super::wsdtypes::WSD_EVENTING_EXPIRES>, pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pasyncstate: P5, pasynccallback: P6) -> windows_core::Result<IWSDAsyncResult>
    where
        P2: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<windows_core::IUnknown>,
        P6: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginSubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), punknown.param().abi(), pexpires.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, pasyncstate.param().abi(), pasynccallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub unsafe fn EndSubscribeToMultipleOperations<P2>(&self, poperations: &[super::wsdtypes::WSD_OPERATION], presult: P2, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndSubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), presult.param().abi(), ppexpires as _, ppany as _) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn UnsubscribeToMultipleOperations(&self, poperations: &[super::wsdtypes::WSD_OPERATION], pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnsubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), pany) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn BeginUnsubscribeToMultipleOperations<P3, P4>(&self, poperations: &[super::wsdtypes::WSD_OPERATION], pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pasyncstate: P3, pasynccallback: P4) -> windows_core::Result<IWSDAsyncResult>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginUnsubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), pany.unwrap_or(core::mem::zeroed()) as _, pasyncstate.param().abi(), pasynccallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn EndUnsubscribeToMultipleOperations<P2>(&self, poperations: &[super::wsdtypes::WSD_OPERATION], presult: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndUnsubscribeToMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), presult.param().abi()) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub unsafe fn RenewMultipleOperations(&self, poperations: &[super::wsdtypes::WSD_OPERATION], pexpires: Option<*const super::wsdtypes::WSD_EVENTING_EXPIRES>, pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RenewMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), pexpires.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, ppexpires as _, ppany as _) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub unsafe fn BeginRenewMultipleOperations<P4, P5>(&self, poperations: &[super::wsdtypes::WSD_OPERATION], pexpires: Option<*const super::wsdtypes::WSD_EVENTING_EXPIRES>, pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pasyncstate: P4, pasynccallback: P5) -> windows_core::Result<IWSDAsyncResult>
    where
        P4: windows_core::Param<windows_core::IUnknown>,
        P5: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginRenewMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), pexpires.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, pasyncstate.param().abi(), pasynccallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub unsafe fn EndRenewMultipleOperations<P2>(&self, poperations: &[super::wsdtypes::WSD_OPERATION], presult: P2, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndRenewMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), presult.param().abi(), ppexpires as _, ppany as _) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub unsafe fn GetStatusForMultipleOperations(&self, poperations: &[super::wsdtypes::WSD_OPERATION], pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStatusForMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), pany.unwrap_or(core::mem::zeroed()) as _, ppexpires as _, ppany as _) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn BeginGetStatusForMultipleOperations<P3, P4>(&self, poperations: &[super::wsdtypes::WSD_OPERATION], pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pasyncstate: P3, pasynccallback: P4) -> windows_core::Result<IWSDAsyncResult>
    where
        P3: windows_core::Param<windows_core::IUnknown>,
        P4: windows_core::Param<IWSDAsyncCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BeginGetStatusForMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), pany.unwrap_or(core::mem::zeroed()) as _, pasyncstate.param().abi(), pasynccallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub unsafe fn EndGetStatusForMultipleOperations<P2>(&self, poperations: &[super::wsdtypes::WSD_OPERATION], presult: P2, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndGetStatusForMultipleOperations)(windows_core::Interface::as_raw(self), core::mem::transmute(poperations.as_ptr()), poperations.len().try_into().unwrap(), presult.param().abi(), ppexpires as _, ppany as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceProxyEventing_Vtbl {
    pub base__: IWSDServiceProxy_Vtbl,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub SubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *mut core::ffi::c_void, *const super::wsdtypes::WSD_EVENTING_EXPIRES, *const super::wsdxmldom::WSDXML_ELEMENT, *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom")))]
    SubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub BeginSubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *mut core::ffi::c_void, *const super::wsdtypes::WSD_EVENTING_EXPIRES, *const super::wsdxmldom::WSDXML_ELEMENT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom")))]
    BeginSubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub EndSubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom")))]
    EndSubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub UnsubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    UnsubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub BeginUnsubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *const super::wsdxmldom::WSDXML_ELEMENT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    BeginUnsubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub EndUnsubscribeToMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    EndUnsubscribeToMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub RenewMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *const super::wsdtypes::WSD_EVENTING_EXPIRES, *const super::wsdxmldom::WSDXML_ELEMENT, *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom")))]
    RenewMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub BeginRenewMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *const super::wsdtypes::WSD_EVENTING_EXPIRES, *const super::wsdxmldom::WSDXML_ELEMENT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom")))]
    BeginRenewMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub EndRenewMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom")))]
    EndRenewMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub GetStatusForMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *const super::wsdxmldom::WSDXML_ELEMENT, *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom")))]
    GetStatusForMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub BeginGetStatusForMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *const super::wsdxmldom::WSDXML_ELEMENT, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    BeginGetStatusForMultipleOperations: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
    pub EndGetStatusForMultipleOperations: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, u32, *mut core::ffi::c_void, *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom")))]
    EndGetStatusForMultipleOperations: usize,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
pub trait IWSDServiceProxyEventing_Impl: IWSDServiceProxy_Impl {
    fn SubscribeToMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, punknown: windows_core::Ref<windows_core::IUnknown>, pexpires: *const super::wsdtypes::WSD_EVENTING_EXPIRES, pany: *const super::wsdxmldom::WSDXML_ELEMENT, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginSubscribeToMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, punknown: windows_core::Ref<windows_core::IUnknown>, pexpires: *const super::wsdtypes::WSD_EVENTING_EXPIRES, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pasynccallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndSubscribeToMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, presult: windows_core::Ref<IWSDAsyncResult>, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn UnsubscribeToMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginUnsubscribeToMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pasynccallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndUnsubscribeToMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, presult: windows_core::Ref<IWSDAsyncResult>) -> windows_core::Result<()>;
    fn RenewMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pexpires: *const super::wsdtypes::WSD_EVENTING_EXPIRES, pany: *const super::wsdxmldom::WSDXML_ELEMENT, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginRenewMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pexpires: *const super::wsdtypes::WSD_EVENTING_EXPIRES, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pasynccallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndRenewMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, presult: windows_core::Ref<IWSDAsyncResult>, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn GetStatusForMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pany: *const super::wsdxmldom::WSDXML_ELEMENT, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginGetStatusForMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pasyncstate: windows_core::Ref<windows_core::IUnknown>, pasynccallback: windows_core::Ref<IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndGetStatusForMultipleOperations(&self, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, presult: windows_core::Ref<IWSDAsyncResult>, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl IWSDServiceProxyEventing_Vtbl {
    pub const fn new<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, punknown: *mut core::ffi::c_void, pexpires: *const super::wsdtypes::WSD_EVENTING_EXPIRES, pany: *const super::wsdxmldom::WSDXML_ELEMENT, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::SubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn BeginSubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, punknown: *mut core::ffi::c_void, pexpires: *const super::wsdtypes::WSD_EVENTING_EXPIRES, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxyEventing_Impl::BeginSubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&punknown), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pasynccallback)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndSubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, presult: *mut core::ffi::c_void, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::EndSubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn UnsubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pany: *const super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::UnsubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany)).into()
            }
        }
        unsafe extern "system" fn BeginUnsubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxyEventing_Impl::BeginUnsubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pasynccallback)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndUnsubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::EndUnsubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn RenewMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pexpires: *const super::wsdtypes::WSD_EVENTING_EXPIRES, pany: *const super::wsdxmldom::WSDXML_ELEMENT, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::RenewMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn BeginRenewMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pexpires: *const super::wsdtypes::WSD_EVENTING_EXPIRES, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxyEventing_Impl::BeginRenewMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pasynccallback)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndRenewMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, presult: *mut core::ffi::c_void, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::EndRenewMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn GetStatusForMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pany: *const super::wsdxmldom::WSDXML_ELEMENT, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::GetStatusForMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        unsafe extern "system" fn BeginGetStatusForMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDServiceProxyEventing_Impl::BeginGetStatusForMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pasyncstate), core::mem::transmute_copy(&pasynccallback)) {
                    Ok(ok__) => {
                        ppresult.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndGetStatusForMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const super::wsdtypes::WSD_OPERATION, dwoperationcount: u32, presult: *mut core::ffi::c_void, ppexpires: *mut *mut super::wsdtypes::WSD_EVENTING_EXPIRES, ppany: *mut *mut super::wsdxmldom::WSDXML_ELEMENT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceProxyEventing_Impl::EndGetStatusForMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
            }
        }
        Self {
            base__: IWSDServiceProxy_Vtbl::new::<Identity, OFFSET>(),
            SubscribeToMultipleOperations: SubscribeToMultipleOperations::<Identity, OFFSET>,
            BeginSubscribeToMultipleOperations: BeginSubscribeToMultipleOperations::<Identity, OFFSET>,
            EndSubscribeToMultipleOperations: EndSubscribeToMultipleOperations::<Identity, OFFSET>,
            UnsubscribeToMultipleOperations: UnsubscribeToMultipleOperations::<Identity, OFFSET>,
            BeginUnsubscribeToMultipleOperations: BeginUnsubscribeToMultipleOperations::<Identity, OFFSET>,
            EndUnsubscribeToMultipleOperations: EndUnsubscribeToMultipleOperations::<Identity, OFFSET>,
            RenewMultipleOperations: RenewMultipleOperations::<Identity, OFFSET>,
            BeginRenewMultipleOperations: BeginRenewMultipleOperations::<Identity, OFFSET>,
            EndRenewMultipleOperations: EndRenewMultipleOperations::<Identity, OFFSET>,
            GetStatusForMultipleOperations: GetStatusForMultipleOperations::<Identity, OFFSET>,
            BeginGetStatusForMultipleOperations: BeginGetStatusForMultipleOperations::<Identity, OFFSET>,
            EndGetStatusForMultipleOperations: EndGetStatusForMultipleOperations::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDServiceProxyEventing as windows_core::Interface>::IID || iid == &<IWSDMetadataExchange as windows_core::Interface>::IID || iid == &<IWSDServiceProxy as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdhost", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDServiceProxyEventing {}
