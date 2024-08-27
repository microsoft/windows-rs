pub trait IWSDAddress_Impl: Sized + windows_core::IUnknownImpl {
    fn Serialize(&self, pszbuffer: windows_core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn Deserialize(&self, pszbuffer: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDAddress {}
impl IWSDAddress_Vtbl {
    pub const fn new<Identity: IWSDAddress_Impl, const OFFSET: isize>() -> IWSDAddress_Vtbl {
        unsafe extern "system" fn Serialize<Identity: IWSDAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuffer: windows_core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDAddress_Impl::Serialize(this, core::mem::transmute_copy(&pszbuffer), core::mem::transmute_copy(&cchlength), core::mem::transmute_copy(&fsafe)).into()
        }
        unsafe extern "system" fn Deserialize<Identity: IWSDAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszbuffer: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDAddress_Impl::Deserialize(this, core::mem::transmute(&pszbuffer)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, OFFSET>,
            Deserialize: Deserialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAddress as windows_core::Interface>::IID
    }
}
pub trait IWSDAsyncCallback_Impl: Sized + windows_core::IUnknownImpl {
    fn AsyncOperationComplete(&self, pasyncresult: Option<&IWSDAsyncResult>, pasyncstate: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDAsyncCallback {}
impl IWSDAsyncCallback_Vtbl {
    pub const fn new<Identity: IWSDAsyncCallback_Impl, const OFFSET: isize>() -> IWSDAsyncCallback_Vtbl {
        unsafe extern "system" fn AsyncOperationComplete<Identity: IWSDAsyncCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void, pasyncstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDAsyncCallback_Impl::AsyncOperationComplete(this, windows_core::from_raw_borrowed(&pasyncresult), windows_core::from_raw_borrowed(&pasyncstate)).into()
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAsyncCallback as windows_core::Interface>::IID
    }
}
pub trait IWSDAsyncResult_Impl: Sized + windows_core::IUnknownImpl {
    fn SetCallback(&self, pcallback: Option<&IWSDAsyncCallback>, pasyncstate: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn SetWaitHandle(&self, hwaithandle: super::super::Foundation::HANDLE) -> windows_core::Result<()>;
    fn HasCompleted(&self) -> windows_core::Result<()>;
    fn GetAsyncState(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Abort(&self) -> windows_core::Result<()>;
    fn GetEvent(&self, pevent: *mut WSD_EVENT) -> windows_core::Result<()>;
    fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy>;
}
impl windows_core::RuntimeName for IWSDAsyncResult {}
impl IWSDAsyncResult_Vtbl {
    pub const fn new<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>() -> IWSDAsyncResult_Vtbl {
        unsafe extern "system" fn SetCallback<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pasyncstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDAsyncResult_Impl::SetCallback(this, windows_core::from_raw_borrowed(&pcallback), windows_core::from_raw_borrowed(&pasyncstate)).into()
        }
        unsafe extern "system" fn SetWaitHandle<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwaithandle: super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDAsyncResult_Impl::SetWaitHandle(this, core::mem::transmute_copy(&hwaithandle)).into()
        }
        unsafe extern "system" fn HasCompleted<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDAsyncResult_Impl::HasCompleted(this).into()
        }
        unsafe extern "system" fn GetAsyncState<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppasyncstate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDAsyncResult_Impl::GetAsyncState(this) {
                Ok(ok__) => {
                    ppasyncstate.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDAsyncResult_Impl::Abort(this).into()
        }
        unsafe extern "system" fn GetEvent<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut WSD_EVENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDAsyncResult_Impl::GetEvent(this, core::mem::transmute_copy(&pevent)).into()
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppendpoint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDAsyncResult_Impl::GetEndpointProxy(this) {
                Ok(ok__) => {
                    ppendpoint.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IWSDAttachment_Impl: Sized + windows_core::IUnknownImpl {}
impl windows_core::RuntimeName for IWSDAttachment {}
impl IWSDAttachment_Vtbl {
    pub const fn new<Identity: IWSDAttachment_Impl, const OFFSET: isize>() -> IWSDAttachment_Vtbl {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
pub trait IWSDDeviceHost_Impl: Sized + windows_core::IUnknownImpl {
    fn Init(&self, pszlocalid: &windows_core::PCWSTR, pcontext: Option<&IWSDXMLContext>, pphostaddresses: *const Option<IWSDAddress>, dwhostaddresscount: u32) -> windows_core::Result<()>;
    fn Start(&self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: Option<&IWSDDeviceHostNotify>) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Terminate(&self) -> windows_core::Result<()>;
    fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> windows_core::Result<()>;
    fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> windows_core::Result<()>;
    fn RegisterService(&self, pszserviceid: &windows_core::PCWSTR, pservice: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RetireService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddDynamicService(&self, pszserviceid: &windows_core::PCWSTR, pszendpointaddress: &windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn RemoveDynamicService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetServiceDiscoverable(&self, pszserviceid: &windows_core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn SignalEvent(&self, pszserviceid: &windows_core::PCWSTR, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDDeviceHost {}
impl IWSDDeviceHost_Vtbl {
    pub const fn new<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>() -> IWSDDeviceHost_Vtbl {
        unsafe extern "system" fn Init<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszlocalid: windows_core::PCWSTR, pcontext: *mut core::ffi::c_void, pphostaddresses: *const *mut core::ffi::c_void, dwhostaddresscount: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::Init(this, core::mem::transmute(&pszlocalid), windows_core::from_raw_borrowed(&pcontext), core::mem::transmute_copy(&pphostaddresses), core::mem::transmute_copy(&dwhostaddresscount)).into()
        }
        unsafe extern "system" fn Start<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::Start(this, core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&pscopelist), windows_core::from_raw_borrowed(&pnotificationsink)).into()
        }
        unsafe extern "system" fn Stop<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::Stop(this).into()
        }
        unsafe extern "system" fn Terminate<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::Terminate(this).into()
        }
        unsafe extern "system" fn RegisterPortType<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::RegisterPortType(this, core::mem::transmute_copy(&pporttype)).into()
        }
        unsafe extern "system" fn SetMetadata<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::SetMetadata(this, core::mem::transmute_copy(&pthismodelmetadata), core::mem::transmute_copy(&pthisdevicemetadata), core::mem::transmute_copy(&phostmetadata), core::mem::transmute_copy(&pcustommetadata)).into()
        }
        unsafe extern "system" fn RegisterService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::RegisterService(this, core::mem::transmute(&pszserviceid), windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn RetireService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::RetireService(this, core::mem::transmute(&pszserviceid)).into()
        }
        unsafe extern "system" fn AddDynamicService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, pszendpointaddress: windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::AddDynamicService(this, core::mem::transmute(&pszserviceid), core::mem::transmute(&pszendpointaddress), core::mem::transmute_copy(&pporttype), core::mem::transmute_copy(&pportname), core::mem::transmute_copy(&pany), windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn RemoveDynamicService<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::RemoveDynamicService(this, core::mem::transmute(&pszserviceid)).into()
        }
        unsafe extern "system" fn SetServiceDiscoverable<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::SetServiceDiscoverable(this, core::mem::transmute(&pszserviceid), core::mem::transmute_copy(&fdiscoverable)).into()
        }
        unsafe extern "system" fn SignalEvent<Identity: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceHost_Impl::SignalEvent(this, core::mem::transmute(&pszserviceid), core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation)).into()
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
pub trait IWSDDeviceHostNotify_Impl: Sized + windows_core::IUnknownImpl {
    fn GetService(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<windows_core::IUnknown>;
}
impl windows_core::RuntimeName for IWSDDeviceHostNotify {}
impl IWSDDeviceHostNotify_Vtbl {
    pub const fn new<Identity: IWSDDeviceHostNotify_Impl, const OFFSET: isize>() -> IWSDDeviceHostNotify_Vtbl {
        unsafe extern "system" fn GetService<Identity: IWSDDeviceHostNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, ppservice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDDeviceHostNotify_Impl::GetService(this, core::mem::transmute(&pszserviceid)) {
                Ok(ok__) => {
                    ppservice.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDDeviceHostNotify as windows_core::Interface>::IID
    }
}
pub trait IWSDDeviceProxy_Impl: Sized + windows_core::IUnknownImpl {
    fn Init(&self, pszdeviceid: &windows_core::PCWSTR, pdeviceaddress: Option<&IWSDAddress>, pszlocalid: &windows_core::PCWSTR, pcontext: Option<&IWSDXMLContext>, psponsor: Option<&IWSDDeviceProxy>) -> windows_core::Result<()>;
    fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&self, presult: Option<&IWSDAsyncResult>) -> windows_core::Result<()>;
    fn GetHostMetadata(&self) -> windows_core::Result<*mut WSD_HOST_METADATA>;
    fn GetThisModelMetadata(&self) -> windows_core::Result<*mut WSD_THIS_MODEL_METADATA>;
    fn GetThisDeviceMetadata(&self) -> windows_core::Result<*mut WSD_THIS_DEVICE_METADATA>;
    fn GetAllMetadata(&self) -> windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceProxyById(&self, pszserviceid: &windows_core::PCWSTR) -> windows_core::Result<IWSDServiceProxy>;
    fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> windows_core::Result<IWSDServiceProxy>;
    fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy>;
}
impl windows_core::RuntimeName for IWSDDeviceProxy {}
impl IWSDDeviceProxy_Vtbl {
    pub const fn new<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>() -> IWSDDeviceProxy_Vtbl {
        unsafe extern "system" fn Init<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdeviceid: windows_core::PCWSTR, pdeviceaddress: *mut core::ffi::c_void, pszlocalid: windows_core::PCWSTR, pcontext: *mut core::ffi::c_void, psponsor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceProxy_Impl::Init(this, core::mem::transmute(&pszdeviceid), windows_core::from_raw_borrowed(&pdeviceaddress), core::mem::transmute(&pszlocalid), windows_core::from_raw_borrowed(&pcontext), windows_core::from_raw_borrowed(&psponsor)).into()
        }
        unsafe extern "system" fn BeginGetMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDDeviceProxy_Impl::BeginGetMetadata(this) {
                Ok(ok__) => {
                    ppresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDDeviceProxy_Impl::EndGetMetadata(this, windows_core::from_raw_borrowed(&presult)).into()
        }
        unsafe extern "system" fn GetHostMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDDeviceProxy_Impl::GetHostMetadata(this) {
                Ok(ok__) => {
                    pphostmetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisModelMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDDeviceProxy_Impl::GetThisModelMetadata(this) {
                Ok(ok__) => {
                    ppmanufacturermetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisDeviceMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDDeviceProxy_Impl::GetThisDeviceMetadata(this) {
                Ok(ok__) => {
                    ppthisdevicemetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllMetadata<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDDeviceProxy_Impl::GetAllMetadata(this) {
                Ok(ok__) => {
                    ppmetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyById<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszserviceid: windows_core::PCWSTR, ppserviceproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDDeviceProxy_Impl::GetServiceProxyById(this, core::mem::transmute(&pszserviceid)) {
                Ok(ok__) => {
                    ppserviceproxy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyByType<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDDeviceProxy_Impl::GetServiceProxyByType(this, core::mem::transmute_copy(&ptype)) {
                Ok(ok__) => {
                    ppserviceproxy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDDeviceProxy_Impl::GetEndpointProxy(this) {
                Ok(ok__) => {
                    ppproxy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IWSDEndpointProxy_Impl: Sized + windows_core::IUnknownImpl {
    fn SendOneWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::Result<()>;
    fn SendTwoWayRequest(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> windows_core::Result<()>;
    fn SendTwoWayRequestAsync(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: Option<&windows_core::IUnknown>, pcallback: Option<&IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn AbortAsyncOperation(&self, pasyncresult: Option<&IWSDAsyncResult>) -> windows_core::Result<()>;
    fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> windows_core::Result<()>;
    fn GetErrorInfo(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetFaultInfo(&self) -> windows_core::Result<*mut WSD_SOAP_FAULT>;
}
impl windows_core::RuntimeName for IWSDEndpointProxy {}
impl IWSDEndpointProxy_Vtbl {
    pub const fn new<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>() -> IWSDEndpointProxy_Vtbl {
        unsafe extern "system" fn SendOneWayRequest<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDEndpointProxy_Impl::SendOneWayRequest(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation)).into()
        }
        unsafe extern "system" fn SendTwoWayRequest<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDEndpointProxy_Impl::SendTwoWayRequest(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation), core::mem::transmute_copy(&presponsecontext)).into()
        }
        unsafe extern "system" fn SendTwoWayRequestAsync<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, presult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDEndpointProxy_Impl::SendTwoWayRequestAsync(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation), windows_core::from_raw_borrowed(&pasyncstate), windows_core::from_raw_borrowed(&pcallback)) {
                Ok(ok__) => {
                    presult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAsyncOperation<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pasyncresult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDEndpointProxy_Impl::AbortAsyncOperation(this, windows_core::from_raw_borrowed(&pasyncresult)).into()
        }
        unsafe extern "system" fn ProcessFault<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDEndpointProxy_Impl::ProcessFault(this, core::mem::transmute_copy(&pfault)).into()
        }
        unsafe extern "system" fn GetErrorInfo<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszerrorinfo: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDEndpointProxy_Impl::GetErrorInfo(this) {
                Ok(ok__) => {
                    ppszerrorinfo.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaultInfo<Identity: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDEndpointProxy_Impl::GetFaultInfo(this) {
                Ok(ok__) => {
                    ppfault.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IWSDEventingStatus_Impl: Sized + windows_core::IUnknownImpl {
    fn SubscriptionRenewed(&self, pszsubscriptionaction: &windows_core::PCWSTR);
    fn SubscriptionRenewalFailed(&self, pszsubscriptionaction: &windows_core::PCWSTR, hr: windows_core::HRESULT);
    fn SubscriptionEnded(&self, pszsubscriptionaction: &windows_core::PCWSTR);
}
impl windows_core::RuntimeName for IWSDEventingStatus {}
impl IWSDEventingStatus_Vtbl {
    pub const fn new<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>() -> IWSDEventingStatus_Vtbl {
        unsafe extern "system" fn SubscriptionRenewed<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDEventingStatus_Impl::SubscriptionRenewed(this, core::mem::transmute(&pszsubscriptionaction))
        }
        unsafe extern "system" fn SubscriptionRenewalFailed<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR, hr: windows_core::HRESULT) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDEventingStatus_Impl::SubscriptionRenewalFailed(this, core::mem::transmute(&pszsubscriptionaction), core::mem::transmute_copy(&hr))
        }
        unsafe extern "system" fn SubscriptionEnded<Identity: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszsubscriptionaction: windows_core::PCWSTR) {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDEventingStatus_Impl::SubscriptionEnded(this, core::mem::transmute(&pszsubscriptionaction))
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
pub trait IWSDHttpAddress_Impl: Sized + IWSDTransportAddress_Impl {
    fn GetSecure(&self) -> windows_core::Result<()>;
    fn SetSecure(&self, fsecure: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetPath(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetPath(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDHttpAddress {}
impl IWSDHttpAddress_Vtbl {
    pub const fn new<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>() -> IWSDHttpAddress_Vtbl {
        unsafe extern "system" fn GetSecure<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDHttpAddress_Impl::GetSecure(this).into()
        }
        unsafe extern "system" fn SetSecure<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsecure: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDHttpAddress_Impl::SetSecure(this, core::mem::transmute_copy(&fsecure)).into()
        }
        unsafe extern "system" fn GetPath<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszpath: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDHttpAddress_Impl::GetPath(this) {
                Ok(ok__) => {
                    ppszpath.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDHttpAddress_Impl::SetPath(this, core::mem::transmute(&pszpath)).into()
        }
        Self {
            base__: IWSDTransportAddress_Vtbl::new::<Identity, OFFSET>(),
            GetSecure: GetSecure::<Identity, OFFSET>,
            SetSecure: SetSecure::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            SetPath: SetPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDHttpAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID || iid == &<IWSDTransportAddress as windows_core::Interface>::IID
    }
}
pub trait IWSDHttpAuthParameters_Impl: Sized + windows_core::IUnknownImpl {
    fn GetClientAccessToken(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetAuthType(&self) -> windows_core::Result<u32>;
}
impl windows_core::RuntimeName for IWSDHttpAuthParameters {}
impl IWSDHttpAuthParameters_Vtbl {
    pub const fn new<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>() -> IWSDHttpAuthParameters_Vtbl {
        unsafe extern "system" fn GetClientAccessToken<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDHttpAuthParameters_Impl::GetClientAccessToken(this) {
                Ok(ok__) => {
                    phtoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthType<Identity: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pauthtype: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDHttpAuthParameters_Impl::GetAuthType(this) {
                Ok(ok__) => {
                    pauthtype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientAccessToken: GetClientAccessToken::<Identity, OFFSET>,
            GetAuthType: GetAuthType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDHttpAuthParameters as windows_core::Interface>::IID
    }
}
pub trait IWSDHttpMessageParameters_Impl: Sized + IWSDMessageParameters_Impl {
    fn SetInboundHttpHeaders(&self, pszheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetInboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetOutboundHttpHeaders(&self, pszheaders: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetOutboundHttpHeaders(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetID(&self, pszid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetID(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetContext(&self, pcontext: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetContext(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDHttpMessageParameters {}
impl IWSDHttpMessageParameters_Vtbl {
    pub const fn new<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>() -> IWSDHttpMessageParameters_Vtbl {
        unsafe extern "system" fn SetInboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDHttpMessageParameters_Impl::SetInboundHttpHeaders(this, core::mem::transmute(&pszheaders)).into()
        }
        unsafe extern "system" fn GetInboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszheaders: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDHttpMessageParameters_Impl::GetInboundHttpHeaders(this) {
                Ok(ok__) => {
                    ppszheaders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszheaders: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDHttpMessageParameters_Impl::SetOutboundHttpHeaders(this, core::mem::transmute(&pszheaders)).into()
        }
        unsafe extern "system" fn GetOutboundHttpHeaders<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszheaders: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDHttpMessageParameters_Impl::GetOutboundHttpHeaders(this) {
                Ok(ok__) => {
                    ppszheaders.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetID<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDHttpMessageParameters_Impl::SetID(this, core::mem::transmute(&pszid)).into()
        }
        unsafe extern "system" fn GetID<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszid: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDHttpMessageParameters_Impl::GetID(this) {
                Ok(ok__) => {
                    ppszid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcontext: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDHttpMessageParameters_Impl::SetContext(this, windows_core::from_raw_borrowed(&pcontext)).into()
        }
        unsafe extern "system" fn GetContext<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDHttpMessageParameters_Impl::GetContext(this) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDHttpMessageParameters_Impl::Clear(this).into()
        }
        Self {
            base__: IWSDMessageParameters_Vtbl::new::<Identity, OFFSET>(),
            SetInboundHttpHeaders: SetInboundHttpHeaders::<Identity, OFFSET>,
            GetInboundHttpHeaders: GetInboundHttpHeaders::<Identity, OFFSET>,
            SetOutboundHttpHeaders: SetOutboundHttpHeaders::<Identity, OFFSET>,
            GetOutboundHttpHeaders: GetOutboundHttpHeaders::<Identity, OFFSET>,
            SetID: SetID::<Identity, OFFSET>,
            GetID: GetID::<Identity, OFFSET>,
            SetContext: SetContext::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDHttpMessageParameters as windows_core::Interface>::IID || iid == &<IWSDMessageParameters as windows_core::Interface>::IID
    }
}
pub trait IWSDInboundAttachment_Impl: Sized + IWSDAttachment_Impl {
    fn Read(&self, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDInboundAttachment {}
impl IWSDInboundAttachment_Vtbl {
    pub const fn new<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>() -> IWSDInboundAttachment_Vtbl {
        unsafe extern "system" fn Read<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDInboundAttachment_Impl::Read(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbytestoread), core::mem::transmute_copy(&pdwnumberofbytesread)).into()
        }
        unsafe extern "system" fn Close<Identity: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDInboundAttachment_Impl::Close(this).into()
        }
        Self { base__: IWSDAttachment_Vtbl::new::<Identity, OFFSET>(), Read: Read::<Identity, OFFSET>, Close: Close::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDInboundAttachment as windows_core::Interface>::IID || iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
pub trait IWSDMessageParameters_Impl: Sized + windows_core::IUnknownImpl {
    fn GetLocalAddress(&self) -> windows_core::Result<IWSDAddress>;
    fn SetLocalAddress(&self, paddress: Option<&IWSDAddress>) -> windows_core::Result<()>;
    fn GetRemoteAddress(&self) -> windows_core::Result<IWSDAddress>;
    fn SetRemoteAddress(&self, paddress: Option<&IWSDAddress>) -> windows_core::Result<()>;
    fn GetLowerParameters(&self) -> windows_core::Result<IWSDMessageParameters>;
}
impl windows_core::RuntimeName for IWSDMessageParameters {}
impl IWSDMessageParameters_Vtbl {
    pub const fn new<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>() -> IWSDMessageParameters_Vtbl {
        unsafe extern "system" fn GetLocalAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDMessageParameters_Impl::GetLocalAddress(this) {
                Ok(ok__) => {
                    ppaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDMessageParameters_Impl::SetLocalAddress(this, windows_core::from_raw_borrowed(&paddress)).into()
        }
        unsafe extern "system" fn GetRemoteAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaddress: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDMessageParameters_Impl::GetRemoteAddress(this) {
                Ok(ok__) => {
                    ppaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddress<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paddress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDMessageParameters_Impl::SetRemoteAddress(this, windows_core::from_raw_borrowed(&paddress)).into()
        }
        unsafe extern "system" fn GetLowerParameters<Identity: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptxparams: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDMessageParameters_Impl::GetLowerParameters(this) {
                Ok(ok__) => {
                    pptxparams.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLocalAddress: GetLocalAddress::<Identity, OFFSET>,
            SetLocalAddress: SetLocalAddress::<Identity, OFFSET>,
            GetRemoteAddress: GetRemoteAddress::<Identity, OFFSET>,
            SetRemoteAddress: SetRemoteAddress::<Identity, OFFSET>,
            GetLowerParameters: GetLowerParameters::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDMessageParameters as windows_core::Interface>::IID
    }
}
pub trait IWSDMetadataExchange_Impl: Sized + windows_core::IUnknownImpl {
    fn GetMetadata(&self) -> windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
}
impl windows_core::RuntimeName for IWSDMetadataExchange {}
impl IWSDMetadataExchange_Vtbl {
    pub const fn new<Identity: IWSDMetadataExchange_Impl, const OFFSET: isize>() -> IWSDMetadataExchange_Vtbl {
        unsafe extern "system" fn GetMetadata<Identity: IWSDMetadataExchange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDMetadataExchange_Impl::GetMetadata(this) {
                Ok(ok__) => {
                    metadataout.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMetadata: GetMetadata::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDMetadataExchange as windows_core::Interface>::IID
    }
}
pub trait IWSDOutboundAttachment_Impl: Sized + IWSDAttachment_Impl {
    fn Write(&self, pbuffer: *const u8, dwbytestowrite: u32) -> windows_core::Result<u32>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Abort(&self) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDOutboundAttachment {}
impl IWSDOutboundAttachment_Vtbl {
    pub const fn new<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>() -> IWSDOutboundAttachment_Vtbl {
        unsafe extern "system" fn Write<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDOutboundAttachment_Impl::Write(this, core::mem::transmute_copy(&pbuffer), core::mem::transmute_copy(&dwbytestowrite)) {
                Ok(ok__) => {
                    pdwnumberofbyteswritten.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDOutboundAttachment_Impl::Close(this).into()
        }
        unsafe extern "system" fn Abort<Identity: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDOutboundAttachment_Impl::Abort(this).into()
        }
        Self {
            base__: IWSDAttachment_Vtbl::new::<Identity, OFFSET>(),
            Write: Write::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Abort: Abort::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDOutboundAttachment as windows_core::Interface>::IID || iid == &<IWSDAttachment as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
pub trait IWSDSSLClientCertificate_Impl: Sized + windows_core::IUnknownImpl {
    fn GetClientCertificate(&self) -> windows_core::Result<*mut super::super::Security::Cryptography::CERT_CONTEXT>;
    fn GetMappedAccessToken(&self) -> windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl windows_core::RuntimeName for IWSDSSLClientCertificate {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl IWSDSSLClientCertificate_Vtbl {
    pub const fn new<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>() -> IWSDSSLClientCertificate_Vtbl {
        unsafe extern "system" fn GetClientCertificate<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDSSLClientCertificate_Impl::GetClientCertificate(this) {
                Ok(ok__) => {
                    ppcertcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMappedAccessToken<Identity: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDSSLClientCertificate_Impl::GetMappedAccessToken(this) {
                Ok(ok__) => {
                    phtoken.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientCertificate: GetClientCertificate::<Identity, OFFSET>,
            GetMappedAccessToken: GetMappedAccessToken::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDSSLClientCertificate as windows_core::Interface>::IID
    }
}
pub trait IWSDScopeMatchingRule_Impl: Sized + windows_core::IUnknownImpl {
    fn GetScopeRule(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn MatchScopes(&self, pszscope1: &windows_core::PCWSTR, pszscope2: &windows_core::PCWSTR) -> windows_core::Result<super::super::Foundation::BOOL>;
}
impl windows_core::RuntimeName for IWSDScopeMatchingRule {}
impl IWSDScopeMatchingRule_Vtbl {
    pub const fn new<Identity: IWSDScopeMatchingRule_Impl, const OFFSET: isize>() -> IWSDScopeMatchingRule_Vtbl {
        unsafe extern "system" fn GetScopeRule<Identity: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszscopematchingrule: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDScopeMatchingRule_Impl::GetScopeRule(this) {
                Ok(ok__) => {
                    ppszscopematchingrule.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchScopes<Identity: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszscope1: windows_core::PCWSTR, pszscope2: windows_core::PCWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDScopeMatchingRule_Impl::MatchScopes(this, core::mem::transmute(&pszscope1), core::mem::transmute(&pszscope2)) {
                Ok(ok__) => {
                    pfmatch.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetScopeRule: GetScopeRule::<Identity, OFFSET>,
            MatchScopes: MatchScopes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDScopeMatchingRule as windows_core::Interface>::IID
    }
}
pub trait IWSDServiceMessaging_Impl: Sized + windows_core::IUnknownImpl {
    fn SendResponse(&self, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: Option<&IWSDMessageParameters>) -> windows_core::Result<()>;
    fn FaultRequest(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: Option<&IWSDMessageParameters>, pfault: *const WSD_SOAP_FAULT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDServiceMessaging {}
impl IWSDServiceMessaging_Vtbl {
    pub const fn new<Identity: IWSDServiceMessaging_Impl, const OFFSET: isize>() -> IWSDServiceMessaging_Vtbl {
        unsafe extern "system" fn SendResponse<Identity: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbody: *const core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceMessaging_Impl::SendResponse(this, core::mem::transmute_copy(&pbody), core::mem::transmute_copy(&poperation), windows_core::from_raw_borrowed(&pmessageparameters)).into()
        }
        unsafe extern "system" fn FaultRequest<Identity: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: *mut core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceMessaging_Impl::FaultRequest(this, core::mem::transmute_copy(&prequestheader), windows_core::from_raw_borrowed(&pmessageparameters), core::mem::transmute_copy(&pfault)).into()
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
pub trait IWSDServiceProxy_Impl: Sized + IWSDMetadataExchange_Impl {
    fn BeginGetMetadata(&self) -> windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&self, presult: Option<&IWSDAsyncResult>) -> windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceMetadata(&self) -> windows_core::Result<*mut WSD_SERVICE_METADATA>;
    fn SubscribeToOperation(&self, poperation: *const WSD_OPERATION, punknown: Option<&windows_core::IUnknown>, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> windows_core::Result<()>;
    fn SetEventingStatusCallback(&self, pstatus: Option<&IWSDEventingStatus>) -> windows_core::Result<()>;
    fn GetEndpointProxy(&self) -> windows_core::Result<IWSDEndpointProxy>;
}
impl windows_core::RuntimeName for IWSDServiceProxy {}
impl IWSDServiceProxy_Vtbl {
    pub const fn new<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>() -> IWSDServiceProxy_Vtbl {
        unsafe extern "system" fn BeginGetMetadata<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDServiceProxy_Impl::BeginGetMetadata(this) {
                Ok(ok__) => {
                    ppresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDServiceProxy_Impl::EndGetMetadata(this, windows_core::from_raw_borrowed(&presult)) {
                Ok(ok__) => {
                    ppmetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceMetadata<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDServiceProxy_Impl::GetServiceMetadata(this) {
                Ok(ok__) => {
                    ppservicemetadata.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscribeToOperation<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxy_Impl::SubscribeToOperation(this, core::mem::transmute_copy(&poperation), windows_core::from_raw_borrowed(&punknown), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn UnsubscribeToOperation<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperation: *const WSD_OPERATION) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxy_Impl::UnsubscribeToOperation(this, core::mem::transmute_copy(&poperation)).into()
        }
        unsafe extern "system" fn SetEventingStatusCallback<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxy_Impl::SetEventingStatusCallback(this, windows_core::from_raw_borrowed(&pstatus)).into()
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproxy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDServiceProxy_Impl::GetEndpointProxy(this) {
                Ok(ok__) => {
                    ppproxy.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
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
pub trait IWSDServiceProxyEventing_Impl: Sized + IWSDServiceProxy_Impl {
    fn SubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: Option<&windows_core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginSubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: Option<&windows_core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: Option<&windows_core::IUnknown>, pasynccallback: Option<&IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndSubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Option<&IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn UnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginUnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: Option<&windows_core::IUnknown>, pasynccallback: Option<&IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndUnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Option<&IWSDAsyncResult>) -> windows_core::Result<()>;
    fn RenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginRenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: Option<&windows_core::IUnknown>, pasynccallback: Option<&IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndRenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Option<&IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn GetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn BeginGetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: Option<&windows_core::IUnknown>, pasynccallback: Option<&IWSDAsyncCallback>) -> windows_core::Result<IWSDAsyncResult>;
    fn EndGetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: Option<&IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDServiceProxyEventing {}
impl IWSDServiceProxyEventing_Vtbl {
    pub const fn new<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>() -> IWSDServiceProxyEventing_Vtbl {
        unsafe extern "system" fn SubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxyEventing_Impl::SubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), windows_core::from_raw_borrowed(&punknown), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginSubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDServiceProxyEventing_Impl::BeginSubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), windows_core::from_raw_borrowed(&punknown), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), windows_core::from_raw_borrowed(&pasyncstate), windows_core::from_raw_borrowed(&pasynccallback)) {
                Ok(ok__) => {
                    ppresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxyEventing_Impl::EndSubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), windows_core::from_raw_borrowed(&presult), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn UnsubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxyEventing_Impl::UnsubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany)).into()
        }
        unsafe extern "system" fn BeginUnsubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDServiceProxyEventing_Impl::BeginUnsubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany), windows_core::from_raw_borrowed(&pasyncstate), windows_core::from_raw_borrowed(&pasynccallback)) {
                Ok(ok__) => {
                    ppresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnsubscribeToMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxyEventing_Impl::EndUnsubscribeToMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), windows_core::from_raw_borrowed(&presult)).into()
        }
        unsafe extern "system" fn RenewMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxyEventing_Impl::RenewMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginRenewMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDServiceProxyEventing_Impl::BeginRenewMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pexpires), core::mem::transmute_copy(&pany), windows_core::from_raw_borrowed(&pasyncstate), windows_core::from_raw_borrowed(&pasynccallback)) {
                Ok(ok__) => {
                    ppresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRenewMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxyEventing_Impl::EndRenewMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), windows_core::from_raw_borrowed(&presult), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn GetStatusForMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxyEventing_Impl::GetStatusForMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginGetStatusForMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut core::ffi::c_void, pasynccallback: *mut core::ffi::c_void, ppresult: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDServiceProxyEventing_Impl::BeginGetStatusForMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), core::mem::transmute_copy(&pany), windows_core::from_raw_borrowed(&pasyncstate), windows_core::from_raw_borrowed(&pasynccallback)) {
                Ok(ok__) => {
                    ppresult.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetStatusForMultipleOperations<Identity: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDServiceProxyEventing_Impl::EndGetStatusForMultipleOperations(this, core::mem::transmute_copy(&poperations), core::mem::transmute_copy(&dwoperationcount), windows_core::from_raw_borrowed(&presult), core::mem::transmute_copy(&ppexpires), core::mem::transmute_copy(&ppany)).into()
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
pub trait IWSDSignatureProperty_Impl: Sized + windows_core::IUnknownImpl {
    fn IsMessageSigned(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn IsMessageSignatureTrusted(&self) -> windows_core::Result<super::super::Foundation::BOOL>;
    fn GetKeyInfo(&self, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> windows_core::Result<()>;
    fn GetSignature(&self, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> windows_core::Result<()>;
    fn GetSignedInfoHash(&self, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDSignatureProperty {}
impl IWSDSignatureProperty_Vtbl {
    pub const fn new<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>() -> IWSDSignatureProperty_Vtbl {
        unsafe extern "system" fn IsMessageSigned<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsigned: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDSignatureProperty_Impl::IsMessageSigned(this) {
                Ok(ok__) => {
                    pbsigned.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMessageSignatureTrusted<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDSignatureProperty_Impl::IsMessageSignatureTrusted(this) {
                Ok(ok__) => {
                    pbsignaturetrusted.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyInfo<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDSignatureProperty_Impl::GetKeyInfo(this, core::mem::transmute_copy(&pbkeyinfo), core::mem::transmute_copy(&pdwkeyinfosize)).into()
        }
        unsafe extern "system" fn GetSignature<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDSignatureProperty_Impl::GetSignature(this, core::mem::transmute_copy(&pbsignature), core::mem::transmute_copy(&pdwsignaturesize)).into()
        }
        unsafe extern "system" fn GetSignedInfoHash<Identity: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDSignatureProperty_Impl::GetSignedInfoHash(this, core::mem::transmute_copy(&pbsignedinfohash), core::mem::transmute_copy(&pdwhashsize)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsMessageSigned: IsMessageSigned::<Identity, OFFSET>,
            IsMessageSignatureTrusted: IsMessageSignatureTrusted::<Identity, OFFSET>,
            GetKeyInfo: GetKeyInfo::<Identity, OFFSET>,
            GetSignature: GetSignature::<Identity, OFFSET>,
            GetSignedInfoHash: GetSignedInfoHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDSignatureProperty as windows_core::Interface>::IID
    }
}
pub trait IWSDTransportAddress_Impl: Sized + IWSDAddress_Impl {
    fn GetPort(&self) -> windows_core::Result<u16>;
    fn SetPort(&self, wport: u16) -> windows_core::Result<()>;
    fn GetTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetTransportAddressEx(&self, fsafe: super::super::Foundation::BOOL) -> windows_core::Result<windows_core::PCWSTR>;
    fn SetTransportAddress(&self, pszaddress: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDTransportAddress {}
impl IWSDTransportAddress_Vtbl {
    pub const fn new<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>() -> IWSDTransportAddress_Vtbl {
        unsafe extern "system" fn GetPort<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwport: *mut u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDTransportAddress_Impl::GetPort(this) {
                Ok(ok__) => {
                    pwport.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wport: u16) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDTransportAddress_Impl::SetPort(this, core::mem::transmute_copy(&wport)).into()
        }
        unsafe extern "system" fn GetTransportAddress<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDTransportAddress_Impl::GetTransportAddress(this) {
                Ok(ok__) => {
                    ppszaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransportAddressEx<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDTransportAddress_Impl::GetTransportAddressEx(this, core::mem::transmute_copy(&fsafe)) {
                Ok(ok__) => {
                    ppszaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportAddress<Identity: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaddress: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDTransportAddress_Impl::SetTransportAddress(this, core::mem::transmute(&pszaddress)).into()
        }
        Self {
            base__: IWSDAddress_Vtbl::new::<Identity, OFFSET>(),
            GetPort: GetPort::<Identity, OFFSET>,
            SetPort: SetPort::<Identity, OFFSET>,
            GetTransportAddress: GetTransportAddress::<Identity, OFFSET>,
            GetTransportAddressEx: GetTransportAddressEx::<Identity, OFFSET>,
            SetTransportAddress: SetTransportAddress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDTransportAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
pub trait IWSDUdpAddress_Impl: Sized + IWSDTransportAddress_Impl {
    fn SetSockaddr(&self, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> windows_core::Result<()>;
    fn GetSockaddr(&self, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> windows_core::Result<()>;
    fn SetExclusive(&self, fexclusive: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetExclusive(&self) -> windows_core::Result<()>;
    fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> windows_core::Result<()>;
    fn GetMessageType(&self) -> windows_core::Result<WSDUdpMessageType>;
    fn SetTTL(&self, dwttl: u32) -> windows_core::Result<()>;
    fn GetTTL(&self) -> windows_core::Result<u32>;
    fn SetAlias(&self, palias: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetAlias(&self) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl windows_core::RuntimeName for IWSDUdpAddress {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl IWSDUdpAddress_Vtbl {
    pub const fn new<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>() -> IWSDUdpAddress_Vtbl {
        unsafe extern "system" fn SetSockaddr<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDUdpAddress_Impl::SetSockaddr(this, core::mem::transmute_copy(&psockaddr)).into()
        }
        unsafe extern "system" fn GetSockaddr<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDUdpAddress_Impl::GetSockaddr(this, core::mem::transmute_copy(&psockaddr)).into()
        }
        unsafe extern "system" fn SetExclusive<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fexclusive: super::super::Foundation::BOOL) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDUdpAddress_Impl::SetExclusive(this, core::mem::transmute_copy(&fexclusive)).into()
        }
        unsafe extern "system" fn GetExclusive<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDUdpAddress_Impl::GetExclusive(this).into()
        }
        unsafe extern "system" fn SetMessageType<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, messagetype: WSDUdpMessageType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDUdpAddress_Impl::SetMessageType(this, core::mem::transmute_copy(&messagetype)).into()
        }
        unsafe extern "system" fn GetMessageType<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDUdpAddress_Impl::GetMessageType(this) {
                Ok(ok__) => {
                    pmessagetype.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTTL<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwttl: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDUdpAddress_Impl::SetTTL(this, core::mem::transmute_copy(&dwttl)).into()
        }
        unsafe extern "system" fn GetTTL<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwttl: *mut u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDUdpAddress_Impl::GetTTL(this) {
                Ok(ok__) => {
                    pdwttl.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlias<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palias: *const windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDUdpAddress_Impl::SetAlias(this, core::mem::transmute_copy(&palias)).into()
        }
        unsafe extern "system" fn GetAlias<Identity: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, palias: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDUdpAddress_Impl::GetAlias(this) {
                Ok(ok__) => {
                    palias.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: IWSDTransportAddress_Vtbl::new::<Identity, OFFSET>(),
            SetSockaddr: SetSockaddr::<Identity, OFFSET>,
            GetSockaddr: GetSockaddr::<Identity, OFFSET>,
            SetExclusive: SetExclusive::<Identity, OFFSET>,
            GetExclusive: GetExclusive::<Identity, OFFSET>,
            SetMessageType: SetMessageType::<Identity, OFFSET>,
            GetMessageType: GetMessageType::<Identity, OFFSET>,
            SetTTL: SetTTL::<Identity, OFFSET>,
            GetTTL: GetTTL::<Identity, OFFSET>,
            SetAlias: SetAlias::<Identity, OFFSET>,
            GetAlias: GetAlias::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDUdpAddress as windows_core::Interface>::IID || iid == &<IWSDAddress as windows_core::Interface>::IID || iid == &<IWSDTransportAddress as windows_core::Interface>::IID
    }
}
pub trait IWSDUdpMessageParameters_Impl: Sized + IWSDMessageParameters_Impl {
    fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> windows_core::Result<()>;
    fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDUdpMessageParameters {}
impl IWSDUdpMessageParameters_Vtbl {
    pub const fn new<Identity: IWSDUdpMessageParameters_Impl, const OFFSET: isize>() -> IWSDUdpMessageParameters_Vtbl {
        unsafe extern "system" fn SetRetransmitParams<Identity: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDUdpMessageParameters_Impl::SetRetransmitParams(this, core::mem::transmute_copy(&pparams)).into()
        }
        unsafe extern "system" fn GetRetransmitParams<Identity: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDUdpMessageParameters_Impl::GetRetransmitParams(this, core::mem::transmute_copy(&pparams)).into()
        }
        Self {
            base__: IWSDMessageParameters_Vtbl::new::<Identity, OFFSET>(),
            SetRetransmitParams: SetRetransmitParams::<Identity, OFFSET>,
            GetRetransmitParams: GetRetransmitParams::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDUdpMessageParameters as windows_core::Interface>::IID || iid == &<IWSDMessageParameters as windows_core::Interface>::IID
    }
}
pub trait IWSDXMLContext_Impl: Sized + windows_core::IUnknownImpl {
    fn AddNamespace(&self, pszuri: &windows_core::PCWSTR, pszsuggestedprefix: &windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> windows_core::Result<()>;
    fn AddNameToNamespace(&self, pszuri: &windows_core::PCWSTR, pszname: &windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> windows_core::Result<()>;
    fn SetNamespaces(&self, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> windows_core::Result<()>;
    fn SetTypes(&self, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDXMLContext {}
impl IWSDXMLContext_Vtbl {
    pub const fn new<Identity: IWSDXMLContext_Impl, const OFFSET: isize>() -> IWSDXMLContext_Vtbl {
        unsafe extern "system" fn AddNamespace<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuri: windows_core::PCWSTR, pszsuggestedprefix: windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDXMLContext_Impl::AddNamespace(this, core::mem::transmute(&pszuri), core::mem::transmute(&pszsuggestedprefix), core::mem::transmute_copy(&ppnamespace)).into()
        }
        unsafe extern "system" fn AddNameToNamespace<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszuri: windows_core::PCWSTR, pszname: windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDXMLContext_Impl::AddNameToNamespace(this, core::mem::transmute(&pszuri), core::mem::transmute(&pszname), core::mem::transmute_copy(&ppname)).into()
        }
        unsafe extern "system" fn SetNamespaces<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDXMLContext_Impl::SetNamespaces(this, core::mem::transmute_copy(&pnamespaces), core::mem::transmute_copy(&wnamespacescount), core::mem::transmute_copy(&blayernumber)).into()
        }
        unsafe extern "system" fn SetTypes<Identity: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDXMLContext_Impl::SetTypes(this, core::mem::transmute_copy(&ptypes), core::mem::transmute_copy(&dwtypescount), core::mem::transmute_copy(&blayernumber)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddNamespace: AddNamespace::<Identity, OFFSET>,
            AddNameToNamespace: AddNameToNamespace::<Identity, OFFSET>,
            SetNamespaces: SetNamespaces::<Identity, OFFSET>,
            SetTypes: SetTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDXMLContext as windows_core::Interface>::IID
    }
}
pub trait IWSDiscoveredService_Impl: Sized + windows_core::IUnknownImpl {
    fn GetEndpointReference(&self) -> windows_core::Result<*mut WSD_ENDPOINT_REFERENCE>;
    fn GetTypes(&self) -> windows_core::Result<*mut WSD_NAME_LIST>;
    fn GetScopes(&self) -> windows_core::Result<*mut WSD_URI_LIST>;
    fn GetXAddrs(&self) -> windows_core::Result<*mut WSD_URI_LIST>;
    fn GetMetadataVersion(&self) -> windows_core::Result<u64>;
    fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn GetProbeResolveTag(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetRemoteTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetLocalTransportAddress(&self) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetLocalInterfaceGUID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetInstanceId(&self) -> windows_core::Result<u64>;
}
impl windows_core::RuntimeName for IWSDiscoveredService {}
impl IWSDiscoveredService_Vtbl {
    pub const fn new<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>() -> IWSDiscoveredService_Vtbl {
        unsafe extern "system" fn GetEndpointReference<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetEndpointReference(this) {
                Ok(ok__) => {
                    ppendpointreference.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypes<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetTypes(this) {
                Ok(ok__) => {
                    pptypeslist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopes<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetScopes(this) {
                Ok(ok__) => {
                    ppscopeslist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXAddrs<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetXAddrs(this) {
                Ok(ok__) => {
                    ppxaddrslist.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataVersion<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullmetadataversion: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetMetadataVersion(this) {
                Ok(ok__) => {
                    pullmetadataversion.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendedDiscoXML<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveredService_Impl::GetExtendedDiscoXML(this, core::mem::transmute_copy(&ppheaderany), core::mem::transmute_copy(&ppbodyany)).into()
        }
        unsafe extern "system" fn GetProbeResolveTag<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsztag: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetProbeResolveTag(this) {
                Ok(ok__) => {
                    ppsztag.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteTransportAddress<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszremotetransportaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetRemoteTransportAddress(this) {
                Ok(ok__) => {
                    ppszremotetransportaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalTransportAddress<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszlocaltransportaddress: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetLocalTransportAddress(this) {
                Ok(ok__) => {
                    ppszlocaltransportaddress.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalInterfaceGUID<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguid: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetLocalInterfaceGUID(this) {
                Ok(ok__) => {
                    pguid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Identity: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pullinstanceid: *mut u64) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveredService_Impl::GetInstanceId(this) {
                Ok(ok__) => {
                    pullinstanceid.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEndpointReference: GetEndpointReference::<Identity, OFFSET>,
            GetTypes: GetTypes::<Identity, OFFSET>,
            GetScopes: GetScopes::<Identity, OFFSET>,
            GetXAddrs: GetXAddrs::<Identity, OFFSET>,
            GetMetadataVersion: GetMetadataVersion::<Identity, OFFSET>,
            GetExtendedDiscoXML: GetExtendedDiscoXML::<Identity, OFFSET>,
            GetProbeResolveTag: GetProbeResolveTag::<Identity, OFFSET>,
            GetRemoteTransportAddress: GetRemoteTransportAddress::<Identity, OFFSET>,
            GetLocalTransportAddress: GetLocalTransportAddress::<Identity, OFFSET>,
            GetLocalInterfaceGUID: GetLocalInterfaceGUID::<Identity, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveredService as windows_core::Interface>::IID
    }
}
pub trait IWSDiscoveryProvider_Impl: Sized + windows_core::IUnknownImpl {
    fn SetAddressFamily(&self, dwaddressfamily: u32) -> windows_core::Result<()>;
    fn Attach(&self, psink: Option<&IWSDiscoveryProviderNotify>) -> windows_core::Result<()>;
    fn Detach(&self) -> windows_core::Result<()>;
    fn SearchById(&self, pszid: &windows_core::PCWSTR, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SearchByAddress(&self, pszaddress: &windows_core::PCWSTR, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SearchByType(&self, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: &windows_core::PCWSTR, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetXMLContext(&self) -> windows_core::Result<IWSDXMLContext>;
}
impl windows_core::RuntimeName for IWSDiscoveryProvider {}
impl IWSDiscoveryProvider_Vtbl {
    pub const fn new<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>() -> IWSDiscoveryProvider_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddressfamily: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProvider_Impl::SetAddressFamily(this, core::mem::transmute_copy(&dwaddressfamily)).into()
        }
        unsafe extern "system" fn Attach<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProvider_Impl::Attach(this, windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn Detach<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProvider_Impl::Detach(this).into()
        }
        unsafe extern "system" fn SearchById<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProvider_Impl::SearchById(this, core::mem::transmute(&pszid), core::mem::transmute(&psztag)).into()
        }
        unsafe extern "system" fn SearchByAddress<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszaddress: windows_core::PCWSTR, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProvider_Impl::SearchByAddress(this, core::mem::transmute(&pszaddress), core::mem::transmute(&psztag)).into()
        }
        unsafe extern "system" fn SearchByType<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: windows_core::PCWSTR, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProvider_Impl::SearchByType(this, core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute(&pszmatchby), core::mem::transmute(&psztag)).into()
        }
        unsafe extern "system" fn GetXMLContext<Identity: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveryProvider_Impl::GetXMLContext(this) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, OFFSET>,
            Attach: Attach::<Identity, OFFSET>,
            Detach: Detach::<Identity, OFFSET>,
            SearchById: SearchById::<Identity, OFFSET>,
            SearchByAddress: SearchByAddress::<Identity, OFFSET>,
            SearchByType: SearchByType::<Identity, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryProvider as windows_core::Interface>::IID
    }
}
pub trait IWSDiscoveryProviderNotify_Impl: Sized + windows_core::IUnknownImpl {
    fn Add(&self, pservice: Option<&IWSDiscoveredService>) -> windows_core::Result<()>;
    fn Remove(&self, pservice: Option<&IWSDiscoveredService>) -> windows_core::Result<()>;
    fn SearchFailed(&self, hr: windows_core::HRESULT, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SearchComplete(&self, psztag: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDiscoveryProviderNotify {}
impl IWSDiscoveryProviderNotify_Vtbl {
    pub const fn new<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>() -> IWSDiscoveryProviderNotify_Vtbl {
        unsafe extern "system" fn Add<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProviderNotify_Impl::Add(this, windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn Remove<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pservice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProviderNotify_Impl::Remove(this, windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn SearchFailed<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProviderNotify_Impl::SearchFailed(this, core::mem::transmute_copy(&hr), core::mem::transmute(&psztag)).into()
        }
        unsafe extern "system" fn SearchComplete<Identity: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psztag: windows_core::PCWSTR) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryProviderNotify_Impl::SearchComplete(this, core::mem::transmute(&psztag)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            SearchFailed: SearchFailed::<Identity, OFFSET>,
            SearchComplete: SearchComplete::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryProviderNotify as windows_core::Interface>::IID
    }
}
pub trait IWSDiscoveryPublisher_Impl: Sized + windows_core::IUnknownImpl {
    fn SetAddressFamily(&self, dwaddressfamily: u32) -> windows_core::Result<()>;
    fn RegisterNotificationSink(&self, psink: Option<&IWSDiscoveryPublisherNotify>) -> windows_core::Result<()>;
    fn UnRegisterNotificationSink(&self, psink: Option<&IWSDiscoveryPublisherNotify>) -> windows_core::Result<()>;
    fn Publish(&self, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::Result<()>;
    fn UnPublish(&self, pszid: &windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn MatchProbe(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: Option<&IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::Result<()>;
    fn MatchResolve(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: Option<&IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::Result<()>;
    fn PublishEx(&self, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn MatchProbeEx(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: Option<&IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn MatchResolveEx(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: Option<&IWSDMessageParameters>, pszid: &windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::Result<()>;
    fn RegisterScopeMatchingRule(&self, pscopematchingrule: Option<&IWSDScopeMatchingRule>) -> windows_core::Result<()>;
    fn UnRegisterScopeMatchingRule(&self, pscopematchingrule: Option<&IWSDScopeMatchingRule>) -> windows_core::Result<()>;
    fn GetXMLContext(&self) -> windows_core::Result<IWSDXMLContext>;
}
impl windows_core::RuntimeName for IWSDiscoveryPublisher {}
impl IWSDiscoveryPublisher_Vtbl {
    pub const fn new<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>() -> IWSDiscoveryPublisher_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwaddressfamily: u32) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::SetAddressFamily(this, core::mem::transmute_copy(&dwaddressfamily)).into()
        }
        unsafe extern "system" fn RegisterNotificationSink<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::RegisterNotificationSink(this, windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn UnRegisterNotificationSink<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::UnRegisterNotificationSink(this, windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn Publish<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::Publish(this, core::mem::transmute(&pszid), core::mem::transmute_copy(&ullmetadataversion), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn UnPublish<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::UnPublish(this, core::mem::transmute(&pszid), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&pany)).into()
        }
        unsafe extern "system" fn MatchProbe<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::MatchProbe(this, core::mem::transmute_copy(&pprobemessage), windows_core::from_raw_borrowed(&pmessageparameters), core::mem::transmute(&pszid), core::mem::transmute_copy(&ullmetadataversion), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn MatchResolve<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::MatchResolve(this, core::mem::transmute_copy(&presolvemessage), windows_core::from_raw_borrowed(&pmessageparameters), core::mem::transmute(&pszid), core::mem::transmute_copy(&ullmetadataversion), core::mem::transmute_copy(&ullinstanceid), core::mem::transmute_copy(&ullmessagenumber), core::mem::transmute(&pszsessionid), core::mem::transmute_copy(&ptypeslist), core::mem::transmute_copy(&pscopeslist), core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn PublishEx<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::PublishEx(
                this,
                core::mem::transmute(&pszid),
                core::mem::transmute_copy(&ullmetadataversion),
                core::mem::transmute_copy(&ullinstanceid),
                core::mem::transmute_copy(&ullmessagenumber),
                core::mem::transmute(&pszsessionid),
                core::mem::transmute_copy(&ptypeslist),
                core::mem::transmute_copy(&pscopeslist),
                core::mem::transmute_copy(&pxaddrslist),
                core::mem::transmute_copy(&pheaderany),
                core::mem::transmute_copy(&preferenceparameterany),
                core::mem::transmute_copy(&ppolicyany),
                core::mem::transmute_copy(&pendpointreferenceany),
                core::mem::transmute_copy(&pany),
            )
            .into()
        }
        unsafe extern "system" fn MatchProbeEx<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::MatchProbeEx(
                this,
                core::mem::transmute_copy(&pprobemessage),
                windows_core::from_raw_borrowed(&pmessageparameters),
                core::mem::transmute(&pszid),
                core::mem::transmute_copy(&ullmetadataversion),
                core::mem::transmute_copy(&ullinstanceid),
                core::mem::transmute_copy(&ullmessagenumber),
                core::mem::transmute(&pszsessionid),
                core::mem::transmute_copy(&ptypeslist),
                core::mem::transmute_copy(&pscopeslist),
                core::mem::transmute_copy(&pxaddrslist),
                core::mem::transmute_copy(&pheaderany),
                core::mem::transmute_copy(&preferenceparameterany),
                core::mem::transmute_copy(&ppolicyany),
                core::mem::transmute_copy(&pendpointreferenceany),
                core::mem::transmute_copy(&pany),
            )
            .into()
        }
        unsafe extern "system" fn MatchResolveEx<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void, pszid: windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::MatchResolveEx(
                this,
                core::mem::transmute_copy(&presolvemessage),
                windows_core::from_raw_borrowed(&pmessageparameters),
                core::mem::transmute(&pszid),
                core::mem::transmute_copy(&ullmetadataversion),
                core::mem::transmute_copy(&ullinstanceid),
                core::mem::transmute_copy(&ullmessagenumber),
                core::mem::transmute(&pszsessionid),
                core::mem::transmute_copy(&ptypeslist),
                core::mem::transmute_copy(&pscopeslist),
                core::mem::transmute_copy(&pxaddrslist),
                core::mem::transmute_copy(&pheaderany),
                core::mem::transmute_copy(&preferenceparameterany),
                core::mem::transmute_copy(&ppolicyany),
                core::mem::transmute_copy(&pendpointreferenceany),
                core::mem::transmute_copy(&pany),
            )
            .into()
        }
        unsafe extern "system" fn RegisterScopeMatchingRule<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscopematchingrule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::RegisterScopeMatchingRule(this, windows_core::from_raw_borrowed(&pscopematchingrule)).into()
        }
        unsafe extern "system" fn UnRegisterScopeMatchingRule<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pscopematchingrule: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisher_Impl::UnRegisterScopeMatchingRule(this, windows_core::from_raw_borrowed(&pscopematchingrule)).into()
        }
        unsafe extern "system" fn GetXMLContext<Identity: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontext: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IWSDiscoveryPublisher_Impl::GetXMLContext(this) {
                Ok(ok__) => {
                    ppcontext.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, OFFSET>,
            RegisterNotificationSink: RegisterNotificationSink::<Identity, OFFSET>,
            UnRegisterNotificationSink: UnRegisterNotificationSink::<Identity, OFFSET>,
            Publish: Publish::<Identity, OFFSET>,
            UnPublish: UnPublish::<Identity, OFFSET>,
            MatchProbe: MatchProbe::<Identity, OFFSET>,
            MatchResolve: MatchResolve::<Identity, OFFSET>,
            PublishEx: PublishEx::<Identity, OFFSET>,
            MatchProbeEx: MatchProbeEx::<Identity, OFFSET>,
            MatchResolveEx: MatchResolveEx::<Identity, OFFSET>,
            RegisterScopeMatchingRule: RegisterScopeMatchingRule::<Identity, OFFSET>,
            UnRegisterScopeMatchingRule: UnRegisterScopeMatchingRule::<Identity, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisher as windows_core::Interface>::IID
    }
}
pub trait IWSDiscoveryPublisherNotify_Impl: Sized + windows_core::IUnknownImpl {
    fn ProbeHandler(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: Option<&IWSDMessageParameters>) -> windows_core::Result<()>;
    fn ResolveHandler(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: Option<&IWSDMessageParameters>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IWSDiscoveryPublisherNotify {}
impl IWSDiscoveryPublisherNotify_Vtbl {
    pub const fn new<Identity: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>() -> IWSDiscoveryPublisherNotify_Vtbl {
        unsafe extern "system" fn ProbeHandler<Identity: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisherNotify_Impl::ProbeHandler(this, core::mem::transmute_copy(&psoap), windows_core::from_raw_borrowed(&pmessageparameters)).into()
        }
        unsafe extern "system" fn ResolveHandler<Identity: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IWSDiscoveryPublisherNotify_Impl::ResolveHandler(this, core::mem::transmute_copy(&psoap), windows_core::from_raw_borrowed(&pmessageparameters)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProbeHandler: ProbeHandler::<Identity, OFFSET>,
            ResolveHandler: ResolveHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisherNotify as windows_core::Interface>::IID
    }
}
