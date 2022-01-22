#[cfg(feature = "Win32_Foundation")]
pub trait IWSDAddress_Impl: Sized {
    fn Serialize(&mut self, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Deserialize(&mut self, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IWSDAddress_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAddress_Impl, const OFFSET: isize>() -> IWSDAddress_Vtbl {
        unsafe extern "system" fn Serialize<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Serialize(::core::mem::transmute_copy(&pszbuffer), ::core::mem::transmute_copy(&cchlength), ::core::mem::transmute_copy(&fsafe)).into()
        }
        unsafe extern "system" fn Deserialize<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Deserialize(::core::mem::transmute_copy(&pszbuffer)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncCallback_Impl, const OFFSET: isize>() -> IWSDAsyncCallback_Vtbl {
        unsafe extern "system" fn AsyncOperationComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AsyncOperationComplete(::core::mem::transmute(&pasyncresult), ::core::mem::transmute(&pasyncstate)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), AsyncOperationComplete: AsyncOperationComplete::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>() -> IWSDAsyncResult_Vtbl {
        unsafe extern "system" fn SetCallback<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCallback(::core::mem::transmute(&pcallback), ::core::mem::transmute(&pasyncstate)).into()
        }
        unsafe extern "system" fn SetWaitHandle<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwaithandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetWaitHandle(::core::mem::transmute_copy(&hwaithandle)).into()
        }
        unsafe extern "system" fn HasCompleted<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).HasCompleted().into()
        }
        unsafe extern "system" fn GetAsyncState<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAsyncState() {
                ::core::result::Result::Ok(ok__) => {
                    *ppasyncstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abort().into()
        }
        unsafe extern "system" fn GetEvent<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut WSD_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEvent() {
                ::core::result::Result::Ok(ok__) => {
                    *pevent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppendpoint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEndpointProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppendpoint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetCallback: SetCallback::<Identity, Impl, OFFSET>,
            SetWaitHandle: SetWaitHandle::<Identity, Impl, OFFSET>,
            HasCompleted: HasCompleted::<Identity, Impl, OFFSET>,
            GetAsyncState: GetAsyncState::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            GetEvent: GetEvent::<Identity, Impl, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDAsyncResult as ::windows::core::Interface>::IID
    }
}
pub trait IWSDAttachment_Impl: Sized {}
impl IWSDAttachment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAttachment_Impl, const OFFSET: isize>() -> IWSDAttachment_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>() -> IWSDDeviceHost_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, pphostaddresses: *const ::windows::core::RawPtr, dwhostaddresscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pszlocalid), ::core::mem::transmute(&pcontext), ::core::mem::transmute_copy(&pphostaddresses), ::core::mem::transmute_copy(&dwhostaddresscount)).into()
        }
        unsafe extern "system" fn Start<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Start(::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&pscopelist), ::core::mem::transmute(&pnotificationsink)).into()
        }
        unsafe extern "system" fn Stop<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Stop().into()
        }
        unsafe extern "system" fn Terminate<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Terminate().into()
        }
        unsafe extern "system" fn RegisterPortType<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterPortType(::core::mem::transmute_copy(&pporttype)).into()
        }
        unsafe extern "system" fn SetMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMetadata(::core::mem::transmute_copy(&pthismodelmetadata), ::core::mem::transmute_copy(&pthisdevicemetadata), ::core::mem::transmute_copy(&phostmetadata), ::core::mem::transmute_copy(&pcustommetadata)).into()
        }
        unsafe extern "system" fn RegisterService<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterService(::core::mem::transmute_copy(&pszserviceid), ::core::mem::transmute(&pservice)).into()
        }
        unsafe extern "system" fn RetireService<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RetireService(::core::mem::transmute_copy(&pszserviceid)).into()
        }
        unsafe extern "system" fn AddDynamicService<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, pszendpointaddress: super::super::Foundation::PWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDynamicService(::core::mem::transmute_copy(&pszserviceid), ::core::mem::transmute_copy(&pszendpointaddress), ::core::mem::transmute_copy(&pporttype), ::core::mem::transmute_copy(&pportname), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pservice)).into()
        }
        unsafe extern "system" fn RemoveDynamicService<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveDynamicService(::core::mem::transmute_copy(&pszserviceid)).into()
        }
        unsafe extern "system" fn SetServiceDiscoverable<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServiceDiscoverable(::core::mem::transmute_copy(&pszserviceid), ::core::mem::transmute_copy(&fdiscoverable)).into()
        }
        unsafe extern "system" fn SignalEvent<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SignalEvent(::core::mem::transmute_copy(&pszserviceid), ::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHostNotify_Impl, const OFFSET: isize>() -> IWSDDeviceHostNotify_Vtbl {
        unsafe extern "system" fn GetService<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHostNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetService(::core::mem::transmute_copy(&pszserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppservice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>() -> IWSDDeviceProxy_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: ::windows::core::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, psponsor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pszdeviceid), ::core::mem::transmute(&pdeviceaddress), ::core::mem::transmute_copy(&pszlocalid), ::core::mem::transmute(&pcontext), ::core::mem::transmute(&psponsor)).into()
        }
        unsafe extern "system" fn BeginGetMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginGetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndGetMetadata(::core::mem::transmute(&presult)).into()
        }
        unsafe extern "system" fn GetHostMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetHostMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *pphostmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisModelMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetThisModelMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmanufacturermetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisDeviceMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetThisDeviceMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppthisdevicemetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAllMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyById<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, ppserviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetServiceProxyById(::core::mem::transmute_copy(&pszserviceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppserviceproxy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyByType<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetServiceProxyByType(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppserviceproxy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEndpointProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproxy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>() -> IWSDEndpointProxy_Vtbl {
        unsafe extern "system" fn SendOneWayRequest<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendOneWayRequest(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation)).into()
        }
        unsafe extern "system" fn SendTwoWayRequest<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendTwoWayRequest(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::core::mem::transmute_copy(&presponsecontext)).into()
        }
        unsafe extern "system" fn SendTwoWayRequestAsync<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, presult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SendTwoWayRequestAsync(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pcallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *presult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAsyncOperation<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AbortAsyncOperation(::core::mem::transmute(&pasyncresult)).into()
        }
        unsafe extern "system" fn ProcessFault<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProcessFault(::core::mem::transmute_copy(&pfault)).into()
        }
        unsafe extern "system" fn GetErrorInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszerrorinfo: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszerrorinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaultInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetFaultInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppfault = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SendOneWayRequest: SendOneWayRequest::<Identity, Impl, OFFSET>,
            SendTwoWayRequest: SendTwoWayRequest::<Identity, Impl, OFFSET>,
            SendTwoWayRequestAsync: SendTwoWayRequestAsync::<Identity, Impl, OFFSET>,
            AbortAsyncOperation: AbortAsyncOperation::<Identity, Impl, OFFSET>,
            ProcessFault: ProcessFault::<Identity, Impl, OFFSET>,
            GetErrorInfo: GetErrorInfo::<Identity, Impl, OFFSET>,
            GetFaultInfo: GetFaultInfo::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEventingStatus_Impl, const OFFSET: isize>() -> IWSDEventingStatus_Vtbl {
        unsafe extern "system" fn SubscriptionRenewed<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SubscriptionRenewed(::core::mem::transmute_copy(&pszsubscriptionaction))
        }
        unsafe extern "system" fn SubscriptionRenewalFailed<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: super::super::Foundation::PWSTR, hr: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SubscriptionRenewalFailed(::core::mem::transmute_copy(&pszsubscriptionaction), ::core::mem::transmute_copy(&hr))
        }
        unsafe extern "system" fn SubscriptionEnded<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEventingStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SubscriptionEnded(::core::mem::transmute_copy(&pszsubscriptionaction))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SubscriptionRenewed: SubscriptionRenewed::<Identity, Impl, OFFSET>,
            SubscriptionRenewalFailed: SubscriptionRenewalFailed::<Identity, Impl, OFFSET>,
            SubscriptionEnded: SubscriptionEnded::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>() -> IWSDHttpAddress_Vtbl {
        unsafe extern "system" fn GetSecure<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSecure().into()
        }
        unsafe extern "system" fn SetSecure<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSecure(::core::mem::transmute_copy(&fsecure)).into()
        }
        unsafe extern "system" fn GetPath<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&pszpath)).into()
        }
        Self {
            base: IWSDTransportAddress_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetSecure: GetSecure::<Identity, Impl, OFFSET>,
            SetSecure: SetSecure::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAuthParameters_Impl, const OFFSET: isize>() -> IWSDHttpAuthParameters_Vtbl {
        unsafe extern "system" fn GetClientAccessToken<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClientAccessToken() {
                ::core::result::Result::Ok(ok__) => {
                    *phtoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthType<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAuthParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAuthType() {
                ::core::result::Result::Ok(ok__) => {
                    *pauthtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClientAccessToken: GetClientAccessToken::<Identity, Impl, OFFSET>,
            GetAuthType: GetAuthType::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>() -> IWSDHttpMessageParameters_Vtbl {
        unsafe extern "system" fn SetInboundHttpHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInboundHttpHeaders(::core::mem::transmute_copy(&pszheaders)).into()
        }
        unsafe extern "system" fn GetInboundHttpHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInboundHttpHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundHttpHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOutboundHttpHeaders(::core::mem::transmute_copy(&pszheaders)).into()
        }
        unsafe extern "system" fn GetOutboundHttpHeaders<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOutboundHttpHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetID<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetID(::core::mem::transmute_copy(&pszid)).into()
        }
        unsafe extern "system" fn GetID<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetContext(::core::mem::transmute(&pcontext)).into()
        }
        unsafe extern "system" fn GetContext<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Clear().into()
        }
        Self {
            base: IWSDMessageParameters_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDHttpMessageParameters as ::windows::core::Interface>::IID || iid == &<IWSDMessageParameters as ::windows::core::Interface>::IID
    }
}
pub trait IWSDInboundAttachment_Impl: Sized + IWSDAttachment_Impl {
    fn Read(&mut self, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl IWSDInboundAttachment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDInboundAttachment_Impl, const OFFSET: isize>() -> IWSDInboundAttachment_Vtbl {
        unsafe extern "system" fn Read<Identity: ::windows::core::IUnknownImpl, Impl: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Read(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbytestoread), ::core::mem::transmute_copy(&pdwnumberofbytesread)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWSDInboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        Self { base: IWSDAttachment_Vtbl::new::<Identity, Impl, OFFSET>(), Read: Read::<Identity, Impl, OFFSET>, Close: Close::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>() -> IWSDMessageParameters_Vtbl {
        unsafe extern "system" fn GetLocalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLocalAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLocalAddress(::core::mem::transmute(&paddress)).into()
        }
        unsafe extern "system" fn GetRemoteAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRemoteAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRemoteAddress(::core::mem::transmute(&paddress)).into()
        }
        unsafe extern "system" fn GetLowerParameters<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptxparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLowerParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *pptxparams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetLocalAddress: GetLocalAddress::<Identity, Impl, OFFSET>,
            SetLocalAddress: SetLocalAddress::<Identity, Impl, OFFSET>,
            GetRemoteAddress: GetRemoteAddress::<Identity, Impl, OFFSET>,
            SetRemoteAddress: SetRemoteAddress::<Identity, Impl, OFFSET>,
            GetLowerParameters: GetLowerParameters::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMetadataExchange_Impl, const OFFSET: isize>() -> IWSDMetadataExchange_Vtbl {
        unsafe extern "system" fn GetMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMetadataExchange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *metadataout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetMetadata: GetMetadata::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>() -> IWSDOutboundAttachment_Vtbl {
        unsafe extern "system" fn Write<Identity: ::windows::core::IUnknownImpl, Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Write(::core::mem::transmute_copy(&pbuffer), ::core::mem::transmute_copy(&dwbytestowrite)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwnumberofbyteswritten = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl, Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl, Impl: IWSDOutboundAttachment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Abort().into()
        }
        Self {
            base: IWSDAttachment_Vtbl::new::<Identity, Impl, OFFSET>(),
            Write: Write::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSSLClientCertificate_Impl, const OFFSET: isize>() -> IWSDSSLClientCertificate_Vtbl {
        unsafe extern "system" fn GetClientCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcertcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMappedAccessToken<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSSLClientCertificate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMappedAccessToken() {
                ::core::result::Result::Ok(ok__) => {
                    *phtoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClientCertificate: GetClientCertificate::<Identity, Impl, OFFSET>,
            GetMappedAccessToken: GetMappedAccessToken::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDScopeMatchingRule_Impl, const OFFSET: isize>() -> IWSDScopeMatchingRule_Vtbl {
        unsafe extern "system" fn GetScopeRule<Identity: ::windows::core::IUnknownImpl, Impl: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszscopematchingrule: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetScopeRule() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszscopematchingrule = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchScopes<Identity: ::windows::core::IUnknownImpl, Impl: IWSDScopeMatchingRule_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszscope1: super::super::Foundation::PWSTR, pszscope2: super::super::Foundation::PWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MatchScopes(::core::mem::transmute_copy(&pszscope1), ::core::mem::transmute_copy(&pszscope2)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfmatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetScopeRule: GetScopeRule::<Identity, Impl, OFFSET>,
            MatchScopes: MatchScopes::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceMessaging_Impl, const OFFSET: isize>() -> IWSDServiceMessaging_Vtbl {
        unsafe extern "system" fn SendResponse<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendResponse(::core::mem::transmute_copy(&pbody), ::core::mem::transmute_copy(&poperation), ::core::mem::transmute(&pmessageparameters)).into()
        }
        unsafe extern "system" fn FaultRequest<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceMessaging_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: ::windows::core::RawPtr, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FaultRequest(::core::mem::transmute_copy(&prequestheader), ::core::mem::transmute(&pmessageparameters), ::core::mem::transmute_copy(&pfault)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SendResponse: SendResponse::<Identity, Impl, OFFSET>,
            FaultRequest: FaultRequest::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>() -> IWSDServiceProxy_Vtbl {
        unsafe extern "system" fn BeginGetMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginGetMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EndGetMetadata(::core::mem::transmute(&presult)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppmetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceMetadata<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetServiceMetadata() {
                ::core::result::Result::Ok(ok__) => {
                    *ppservicemetadata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscribeToOperation<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubscribeToOperation(::core::mem::transmute_copy(&poperation), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&pany)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppany = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnsubscribeToOperation<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnsubscribeToOperation(::core::mem::transmute_copy(&poperation)).into()
        }
        unsafe extern "system" fn SetEventingStatusCallback<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventingStatusCallback(::core::mem::transmute(&pstatus)).into()
        }
        unsafe extern "system" fn GetEndpointProxy<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEndpointProxy() {
                ::core::result::Result::Ok(ok__) => {
                    *ppproxy = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSDMetadataExchange_Vtbl::new::<Identity, Impl, OFFSET>(),
            BeginGetMetadata: BeginGetMetadata::<Identity, Impl, OFFSET>,
            EndGetMetadata: EndGetMetadata::<Identity, Impl, OFFSET>,
            GetServiceMetadata: GetServiceMetadata::<Identity, Impl, OFFSET>,
            SubscribeToOperation: SubscribeToOperation::<Identity, Impl, OFFSET>,
            UnsubscribeToOperation: UnsubscribeToOperation::<Identity, Impl, OFFSET>,
            SetEventingStatusCallback: SetEventingStatusCallback::<Identity, Impl, OFFSET>,
            GetEndpointProxy: GetEndpointProxy::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>() -> IWSDServiceProxyEventing_Vtbl {
        unsafe extern "system" fn SubscribeToMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginSubscribeToMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginSubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&punknown), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndSubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn UnsubscribeToMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnsubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany)).into()
        }
        unsafe extern "system" fn BeginUnsubscribeToMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginUnsubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnsubscribeToMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndUnsubscribeToMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&presult)).into()
        }
        unsafe extern "system" fn RenewMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RenewMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginRenewMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginRenewMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pexpires), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRenewMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndRenewMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn GetStatusForMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetStatusForMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        unsafe extern "system" fn BeginGetStatusForMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BeginGetStatusForMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute_copy(&pany), ::core::mem::transmute(&pasyncstate), ::core::mem::transmute(&pasynccallback)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetStatusForMultipleOperations<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndGetStatusForMultipleOperations(::core::mem::transmute_copy(&poperations), ::core::mem::transmute_copy(&dwoperationcount), ::core::mem::transmute(&presult), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)).into()
        }
        Self {
            base: IWSDServiceProxy_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>() -> IWSDSignatureProperty_Vtbl {
        unsafe extern "system" fn IsMessageSigned<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMessageSigned() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsigned = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMessageSignatureTrusted<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsMessageSignatureTrusted() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsignaturetrusted = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyInfo<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetKeyInfo(::core::mem::transmute_copy(&pbkeyinfo), ::core::mem::transmute_copy(&pdwkeyinfosize)).into()
        }
        unsafe extern "system" fn GetSignature<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSignature(::core::mem::transmute_copy(&pbsignature), ::core::mem::transmute_copy(&pdwsignaturesize)).into()
        }
        unsafe extern "system" fn GetSignedInfoHash<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSignatureProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSignedInfoHash(::core::mem::transmute_copy(&pbsignedinfohash), ::core::mem::transmute_copy(&pdwhashsize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsMessageSigned: IsMessageSigned::<Identity, Impl, OFFSET>,
            IsMessageSignatureTrusted: IsMessageSignatureTrusted::<Identity, Impl, OFFSET>,
            GetKeyInfo: GetKeyInfo::<Identity, Impl, OFFSET>,
            GetSignature: GetSignature::<Identity, Impl, OFFSET>,
            GetSignedInfoHash: GetSignedInfoHash::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>() -> IWSDTransportAddress_Vtbl {
        unsafe extern "system" fn GetPort<Identity: ::windows::core::IUnknownImpl, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwport: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPort() {
                ::core::result::Result::Ok(ok__) => {
                    *pwport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Identity: ::windows::core::IUnknownImpl, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wport: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPort(::core::mem::transmute_copy(&wport)).into()
        }
        unsafe extern "system" fn GetTransportAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransportAddressEx<Identity: ::windows::core::IUnknownImpl, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTransportAddressEx(::core::mem::transmute_copy(&fsafe)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWSDTransportAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTransportAddress(::core::mem::transmute_copy(&pszaddress)).into()
        }
        Self {
            base: IWSDAddress_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPort: GetPort::<Identity, Impl, OFFSET>,
            SetPort: SetPort::<Identity, Impl, OFFSET>,
            GetTransportAddress: GetTransportAddress::<Identity, Impl, OFFSET>,
            GetTransportAddressEx: GetTransportAddressEx::<Identity, Impl, OFFSET>,
            SetTransportAddress: SetTransportAddress::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>() -> IWSDUdpAddress_Vtbl {
        unsafe extern "system" fn SetSockaddr<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSockaddr(::core::mem::transmute_copy(&psockaddr)).into()
        }
        unsafe extern "system" fn GetSockaddr<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSockaddr() {
                ::core::result::Result::Ok(ok__) => {
                    *psockaddr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExclusive<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fexclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetExclusive(::core::mem::transmute_copy(&fexclusive)).into()
        }
        unsafe extern "system" fn GetExclusive<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetExclusive().into()
        }
        unsafe extern "system" fn SetMessageType<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: WSDUdpMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMessageType(::core::mem::transmute_copy(&messagetype)).into()
        }
        unsafe extern "system" fn GetMessageType<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMessageType() {
                ::core::result::Result::Ok(ok__) => {
                    *pmessagetype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTTL<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwttl: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTTL(::core::mem::transmute_copy(&dwttl)).into()
        }
        unsafe extern "system" fn GetTTL<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwttl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTTL() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwttl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlias<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palias: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAlias(::core::mem::transmute_copy(&palias)).into()
        }
        unsafe extern "system" fn GetAlias<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddress_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palias: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAlias() {
                ::core::result::Result::Ok(ok__) => {
                    *palias = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSDTransportAddress_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDUdpAddress as ::windows::core::Interface>::IID || iid == &<IWSDAddress as ::windows::core::Interface>::IID || iid == &<IWSDTransportAddress as ::windows::core::Interface>::IID
    }
}
pub trait IWSDUdpMessageParameters_Impl: Sized + IWSDMessageParameters_Impl {
    fn SetRetransmitParams(&mut self, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::Result<()>;
    fn GetRetransmitParams(&mut self) -> ::windows::core::Result<WSDUdpRetransmitParams>;
}
impl IWSDUdpMessageParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpMessageParameters_Impl, const OFFSET: isize>() -> IWSDUdpMessageParameters_Vtbl {
        unsafe extern "system" fn SetRetransmitParams<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRetransmitParams(::core::mem::transmute_copy(&pparams)).into()
        }
        unsafe extern "system" fn GetRetransmitParams<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpMessageParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRetransmitParams() {
                ::core::result::Result::Ok(ok__) => {
                    *pparams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSDMessageParameters_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRetransmitParams: SetRetransmitParams::<Identity, Impl, OFFSET>,
            GetRetransmitParams: GetRetransmitParams::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDXMLContext_Impl, const OFFSET: isize>() -> IWSDXMLContext_Vtbl {
        unsafe extern "system" fn AddNamespace<Identity: ::windows::core::IUnknownImpl, Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuri: super::super::Foundation::PWSTR, pszsuggestedprefix: super::super::Foundation::PWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddNamespace(::core::mem::transmute_copy(&pszuri), ::core::mem::transmute_copy(&pszsuggestedprefix)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnamespace = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddNameToNamespace<Identity: ::windows::core::IUnknownImpl, Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuri: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddNameToNamespace(::core::mem::transmute_copy(&pszuri), ::core::mem::transmute_copy(&pszname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaces<Identity: ::windows::core::IUnknownImpl, Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetNamespaces(::core::mem::transmute_copy(&pnamespaces), ::core::mem::transmute_copy(&wnamespacescount), ::core::mem::transmute_copy(&blayernumber)).into()
        }
        unsafe extern "system" fn SetTypes<Identity: ::windows::core::IUnknownImpl, Impl: IWSDXMLContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTypes(::core::mem::transmute_copy(&ptypes), ::core::mem::transmute_copy(&dwtypescount), ::core::mem::transmute_copy(&blayernumber)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddNamespace: AddNamespace::<Identity, Impl, OFFSET>,
            AddNameToNamespace: AddNameToNamespace::<Identity, Impl, OFFSET>,
            SetNamespaces: SetNamespaces::<Identity, Impl, OFFSET>,
            SetTypes: SetTypes::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>() -> IWSDiscoveredService_Vtbl {
        unsafe extern "system" fn GetEndpointReference<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetEndpointReference() {
                ::core::result::Result::Ok(ok__) => {
                    *ppendpointreference = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypes<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *pptypeslist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopes<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetScopes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppscopeslist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXAddrs<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXAddrs() {
                ::core::result::Result::Ok(ok__) => {
                    *ppxaddrslist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataVersion<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullmetadataversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMetadataVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *pullmetadataversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendedDiscoXML<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetExtendedDiscoXML(::core::mem::transmute_copy(&ppheaderany), ::core::mem::transmute_copy(&ppbodyany)).into()
        }
        unsafe extern "system" fn GetProbeResolveTag<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztag: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProbeResolveTag() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsztag = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteTransportAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszremotetransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRemoteTransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszremotetransportaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalTransportAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlocaltransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLocalTransportAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszlocaltransportaddress = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalInterfaceGUID<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetLocalInterfaceGUID() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredService_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullinstanceid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *pullinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>() -> IWSDiscoveryProvider_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAddressFamily(::core::mem::transmute_copy(&dwaddressfamily)).into()
        }
        unsafe extern "system" fn Attach<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Attach(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn Detach<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Detach().into()
        }
        unsafe extern "system" fn SearchById<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SearchById(::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&psztag)).into()
        }
        unsafe extern "system" fn SearchByAddress<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaddress: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SearchByAddress(::core::mem::transmute_copy(&pszaddress), ::core::mem::transmute_copy(&psztag)).into()
        }
        unsafe extern "system" fn SearchByType<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SearchByType(::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pszmatchby), ::core::mem::transmute_copy(&psztag)).into()
        }
        unsafe extern "system" fn GetXMLContext<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXMLContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetAddressFamily: SetAddressFamily::<Identity, Impl, OFFSET>,
            Attach: Attach::<Identity, Impl, OFFSET>,
            Detach: Detach::<Identity, Impl, OFFSET>,
            SearchById: SearchById::<Identity, Impl, OFFSET>,
            SearchByAddress: SearchByAddress::<Identity, Impl, OFFSET>,
            SearchByType: SearchByType::<Identity, Impl, OFFSET>,
            GetXMLContext: GetXMLContext::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>() -> IWSDiscoveryProviderNotify_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&pservice)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute(&pservice)).into()
        }
        unsafe extern "system" fn SearchFailed<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SearchFailed(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&psztag)).into()
        }
        unsafe extern "system" fn SearchComplete<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProviderNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SearchComplete(::core::mem::transmute_copy(&psztag)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            SearchFailed: SearchFailed::<Identity, Impl, OFFSET>,
            SearchComplete: SearchComplete::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>() -> IWSDiscoveryPublisher_Vtbl {
        unsafe extern "system" fn SetAddressFamily<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAddressFamily(::core::mem::transmute_copy(&dwaddressfamily)).into()
        }
        unsafe extern "system" fn RegisterNotificationSink<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterNotificationSink(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn UnRegisterNotificationSink<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnRegisterNotificationSink(::core::mem::transmute(&psink)).into()
        }
        unsafe extern "system" fn Publish<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Publish(::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn UnPublish<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnPublish(::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&pany)).into()
        }
        unsafe extern "system" fn MatchProbe<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MatchProbe(::core::mem::transmute_copy(&pprobemessage), ::core::mem::transmute(&pmessageparameters), ::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn MatchResolve<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).MatchResolve(::core::mem::transmute_copy(&presolvemessage), ::core::mem::transmute(&pmessageparameters), ::core::mem::transmute_copy(&pszid), ::core::mem::transmute_copy(&ullmetadataversion), ::core::mem::transmute_copy(&ullinstanceid), ::core::mem::transmute_copy(&ullmessagenumber), ::core::mem::transmute_copy(&pszsessionid), ::core::mem::transmute_copy(&ptypeslist), ::core::mem::transmute_copy(&pscopeslist), ::core::mem::transmute_copy(&pxaddrslist)).into()
        }
        unsafe extern "system" fn PublishEx<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn MatchProbeEx<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn MatchResolveEx<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
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
        unsafe extern "system" fn RegisterScopeMatchingRule<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopematchingrule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RegisterScopeMatchingRule(::core::mem::transmute(&pscopematchingrule)).into()
        }
        unsafe extern "system" fn UnRegisterScopeMatchingRule<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopematchingrule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnRegisterScopeMatchingRule(::core::mem::transmute(&pscopematchingrule)).into()
        }
        unsafe extern "system" fn GetXMLContext<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetXMLContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>() -> IWSDiscoveryPublisherNotify_Vtbl {
        unsafe extern "system" fn ProbeHandler<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ProbeHandler(::core::mem::transmute_copy(&psoap), ::core::mem::transmute(&pmessageparameters)).into()
        }
        unsafe extern "system" fn ResolveHandler<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisherNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResolveHandler(::core::mem::transmute_copy(&psoap), ::core::mem::transmute(&pmessageparameters)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ProbeHandler: ProbeHandler::<Identity, Impl, OFFSET>,
            ResolveHandler: ResolveHandler::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSDiscoveryPublisherNotify as ::windows::core::Interface>::IID
    }
}
