#[cfg(feature = "Win32_wsdxml")]
#[inline]
pub unsafe fn WSDCreateDeviceHost<P0, P1>(pszlocalid: P0, pcontext: P1) -> windows_core::Result<IWSDDeviceHost>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost(pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceHost(pszlocalid.param().abi(), pcontext.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
#[inline]
pub unsafe fn WSDCreateDeviceHost2<P0, P1>(pszlocalid: P0, pcontext: P1, pconfigparams: Option<&[super::wsdbase::WSD_CONFIG_PARAM]>) -> windows_core::Result<IWSDDeviceHost>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceHost2(pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, pconfigparams : *const super::wsdbase::WSD_CONFIG_PARAM, dwconfigparamcount : u32, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceHost2(pszlocalid.param().abi(), pcontext.param().abi(), core::mem::transmute(pconfigparams.map_or(core::ptr::null(), |slice| slice.as_ptr())), pconfigparams.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
#[inline]
pub unsafe fn WSDCreateDeviceHostAdvanced<P0, P1>(pszlocalid: P0, pcontext: P1, pphostaddresses: Option<&[Option<super::wsdbase::IWSDAddress>]>) -> windows_core::Result<IWSDDeviceHost>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::wsdxml::IWSDXMLContext>,
{
    windows_core::link!("wsdapi.dll" "system" fn WSDCreateDeviceHostAdvanced(pszlocalid : windows_core::PCWSTR, pcontext : *mut core::ffi::c_void, pphostaddresses : *const *mut core::ffi::c_void, dwhostaddresscount : u32, ppdevicehost : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WSDCreateDeviceHostAdvanced(pszlocalid.param().abi(), pcontext.param().abi(), core::mem::transmute(pphostaddresses.map_or(core::ptr::null(), |slice| slice.as_ptr())), pphostaddresses.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(IWSDDeviceHost, IWSDDeviceHost_Vtbl, 0x917fe891_3d13_4138_9809_934c8abeb12c);
windows_core::imp::interface_hierarchy!(IWSDDeviceHost, windows_core::IUnknown);
impl IWSDDeviceHost {
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
    pub unsafe fn Init<P0, P1>(&self, pszlocalid: P0, pcontext: P1, pphostaddresses: Option<&[Option<super::wsdbase::IWSDAddress>]>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::wsdxml::IWSDXMLContext>,
    {
        unsafe { (windows_core::Interface::vtable(self).Init)(windows_core::Interface::as_raw(self), pszlocalid.param().abi(), pcontext.param().abi(), core::mem::transmute(pphostaddresses.map_or(core::ptr::null(), |slice| slice.as_ptr())), pphostaddresses.map_or(0, |slice| slice.len().try_into().unwrap())) }
    }
    #[cfg(feature = "Win32_wsdtypes")]
    pub unsafe fn Start<P2>(&self, ullinstanceid: u64, pscopelist: *const super::wsdtypes::WSD_URI_LIST, pnotificationsink: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IWSDDeviceHostNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), ullinstanceid, pscopelist, pnotificationsink.param().abi()) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Terminate(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Terminate)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn RegisterPortType(&self, pporttype: *const super::wsdtypes::WSD_PORT_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterPortType)(windows_core::Interface::as_raw(self), pporttype) }
    }
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn SetMetadata(&self, pthismodelmetadata: *const super::wsdtypes::WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const super::wsdtypes::WSD_THIS_DEVICE_METADATA, phostmetadata: Option<*const super::wsdtypes::WSD_HOST_METADATA>, pcustommetadata: Option<*const super::wsdtypes::WSD_METADATA_SECTION_LIST>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMetadata)(windows_core::Interface::as_raw(self), pthismodelmetadata, pthisdevicemetadata, phostmetadata.unwrap_or(core::mem::zeroed()) as _, pcustommetadata.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn RegisterService<P0, P1>(&self, pszserviceid: P0, pservice: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), pservice.param().abi()) }
    }
    pub unsafe fn RetireService<P0>(&self, pszserviceid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RetireService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi()) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn AddDynamicService<P0, P1, P5>(&self, pszserviceid: P0, pszendpointaddress: P1, pporttype: Option<*const super::wsdtypes::WSD_PORT_TYPE>, pportname: Option<*const super::wsdxmldom::WSDXML_NAME>, pany: Option<*const super::wsdxmldom::WSDXML_ELEMENT>, pservice: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDynamicService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), pszendpointaddress.param().abi(), pporttype.unwrap_or(core::mem::zeroed()) as _, pportname.unwrap_or(core::mem::zeroed()) as _, pany.unwrap_or(core::mem::zeroed()) as _, pservice.param().abi()) }
    }
    pub unsafe fn RemoveDynamicService<P0>(&self, pszserviceid: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveDynamicService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi()) }
    }
    pub unsafe fn SetServiceDiscoverable<P0>(&self, pszserviceid: P0, fdiscoverable: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServiceDiscoverable)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), fdiscoverable.into()) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn SignalEvent<P0>(&self, pszserviceid: P0, pbody: Option<*const core::ffi::c_void>, poperation: *const super::wsdtypes::WSD_OPERATION) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SignalEvent)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), pbody.unwrap_or(core::mem::zeroed()) as _, poperation) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml"))]
    pub Init: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdxml")))]
    Init: usize,
    #[cfg(feature = "Win32_wsdtypes")]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *const super::wsdtypes::WSD_URI_LIST, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wsdtypes"))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Terminate: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub RegisterPortType: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_PORT_TYPE) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    RegisterPortType: usize,
    #[cfg(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub SetMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_THIS_MODEL_METADATA, *const super::wsdtypes::WSD_THIS_DEVICE_METADATA, *const super::wsdtypes::WSD_HOST_METADATA, *const super::wsdtypes::WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    SetMetadata: usize,
    pub RegisterService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RetireService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub AddDynamicService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const super::wsdtypes::WSD_PORT_TYPE, *const super::wsdxmldom::WSDXML_NAME, *const super::wsdxmldom::WSDXML_ELEMENT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    AddDynamicService: usize,
    pub RemoveDynamicService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetServiceDiscoverable: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub SignalEvent: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    SignalEvent: usize,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
