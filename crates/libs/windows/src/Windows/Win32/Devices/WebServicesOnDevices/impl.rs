#[cfg(feature = "Win32_Foundation")]
pub trait IWSDAddress_Impl: Sized {
    fn Serialize(&mut self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Deserialize(&mut self, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAddress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDAddress_Vtbl {
        unsafe extern "system" fn Serialize<Impl: IWSDAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchlength), ::core::mem::transmute_copy(&fsafe)).into()
        }
        unsafe extern "system" fn Deserialize<Impl: IWSDAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deserialize(::core::mem::transmute_copy(&pszbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Serialize: Serialize::<Impl, IMPL_OFFSET>,
            Deserialize: Deserialize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDAddress as ::windows::core::Interface>::IID
    }
}
pub trait IWSDAsyncCallback_Impl: Sized {
    fn AsyncOperationComplete(&mut self, pasyncresult: &::core::option::Option<IWSDAsyncResult>, pasyncstate: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IWSDAsyncCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDAsyncCallback_Vtbl {
        unsafe extern "system" fn AsyncOperationComplete<Impl: IWSDAsyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncOperationComplete(::core::mem::transmute(&pasyncresult), ::core::mem::transmute(&pasyncstate)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDAsyncCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDAsyncResult_Impl: Sized {
    fn SetCallback(&mut self, pcallback: &::core::option::Option<IWSDAsyncCallback>, pasyncstate: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetWaitHandle(&mut self, hwaithandle: super::super::Foundation::HANDLE) -> ::windows::core::Result<()>;
    fn HasCompleted(&mut self) -> ::windows::core::Result<()>;
    fn GetAsyncState(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
    fn GetEvent(&mut self) -> ::windows::core::Result<WSD_EVENT>;
    fn GetEndpointProxy(&mut self) -> ::windows::core::Result<IWSDEndpointProxy>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDAsyncResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDAsyncResult_Vtbl {
        unsafe extern "system" fn SetCallback<Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCallback(::core::mem::transmute(&pcallback), ::core::mem::transmute(&pasyncstate)).into()
        }
        unsafe extern "system" fn SetWaitHandle<Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwaithandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWaitHandle(::core::mem::transmute_copy(&hwaithandle)).into()
        }
        unsafe extern "system" fn HasCompleted<Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HasCompleted().into()
        }
        unsafe extern "system" fn GetAsyncState<Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasyncstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        unsafe extern "system" fn GetEvent<Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut WSD_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppendpoint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppendpoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetCallback: SetCallback::<Impl, IMPL_OFFSET>,
            SetWaitHandle: SetWaitHandle::<Impl, IMPL_OFFSET>,
            HasCompleted: HasCompleted::<Impl, IMPL_OFFSET>,
            GetAsyncState: GetAsyncState::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
            GetEvent: GetEvent::<Impl, IMPL_OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDAsyncResult as ::windows::core::Interface>::IID
    }
}
pub trait IWSDAttachment_Impl: Sized {}
impl IWSDAttachment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAttachment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDAttachment_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDAttachment as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDDeviceHost_Impl: Sized {
    fn Init(&mut self, pszlocalid: super::super::Foundation::PWSTR, pcontext: &::core::option::Option<IWSDXMLContext>, pphostaddresses: *const ::core::option::Option<IWSDAddress>, dwhostaddresscount: u32) -> ::windows::core::Result<()>;
    fn Start(&mut self, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: &::core::option::Option<IWSDDeviceHostNotify>) -> ::windows::core::Result<()>;
    fn Stop(&mut self) -> ::windows::core::Result<()>;
    fn Terminate(&mut self) -> ::windows::core::Result<()>;
    fn RegisterPortType(&mut self, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::Result<()>;
    fn SetMetadata(&mut self, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::core::Result<()>;
    fn RegisterService(&mut self, pszserviceid: super::super::Foundation::PWSTR, pservice: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RetireService(&mut self, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddDynamicService(&mut self, pszserviceid: super::super::Foundation::PWSTR, pszendpointaddress: super::super::Foundation::PWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn RemoveDynamicService(&mut self, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetServiceDiscoverable(&mut self, pszserviceid: super::super::Foundation::PWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SignalEvent(&mut self, pszserviceid: super::super::Foundation::PWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDDeviceHost_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDDeviceHost_Vtbl {
        unsafe extern "system" fn Init<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, pphostaddresses: *const ::windows::core::RawPtr, dwhostaddresscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pszlocalid), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pphostaddresses), ::core::mem::transmute_copy(&dwhostaddresscount)).into()
        }
        unsafe extern "system" fn Start<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&pscopelist), ::core::mem::transmute(&pnotificationsink)).into()
        }
        unsafe extern "system" fn Stop<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Terminate<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Terminate().into()
        }
        unsafe extern "system" fn RegisterPortType<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterPortType(::core::mem::transmute_copy(&pporttype)).into()
        }
        unsafe extern "system" fn SetMetadata<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMetadata(::core::mem::transmute_copy(&pthismodelmetadata), ::core::mem::transmute_copy(&pthisdevicemetadata), ::core::mem::transmute_copy(&phostmetadata), ::core::mem::transmute_copy(&pcustommetadata)).into()
        }
        unsafe extern "system" fn RegisterService<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterService(::core::mem::transmute_copy(&pszserviceid), ::core::mem::transmute(&pservice)).into()
        }
        unsafe extern "system" fn RetireService<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RetireService(::core::mem::transmute_copy(&pszserviceid)).into()
        }
        unsafe extern "system" fn AddDynamicService<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, pszendpointaddress: super::super::Foundation::PWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDynamicService(::core::mem::transmute_copy(&pszserviceid), ::core::mem::transmute_copy(&pszendpointaddress), ::core::mem::transmute_copy(&pporttype), ::core::mem::transmute_copy(&pportname), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pservice)).into()
        }
        unsafe extern "system" fn RemoveDynamicService<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDynamicService(::core::mem::transmute_copy(&pszserviceid)).into()
        }
        unsafe extern "system" fn SetServiceDiscoverable<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetServiceDiscoverable(::core::mem::transmute_copy(&pszserviceid), ::core::mem::transmute_copy(&fdiscoverable)).into()
        }
        unsafe extern "system" fn SignalEvent<Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SignalEvent(::core::mem::transmute_copy(&pszserviceid), ::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            Start: Start::<Impl, IMPL_OFFSET>,
            Stop: Stop::<Impl, IMPL_OFFSET>,
            Terminate: Terminate::<Impl, IMPL_OFFSET>,
            RegisterPortType: RegisterPortType::<Impl, IMPL_OFFSET>,
            SetMetadata: SetMetadata::<Impl, IMPL_OFFSET>,
            RegisterService: RegisterService::<Impl, IMPL_OFFSET>,
            RetireService: RetireService::<Impl, IMPL_OFFSET>,
            AddDynamicService: AddDynamicService::<Impl, IMPL_OFFSET>,
            RemoveDynamicService: RemoveDynamicService::<Impl, IMPL_OFFSET>,
            SetServiceDiscoverable: SetServiceDiscoverable::<Impl, IMPL_OFFSET>,
            SignalEvent: SignalEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDDeviceHost as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDDeviceHostNotify_Impl: Sized {
    fn GetService(&mut self, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDDeviceHostNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHostNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDDeviceHostNotify_Vtbl {
        unsafe extern "system" fn GetService<Impl: IWSDDeviceHostNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetService(::core::mem::transmute_copy(&pszserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetService: GetService::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDDeviceHostNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDDeviceProxy_Impl: Sized {
    fn Init(&mut self, pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: &::core::option::Option<IWSDAddress>, pszlocalid: super::super::Foundation::PWSTR, pcontext: &::core::option::Option<IWSDXMLContext>, psponsor: &::core::option::Option<IWSDDeviceProxy>) -> ::windows::core::Result<()>;
    fn BeginGetMetadata(&mut self) -> ::windows::core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&mut self, presult: &::core::option::Option<IWSDAsyncResult>) -> ::windows::core::Result<()>;
    fn GetHostMetadata(&mut self) -> ::windows::core::Result<*mut WSD_HOST_METADATA>;
    fn GetThisModelMetadata(&mut self) -> ::windows::core::Result<*mut WSD_THIS_MODEL_METADATA>;
    fn GetThisDeviceMetadata(&mut self) -> ::windows::core::Result<*mut WSD_THIS_DEVICE_METADATA>;
    fn GetAllMetadata(&mut self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceProxyById(&mut self, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::Result<IWSDServiceProxy>;
    fn GetServiceProxyByType(&mut self, ptype: *const WSDXML_NAME) -> ::windows::core::Result<IWSDServiceProxy>;
    fn GetEndpointProxy(&mut self) -> ::windows::core::Result<IWSDEndpointProxy>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDDeviceProxy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDDeviceProxy_Vtbl {
        unsafe extern "system" fn Init<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: ::windows::core::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, psponsor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pszdeviceid), ::core::mem::transmute(&pdeviceaddress), ::core::mem::transmute_copy(&pszlocalid), ::core::mem::transmute(&pcontext), ::core::mem::transmute(&psponsor)).into()
        }
        unsafe extern "system" fn BeginGetMetadata<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginGetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndGetMetadata(::core::mem::transmute(&presult)).into()
        }
        unsafe extern "system" fn GetHostMetadata<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHostMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *pphostmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisModelMetadata<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThisModelMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmanufacturermetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisDeviceMetadata<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThisDeviceMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppthisdevicemetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllMetadata<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyById<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, ppserviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceProxyById(::core::mem::transmute_copy(&pszserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppserviceproxy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyByType<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceProxyByType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppserviceproxy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproxy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            BeginGetMetadata: BeginGetMetadata::<Impl, IMPL_OFFSET>,
            EndGetMetadata: EndGetMetadata::<Impl, IMPL_OFFSET>,
            GetHostMetadata: GetHostMetadata::<Impl, IMPL_OFFSET>,
            GetThisModelMetadata: GetThisModelMetadata::<Impl, IMPL_OFFSET>,
            GetThisDeviceMetadata: GetThisDeviceMetadata::<Impl, IMPL_OFFSET>,
            GetAllMetadata: GetAllMetadata::<Impl, IMPL_OFFSET>,
            GetServiceProxyById: GetServiceProxyById::<Impl, IMPL_OFFSET>,
            GetServiceProxyByType: GetServiceProxyByType::<Impl, IMPL_OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDDeviceProxy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDEndpointProxy_Impl: Sized {
    fn SendOneWayRequest(&mut self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()>;
    fn SendTwoWayRequest(&mut self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows::core::Result<()>;
    fn SendTwoWayRequestAsync(&mut self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: &::core::option::Option<::windows::core::IUnknown>, pcallback: &::core::option::Option<IWSDAsyncCallback>) -> ::windows::core::Result<IWSDAsyncResult>;
    fn AbortAsyncOperation(&mut self, pasyncresult: &::core::option::Option<IWSDAsyncResult>) -> ::windows::core::Result<()>;
    fn ProcessFault(&mut self, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::Result<()>;
    fn GetErrorInfo(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetFaultInfo(&mut self) -> ::windows::core::Result<*mut WSD_SOAP_FAULT>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDEndpointProxy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDEndpointProxy_Vtbl {
        unsafe extern "system" fn SendOneWayRequest<Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendOneWayRequest(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation)).into()
        }
        unsafe extern "system" fn SendTwoWayRequest<Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendTwoWayRequest(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::core::mem::transmute_copy(&presponsecontext)).into()
        }
        unsafe extern "system" fn SendTwoWayRequestAsync<Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, presult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendTwoWayRequestAsync(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAsyncOperation<Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AbortAsyncOperation(::core::mem::transmute(&pasyncresult)).into()
        }
        unsafe extern "system" fn ProcessFault<Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessFault(::core::mem::transmute_copy(&pfault)).into()
        }
        unsafe extern "system" fn GetErrorInfo<Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszerrorinfo: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszerrorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaultInfo<Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFaultInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfault = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SendOneWayRequest: SendOneWayRequest::<Impl, IMPL_OFFSET>,
            SendTwoWayRequest: SendTwoWayRequest::<Impl, IMPL_OFFSET>,
            SendTwoWayRequestAsync: SendTwoWayRequestAsync::<Impl, IMPL_OFFSET>,
            AbortAsyncOperation: AbortAsyncOperation::<Impl, IMPL_OFFSET>,
            ProcessFault: ProcessFault::<Impl, IMPL_OFFSET>,
            GetErrorInfo: GetErrorInfo::<Impl, IMPL_OFFSET>,
            GetFaultInfo: GetFaultInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDEndpointProxy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDEventingStatus_Impl: Sized {
    fn SubscriptionRenewed(&mut self, pszsubscriptionaction: super::super::Foundation::PWSTR);
    fn SubscriptionRenewalFailed(&mut self, pszsubscriptionaction: super::super::Foundation::PWSTR, hr: ::windows::core::HRESULT);
    fn SubscriptionEnded(&mut self, pszsubscriptionaction: super::super::Foundation::PWSTR);
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDEventingStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEventingStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDEventingStatus_Vtbl {
        unsafe extern "system" fn SubscriptionRenewed<Impl: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubscriptionRenewed(::core::mem::transmute_copy(&pszsubscriptionaction))
        }
        unsafe extern "system" fn SubscriptionRenewalFailed<Impl: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: super::super::Foundation::PWSTR, hr: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubscriptionRenewalFailed(::core::mem::transmute_copy(&pszsubscriptionaction), ::core::mem::transmute_copy(&hr))
        }
        unsafe extern "system" fn SubscriptionEnded<Impl: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubscriptionEnded(::core::mem::transmute_copy(&pszsubscriptionaction))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SubscriptionRenewed: SubscriptionRenewed::<Impl, IMPL_OFFSET>,
            SubscriptionRenewalFailed: SubscriptionRenewalFailed::<Impl, IMPL_OFFSET>,
            SubscriptionEnded: SubscriptionEnded::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDEventingStatus as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDHttpAddress_Impl: Sized + IWSDAddress_Impl + IWSDTransportAddress_Impl {
    fn GetSecure(&mut self) -> ::windows::core::Result<()>;
    fn SetSecure(&mut self, fsecure: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPath(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetPath(&mut self, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDHttpAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAddress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDHttpAddress_Vtbl {
        unsafe extern "system" fn GetSecure<Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSecure().into()
        }
        unsafe extern "system" fn SetSecure<Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSecure(::core::mem::transmute_copy(&fsecure)).into()
        }
        unsafe extern "system" fn GetPath<Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&pszpath)).into()
        }
        Self {
            base: IWSDTransportAddress_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSecure: GetSecure::<Impl, IMPL_OFFSET>,
            SetSecure: SetSecure::<Impl, IMPL_OFFSET>,
            GetPath: GetPath::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDHttpAddress as ::windows::core::Interface>::IID || iid == &<IWSDAddress as ::windows::core::Interface>::IID || iid == &<IWSDTransportAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDHttpAuthParameters_Impl: Sized {
    fn GetClientAccessToken(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
    fn GetAuthType(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDHttpAuthParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAuthParameters_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDHttpAuthParameters_Vtbl {
        unsafe extern "system" fn GetClientAccessToken<Impl: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientAccessToken() {
                ::core::result::Result::Ok(ok__) => {
                    *phtoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthType<Impl: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthType() {
                ::core::result::Result::Ok(ok__) => {
                    *pauthtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClientAccessToken: GetClientAccessToken::<Impl, IMPL_OFFSET>,
            GetAuthType: GetAuthType::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDHttpAuthParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDHttpMessageParameters_Impl: Sized + IWSDMessageParameters_Impl {
    fn SetInboundHttpHeaders(&mut self, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetInboundHttpHeaders(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetOutboundHttpHeaders(&mut self, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetOutboundHttpHeaders(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetID(&mut self, pszid: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetID(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetContext(&mut self, pcontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetContext(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDHttpMessageParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDHttpMessageParameters_Vtbl {
        unsafe extern "system" fn SetInboundHttpHeaders<Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInboundHttpHeaders(::core::mem::transmute_copy(&pszheaders)).into()
        }
        unsafe extern "system" fn GetInboundHttpHeaders<Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInboundHttpHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundHttpHeaders<Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOutboundHttpHeaders(::core::mem::transmute_copy(&pszheaders)).into()
        }
        unsafe extern "system" fn GetOutboundHttpHeaders<Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutboundHttpHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetID<Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetID(::core::mem::transmute_copy(&pszid)).into()
        }
        unsafe extern "system" fn GetID<Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn GetContext<Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IWSDMessageParameters_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetInboundHttpHeaders: SetInboundHttpHeaders::<Impl, IMPL_OFFSET>,
            GetInboundHttpHeaders: GetInboundHttpHeaders::<Impl, IMPL_OFFSET>,
            SetOutboundHttpHeaders: SetOutboundHttpHeaders::<Impl, IMPL_OFFSET>,
            GetOutboundHttpHeaders: GetOutboundHttpHeaders::<Impl, IMPL_OFFSET>,
            SetID: SetID::<Impl, IMPL_OFFSET>,
            GetID: GetID::<Impl, IMPL_OFFSET>,
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDHttpMessageParameters as ::windows::core::Interface>::IID || iid == &<IWSDMessageParameters as ::windows::core::Interface>::IID
    }
}
pub trait IWSDInboundAttachment_Impl: Sized + IWSDAttachment_Impl {
    fn Read(&mut self, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl IWSDInboundAttachment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDInboundAttachment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDInboundAttachment_Vtbl {
        unsafe extern "system" fn Read<Impl: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbytestoread), ::core::mem::transmute_copy(&pdwnumberofbytesread)).into()
        }
        unsafe extern "system" fn Close<Impl: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self { base: IWSDAttachment_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Read: Read::<Impl, IMPL_OFFSET>, Close: Close::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDInboundAttachment as ::windows::core::Interface>::IID || iid == &<IWSDAttachment as ::windows::core::Interface>::IID
    }
}
pub trait IWSDMessageParameters_Impl: Sized {
    fn GetLocalAddress(&mut self) -> ::windows::core::Result<IWSDAddress>;
    fn SetLocalAddress(&mut self, paddress: &::core::option::Option<IWSDAddress>) -> ::windows::core::Result<()>;
    fn GetRemoteAddress(&mut self) -> ::windows::core::Result<IWSDAddress>;
    fn SetRemoteAddress(&mut self, paddress: &::core::option::Option<IWSDAddress>) -> ::windows::core::Result<()>;
    fn GetLowerParameters(&mut self) -> ::windows::core::Result<IWSDMessageParameters>;
}
impl IWSDMessageParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMessageParameters_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDMessageParameters_Vtbl {
        unsafe extern "system" fn GetLocalAddress<Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddress<Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLocalAddress(::core::mem::transmute(&paddress)).into()
        }
        unsafe extern "system" fn GetRemoteAddress<Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddress<Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRemoteAddress(::core::mem::transmute(&paddress)).into()
        }
        unsafe extern "system" fn GetLowerParameters<Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptxparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLowerParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pptxparams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetLocalAddress: GetLocalAddress::<Impl, IMPL_OFFSET>,
            SetLocalAddress: SetLocalAddress::<Impl, IMPL_OFFSET>,
            GetRemoteAddress: GetRemoteAddress::<Impl, IMPL_OFFSET>,
            SetRemoteAddress: SetRemoteAddress::<Impl, IMPL_OFFSET>,
            GetLowerParameters: GetLowerParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDMessageParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDMetadataExchange_Impl: Sized {
    fn GetMetadata(&mut self) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDMetadataExchange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMetadataExchange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDMetadataExchange_Vtbl {
        unsafe extern "system" fn GetMetadata<Impl: IWSDMetadataExchange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *metadataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetMetadata: GetMetadata::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDMetadataExchange as ::windows::core::Interface>::IID
    }
}
pub trait IWSDOutboundAttachment_Impl: Sized + IWSDAttachment_Impl {
    fn Write(&mut self, pbuffer: *const u8, dwbytestowrite: u32) -> ::windows::core::Result<u32>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
}
impl IWSDOutboundAttachment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDOutboundAttachment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDOutboundAttachment_Vtbl {
        unsafe extern "system" fn Write<Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbytestowrite)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwnumberofbyteswritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Abort<Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        Self {
            base: IWSDAttachment_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Write: Write::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDOutboundAttachment as ::windows::core::Interface>::IID || iid == &<IWSDAttachment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub trait IWSDSSLClientCertificate_Impl: Sized {
    fn GetClientCertificate(&mut self) -> ::windows::core::Result<*mut super::super::Security::Cryptography::CERT_CONTEXT>;
    fn GetMappedAccessToken(&mut self) -> ::windows::core::Result<super::super::Foundation::HANDLE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl IWSDSSLClientCertificate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSSLClientCertificate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDSSLClientCertificate_Vtbl {
        unsafe extern "system" fn GetClientCertificate<Impl: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcertcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMappedAccessToken<Impl: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMappedAccessToken() {
                ::core::result::Result::Ok(ok__) => {
                    *phtoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClientCertificate: GetClientCertificate::<Impl, IMPL_OFFSET>,
            GetMappedAccessToken: GetMappedAccessToken::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDSSLClientCertificate as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDScopeMatchingRule_Impl: Sized {
    fn GetScopeRule(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn MatchScopes(&mut self, pszscope1: super::super::Foundation::PWSTR, pszscope2: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDScopeMatchingRule_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDScopeMatchingRule_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDScopeMatchingRule_Vtbl {
        unsafe extern "system" fn GetScopeRule<Impl: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszscopematchingrule: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScopeRule() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszscopematchingrule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchScopes<Impl: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszscope1: super::super::Foundation::PWSTR, pszscope2: super::super::Foundation::PWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchScopes(::core::mem::transmute_copy(&pszscope1), ::core::mem::transmute_copy(&pszscope2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetScopeRule: GetScopeRule::<Impl, IMPL_OFFSET>,
            MatchScopes: MatchScopes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDScopeMatchingRule as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDServiceMessaging_Impl: Sized {
    fn SendResponse(&mut self, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: &::core::option::Option<IWSDMessageParameters>) -> ::windows::core::Result<()>;
    fn FaultRequest(&mut self, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: &::core::option::Option<IWSDMessageParameters>, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDServiceMessaging_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceMessaging_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDServiceMessaging_Vtbl {
        unsafe extern "system" fn SendResponse<Impl: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendResponse(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::core::mem::transmute(&pmessageparameters)).into()
        }
        unsafe extern "system" fn FaultRequest<Impl: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: ::windows::core::RawPtr, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FaultRequest(::core::mem::transmute_copy(&prequestheader), ::core::mem::transmute(&pmessageparameters), ::core::mem::transmute_copy(&pfault)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SendResponse: SendResponse::<Impl, IMPL_OFFSET>,
            FaultRequest: FaultRequest::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDServiceMessaging as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDServiceProxy_Impl: Sized + IWSDMetadataExchange_Impl {
    fn BeginGetMetadata(&mut self) -> ::windows::core::Result<IWSDAsyncResult>;
    fn EndGetMetadata(&mut self, presult: &::core::option::Option<IWSDAsyncResult>) -> ::windows::core::Result<*mut WSD_METADATA_SECTION_LIST>;
    fn GetServiceMetadata(&mut self) -> ::windows::core::Result<*mut WSD_SERVICE_METADATA>;
    fn SubscribeToOperation(&mut self, poperation: *const WSD_OPERATION, punknown: &::core::option::Option<::windows::core::IUnknown>, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<*mut WSDXML_ELEMENT>;
    fn UnsubscribeToOperation(&mut self, poperation: *const WSD_OPERATION) -> ::windows::core::Result<()>;
    fn SetEventingStatusCallback(&mut self, pstatus: &::core::option::Option<IWSDEventingStatus>) -> ::windows::core::Result<()>;
    fn GetEndpointProxy(&mut self) -> ::windows::core::Result<IWSDEndpointProxy>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDServiceProxy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDServiceProxy_Vtbl {
        unsafe extern "system" fn BeginGetMetadata<Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginGetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndGetMetadata(::core::mem::transmute(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceMetadata<Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppservicemetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscribeToOperation<Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscribeToOperation(::core::mem::transmute_copy(&poperation), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&pany)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppany = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnsubscribeToOperation<Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnsubscribeToOperation(::core::mem::transmute_copy(&poperation)).into()
        }
        unsafe extern "system" fn SetEventingStatusCallback<Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventingStatusCallback(::core::mem::transmute(&pstatus)).into()
        }
        unsafe extern "system" fn GetEndpointProxy<Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproxy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSDMetadataExchange_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BeginGetMetadata: BeginGetMetadata::<Impl, IMPL_OFFSET>,
            EndGetMetadata: EndGetMetadata::<Impl, IMPL_OFFSET>,
            GetServiceMetadata: GetServiceMetadata::<Impl, IMPL_OFFSET>,
            SubscribeToOperation: SubscribeToOperation::<Impl, IMPL_OFFSET>,
            UnsubscribeToOperation: UnsubscribeToOperation::<Impl, IMPL_OFFSET>,
            SetEventingStatusCallback: SetEventingStatusCallback::<Impl, IMPL_OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDServiceProxy as ::windows::core::Interface>::IID || iid == &<IWSDMetadataExchange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDServiceProxyEventing_Impl: Sized + IWSDMetadataExchange_Impl + IWSDServiceProxy_Impl {
    fn SubscribeToMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: &::core::option::Option<::windows::core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn BeginSubscribeToMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: &::core::option::Option<::windows::core::IUnknown>, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: &::core::option::Option<::windows::core::IUnknown>, pasynccallback: &::core::option::Option<IWSDAsyncCallback>) -> ::windows::core::Result<IWSDAsyncResult>;
    fn EndSubscribeToMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: &::core::option::Option<IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn UnsubscribeToMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn BeginUnsubscribeToMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: &::core::option::Option<::windows::core::IUnknown>, pasynccallback: &::core::option::Option<IWSDAsyncCallback>) -> ::windows::core::Result<IWSDAsyncResult>;
    fn EndUnsubscribeToMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: &::core::option::Option<IWSDAsyncResult>) -> ::windows::core::Result<()>;
    fn RenewMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn BeginRenewMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: &::core::option::Option<::windows::core::IUnknown>, pasynccallback: &::core::option::Option<IWSDAsyncCallback>) -> ::windows::core::Result<IWSDAsyncResult>;
    fn EndRenewMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: &::core::option::Option<IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn GetStatusForMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn BeginGetStatusForMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: &::core::option::Option<::windows::core::IUnknown>, pasynccallback: &::core::option::Option<IWSDAsyncCallback>) -> ::windows::core::Result<IWSDAsyncResult>;
    fn EndGetStatusForMultipleOperations(&mut self, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: &::core::option::Option<IWSDAsyncResult>, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDServiceProxyEventing_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDServiceProxyEventing_Vtbl {
        unsafe extern "system" fn SubscribeToMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginSubscribeToMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginSubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndSubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn UnsubscribeToMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnsubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany)).into()
        }
        unsafe extern "system" fn BeginUnsubscribeToMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginUnsubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnsubscribeToMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndUnsubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&presult)).into()
        }
        unsafe extern "system" fn RenewMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RenewMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginRenewMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginRenewMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRenewMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndRenewMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn GetStatusForMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetStatusForMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginGetStatusForMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginGetStatusForMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetStatusForMultipleOperations<Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndGetStatusForMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        Self {
            base: IWSDServiceProxy_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SubscribeToMultipleOperations: SubscribeToMultipleOperations::<Impl, IMPL_OFFSET>,
            BeginSubscribeToMultipleOperations: BeginSubscribeToMultipleOperations::<Impl, IMPL_OFFSET>,
            EndSubscribeToMultipleOperations: EndSubscribeToMultipleOperations::<Impl, IMPL_OFFSET>,
            UnsubscribeToMultipleOperations: UnsubscribeToMultipleOperations::<Impl, IMPL_OFFSET>,
            BeginUnsubscribeToMultipleOperations: BeginUnsubscribeToMultipleOperations::<Impl, IMPL_OFFSET>,
            EndUnsubscribeToMultipleOperations: EndUnsubscribeToMultipleOperations::<Impl, IMPL_OFFSET>,
            RenewMultipleOperations: RenewMultipleOperations::<Impl, IMPL_OFFSET>,
            BeginRenewMultipleOperations: BeginRenewMultipleOperations::<Impl, IMPL_OFFSET>,
            EndRenewMultipleOperations: EndRenewMultipleOperations::<Impl, IMPL_OFFSET>,
            GetStatusForMultipleOperations: GetStatusForMultipleOperations::<Impl, IMPL_OFFSET>,
            BeginGetStatusForMultipleOperations: BeginGetStatusForMultipleOperations::<Impl, IMPL_OFFSET>,
            EndGetStatusForMultipleOperations: EndGetStatusForMultipleOperations::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDServiceProxyEventing as ::windows::core::Interface>::IID || iid == &<IWSDMetadataExchange as ::windows::core::Interface>::IID || iid == &<IWSDServiceProxy as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDSignatureProperty_Impl: Sized {
    fn IsMessageSigned(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn IsMessageSignatureTrusted(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetKeyInfo(&mut self, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::core::Result<()>;
    fn GetSignature(&mut self, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetSignedInfoHash(&mut self, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDSignatureProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSignatureProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDSignatureProperty_Vtbl {
        unsafe extern "system" fn IsMessageSigned<Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMessageSigned() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsigned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMessageSignatureTrusted<Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMessageSignatureTrusted() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsignaturetrusted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyInfo<Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetKeyInfo(::core::mem::transmute_copy(&pbkeyinfo), ::core::mem::transmute_copy(&pdwkeyinfosize)).into()
        }
        unsafe extern "system" fn GetSignature<Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSignature(::core::mem::transmute_copy(&pbsignature), ::core::mem::transmute_copy(&pdwsignaturesize)).into()
        }
        unsafe extern "system" fn GetSignedInfoHash<Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSignedInfoHash(::core::mem::transmute_copy(&pbsignedinfohash), ::core::mem::transmute_copy(&pdwhashsize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsMessageSigned: IsMessageSigned::<Impl, IMPL_OFFSET>,
            IsMessageSignatureTrusted: IsMessageSignatureTrusted::<Impl, IMPL_OFFSET>,
            GetKeyInfo: GetKeyInfo::<Impl, IMPL_OFFSET>,
            GetSignature: GetSignature::<Impl, IMPL_OFFSET>,
            GetSignedInfoHash: GetSignedInfoHash::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDSignatureProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDTransportAddress_Impl: Sized + IWSDAddress_Impl {
    fn GetPort(&mut self) -> ::windows::core::Result<u16>;
    fn SetPort(&mut self, wport: u16) -> ::windows::core::Result<()>;
    fn GetTransportAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetTransportAddressEx(&mut self, fsafe: super::super::Foundation::BOOL) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetTransportAddress(&mut self, pszaddress: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDTransportAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDTransportAddress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDTransportAddress_Vtbl {
        unsafe extern "system" fn GetPort<Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwport: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pwport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wport: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPort(::core::mem::transmute_copy(&wport)).into()
        }
        unsafe extern "system" fn GetTransportAddress<Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransportAddressEx<Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransportAddressEx(::core::mem::transmute_copy(&fsafe)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportAddress<Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransportAddress(::core::mem::transmute_copy(&pszaddress)).into()
        }
        Self {
            base: IWSDAddress_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPort: GetPort::<Impl, IMPL_OFFSET>,
            SetPort: SetPort::<Impl, IMPL_OFFSET>,
            GetTransportAddress: GetTransportAddress::<Impl, IMPL_OFFSET>,
            GetTransportAddressEx: GetTransportAddressEx::<Impl, IMPL_OFFSET>,
            SetTransportAddress: SetTransportAddress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDTransportAddress as ::windows::core::Interface>::IID || iid == &<IWSDAddress as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub trait IWSDUdpAddress_Impl: Sized + IWSDAddress_Impl + IWSDTransportAddress_Impl {
    fn SetSockaddr(&mut self, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::Result<()>;
    fn GetSockaddr(&mut self) -> ::windows::core::Result<super::super::Networking::WinSock::SOCKADDR_STORAGE>;
    fn SetExclusive(&mut self, fexclusive: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetExclusive(&mut self) -> ::windows::core::Result<()>;
    fn SetMessageType(&mut self, messagetype: WSDUdpMessageType) -> ::windows::core::Result<()>;
    fn GetMessageType(&mut self) -> ::windows::core::Result<WSDUdpMessageType>;
    fn SetTTL(&mut self, dwttl: u32) -> ::windows::core::Result<()>;
    fn GetTTL(&mut self) -> ::windows::core::Result<u32>;
    fn SetAlias(&mut self, palias: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetAlias(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl IWSDUdpAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDUdpAddress_Vtbl {
        unsafe extern "system" fn SetSockaddr<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSockaddr(::core::mem::transmute_copy(&psockaddr)).into()
        }
        unsafe extern "system" fn GetSockaddr<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSockaddr() {
                ::core::result::Result::Ok(ok__) => {
                    *psockaddr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExclusive<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fexclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExclusive(::core::mem::transmute_copy(&fexclusive)).into()
        }
        unsafe extern "system" fn GetExclusive<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetExclusive().into()
        }
        unsafe extern "system" fn SetMessageType<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: WSDUdpMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMessageType(::core::mem::transmute_copy(&messagetype)).into()
        }
        unsafe extern "system" fn GetMessageType<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageType() {
                ::core::result::Result::Ok(ok__) => {
                    *pmessagetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTTL<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwttl: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTTL(::core::mem::transmute_copy(&dwttl)).into()
        }
        unsafe extern "system" fn GetTTL<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwttl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlias<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palias: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAlias(::core::mem::transmute_copy(&palias)).into()
        }
        unsafe extern "system" fn GetAlias<Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palias: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlias() {
                ::core::result::Result::Ok(ok__) => {
                    *palias = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSDTransportAddress_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSockaddr: SetSockaddr::<Impl, IMPL_OFFSET>,
            GetSockaddr: GetSockaddr::<Impl, IMPL_OFFSET>,
            SetExclusive: SetExclusive::<Impl, IMPL_OFFSET>,
            GetExclusive: GetExclusive::<Impl, IMPL_OFFSET>,
            SetMessageType: SetMessageType::<Impl, IMPL_OFFSET>,
            GetMessageType: GetMessageType::<Impl, IMPL_OFFSET>,
            SetTTL: SetTTL::<Impl, IMPL_OFFSET>,
            GetTTL: GetTTL::<Impl, IMPL_OFFSET>,
            SetAlias: SetAlias::<Impl, IMPL_OFFSET>,
            GetAlias: GetAlias::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDUdpAddress as ::windows::core::Interface>::IID || iid == &<IWSDAddress as ::windows::core::Interface>::IID || iid == &<IWSDTransportAddress as ::windows::core::Interface>::IID
    }
}
pub trait IWSDUdpMessageParameters_Impl: Sized + IWSDMessageParameters_Impl {
    fn SetRetransmitParams(&mut self, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::Result<()>;
    fn GetRetransmitParams(&mut self) -> ::windows::core::Result<WSDUdpRetransmitParams>;
}
impl IWSDUdpMessageParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpMessageParameters_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDUdpMessageParameters_Vtbl {
        unsafe extern "system" fn SetRetransmitParams<Impl: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRetransmitParams(::core::mem::transmute_copy(&pparams)).into()
        }
        unsafe extern "system" fn GetRetransmitParams<Impl: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetransmitParams() {
                ::core::result::Result::Ok(ok__) => {
                    *pparams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSDMessageParameters_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRetransmitParams: SetRetransmitParams::<Impl, IMPL_OFFSET>,
            GetRetransmitParams: GetRetransmitParams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDUdpMessageParameters as ::windows::core::Interface>::IID || iid == &<IWSDMessageParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDXMLContext_Impl: Sized {
    fn AddNamespace(&mut self, pszuri: super::super::Foundation::PWSTR, pszsuggestedprefix: super::super::Foundation::PWSTR) -> ::windows::core::Result<*mut WSDXML_NAMESPACE>;
    fn AddNameToNamespace(&mut self, pszuri: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR) -> ::windows::core::Result<*mut WSDXML_NAME>;
    fn SetNamespaces(&mut self, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::core::Result<()>;
    fn SetTypes(&mut self, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDXMLContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDXMLContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDXMLContext_Vtbl {
        unsafe extern "system" fn AddNamespace<Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuri: super::super::Foundation::PWSTR, pszsuggestedprefix: super::super::Foundation::PWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNamespace(::core::mem::transmute_copy(&pszuri), ::core::mem::transmute_copy(&pszsuggestedprefix)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddNameToNamespace<Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuri: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNameToNamespace(::core::mem::transmute_copy(&pszuri), ::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaces<Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetNamespaces(::core::mem::transmute_copy(&pnamespaces), ::core::mem::transmute_copy(&wnamespacescount), ::core::mem::transmute_copy(&blayernumber)).into()
        }
        unsafe extern "system" fn SetTypes<Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypes(::core::mem::transmute_copy(&ptypes), ::core::mem::transmute_copy(&dwtypescount), ::core::mem::transmute_copy(&blayernumber)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddNamespace: AddNamespace::<Impl, IMPL_OFFSET>,
            AddNameToNamespace: AddNameToNamespace::<Impl, IMPL_OFFSET>,
            SetNamespaces: SetNamespaces::<Impl, IMPL_OFFSET>,
            SetTypes: SetTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDXMLContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDiscoveredService_Impl: Sized {
    fn GetEndpointReference(&mut self) -> ::windows::core::Result<*mut WSD_ENDPOINT_REFERENCE>;
    fn GetTypes(&mut self) -> ::windows::core::Result<*mut WSD_NAME_LIST>;
    fn GetScopes(&mut self) -> ::windows::core::Result<*mut WSD_URI_LIST>;
    fn GetXAddrs(&mut self) -> ::windows::core::Result<*mut WSD_URI_LIST>;
    fn GetMetadataVersion(&mut self) -> ::windows::core::Result<u64>;
    fn GetExtendedDiscoXML(&mut self, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn GetProbeResolveTag(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetRemoteTransportAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetLocalTransportAddress(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetLocalInterfaceGUID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetInstanceId(&mut self) -> ::windows::core::Result<u64>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDiscoveredService_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDiscoveredService_Vtbl {
        unsafe extern "system" fn GetEndpointReference<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointReference() {
                ::core::result::Result::Ok(ok__) => {
                    *ppendpointreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypes<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pptypeslist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopes<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScopes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppscopeslist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXAddrs<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXAddrs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppxaddrslist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataVersion<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullmetadataversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pullmetadataversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendedDiscoXML<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetExtendedDiscoXML(::core::mem::transmute_copy(&ppheaderany), ::core::mem::transmute_copy(&ppbodyany)).into()
        }
        unsafe extern "system" fn GetProbeResolveTag<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztag: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProbeResolveTag() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteTransportAddress<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszremotetransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteTransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszremotetransportaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalTransportAddress<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlocaltransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalTransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszlocaltransportaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalInterfaceGUID<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalInterfaceGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullinstanceid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pullinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetEndpointReference: GetEndpointReference::<Impl, IMPL_OFFSET>,
            GetTypes: GetTypes::<Impl, IMPL_OFFSET>,
            GetScopes: GetScopes::<Impl, IMPL_OFFSET>,
            GetXAddrs: GetXAddrs::<Impl, IMPL_OFFSET>,
            GetMetadataVersion: GetMetadataVersion::<Impl, IMPL_OFFSET>,
            GetExtendedDiscoXML: GetExtendedDiscoXML::<Impl, IMPL_OFFSET>,
            GetProbeResolveTag: GetProbeResolveTag::<Impl, IMPL_OFFSET>,
            GetRemoteTransportAddress: GetRemoteTransportAddress::<Impl, IMPL_OFFSET>,
            GetLocalTransportAddress: GetLocalTransportAddress::<Impl, IMPL_OFFSET>,
            GetLocalInterfaceGUID: GetLocalInterfaceGUID::<Impl, IMPL_OFFSET>,
            GetInstanceId: GetInstanceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDiscoveredService as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDiscoveryProvider_Impl: Sized {
    fn SetAddressFamily(&mut self, dwaddressfamily: u32) -> ::windows::core::Result<()>;
    fn Attach(&mut self, psink: &::core::option::Option<IWSDiscoveryProviderNotify>) -> ::windows::core::Result<()>;
    fn Detach(&mut self) -> ::windows::core::Result<()>;
    fn SearchById(&mut self, pszid: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SearchByAddress(&mut self, pszaddress: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SearchByType(&mut self, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetXMLContext(&mut self) -> ::windows::core::Result<IWSDXMLContext>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDiscoveryProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDiscoveryProvider_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddressFamily(::core::mem::transmute_copy(&dwaddressfamily)).into()
        }
        unsafe extern "system" fn Attach<Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Attach(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn Detach<Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Detach().into()
        }
        unsafe extern "system" fn SearchById<Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SearchById(::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&psztag)).into()
        }
        unsafe extern "system" fn SearchByAddress<Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaddress: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SearchByAddress(::core::mem::transmute_copy(&pszaddress), ::core::mem::transmute_copy(&psztag)).into()
        }
        unsafe extern "system" fn SearchByType<Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SearchByType(::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pszmatchby), ::core::mem::transmute_copy(&psztag)).into()
        }
        unsafe extern "system" fn GetXMLContext<Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Impl, IMPL_OFFSET>,
            Attach: Attach::<Impl, IMPL_OFFSET>,
            Detach: Detach::<Impl, IMPL_OFFSET>,
            SearchById: SearchById::<Impl, IMPL_OFFSET>,
            SearchByAddress: SearchByAddress::<Impl, IMPL_OFFSET>,
            SearchByType: SearchByType::<Impl, IMPL_OFFSET>,
            GetXMLContext: GetXMLContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDiscoveryProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDiscoveryProviderNotify_Impl: Sized {
    fn Add(&mut self, pservice: &::core::option::Option<IWSDiscoveredService>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, pservice: &::core::option::Option<IWSDiscoveredService>) -> ::windows::core::Result<()>;
    fn SearchFailed(&mut self, hr: ::windows::core::HRESULT, psztag: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SearchComplete(&mut self, psztag: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDiscoveryProviderNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProviderNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDiscoveryProviderNotify_Vtbl {
        unsafe extern "system" fn Add<Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pservice)).into()
        }
        unsafe extern "system" fn Remove<Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute(&pservice)).into()
        }
        unsafe extern "system" fn SearchFailed<Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SearchFailed(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&psztag)).into()
        }
        unsafe extern "system" fn SearchComplete<Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SearchComplete(::core::mem::transmute_copy(&psztag)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            SearchFailed: SearchFailed::<Impl, IMPL_OFFSET>,
            SearchComplete: SearchComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDiscoveryProviderNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDiscoveryPublisher_Impl: Sized {
    fn SetAddressFamily(&mut self, dwaddressfamily: u32) -> ::windows::core::Result<()>;
    fn RegisterNotificationSink(&mut self, psink: &::core::option::Option<IWSDiscoveryPublisherNotify>) -> ::windows::core::Result<()>;
    fn UnRegisterNotificationSink(&mut self, psink: &::core::option::Option<IWSDiscoveryPublisherNotify>) -> ::windows::core::Result<()>;
    fn Publish(&mut self, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::Result<()>;
    fn UnPublish(&mut self, pszid: super::super::Foundation::PWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn MatchProbe(&mut self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: &::core::option::Option<IWSDMessageParameters>, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::Result<()>;
    fn MatchResolve(&mut self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: &::core::option::Option<IWSDMessageParameters>, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::Result<()>;
    fn PublishEx(&mut self, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn MatchProbeEx(&mut self, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: &::core::option::Option<IWSDMessageParameters>, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn MatchResolveEx(&mut self, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: &::core::option::Option<IWSDMessageParameters>, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::Result<()>;
    fn RegisterScopeMatchingRule(&mut self, pscopematchingrule: &::core::option::Option<IWSDScopeMatchingRule>) -> ::windows::core::Result<()>;
    fn UnRegisterScopeMatchingRule(&mut self, pscopematchingrule: &::core::option::Option<IWSDScopeMatchingRule>) -> ::windows::core::Result<()>;
    fn GetXMLContext(&mut self) -> ::windows::core::Result<IWSDXMLContext>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDiscoveryPublisher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDiscoveryPublisher_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAddressFamily(::core::mem::transmute_copy(&dwaddressfamily)).into()
        }
        unsafe extern "system" fn RegisterNotificationSink<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterNotificationSink(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn UnRegisterNotificationSink<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnRegisterNotificationSink(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn Publish<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Publish(::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn UnPublish<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnPublish(::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pany)).into()
        }
        unsafe extern "system" fn MatchProbe<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MatchProbe(::core::mem::transmute_copy(&pprobemessage), ::core::mem::transmute(&pmessageparameters), ::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn MatchResolve<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MatchResolve(::core::mem::transmute_copy(&presolvemessage), ::core::mem::transmute(&pmessageparameters), ::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn PublishEx<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .PublishEx(
                    ::core::mem::transmute_copy(&pszid),
                    ::core::mem::transmute_copy(&ullmetadataversion),
                    ::core::mem::transmute_copy(&ullinstanceid),
                    ::core::mem::transmute_copy(&ullmessagenumber),
                    ::core::mem::transmute_copy(&pszsessionid),
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
        unsafe extern "system" fn MatchProbeEx<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .MatchProbeEx(
                    ::core::mem::transmute_copy(&pprobemessage),
                    ::core::mem::transmute(&pmessageparameters),
                    ::core::mem::transmute_copy(&pszid),
                    ::core::mem::transmute_copy(&ullmetadataversion),
                    ::core::mem::transmute_copy(&ullinstanceid),
                    ::core::mem::transmute_copy(&ullmessagenumber),
                    ::core::mem::transmute_copy(&pszsessionid),
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
        unsafe extern "system" fn MatchResolveEx<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .MatchResolveEx(
                    ::core::mem::transmute_copy(&presolvemessage),
                    ::core::mem::transmute(&pmessageparameters),
                    ::core::mem::transmute_copy(&pszid),
                    ::core::mem::transmute_copy(&ullmetadataversion),
                    ::core::mem::transmute_copy(&ullinstanceid),
                    ::core::mem::transmute_copy(&ullmessagenumber),
                    ::core::mem::transmute_copy(&pszsessionid),
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
        unsafe extern "system" fn RegisterScopeMatchingRule<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopematchingrule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterScopeMatchingRule(::core::mem::transmute(&pscopematchingrule)).into()
        }
        unsafe extern "system" fn UnRegisterScopeMatchingRule<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopematchingrule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnRegisterScopeMatchingRule(::core::mem::transmute(&pscopematchingrule)).into()
        }
        unsafe extern "system" fn GetXMLContext<Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Impl, IMPL_OFFSET>,
            RegisterNotificationSink: RegisterNotificationSink::<Impl, IMPL_OFFSET>,
            UnRegisterNotificationSink: UnRegisterNotificationSink::<Impl, IMPL_OFFSET>,
            Publish: Publish::<Impl, IMPL_OFFSET>,
            UnPublish: UnPublish::<Impl, IMPL_OFFSET>,
            MatchProbe: MatchProbe::<Impl, IMPL_OFFSET>,
            MatchResolve: MatchResolve::<Impl, IMPL_OFFSET>,
            PublishEx: PublishEx::<Impl, IMPL_OFFSET>,
            MatchProbeEx: MatchProbeEx::<Impl, IMPL_OFFSET>,
            MatchResolveEx: MatchResolveEx::<Impl, IMPL_OFFSET>,
            RegisterScopeMatchingRule: RegisterScopeMatchingRule::<Impl, IMPL_OFFSET>,
            UnRegisterScopeMatchingRule: UnRegisterScopeMatchingRule::<Impl, IMPL_OFFSET>,
            GetXMLContext: GetXMLContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisher as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IWSDiscoveryPublisherNotify_Impl: Sized {
    fn ProbeHandler(&mut self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: &::core::option::Option<IWSDMessageParameters>) -> ::windows::core::Result<()>;
    fn ResolveHandler(&mut self, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: &::core::option::Option<IWSDMessageParameters>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDiscoveryPublisherNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisherNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSDiscoveryPublisherNotify_Vtbl {
        unsafe extern "system" fn ProbeHandler<Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProbeHandler(::core::mem::transmute_copy(&psoap), ::core::mem::transmute(&pmessageparameters)).into()
        }
        unsafe extern "system" fn ResolveHandler<Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResolveHandler(::core::mem::transmute_copy(&psoap), ::core::mem::transmute(&pmessageparameters)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ProbeHandler: ProbeHandler::<Impl, IMPL_OFFSET>,
            ResolveHandler: ResolveHandler::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisherNotify as ::windows::core::Interface>::IID
    }
}
