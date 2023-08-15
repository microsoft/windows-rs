#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDAddress_Impl: Sized {
    fn Serialize(&self, pszbuffer: ::windows_core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Deserialize(&self, pszbuffer: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDAddress {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDAddress_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAddress_Impl, const OFFSET: isize>() -> IWSDAddress_Vtbl {
        unsafe extern "system" fn Serialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuffer: ::windows_core::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Serialize(::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchlength), ::core::mem::transmute_copy(&fsafe)).into()
        }
        unsafe extern "system" fn Deserialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuffer: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deserialize(::core::mem::transmute(&pszbuffer)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDAddress as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDAsyncCallback_Impl: Sized {
    fn AsyncOperationComplete(&self, pasyncresult: ::core::option::Option<&IWSDAsyncResult>, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWSDAsyncCallback {}
impl IWSDAsyncCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncCallback_Impl, const OFFSET: isize>() -> IWSDAsyncCallback_Vtbl {
        unsafe extern "system" fn AsyncOperationComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncOperationComplete(::windows_core::from_raw_borrowed(&pasyncresult), ::windows_core::from_raw_borrowed(&pasyncstate)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDAsyncCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDAsyncResult_Impl: Sized {
    fn SetCallback(&self, pcallback: ::core::option::Option<&IWSDAsyncCallback>, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetWaitHandle(&self, hwaithandle: super::super::Foundation::HANDLE) -> ::windows_core::Result<()>;
    fn HasCompleted(&self) -> ::windows_core::Result<()>;
    fn GetAsyncState(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Abort(&self) -> ::windows_core::Result<()>;
    fn GetEvent(&self, pevent: *mut WSD_EVENT) -> ::windows_core::Result<()>;
    fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDAsyncResult {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDAsyncResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>() -> IWSDAsyncResult_Vtbl {
        unsafe extern "system" fn SetCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, pasyncstate: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCallback(::windows_core::from_raw_borrowed(&pcallback), ::windows_core::from_raw_borrowed(&pasyncstate)).into()
        }
        unsafe extern "system" fn SetWaitHandle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwaithandle: super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWaitHandle(::core::mem::transmute_copy(&hwaithandle)).into()
        }
        unsafe extern "system" fn HasCompleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HasCompleted().into()
        }
        unsafe extern "system" fn GetAsyncState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppasyncstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn GetEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut WSD_EVENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEvent(::core::mem::transmute_copy(&pevent)).into()
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppendpoint: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEndpointProxy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppendpoint, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCallback: SetCallback::<Identity, Impl, OFFSET>,
            SetWaitHandle: SetWaitHandle::<Identity, Impl, OFFSET>,
            HasCompleted: HasCompleted::<Identity, Impl, OFFSET>,
            GetAsyncState: GetAsyncState::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDAsyncResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDAttachment_Impl: Sized {}
impl ::windows_core::RuntimeName for IWSDAttachment {}
impl IWSDAttachment_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDAttachment_Impl, const OFFSET: isize>() -> IWSDAttachment_Vtbl {
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDAttachment as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDDeviceHost_Impl: Sized {
    fn Init(&self, pszlocalid: &::windows_core::PCWSTR, pcontext: ::core::option::Option<&IWSDXMLContext>, pphostaddresses: *const ::core::option::Option<IWSDAddress>, dwhostaddresscount: u32) -> ::windows_core::Result<()>;
    fn Start(&self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: ::core::option::Option<&IWSDDeviceHostNotify>) -> ::windows_core::Result<()>;
    fn Stop(&self) -> ::windows_core::Result<()>;
    fn Terminate(&self) -> ::windows_core::Result<()>;
    fn RegisterPortType(&self, pporttype: *const WSD_PORT_TYPE) -> ::windows_core::Result<()>;
    fn SetMetadata(&self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows_core::Result<()>;
    fn RegisterService(&self, pszserviceid: &::windows_core::PCWSTR, pservice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RetireService(&self, pszserviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn AddDynamicService(&self, pszserviceid: &::windows_core::PCWSTR, pszendpointaddress: &::windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn RemoveDynamicService(&self, pszserviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SetServiceDiscoverable(&self, pszserviceid: &::windows_core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SignalEvent(&self, pszserviceid: &::windows_core::PCWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDDeviceHost {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDDeviceHost_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>() -> IWSDDeviceHost_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlocalid: ::windows_core::PCWSTR, pcontext: *mut ::core::ffi::c_void, pphostaddresses: *const *mut ::core::ffi::c_void, dwhostaddresscount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute(&pszlocalid), ::windows_core::from_raw_borrowed(&pcontext), ::core::mem::transmute_copy(&pphostaddresses), ::core::mem::transmute_copy(&dwhostaddresscount)).into()
        }
        unsafe extern "system" fn Start<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Start(::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&pscopelist), ::windows_core::from_raw_borrowed(&pnotificationsink)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Stop().into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Terminate().into()
        }
        unsafe extern "system" fn RegisterPortType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterPortType(::core::mem::transmute_copy(&pporttype)).into()
        }
        unsafe extern "system" fn SetMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMetadata(::core::mem::transmute_copy(&pthismodelmetadata), ::core::mem::transmute_copy(&pthisdevicemetadata), ::core::mem::transmute_copy(&phostmetadata), ::core::mem::transmute_copy(&pcustommetadata)).into()
        }
        unsafe extern "system" fn RegisterService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterService(::core::mem::transmute(&pszserviceid), ::windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn RetireService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RetireService(::core::mem::transmute(&pszserviceid)).into()
        }
        unsafe extern "system" fn AddDynamicService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pszendpointaddress: ::windows_core::PCWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDynamicService(::core::mem::transmute(&pszserviceid), ::core::mem::transmute(&pszendpointaddress), ::core::mem::transmute_copy(&pporttype), ::core::mem::transmute_copy(&pportname), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn RemoveDynamicService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveDynamicService(::core::mem::transmute(&pszserviceid)).into()
        }
        unsafe extern "system" fn SetServiceDiscoverable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetServiceDiscoverable(::core::mem::transmute(&pszserviceid), ::core::mem::transmute_copy(&fdiscoverable)).into()
        }
        unsafe extern "system" fn SignalEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SignalEvent(::core::mem::transmute(&pszserviceid), ::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            Start: Start::<Identity, Impl, OFFSET>,
            Stop: Stop::<Identity, Impl, OFFSET>,
            Terminate: Terminate::<Identity, Impl, OFFSET>,
            RegisterPortType: RegisterPortType::<Identity, Impl, OFFSET>,
            SetMetadata: SetMetadata::<Identity, Impl, OFFSET>,
            RegisterService: RegisterService::<Identity, Impl, OFFSET>,
            RetireService: RetireService::<Identity, Impl, OFFSET>,
            AddDynamicService: AddDynamicService::<Identity, Impl, OFFSET>,
            RemoveDynamicService: RemoveDynamicService::<Identity, Impl, OFFSET>,
            SetServiceDiscoverable: SetServiceDiscoverable::<Identity, Impl, OFFSET>,
            SignalEvent: SignalEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDDeviceHost as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDDeviceHostNotify_Impl: Sized {
    fn GetService(&self, pszserviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for IWSDDeviceHostNotify {}
impl IWSDDeviceHostNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHostNotify_Impl, const OFFSET: isize>() -> IWSDDeviceHostNotify_Vtbl {
        unsafe extern "system" fn GetService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceHostNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetService(::core::mem::transmute(&pszserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDDeviceHostNotify as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDDeviceProxy_Impl: Sized {
    fn Init(&self, pszdeviceid: &::windows_core::PCWSTR, pdeviceaddress: ::core::option::Option<&IWSDAddress>, pszlocalid: &::windows_core::PCWSTR, pcontext: ::core::option::Option<&IWSDXMLContext>, psponsor: ::core::option::Option<&IWSDDeviceProxy>) -> ::windows_core::Result<()>;
    fn BeginGetMetadata(&self) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&self, presult: ::core::option::Option<&IWSDAsyncResult>) -> ::windows_core::Result<()>;
    fn GetHostMetadata(&self) -> ::windows_core::Result<*mut WSD_HOST_METADATA>;
    fn GetThisModelMetadata(&self) -> ::windows_core::Result<*mut WSD_THIS_MODEL_METADATA>;
    fn GetThisDeviceMetadata(&self) -> ::windows_core::Result<*mut WSD_THIS_DEVICE_METADATA>;
    fn GetAllMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceProxyById(&self, pszserviceid: &::windows_core::PCWSTR) -> ::windows_core::Result<IWSDServiceProxy>;
    fn GetServiceProxyByType(&self, ptype: *const WSDXML_NAME) -> ::windows_core::Result<IWSDServiceProxy>;
    fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy>;
}
impl ::windows_core::RuntimeName for IWSDDeviceProxy {}
impl IWSDDeviceProxy_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>() -> IWSDDeviceProxy_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdeviceid: ::windows_core::PCWSTR, pdeviceaddress: *mut ::core::ffi::c_void, pszlocalid: ::windows_core::PCWSTR, pcontext: *mut ::core::ffi::c_void, psponsor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute(&pszdeviceid), ::windows_core::from_raw_borrowed(&pdeviceaddress), ::core::mem::transmute(&pszlocalid), ::windows_core::from_raw_borrowed(&pcontext), ::windows_core::from_raw_borrowed(&psponsor)).into()
        }
        unsafe extern "system" fn BeginGetMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndGetMetadata(::windows_core::from_raw_borrowed(&presult)).into()
        }
        unsafe extern "system" fn GetHostMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHostMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pphostmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisModelMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThisModelMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmanufacturermetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisDeviceMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThisDeviceMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppthisdevicemetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAllMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyById<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: ::windows_core::PCWSTR, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceProxyById(::core::mem::transmute(&pszserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserviceproxy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyByType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceProxyByType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppserviceproxy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEndpointProxy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproxy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            BeginGetMetadata: BeginGetMetadata::<Identity, Impl, OFFSET>,
            EndGetMetadata: EndGetMetadata::<Identity, Impl, OFFSET>,
            GetHostMetadata: GetHostMetadata::<Identity, Impl, OFFSET>,
            GetThisModelMetadata: GetThisModelMetadata::<Identity, Impl, OFFSET>,
            GetThisDeviceMetadata: GetThisDeviceMetadata::<Identity, Impl, OFFSET>,
            GetAllMetadata: GetAllMetadata::<Identity, Impl, OFFSET>,
            GetServiceProxyById: GetServiceProxyById::<Identity, Impl, OFFSET>,
            GetServiceProxyByType: GetServiceProxyByType::<Identity, Impl, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDDeviceProxy as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDEndpointProxy_Impl: Sized {
    fn SendOneWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()>;
    fn SendTwoWayRequest(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows_core::Result<()>;
    fn SendTwoWayRequestAsync(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pcallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn AbortAsyncOperation(&self, pasyncresult: ::core::option::Option<&IWSDAsyncResult>) -> ::windows_core::Result<()>;
    fn ProcessFault(&self, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::Result<()>;
    fn GetErrorInfo(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetFaultInfo(&self) -> ::windows_core::Result<*mut WSD_SOAP_FAULT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDEndpointProxy {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDEndpointProxy_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>() -> IWSDEndpointProxy_Vtbl {
        unsafe extern "system" fn SendOneWayRequest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendOneWayRequest(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation)).into()
        }
        unsafe extern "system" fn SendTwoWayRequest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendTwoWayRequest(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::core::mem::transmute_copy(&presponsecontext)).into()
        }
        unsafe extern "system" fn SendTwoWayRequestAsync<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, presult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SendTwoWayRequestAsync(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(presult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAsyncOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AbortAsyncOperation(::windows_core::from_raw_borrowed(&pasyncresult)).into()
        }
        unsafe extern "system" fn ProcessFault<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProcessFault(::core::mem::transmute_copy(&pfault)).into()
        }
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszerrorinfo: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszerrorinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaultInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFaultInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendOneWayRequest: SendOneWayRequest::<Identity, Impl, OFFSET>,
            SendTwoWayRequest: SendTwoWayRequest::<Identity, Impl, OFFSET>,
            SendTwoWayRequestAsync: SendTwoWayRequestAsync::<Identity, Impl, OFFSET>,
            AbortAsyncOperation: AbortAsyncOperation::<Identity, Impl, OFFSET>,
            ProcessFault: ProcessFault::<Identity, Impl, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET>,
            GetFaultInfo: GetFaultInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDEndpointProxy as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDEventingStatus_Impl: Sized {
    fn SubscriptionRenewed(&self, pszsubscriptionaction: &::windows_core::PCWSTR);
    fn SubscriptionRenewalFailed(&self, pszsubscriptionaction: &::windows_core::PCWSTR, hr: ::windows_core::HRESULT);
    fn SubscriptionEnded(&self, pszsubscriptionaction: &::windows_core::PCWSTR);
}
impl ::windows_core::RuntimeName for IWSDEventingStatus {}
impl IWSDEventingStatus_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEventingStatus_Impl, const OFFSET: isize>() -> IWSDEventingStatus_Vtbl {
        unsafe extern "system" fn SubscriptionRenewed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SubscriptionRenewed(::core::mem::transmute(&pszsubscriptionaction))
        }
        unsafe extern "system" fn SubscriptionRenewalFailed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR, hr: ::windows_core::HRESULT) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SubscriptionRenewalFailed(::core::mem::transmute(&pszsubscriptionaction), ::core::mem::transmute_copy(&hr))
        }
        unsafe extern "system" fn SubscriptionEnded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: ::windows_core::PCWSTR) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SubscriptionEnded(::core::mem::transmute(&pszsubscriptionaction))
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SubscriptionRenewed: SubscriptionRenewed::<Identity, Impl, OFFSET>,
            SubscriptionRenewalFailed: SubscriptionRenewalFailed::<Identity, Impl, OFFSET>,
            SubscriptionEnded: SubscriptionEnded::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDEventingStatus as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDHttpAddress_Impl: Sized + IWSDTransportAddress_Impl {
    fn GetSecure(&self) -> ::windows_core::Result<()>;
    fn SetSecure(&self, fsecure: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetPath(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetPath(&self, pszpath: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDHttpAddress {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDHttpAddress_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>() -> IWSDHttpAddress_Vtbl {
        unsafe extern "system" fn GetSecure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSecure().into()
        }
        unsafe extern "system" fn SetSecure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSecure(::core::mem::transmute_copy(&fsecure)).into()
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPath(::core::mem::transmute(&pszpath)).into()
        }
        Self {
            base__: IWSDTransportAddress_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSecure: GetSecure::<Identity, Impl, OFFSET>,
            SetSecure: SetSecure::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDHttpAddress as ::windows_core::ComInterface>::IID || iid == &<IWSDAddress as ::windows_core::ComInterface>::IID || iid == &<IWSDTransportAddress as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDHttpAuthParameters_Impl: Sized {
    fn GetClientAccessToken(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
    fn GetAuthType(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDHttpAuthParameters {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDHttpAuthParameters_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpAuthParameters_Impl, const OFFSET: isize>() -> IWSDHttpAuthParameters_Vtbl {
        unsafe extern "system" fn GetClientAccessToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClientAccessToken() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phtoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthtype: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAuthType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pauthtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientAccessToken: GetClientAccessToken::<Identity, Impl, OFFSET>,
            GetAuthType: GetAuthType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDHttpAuthParameters as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDHttpMessageParameters_Impl: Sized + IWSDMessageParameters_Impl {
    fn SetInboundHttpHeaders(&self, pszheaders: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetInboundHttpHeaders(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetOutboundHttpHeaders(&self, pszheaders: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetOutboundHttpHeaders(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetID(&self, pszid: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetID(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetContext(&self, pcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetContext(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Clear(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWSDHttpMessageParameters {}
impl IWSDHttpMessageParameters_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>() -> IWSDHttpMessageParameters_Vtbl {
        unsafe extern "system" fn SetInboundHttpHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszheaders: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInboundHttpHeaders(::core::mem::transmute(&pszheaders)).into()
        }
        unsafe extern "system" fn GetInboundHttpHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInboundHttpHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundHttpHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszheaders: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOutboundHttpHeaders(::core::mem::transmute(&pszheaders)).into()
        }
        unsafe extern "system" fn GetOutboundHttpHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOutboundHttpHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetID(::core::mem::transmute(&pszid)).into()
        }
        unsafe extern "system" fn GetID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszid: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContext(::windows_core::from_raw_borrowed(&pcontext)).into()
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: IWSDMessageParameters_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetInboundHttpHeaders: SetInboundHttpHeaders::<Identity, Impl, OFFSET>,
            GetInboundHttpHeaders: GetInboundHttpHeaders::<Identity, Impl, OFFSET>,
            SetOutboundHttpHeaders: SetOutboundHttpHeaders::<Identity, Impl, OFFSET>,
            GetOutboundHttpHeaders: GetOutboundHttpHeaders::<Identity, Impl, OFFSET>,
            SetID: SetID::<Identity, Impl, OFFSET>,
            GetID: GetID::<Identity, Impl, OFFSET>,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDHttpMessageParameters as ::windows_core::ComInterface>::IID || iid == &<IWSDMessageParameters as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDInboundAttachment_Impl: Sized + IWSDAttachment_Impl {
    fn Read(&self, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows_core::Result<()>;
    fn Close(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWSDInboundAttachment {}
impl IWSDInboundAttachment_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDInboundAttachment_Impl, const OFFSET: isize>() -> IWSDInboundAttachment_Vtbl {
        unsafe extern "system" fn Read<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Read(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbytestoread), ::core::mem::transmute_copy(&pdwnumberofbytesread)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self { base__: IWSDAttachment_Vtbl::new::<Identity, Impl, OFFSET>(), Read: Read::<Identity, Impl, OFFSET>, Close: Close::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDInboundAttachment as ::windows_core::ComInterface>::IID || iid == &<IWSDAttachment as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDMessageParameters_Impl: Sized {
    fn GetLocalAddress(&self) -> ::windows_core::Result<IWSDAddress>;
    fn SetLocalAddress(&self, paddress: ::core::option::Option<&IWSDAddress>) -> ::windows_core::Result<()>;
    fn GetRemoteAddress(&self) -> ::windows_core::Result<IWSDAddress>;
    fn SetRemoteAddress(&self, paddress: ::core::option::Option<&IWSDAddress>) -> ::windows_core::Result<()>;
    fn GetLowerParameters(&self) -> ::windows_core::Result<IWSDMessageParameters>;
}
impl ::windows_core::RuntimeName for IWSDMessageParameters {}
impl IWSDMessageParameters_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>() -> IWSDMessageParameters_Vtbl {
        unsafe extern "system" fn GetLocalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLocalAddress(::windows_core::from_raw_borrowed(&paddress)).into()
        }
        unsafe extern "system" fn GetRemoteAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRemoteAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRemoteAddress(::windows_core::from_raw_borrowed(&paddress)).into()
        }
        unsafe extern "system" fn GetLowerParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptxparams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLowerParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptxparams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLocalAddress: GetLocalAddress::<Identity, Impl, OFFSET>,
            SetLocalAddress: SetLocalAddress::<Identity, Impl, OFFSET>,
            GetRemoteAddress: GetRemoteAddress::<Identity, Impl, OFFSET>,
            SetRemoteAddress: SetRemoteAddress::<Identity, Impl, OFFSET>,
            GetLowerParameters: GetLowerParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDMessageParameters as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDMetadataExchange_Impl: Sized {
    fn GetMetadata(&self) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
}
impl ::windows_core::RuntimeName for IWSDMetadataExchange {}
impl IWSDMetadataExchange_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDMetadataExchange_Impl, const OFFSET: isize>() -> IWSDMetadataExchange_Vtbl {
        unsafe extern "system" fn GetMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDMetadataExchange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(metadataout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMetadata: GetMetadata::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDMetadataExchange as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDOutboundAttachment_Impl: Sized + IWSDAttachment_Impl {
    fn Write(&self, pbuffer: *const u8, dwbytestowrite: u32) -> ::windows_core::Result<u32>;
    fn Close(&self) -> ::windows_core::Result<()>;
    fn Abort(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWSDOutboundAttachment {}
impl IWSDOutboundAttachment_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>() -> IWSDOutboundAttachment_Vtbl {
        unsafe extern "system" fn Write<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Write(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbytestowrite)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwnumberofbyteswritten, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        Self {
            base__: IWSDAttachment_Vtbl::new::<Identity, Impl, OFFSET>(),
            Write: Write::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDOutboundAttachment as ::windows_core::ComInterface>::IID || iid == &<IWSDAttachment as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IWSDSSLClientCertificate_Impl: Sized {
    fn GetClientCertificate(&self) -> ::windows_core::Result<*mut super::super::Security::Cryptography::CERT_CONTEXT>;
    fn GetMappedAccessToken(&self) -> ::windows_core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::windows_core::RuntimeName for IWSDSSLClientCertificate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl IWSDSSLClientCertificate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDSSLClientCertificate_Impl, const OFFSET: isize>() -> IWSDSSLClientCertificate_Vtbl {
        unsafe extern "system" fn GetClientCertificate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcertcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMappedAccessToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMappedAccessToken() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phtoken, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientCertificate: GetClientCertificate::<Identity, Impl, OFFSET>,
            GetMappedAccessToken: GetMappedAccessToken::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDSSLClientCertificate as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDScopeMatchingRule_Impl: Sized {
    fn GetScopeRule(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn MatchScopes(&self, pszscope1: &::windows_core::PCWSTR, pszscope2: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDScopeMatchingRule {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDScopeMatchingRule_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDScopeMatchingRule_Impl, const OFFSET: isize>() -> IWSDScopeMatchingRule_Vtbl {
        unsafe extern "system" fn GetScopeRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszscopematchingrule: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScopeRule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszscopematchingrule, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchScopes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszscope1: ::windows_core::PCWSTR, pszscope2: ::windows_core::PCWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MatchScopes(::core::mem::transmute(&pszscope1), ::core::mem::transmute(&pszscope2)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfmatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetScopeRule: GetScopeRule::<Identity, Impl, OFFSET>,
            MatchScopes: MatchScopes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDScopeMatchingRule as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDServiceMessaging_Impl: Sized {
    fn SendResponse(&self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>) -> ::windows_core::Result<()>;
    fn FaultRequest(&self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWSDServiceMessaging {}
impl IWSDServiceMessaging_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceMessaging_Impl, const OFFSET: isize>() -> IWSDServiceMessaging_Vtbl {
        unsafe extern "system" fn SendResponse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendResponse(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::windows_core::from_raw_borrowed(&pmessageparameters)).into()
        }
        unsafe extern "system" fn FaultRequest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FaultRequest(::core::mem::transmute_copy(&prequestheader), ::windows_core::from_raw_borrowed(&pmessageparameters), ::core::mem::transmute_copy(&pfault)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendResponse: SendResponse::<Identity, Impl, OFFSET>,
            FaultRequest: FaultRequest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDServiceMessaging as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDServiceProxy_Impl: Sized + IWSDMetadataExchange_Impl {
    fn BeginGetMetadata(&self) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&self, presult: ::core::option::Option<&IWSDAsyncResult>) -> ::windows_core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceMetadata(&self) -> ::windows_core::Result<*mut WSD_SERVICE_METADATA>;
    fn SubscribeToOperation(&self, poperation: *const WSD_OPERATION, punknown: ::core::option::Option<&::windows_core::IUnknown>, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn UnsubscribeToOperation(&self, poperation: *const WSD_OPERATION) -> ::windows_core::Result<()>;
    fn SetEventingStatusCallback(&self, pstatus: ::core::option::Option<&IWSDEventingStatus>) -> ::windows_core::Result<()>;
    fn GetEndpointProxy(&self) -> ::windows_core::Result<IWSDEndpointProxy>;
}
impl ::windows_core::RuntimeName for IWSDServiceProxy {}
impl IWSDServiceProxy_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>() -> IWSDServiceProxy_Vtbl {
        unsafe extern "system" fn BeginGetMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndGetMetadata(::windows_core::from_raw_borrowed(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppmetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceMetadata<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservicemetadata, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscribeToOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SubscribeToOperation(::core::mem::transmute_copy(&poperation), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn UnsubscribeToOperation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnsubscribeToOperation(::core::mem::transmute_copy(&poperation)).into()
        }
        unsafe extern "system" fn SetEventingStatusCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventingStatusCallback(::windows_core::from_raw_borrowed(&pstatus)).into()
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproxy: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEndpointProxy() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproxy, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWSDMetadataExchange_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetMetadata: BeginGetMetadata::<Identity, Impl, OFFSET>,
            EndGetMetadata: EndGetMetadata::<Identity, Impl, OFFSET>,
            GetServiceMetadata: GetServiceMetadata::<Identity, Impl, OFFSET>,
            SubscribeToOperation: SubscribeToOperation::<Identity, Impl, OFFSET>,
            UnsubscribeToOperation: UnsubscribeToOperation::<Identity, Impl, OFFSET>,
            SetEventingStatusCallback: SetEventingStatusCallback::<Identity, Impl, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDServiceProxy as ::windows_core::ComInterface>::IID || iid == &<IWSDMetadataExchange as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDServiceProxyEventing_Impl: Sized + IWSDServiceProxy_Impl {
    fn SubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: ::core::option::Option<&::windows_core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn BeginSubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: ::core::option::Option<&::windows_core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pasynccallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndSubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::core::option::Option<&IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn UnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn BeginUnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pasynccallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndUnsubscribeToMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::core::option::Option<&IWSDAsyncResult>) -> ::windows_core::Result<()>;
    fn RenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn BeginRenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pasynccallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndRenewMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::core::option::Option<&IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn GetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn BeginGetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: ::core::option::Option<&::windows_core::IUnknown>, pasynccallback: ::core::option::Option<&IWSDAsyncCallback>) -> ::windows_core::Result<IWSDAsyncResult>;
    fn EndGetStatusForMultipleOperations(&self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::core::option::Option<&IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDServiceProxyEventing {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDServiceProxyEventing_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>() -> IWSDServiceProxyEventing_Vtbl {
        unsafe extern "system" fn SubscribeToMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginSubscribeToMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginSubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&punknown), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndSubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn UnsubscribeToMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnsubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany)).into()
        }
        unsafe extern "system" fn BeginUnsubscribeToMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginUnsubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnsubscribeToMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndUnsubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&presult)).into()
        }
        unsafe extern "system" fn RenewMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RenewMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginRenewMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginRenewMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRenewMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndRenewMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn GetStatusForMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetStatusForMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginGetStatusForMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: *mut ::core::ffi::c_void, ppresult: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BeginGetStatusForMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::windows_core::from_raw_borrowed(&pasyncstate), ::windows_core::from_raw_borrowed(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresult, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetStatusForMultipleOperations<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: *mut ::core::ffi::c_void, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndGetStatusForMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::windows_core::from_raw_borrowed(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        Self {
            base__: IWSDServiceProxy_Vtbl::new::<Identity, Impl, OFFSET>(),
            SubscribeToMultipleOperations: SubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            BeginSubscribeToMultipleOperations: BeginSubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            EndSubscribeToMultipleOperations: EndSubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            UnsubscribeToMultipleOperations: UnsubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            BeginUnsubscribeToMultipleOperations: BeginUnsubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            EndUnsubscribeToMultipleOperations: EndUnsubscribeToMultipleOperations::<Identity, Impl, OFFSET>,
            RenewMultipleOperations: RenewMultipleOperations::<Identity, Impl, OFFSET>,
            BeginRenewMultipleOperations: BeginRenewMultipleOperations::<Identity, Impl, OFFSET>,
            EndRenewMultipleOperations: EndRenewMultipleOperations::<Identity, Impl, OFFSET>,
            GetStatusForMultipleOperations: GetStatusForMultipleOperations::<Identity, Impl, OFFSET>,
            BeginGetStatusForMultipleOperations: BeginGetStatusForMultipleOperations::<Identity, Impl, OFFSET>,
            EndGetStatusForMultipleOperations: EndGetStatusForMultipleOperations::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDServiceProxyEventing as ::windows_core::ComInterface>::IID || iid == &<IWSDMetadataExchange as ::windows_core::ComInterface>::IID || iid == &<IWSDServiceProxy as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDSignatureProperty_Impl: Sized {
    fn IsMessageSigned(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn IsMessageSignatureTrusted(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetKeyInfo(&self, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows_core::Result<()>;
    fn GetSignature(&self, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetSignedInfoHash(&self, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDSignatureProperty {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDSignatureProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>() -> IWSDSignatureProperty_Vtbl {
        unsafe extern "system" fn IsMessageSigned<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMessageSigned() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsigned, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMessageSignatureTrusted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsMessageSignatureTrusted() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsignaturetrusted, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetKeyInfo(::core::mem::transmute_copy(&pbkeyinfo), ::core::mem::transmute_copy(&pdwkeyinfosize)).into()
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSignature(::core::mem::transmute_copy(&pbsignature), ::core::mem::transmute_copy(&pdwsignaturesize)).into()
        }
        unsafe extern "system" fn GetSignedInfoHash<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSignedInfoHash(::core::mem::transmute_copy(&pbsignedinfohash), ::core::mem::transmute_copy(&pdwhashsize)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsMessageSigned: IsMessageSigned::<Identity, Impl, OFFSET>,
            IsMessageSignatureTrusted: IsMessageSignatureTrusted::<Identity, Impl, OFFSET>,
            GetKeyInfo: GetKeyInfo::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            GetSignedInfoHash: GetSignedInfoHash::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDSignatureProperty as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDTransportAddress_Impl: Sized + IWSDAddress_Impl {
    fn GetPort(&self) -> ::windows_core::Result<u16>;
    fn SetPort(&self, wport: u16) -> ::windows_core::Result<()>;
    fn GetTransportAddress(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetTransportAddressEx(&self, fsafe: super::super::Foundation::BOOL) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn SetTransportAddress(&self, pszaddress: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IWSDTransportAddress {}
#[cfg(feature = "Win32_Foundation")]
impl IWSDTransportAddress_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>() -> IWSDTransportAddress_Vtbl {
        unsafe extern "system" fn GetPort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwport: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPort() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pwport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wport: u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPort(::core::mem::transmute_copy(&wport)).into()
        }
        unsafe extern "system" fn GetTransportAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransportAddressEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransportAddressEx(::core::mem::transmute_copy(&fsafe)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaddress: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransportAddress(::core::mem::transmute(&pszaddress)).into()
        }
        Self {
            base__: IWSDAddress_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPort: GetPort::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
            GetTransportAddress: GetTransportAddress::<Identity, Impl, OFFSET>,
            GetTransportAddressEx: GetTransportAddressEx::<Identity, Impl, OFFSET>,
            SetTransportAddress: SetTransportAddress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDTransportAddress as ::windows_core::ComInterface>::IID || iid == &<IWSDAddress as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub trait IWSDUdpAddress_Impl: Sized + IWSDTransportAddress_Impl {
    fn SetSockaddr(&self, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::Result<()>;
    fn GetSockaddr(&self, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::Result<()>;
    fn SetExclusive(&self, fexclusive: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetExclusive(&self) -> ::windows_core::Result<()>;
    fn SetMessageType(&self, messagetype: WSDUdpMessageType) -> ::windows_core::Result<()>;
    fn GetMessageType(&self) -> ::windows_core::Result<WSDUdpMessageType>;
    fn SetTTL(&self, dwttl: u32) -> ::windows_core::Result<()>;
    fn GetTTL(&self) -> ::windows_core::Result<u32>;
    fn SetAlias(&self, palias: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetAlias(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::windows_core::RuntimeName for IWSDUdpAddress {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl IWSDUdpAddress_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>() -> IWSDUdpAddress_Vtbl {
        unsafe extern "system" fn SetSockaddr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSockaddr(::core::mem::transmute_copy(&psockaddr)).into()
        }
        unsafe extern "system" fn GetSockaddr<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSockaddr(::core::mem::transmute_copy(&psockaddr)).into()
        }
        unsafe extern "system" fn SetExclusive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fexclusive: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetExclusive(::core::mem::transmute_copy(&fexclusive)).into()
        }
        unsafe extern "system" fn GetExclusive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExclusive().into()
        }
        unsafe extern "system" fn SetMessageType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: WSDUdpMessageType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMessageType(::core::mem::transmute_copy(&messagetype)).into()
        }
        unsafe extern "system" fn GetMessageType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMessageType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmessagetype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTTL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwttl: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTTL(::core::mem::transmute_copy(&dwttl)).into()
        }
        unsafe extern "system" fn GetTTL<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwttl: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTTL() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwttl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlias<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palias: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAlias(::core::mem::transmute_copy(&palias)).into()
        }
        unsafe extern "system" fn GetAlias<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palias: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAlias() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(palias, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWSDTransportAddress_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSockaddr: SetSockaddr::<Identity, Impl, OFFSET>,
            GetSockaddr: GetSockaddr::<Identity, Impl, OFFSET>,
            SetExclusive: SetExclusive::<Identity, Impl, OFFSET>,
            GetExclusive: GetExclusive::<Identity, Impl, OFFSET>,
            SetMessageType: SetMessageType::<Identity, Impl, OFFSET>,
            GetMessageType: GetMessageType::<Identity, Impl, OFFSET>,
            SetTTL: SetTTL::<Identity, Impl, OFFSET>,
            GetTTL: GetTTL::<Identity, Impl, OFFSET>,
            SetAlias: SetAlias::<Identity, Impl, OFFSET>,
            GetAlias: GetAlias::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDUdpAddress as ::windows_core::ComInterface>::IID || iid == &<IWSDAddress as ::windows_core::ComInterface>::IID || iid == &<IWSDTransportAddress as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDUdpMessageParameters_Impl: Sized + IWSDMessageParameters_Impl {
    fn SetRetransmitParams(&self, pparams: *const WSDUdpRetransmitParams) -> ::windows_core::Result<()>;
    fn GetRetransmitParams(&self, pparams: *mut WSDUdpRetransmitParams) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWSDUdpMessageParameters {}
impl IWSDUdpMessageParameters_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpMessageParameters_Impl, const OFFSET: isize>() -> IWSDUdpMessageParameters_Vtbl {
        unsafe extern "system" fn SetRetransmitParams<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRetransmitParams(::core::mem::transmute_copy(&pparams)).into()
        }
        unsafe extern "system" fn GetRetransmitParams<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRetransmitParams(::core::mem::transmute_copy(&pparams)).into()
        }
        Self {
            base__: IWSDMessageParameters_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRetransmitParams: SetRetransmitParams::<Identity, Impl, OFFSET>,
            GetRetransmitParams: GetRetransmitParams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDUdpMessageParameters as ::windows_core::ComInterface>::IID || iid == &<IWSDMessageParameters as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDXMLContext_Impl: Sized {
    fn AddNamespace(&self, pszuri: &::windows_core::PCWSTR, pszsuggestedprefix: &::windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows_core::Result<()>;
    fn AddNameToNamespace(&self, pszuri: &::windows_core::PCWSTR, pszname: &::windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_core::Result<()>;
    fn SetNamespaces(&self, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows_core::Result<()>;
    fn SetTypes(&self, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWSDXMLContext {}
impl IWSDXMLContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: isize>() -> IWSDXMLContext_Vtbl {
        unsafe extern "system" fn AddNamespace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuri: ::windows_core::PCWSTR, pszsuggestedprefix: ::windows_core::PCWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNamespace(::core::mem::transmute(&pszuri), ::core::mem::transmute(&pszsuggestedprefix), ::core::mem::transmute_copy(&ppnamespace)).into()
        }
        unsafe extern "system" fn AddNameToNamespace<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuri: ::windows_core::PCWSTR, pszname: ::windows_core::PCWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNameToNamespace(::core::mem::transmute(&pszuri), ::core::mem::transmute(&pszname), ::core::mem::transmute_copy(&ppname)).into()
        }
        unsafe extern "system" fn SetNamespaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetNamespaces(::core::mem::transmute_copy(&pnamespaces), ::core::mem::transmute_copy(&wnamespacescount), ::core::mem::transmute_copy(&blayernumber)).into()
        }
        unsafe extern "system" fn SetTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTypes(::core::mem::transmute_copy(&ptypes), ::core::mem::transmute_copy(&dwtypescount), ::core::mem::transmute_copy(&blayernumber)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddNamespace: AddNamespace::<Identity, Impl, OFFSET>,
            AddNameToNamespace: AddNameToNamespace::<Identity, Impl, OFFSET>,
            SetNamespaces: SetNamespaces::<Identity, Impl, OFFSET>,
            SetTypes: SetTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDXMLContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDiscoveredService_Impl: Sized {
    fn GetEndpointReference(&self) -> ::windows_core::Result<*mut WSD_ENDPOINT_REFERENCE>;
    fn GetTypes(&self) -> ::windows_core::Result<*mut WSD_NAME_LIST>;
    fn GetScopes(&self) -> ::windows_core::Result<*mut WSD_URI_LIST>;
    fn GetXAddrs(&self) -> ::windows_core::Result<*mut WSD_URI_LIST>;
    fn GetMetadataVersion(&self) -> ::windows_core::Result<u64>;
    fn GetExtendedDiscoXML(&self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn GetProbeResolveTag(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetRemoteTransportAddress(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetLocalTransportAddress(&self) -> ::windows_core::Result<::windows_core::PCWSTR>;
    fn GetLocalInterfaceGUID(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetInstanceId(&self) -> ::windows_core::Result<u64>;
}
impl ::windows_core::RuntimeName for IWSDiscoveredService {}
impl IWSDiscoveredService_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>() -> IWSDiscoveredService_Vtbl {
        unsafe extern "system" fn GetEndpointReference<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEndpointReference() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppendpointreference, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptypeslist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetScopes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscopeslist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXAddrs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXAddrs() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppxaddrslist, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullmetadataversion: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMetadataVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullmetadataversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendedDiscoXML<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetExtendedDiscoXML(::core::mem::transmute_copy(&ppheaderany), ::core::mem::transmute_copy(&ppbodyany)).into()
        }
        unsafe extern "system" fn GetProbeResolveTag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztag: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProbeResolveTag() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsztag, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteTransportAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszremotetransportaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRemoteTransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszremotetransportaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalTransportAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlocaltransportaddress: *mut ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalTransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszlocaltransportaddress, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalInterfaceGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLocalInterfaceGUID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullinstanceid: *mut u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pullinstanceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEndpointReference: GetEndpointReference::<Identity, Impl, OFFSET>,
            GetTypes: GetTypes::<Identity, Impl, OFFSET>,
            GetScopes: GetScopes::<Identity, Impl, OFFSET>,
            GetXAddrs: GetXAddrs::<Identity, Impl, OFFSET>,
            GetMetadataVersion: GetMetadataVersion::<Identity, Impl, OFFSET>,
            GetExtendedDiscoXML: GetExtendedDiscoXML::<Identity, Impl, OFFSET>,
            GetProbeResolveTag: GetProbeResolveTag::<Identity, Impl, OFFSET>,
            GetRemoteTransportAddress: GetRemoteTransportAddress::<Identity, Impl, OFFSET>,
            GetLocalTransportAddress: GetLocalTransportAddress::<Identity, Impl, OFFSET>,
            GetLocalInterfaceGUID: GetLocalInterfaceGUID::<Identity, Impl, OFFSET>,
            GetInstanceId: GetInstanceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDiscoveredService as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDiscoveryProvider_Impl: Sized {
    fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows_core::Result<()>;
    fn Attach(&self, psink: ::core::option::Option<&IWSDiscoveryProviderNotify>) -> ::windows_core::Result<()>;
    fn Detach(&self) -> ::windows_core::Result<()>;
    fn SearchById(&self, pszid: &::windows_core::PCWSTR, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SearchByAddress(&self, pszaddress: &::windows_core::PCWSTR, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SearchByType(&self, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: &::windows_core::PCWSTR, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetXMLContext(&self) -> ::windows_core::Result<IWSDXMLContext>;
}
impl ::windows_core::RuntimeName for IWSDiscoveryProvider {}
impl IWSDiscoveryProvider_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>() -> IWSDiscoveryProvider_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAddressFamily(::core::mem::transmute_copy(&dwaddressfamily)).into()
        }
        unsafe extern "system" fn Attach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Attach(::windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn Detach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Detach().into()
        }
        unsafe extern "system" fn SearchById<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SearchById(::core::mem::transmute(&pszid), ::core::mem::transmute(&psztag)).into()
        }
        unsafe extern "system" fn SearchByAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaddress: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SearchByAddress(::core::mem::transmute(&pszaddress), ::core::mem::transmute(&psztag)).into()
        }
        unsafe extern "system" fn SearchByType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: ::windows_core::PCWSTR, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SearchByType(::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute(&pszmatchby), ::core::mem::transmute(&psztag)).into()
        }
        unsafe extern "system" fn GetXMLContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXMLContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
            SearchById: SearchById::<Identity, Impl, OFFSET>,
            SearchByAddress: SearchByAddress::<Identity, Impl, OFFSET>,
            SearchByType: SearchByType::<Identity, Impl, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryProvider as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDiscoveryProviderNotify_Impl: Sized {
    fn Add(&self, pservice: ::core::option::Option<&IWSDiscoveredService>) -> ::windows_core::Result<()>;
    fn Remove(&self, pservice: ::core::option::Option<&IWSDiscoveredService>) -> ::windows_core::Result<()>;
    fn SearchFailed(&self, hr: ::windows_core::HRESULT, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SearchComplete(&self, psztag: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWSDiscoveryProviderNotify {}
impl IWSDiscoveryProviderNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>() -> IWSDiscoveryProviderNotify_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pservice: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::windows_core::from_raw_borrowed(&pservice)).into()
        }
        unsafe extern "system" fn SearchFailed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SearchFailed(::core::mem::transmute_copy(&hr), ::core::mem::transmute(&psztag)).into()
        }
        unsafe extern "system" fn SearchComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztag: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SearchComplete(::core::mem::transmute(&psztag)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            SearchFailed: SearchFailed::<Identity, Impl, OFFSET>,
            SearchComplete: SearchComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryProviderNotify as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDiscoveryPublisher_Impl: Sized {
    fn SetAddressFamily(&self, dwaddressfamily: u32) -> ::windows_core::Result<()>;
    fn RegisterNotificationSink(&self, psink: ::core::option::Option<&IWSDiscoveryPublisherNotify>) -> ::windows_core::Result<()>;
    fn UnRegisterNotificationSink(&self, psink: ::core::option::Option<&IWSDiscoveryPublisherNotify>) -> ::windows_core::Result<()>;
    fn Publish(&self, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::Result<()>;
    fn UnPublish(&self, pszid: &::windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn MatchProbe(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::Result<()>;
    fn MatchResolve(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::Result<()>;
    fn PublishEx(&self, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn MatchProbeEx(&self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn MatchResolveEx(&self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>, pszid: &::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: &::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::Result<()>;
    fn RegisterScopeMatchingRule(&self, pscopematchingrule: ::core::option::Option<&IWSDScopeMatchingRule>) -> ::windows_core::Result<()>;
    fn UnRegisterScopeMatchingRule(&self, pscopematchingrule: ::core::option::Option<&IWSDScopeMatchingRule>) -> ::windows_core::Result<()>;
    fn GetXMLContext(&self) -> ::windows_core::Result<IWSDXMLContext>;
}
impl ::windows_core::RuntimeName for IWSDiscoveryPublisher {}
impl IWSDiscoveryPublisher_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>() -> IWSDiscoveryPublisher_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAddressFamily(::core::mem::transmute_copy(&dwaddressfamily)).into()
        }
        unsafe extern "system" fn RegisterNotificationSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterNotificationSink(::windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn UnRegisterNotificationSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnRegisterNotificationSink(::windows_core::from_raw_borrowed(&psink)).into()
        }
        unsafe extern "system" fn Publish<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Publish(::core::mem::transmute(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn UnPublish<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnPublish(::core::mem::transmute(&pszid), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute(&pszsessionid), ::core::mem::transmute_copy(&pany)).into()
        }
        unsafe extern "system" fn MatchProbe<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MatchProbe(::core::mem::transmute_copy(&pprobemessage), ::windows_core::from_raw_borrowed(&pmessageparameters), ::core::mem::transmute(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn MatchResolve<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MatchResolve(::core::mem::transmute_copy(&presolvemessage), ::windows_core::from_raw_borrowed(&pmessageparameters), ::core::mem::transmute(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn PublishEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PublishEx(
                ::core::mem::transmute(&pszid),
                ::core::mem::transmute_copy(&ullmetadataversion),
                ::core::mem::transmute_copy(&ullinstanceid),
                ::core::mem::transmute_copy(&ullmessagenumber),
                ::core::mem::transmute(&pszsessionid),
                ::core::mem::transmute_copy(&ptypeslist),
                ::core::mem::transmute_copy(&pscopeslist),
                ::core::mem::transmute_copy(&pxaddrslist),
                ::core::mem::transmute_copy(&pheaderany),
                ::core::mem::transmute_copy(&preferenceparameterany),
                ::core::mem::transmute_copy(&ppolicyany),
                ::core::mem::transmute_copy(&pendpointreferenceany),
                ::core::mem::transmute_copy(&pany),
            )
            .into()
        }
        unsafe extern "system" fn MatchProbeEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MatchProbeEx(
                ::core::mem::transmute_copy(&pprobemessage),
                ::windows_core::from_raw_borrowed(&pmessageparameters),
                ::core::mem::transmute(&pszid),
                ::core::mem::transmute_copy(&ullmetadataversion),
                ::core::mem::transmute_copy(&ullinstanceid),
                ::core::mem::transmute_copy(&ullmessagenumber),
                ::core::mem::transmute(&pszsessionid),
                ::core::mem::transmute_copy(&ptypeslist),
                ::core::mem::transmute_copy(&pscopeslist),
                ::core::mem::transmute_copy(&pxaddrslist),
                ::core::mem::transmute_copy(&pheaderany),
                ::core::mem::transmute_copy(&preferenceparameterany),
                ::core::mem::transmute_copy(&ppolicyany),
                ::core::mem::transmute_copy(&pendpointreferenceany),
                ::core::mem::transmute_copy(&pany),
            )
            .into()
        }
        unsafe extern "system" fn MatchResolveEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void, pszid: ::windows_core::PCWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: ::windows_core::PCWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MatchResolveEx(
                ::core::mem::transmute_copy(&presolvemessage),
                ::windows_core::from_raw_borrowed(&pmessageparameters),
                ::core::mem::transmute(&pszid),
                ::core::mem::transmute_copy(&ullmetadataversion),
                ::core::mem::transmute_copy(&ullinstanceid),
                ::core::mem::transmute_copy(&ullmessagenumber),
                ::core::mem::transmute(&pszsessionid),
                ::core::mem::transmute_copy(&ptypeslist),
                ::core::mem::transmute_copy(&pscopeslist),
                ::core::mem::transmute_copy(&pxaddrslist),
                ::core::mem::transmute_copy(&pheaderany),
                ::core::mem::transmute_copy(&preferenceparameterany),
                ::core::mem::transmute_copy(&ppolicyany),
                ::core::mem::transmute_copy(&pendpointreferenceany),
                ::core::mem::transmute_copy(&pany),
            )
            .into()
        }
        unsafe extern "system" fn RegisterScopeMatchingRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterScopeMatchingRule(::windows_core::from_raw_borrowed(&pscopematchingrule)).into()
        }
        unsafe extern "system" fn UnRegisterScopeMatchingRule<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopematchingrule: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnRegisterScopeMatchingRule(::windows_core::from_raw_borrowed(&pscopematchingrule)).into()
        }
        unsafe extern "system" fn GetXMLContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetXMLContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontext, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, Impl, OFFSET>,
            RegisterNotificationSink: RegisterNotificationSink::<Identity, Impl, OFFSET>,
            UnRegisterNotificationSink: UnRegisterNotificationSink::<Identity, Impl, OFFSET>,
            Publish: Publish::<Identity, Impl, OFFSET>,
            UnPublish: UnPublish::<Identity, Impl, OFFSET>,
            MatchProbe: MatchProbe::<Identity, Impl, OFFSET>,
            MatchResolve: MatchResolve::<Identity, Impl, OFFSET>,
            PublishEx: PublishEx::<Identity, Impl, OFFSET>,
            MatchProbeEx: MatchProbeEx::<Identity, Impl, OFFSET>,
            MatchResolveEx: MatchResolveEx::<Identity, Impl, OFFSET>,
            RegisterScopeMatchingRule: RegisterScopeMatchingRule::<Identity, Impl, OFFSET>,
            UnRegisterScopeMatchingRule: UnRegisterScopeMatchingRule::<Identity, Impl, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisher as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_WebServicesOnDevices\"`, `\"implement\"`*"]
pub trait IWSDiscoveryPublisherNotify_Impl: Sized {
    fn ProbeHandler(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>) -> ::windows_core::Result<()>;
    fn ResolveHandler(&self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::core::option::Option<&IWSDMessageParameters>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IWSDiscoveryPublisherNotify {}
impl IWSDiscoveryPublisherNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>() -> IWSDiscoveryPublisherNotify_Vtbl {
        unsafe extern "system" fn ProbeHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProbeHandler(::core::mem::transmute_copy(&psoap), ::windows_core::from_raw_borrowed(&pmessageparameters)).into()
        }
        unsafe extern "system" fn ResolveHandler<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResolveHandler(::core::mem::transmute_copy(&psoap), ::windows_core::from_raw_borrowed(&pmessageparameters)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProbeHandler: ProbeHandler::<Identity, Impl, OFFSET>,
            ResolveHandler: ResolveHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisherNotify as ::windows_core::ComInterface>::IID
    }
}