pub trait IWSDDeviceHost_Impl: windows_core::IUnknownImpl {
    fn Init(&self, pszlocalid: &windows_core::PCWSTR, pcontext: windows_core::Ref<super::wsdxml::IWSDXMLContext>, pphostaddresses: *const Option<super::wsdbase::IWSDAddress>, dwhostaddresscount: u32) -> windows_core::Result<()>;
    fn Start(&self, ullinstanceid: u64, pscopelist: *const super::wsdtypes::WSD_URI_LIST, pnotificationsink: windows_core::Ref<IWSDDeviceHostNotify>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
    fn RegisterPortType(&self, pporttype: *const super::wsdtypes::WSD_PORT_TYPE) -> windows_core::Result<()>;
    fn SetMetadata(&self, pthismodelmetadata: *const super::wsdtypes::WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const super::wsdtypes::WSD_THIS_DEVICE_METADATA, phostmetadata: *const super::wsdtypes::WSD_HOST_METADATA, pcustommetadata: *const super::wsdtypes::WSD_METADATA_SECTION_LIST) -> windows_core::Result<()>;
    fn RegisterService(&self, pszserviceid: &windows_core::PCWSTR, pservice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RetireService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddDynamicService(&self, pszserviceid: &windows_core::PCWSTR, pszendpointaddress: &windows_core::PCWSTR, pporttype: *const super::wsdtypes::WSD_PORT_TYPE, pportname: *const super::wsdxmldom::WSDXML_NAME, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pservice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveDynamicService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetServiceDiscoverable(&self, pszserviceid: &windows_core::PCWSTR, fdiscoverable: windows_core::BOOL) -> windows_core::Result<()>;
    fn SignalEvent(&self, pszserviceid: &windows_core::PCWSTR, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl IWSDDeviceHost_Vtbl {
    pub const fn new<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Init<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlocalid: windows_core::PCWSTR, pcontext: *mut core::ffi::c_void, pphostaddresses: *const *mut core::ffi::c_void, dwhostaddresscount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::Init(this, core::mem::transmute(&pszlocalid), core::mem::transmute_copy(&pcontext), core::mem::transmute_copy(&pphostaddresses), core::mem::transmute_copy(&dwhostaddresscount)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullinstanceid: u64, pscopelist: *const super::wsdtypes::WSD_URI_LIST, pnotificationsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::Start(this, core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&pscopelist), core::mem::transmute_copy(&pnotificationsink)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Terminate<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::Terminate(this).into()
            }
        }
        unsafe extern "system" fn RegisterPortType<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporttype: *const super::wsdtypes::WSD_PORT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::RegisterPortType(this, core::mem::transmute_copy(&pporttype)).into()
            }
        }
        unsafe extern "system" fn SetMetadata<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthismodelmetadata: *const super::wsdtypes::WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const super::wsdtypes::WSD_THIS_DEVICE_METADATA, phostmetadata: *const super::wsdtypes::WSD_HOST_METADATA, pcustommetadata: *const super::wsdtypes::WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::SetMetadata(this, core::mem::transmute_copy(&pthismodelmetadata), core::mem::transmute_copy(&pthisdevicemetadata), core::mem::transmute_copy(&phostmetadata), core::mem::transmute_copy(&pcustommetadata)).into()
            }
        }
        unsafe extern "system" fn RegisterService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::RegisterService(this, core::mem::transmute(&pszserviceid), core::mem::transmute_copy(&pservice)).into()
            }
        }
        unsafe extern "system" fn RetireService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::RetireService(this, core::mem::transmute(&pszserviceid)).into()
            }
        }
        unsafe extern "system" fn AddDynamicService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, pszendpointaddress: windows_core::PCWSTR, pporttype: *const super::wsdtypes::WSD_PORT_TYPE, pportname: *const super::wsdxmldom::WSDXML_NAME, pany: *const super::wsdxmldom::WSDXML_ELEMENT, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::AddDynamicService(this, core::mem::transmute(&pszserviceid), core::mem::transmute(&pszendpointaddress), core::mem::transmute_copy(&pporttype), core::mem::transmute_copy(&pportname), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&pservice)).into()
            }
        }
        unsafe extern "system" fn RemoveDynamicService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::RemoveDynamicService(this, core::mem::transmute(&pszserviceid)).into()
            }
        }
        unsafe extern "system" fn SetServiceDiscoverable<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, fdiscoverable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::SetServiceDiscoverable(this, core::mem::transmute(&pszserviceid), core::mem::transmute_copy(&fdiscoverable)).into()
            }
        }
        unsafe extern "system" fn SignalEvent<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDDeviceHost_Impl::SignalEvent(this, core::mem::transmute(&pszserviceid), core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Terminate: Terminate::<Identity, OFFSET>,
            RegisterPortType: RegisterPortType::<Identity, OFFSET>,
            SetMetadata: SetMetadata::<Identity, OFFSET>,
            RegisterService: RegisterService::<Identity, OFFSET>,
            RetireService: RetireService::<Identity, OFFSET>,
            AddDynamicService: AddDynamicService::<Identity, OFFSET>,
            RemoveDynamicService: RemoveDynamicService::<Identity, OFFSET>,
            SetServiceDiscoverable: SetServiceDiscoverable::<Identity, OFFSET>,
            SignalEvent: SignalEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDDeviceHost as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxml", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDDeviceHost {}
windows_core::imp::define_interface!(IWSDDeviceHostNotify, IWSDDeviceHostNotify_Vtbl, 0xb5bee9f9_eeda_41fe_96f7_f45e14990fb0);
windows_core::imp::interface_hierarchy!(IWSDDeviceHostNotify, windows_core::IUnknown);
impl IWSDDeviceHostNotify {
    pub unsafe fn GetService<P0>(&self, pszserviceid: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), pszserviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDDeviceHostNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IWSDDeviceHostNotify_Impl: windows_core::IUnknownImpl {
    fn GetService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
}
impl IWSDDeviceHostNotify_Vtbl {
    pub const fn new<Identity: IWSDDeviceHostNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetService<Identity: IWSDDeviceHostNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWSDDeviceHostNotify_Impl::GetService(this, core::mem::transmute(&pszserviceid)) {
                    Ok(ok__) => {
                        ppservice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDDeviceHostNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWSDDeviceHostNotify {}
windows_core::imp::define_interface!(IWSDServiceMessaging, IWSDServiceMessaging_Vtbl, 0x94974cf4_0cab_460d_a3f6_7a0ad623c0e6);
windows_core::imp::interface_hierarchy!(IWSDServiceMessaging, windows_core::IUnknown);
impl IWSDServiceMessaging {
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn SendResponse<P2>(&self, pbody: Option<*const core::ffi::c_void>, poperation: *const super::wsdtypes::WSD_OPERATION, pmessageparameters: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::wsdbase::IWSDMessageParameters>,
    {
        unsafe { (windows_core::Interface::vtable(self).SendResponse)(windows_core::Interface::as_raw(self), pbody.unwrap_or(core::mem::zeroed()) as _, poperation, pmessageparameters.param().abi()) }
    }
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub unsafe fn FaultRequest<P1>(&self, prequestheader: *const super::wsdtypes::WSD_SOAP_HEADER, pmessageparameters: P1, pfault: Option<*const super::wsdtypes::WSD_SOAP_FAULT>) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::wsdbase::IWSDMessageParameters>,
    {
        unsafe { (windows_core::Interface::vtable(self).FaultRequest)(windows_core::Interface::as_raw(self), prequestheader, pmessageparameters.param().abi(), pfault.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWSDServiceMessaging_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub SendResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void, *const super::wsdtypes::WSD_OPERATION, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    SendResponse: usize,
    #[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
    pub FaultRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wsdtypes::WSD_SOAP_HEADER, *mut core::ffi::c_void, *const super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom")))]
    FaultRequest: usize,
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
pub trait IWSDServiceMessaging_Impl: windows_core::IUnknownImpl {
    fn SendResponse(&self, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION, pmessageparameters: windows_core::Ref<super::wsdbase::IWSDMessageParameters>) -> windows_core::Result<()>;
    fn FaultRequest(&self, prequestheader: *const super::wsdtypes::WSD_SOAP_HEADER, pmessageparameters: windows_core::Ref<super::wsdbase::IWSDMessageParameters>, pfault: *const super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl IWSDServiceMessaging_Vtbl {
    pub const fn new<Identity: IWSDServiceMessaging_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendResponse<Identity: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const super::wsdtypes::WSD_OPERATION, pmessageparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceMessaging_Impl::SendResponse(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&pmessageparameters)).into()
            }
        }
        unsafe extern "system" fn FaultRequest<Identity: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestheader: *const super::wsdtypes::WSD_SOAP_HEADER, pmessageparameters: *mut core::ffi::c_void, pfault: *const super::wsdtypes::WSD_SOAP_FAULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWSDServiceMessaging_Impl::FaultRequest(this, core::mem::transmute_copy(&prequestheader), core::mem::transmute_copy(&pmessageparameters), core::mem::transmute_copy(&pfault)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendResponse: SendResponse::<Identity, OFFSET>,
            FaultRequest: FaultRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDServiceMessaging as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_wsdbase", feature = "Win32_wsdtypes", feature = "Win32_wsdxmldom"))]
impl windows_core::RuntimeName for IWSDServiceMessaging {}
