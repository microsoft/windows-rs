pub trait IWSDAddressImpl: Sized {
    fn Serialize();
    fn Deserialize();
}
impl ::windows::core::RuntimeName for IWSDAddress {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDAddress";
}
impl IWSDAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAddressImpl, const OFFSET: isize>() -> IWSDAddressVtbl {
        unsafe extern "system" fn Serialize<Impl: IWSDAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuffer: super::super::Foundation::PWSTR, cchlength: u32, fsafe: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Serialize(::core::mem::transmute_copy(&pszbuffer), cchlength, &*(&fsafe as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deserialize<Impl: IWSDAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszbuffer: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deserialize(&*(&pszbuffer as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDAddress>, ::windows::core::GetTrustLevel, Serialize::<Impl, OFFSET>, Deserialize::<Impl, OFFSET>)
    }
}
pub trait IWSDAsyncCallbackImpl: Sized {
    fn AsyncOperationComplete();
}
impl ::windows::core::RuntimeName for IWSDAsyncCallback {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDAsyncCallback";
}
impl IWSDAsyncCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncCallbackImpl, const OFFSET: isize>() -> IWSDAsyncCallbackVtbl {
        unsafe extern "system" fn AsyncOperationComplete<Impl: IWSDAsyncCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncOperationComplete(&*(&pasyncresult as *const <IWSDAsyncResult as ::windows::core::Abi>::Abi as *const <IWSDAsyncResult as ::windows::core::DefaultType>::DefaultType), &*(&pasyncstate as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDAsyncCallback>, ::windows::core::GetTrustLevel, AsyncOperationComplete::<Impl, OFFSET>)
    }
}
pub trait IWSDAsyncResultImpl: Sized {
    fn SetCallback();
    fn SetWaitHandle();
    fn HasCompleted();
    fn GetAsyncState();
    fn Abort();
    fn GetEvent();
    fn GetEndpointProxy();
}
impl ::windows::core::RuntimeName for IWSDAsyncResult {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDAsyncResult";
}
impl IWSDAsyncResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAsyncResultImpl, const OFFSET: isize>() -> IWSDAsyncResultVtbl {
        unsafe extern "system" fn SetCallback<Impl: IWSDAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, pasyncstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCallback(&*(&pcallback as *const <IWSDAsyncCallback as ::windows::core::Abi>::Abi as *const <IWSDAsyncCallback as ::windows::core::DefaultType>::DefaultType), &*(&pasyncstate as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaitHandle<Impl: IWSDAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwaithandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWaitHandle(&*(&hwaithandle as *const <super::super::Foundation::HANDLE as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HANDLE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasCompleted<Impl: IWSDAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HasCompleted() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAsyncState<Impl: IWSDAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasyncstate: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAsyncState(::core::mem::transmute_copy(&ppasyncstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IWSDAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEvent<Impl: IWSDAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevent: *mut WSD_EVENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEvent(::core::mem::transmute_copy(&pevent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Impl: IWSDAsyncResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppendpoint: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointProxy(::core::mem::transmute_copy(&ppendpoint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDAsyncResult>, ::windows::core::GetTrustLevel, SetCallback::<Impl, OFFSET>, SetWaitHandle::<Impl, OFFSET>, HasCompleted::<Impl, OFFSET>, GetAsyncState::<Impl, OFFSET>, Abort::<Impl, OFFSET>, GetEvent::<Impl, OFFSET>, GetEndpointProxy::<Impl, OFFSET>)
    }
}
pub trait IWSDAttachmentImpl: Sized {}
impl ::windows::core::RuntimeName for IWSDAttachment {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDAttachment";
}
impl IWSDAttachmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDAttachmentImpl, const OFFSET: isize>() -> IWSDAttachmentVtbl {
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDAttachment>, ::windows::core::GetTrustLevel)
    }
}
pub trait IWSDDeviceHostImpl: Sized {
    fn Init();
    fn Start();
    fn Stop();
    fn Terminate();
    fn RegisterPortType();
    fn SetMetadata();
    fn RegisterService();
    fn RetireService();
    fn AddDynamicService();
    fn RemoveDynamicService();
    fn SetServiceDiscoverable();
    fn SignalEvent();
}
impl ::windows::core::RuntimeName for IWSDDeviceHost {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDDeviceHost";
}
impl IWSDDeviceHostVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHostImpl, const OFFSET: isize>() -> IWSDDeviceHostVtbl {
        unsafe extern "system" fn Init<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, pphostaddresses: *const ::windows::core::RawPtr, dwhostaddresscount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Init(
                &*(&pszlocalid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcontext as *const <IWSDXMLContext as ::windows::core::Abi>::Abi as *const <IWSDXMLContext as ::windows::core::DefaultType>::DefaultType),
                &*(&pphostaddresses as *const <IWSDAddress as ::windows::core::Abi>::Abi as *const <IWSDAddress as ::windows::core::DefaultType>::DefaultType),
                dwhostaddresscount,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Start<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullinstanceid: u64, pscopelist: *const WSD_URI_LIST, pnotificationsink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Start(ullinstanceid, &*(&pscopelist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType), &*(&pnotificationsink as *const <IWSDDeviceHostNotify as ::windows::core::Abi>::Abi as *const <IWSDDeviceHostNotify as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stop<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stop() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Terminate<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Terminate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterPortType<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pporttype: *const WSD_PORT_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterPortType(&*(&pporttype as *const <WSD_PORT_TYPE as ::windows::core::Abi>::Abi as *const <WSD_PORT_TYPE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMetadata<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pthismodelmetadata: *const WSD_THIS_MODEL_METADATA, pthisdevicemetadata: *const WSD_THIS_DEVICE_METADATA, phostmetadata: *const WSD_HOST_METADATA, pcustommetadata: *const WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMetadata(
                &*(&pthismodelmetadata as *const <WSD_THIS_MODEL_METADATA as ::windows::core::Abi>::Abi as *const <WSD_THIS_MODEL_METADATA as ::windows::core::DefaultType>::DefaultType),
                &*(&pthisdevicemetadata as *const <WSD_THIS_DEVICE_METADATA as ::windows::core::Abi>::Abi as *const <WSD_THIS_DEVICE_METADATA as ::windows::core::DefaultType>::DefaultType),
                &*(&phostmetadata as *const <WSD_HOST_METADATA as ::windows::core::Abi>::Abi as *const <WSD_HOST_METADATA as ::windows::core::DefaultType>::DefaultType),
                &*(&pcustommetadata as *const <WSD_METADATA_SECTION_LIST as ::windows::core::Abi>::Abi as *const <WSD_METADATA_SECTION_LIST as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterService<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterService(&*(&pszserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pservice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RetireService<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RetireService(&*(&pszserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDynamicService<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, pszendpointaddress: super::super::Foundation::PWSTR, pporttype: *const WSD_PORT_TYPE, pportname: *const WSDXML_NAME, pany: *const WSDXML_ELEMENT, pservice: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDynamicService(
                &*(&pszserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszendpointaddress as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pporttype as *const <WSD_PORT_TYPE as ::windows::core::Abi>::Abi as *const <WSD_PORT_TYPE as ::windows::core::DefaultType>::DefaultType),
                &*(&pportname as *const <WSDXML_NAME as ::windows::core::Abi>::Abi as *const <WSDXML_NAME as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pservice as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveDynamicService<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveDynamicService(&*(&pszserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServiceDiscoverable<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, fdiscoverable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetServiceDiscoverable(&*(&pszserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&fdiscoverable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SignalEvent<Impl: IWSDDeviceHostImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SignalEvent(&*(&pszserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbody as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&poperation as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWSDDeviceHost>,
            ::windows::core::GetTrustLevel,
            Init::<Impl, OFFSET>,
            Start::<Impl, OFFSET>,
            Stop::<Impl, OFFSET>,
            Terminate::<Impl, OFFSET>,
            RegisterPortType::<Impl, OFFSET>,
            SetMetadata::<Impl, OFFSET>,
            RegisterService::<Impl, OFFSET>,
            RetireService::<Impl, OFFSET>,
            AddDynamicService::<Impl, OFFSET>,
            RemoveDynamicService::<Impl, OFFSET>,
            SetServiceDiscoverable::<Impl, OFFSET>,
            SignalEvent::<Impl, OFFSET>,
        )
    }
}
pub trait IWSDDeviceHostNotifyImpl: Sized {
    fn GetService();
}
impl ::windows::core::RuntimeName for IWSDDeviceHostNotify {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDDeviceHostNotify";
}
impl IWSDDeviceHostNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceHostNotifyImpl, const OFFSET: isize>() -> IWSDDeviceHostNotifyVtbl {
        unsafe extern "system" fn GetService<Impl: IWSDDeviceHostNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, ppservice: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetService(&*(&pszserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppservice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDDeviceHostNotify>, ::windows::core::GetTrustLevel, GetService::<Impl, OFFSET>)
    }
}
pub trait IWSDDeviceProxyImpl: Sized {
    fn Init();
    fn BeginGetMetadata();
    fn EndGetMetadata();
    fn GetHostMetadata();
    fn GetThisModelMetadata();
    fn GetThisDeviceMetadata();
    fn GetAllMetadata();
    fn GetServiceProxyById();
    fn GetServiceProxyByType();
    fn GetEndpointProxy();
}
impl ::windows::core::RuntimeName for IWSDDeviceProxy {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDDeviceProxy";
}
impl IWSDDeviceProxyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDDeviceProxyImpl, const OFFSET: isize>() -> IWSDDeviceProxyVtbl {
        unsafe extern "system" fn Init<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdeviceid: super::super::Foundation::PWSTR, pdeviceaddress: ::windows::core::RawPtr, pszlocalid: super::super::Foundation::PWSTR, pcontext: ::windows::core::RawPtr, psponsor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Init(
                &*(&pszdeviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pdeviceaddress as *const <IWSDAddress as ::windows::core::Abi>::Abi as *const <IWSDAddress as ::windows::core::DefaultType>::DefaultType),
                &*(&pszlocalid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pcontext as *const <IWSDXMLContext as ::windows::core::Abi>::Abi as *const <IWSDXMLContext as ::windows::core::DefaultType>::DefaultType),
                &*(&psponsor as *const <IWSDDeviceProxy as ::windows::core::Abi>::Abi as *const <IWSDDeviceProxy as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetMetadata<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginGetMetadata(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndGetMetadata(&*(&presult as *const <IWSDAsyncResult as ::windows::core::Abi>::Abi as *const <IWSDAsyncResult as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHostMetadata<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pphostmetadata: *mut *mut WSD_HOST_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHostMetadata(::core::mem::transmute_copy(&pphostmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisModelMetadata<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmanufacturermetadata: *mut *mut WSD_THIS_MODEL_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThisModelMetadata(::core::mem::transmute_copy(&ppmanufacturermetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThisDeviceMetadata<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppthisdevicemetadata: *mut *mut WSD_THIS_DEVICE_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThisDeviceMetadata(::core::mem::transmute_copy(&ppthisdevicemetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllMetadata<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAllMetadata(::core::mem::transmute_copy(&ppmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyById<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszserviceid: super::super::Foundation::PWSTR, ppserviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceProxyById(&*(&pszserviceid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppserviceproxy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceProxyByType<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *const WSDXML_NAME, ppserviceproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceProxyByType(&*(&ptype as *const <WSDXML_NAME as ::windows::core::Abi>::Abi as *const <WSDXML_NAME as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppserviceproxy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Impl: IWSDDeviceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointProxy(::core::mem::transmute_copy(&ppproxy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWSDDeviceProxy>,
            ::windows::core::GetTrustLevel,
            Init::<Impl, OFFSET>,
            BeginGetMetadata::<Impl, OFFSET>,
            EndGetMetadata::<Impl, OFFSET>,
            GetHostMetadata::<Impl, OFFSET>,
            GetThisModelMetadata::<Impl, OFFSET>,
            GetThisDeviceMetadata::<Impl, OFFSET>,
            GetAllMetadata::<Impl, OFFSET>,
            GetServiceProxyById::<Impl, OFFSET>,
            GetServiceProxyByType::<Impl, OFFSET>,
            GetEndpointProxy::<Impl, OFFSET>,
        )
    }
}
pub trait IWSDEndpointProxyImpl: Sized {
    fn SendOneWayRequest();
    fn SendTwoWayRequest();
    fn SendTwoWayRequestAsync();
    fn AbortAsyncOperation();
    fn ProcessFault();
    fn GetErrorInfo();
    fn GetFaultInfo();
}
impl ::windows::core::RuntimeName for IWSDEndpointProxy {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDEndpointProxy";
}
impl IWSDEndpointProxyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEndpointProxyImpl, const OFFSET: isize>() -> IWSDEndpointProxyVtbl {
        unsafe extern "system" fn SendOneWayRequest<Impl: IWSDEndpointProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendOneWayRequest(&*(&pbody as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&poperation as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendTwoWayRequest<Impl: IWSDEndpointProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, presponsecontext: *const WSD_SYNCHRONOUS_RESPONSE_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendTwoWayRequest(&*(&pbody as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&poperation as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType), &*(&presponsecontext as *const <WSD_SYNCHRONOUS_RESPONSE_CONTEXT as ::windows::core::Abi>::Abi as *const <WSD_SYNCHRONOUS_RESPONSE_CONTEXT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendTwoWayRequestAsync<Impl: IWSDEndpointProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pasyncstate: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr, presult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendTwoWayRequestAsync(
                &*(&pbody as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                &*(&poperation as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType),
                &*(&pasyncstate as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pcallback as *const <IWSDAsyncCallback as ::windows::core::Abi>::Abi as *const <IWSDAsyncCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&presult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortAsyncOperation<Impl: IWSDEndpointProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pasyncresult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortAsyncOperation(&*(&pasyncresult as *const <IWSDAsyncResult as ::windows::core::Abi>::Abi as *const <IWSDAsyncResult as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessFault<Impl: IWSDEndpointProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessFault(&*(&pfault as *const <WSD_SOAP_FAULT as ::windows::core::Abi>::Abi as *const <WSD_SOAP_FAULT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorInfo<Impl: IWSDEndpointProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszerrorinfo: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorInfo(::core::mem::transmute_copy(&ppszerrorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFaultInfo<Impl: IWSDEndpointProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppfault: *mut *mut WSD_SOAP_FAULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFaultInfo(::core::mem::transmute_copy(&ppfault)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWSDEndpointProxy>,
            ::windows::core::GetTrustLevel,
            SendOneWayRequest::<Impl, OFFSET>,
            SendTwoWayRequest::<Impl, OFFSET>,
            SendTwoWayRequestAsync::<Impl, OFFSET>,
            AbortAsyncOperation::<Impl, OFFSET>,
            ProcessFault::<Impl, OFFSET>,
            GetErrorInfo::<Impl, OFFSET>,
            GetFaultInfo::<Impl, OFFSET>,
        )
    }
}
pub trait IWSDEventingStatusImpl: Sized {
    fn SubscriptionRenewed();
    fn SubscriptionRenewalFailed();
    fn SubscriptionEnded();
}
impl ::windows::core::RuntimeName for IWSDEventingStatus {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDEventingStatus";
}
impl IWSDEventingStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDEventingStatusImpl, const OFFSET: isize>() -> IWSDEventingStatusVtbl {
        unsafe extern "system" fn SubscriptionRenewed<Impl: IWSDEventingStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubscriptionRenewed(&*(&pszsubscriptionaction as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SubscriptionRenewalFailed<Impl: IWSDEventingStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: super::super::Foundation::PWSTR, hr: ::windows::core::HRESULT) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubscriptionRenewalFailed(&*(&pszsubscriptionaction as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), hr).into()
        }
        unsafe extern "system" fn SubscriptionEnded<Impl: IWSDEventingStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubscriptionaction: super::super::Foundation::PWSTR) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SubscriptionEnded(&*(&pszsubscriptionaction as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDEventingStatus>, ::windows::core::GetTrustLevel, SubscriptionRenewed::<Impl, OFFSET>, SubscriptionRenewalFailed::<Impl, OFFSET>, SubscriptionEnded::<Impl, OFFSET>)
    }
}
pub trait IWSDHttpAddressImpl: Sized + IWSDTransportAddressImpl + IWSDAddressImpl {
    fn GetSecure();
    fn SetSecure();
    fn GetPath();
    fn SetPath();
}
impl ::windows::core::RuntimeName for IWSDHttpAddress {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDHttpAddress";
}
impl IWSDHttpAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAddressImpl, const OFFSET: isize>() -> IWSDHttpAddressVtbl {
        unsafe extern "system" fn GetSecure<Impl: IWSDHttpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecure() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSecure<Impl: IWSDHttpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsecure: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSecure(&*(&fsecure as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Impl: IWSDHttpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath(::core::mem::transmute_copy(&ppszpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IWSDHttpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPath(&*(&pszpath as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDHttpAddress>, ::windows::core::GetTrustLevel, GetSecure::<Impl, OFFSET>, SetSecure::<Impl, OFFSET>, GetPath::<Impl, OFFSET>, SetPath::<Impl, OFFSET>)
    }
}
pub trait IWSDHttpAuthParametersImpl: Sized {
    fn GetClientAccessToken();
    fn GetAuthType();
}
impl ::windows::core::RuntimeName for IWSDHttpAuthParameters {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDHttpAuthParameters";
}
impl IWSDHttpAuthParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpAuthParametersImpl, const OFFSET: isize>() -> IWSDHttpAuthParametersVtbl {
        unsafe extern "system" fn GetClientAccessToken<Impl: IWSDHttpAuthParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientAccessToken(::core::mem::transmute_copy(&phtoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthType<Impl: IWSDHttpAuthParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pauthtype: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthType(::core::mem::transmute_copy(&pauthtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDHttpAuthParameters>, ::windows::core::GetTrustLevel, GetClientAccessToken::<Impl, OFFSET>, GetAuthType::<Impl, OFFSET>)
    }
}
pub trait IWSDHttpMessageParametersImpl: Sized + IWSDMessageParametersImpl {
    fn SetInboundHttpHeaders();
    fn GetInboundHttpHeaders();
    fn SetOutboundHttpHeaders();
    fn GetOutboundHttpHeaders();
    fn SetID();
    fn GetID();
    fn SetContext();
    fn GetContext();
    fn Clear();
}
impl ::windows::core::RuntimeName for IWSDHttpMessageParameters {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDHttpMessageParameters";
}
impl IWSDHttpMessageParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>() -> IWSDHttpMessageParametersVtbl {
        unsafe extern "system" fn SetInboundHttpHeaders<Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInboundHttpHeaders(&*(&pszheaders as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInboundHttpHeaders<Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInboundHttpHeaders(::core::mem::transmute_copy(&ppszheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOutboundHttpHeaders<Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszheaders: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOutboundHttpHeaders(&*(&pszheaders as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutboundHttpHeaders<Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszheaders: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOutboundHttpHeaders(::core::mem::transmute_copy(&ppszheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetID<Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetID(&*(&pszid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetID<Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetID(::core::mem::transmute_copy(&ppszid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContext(&*(&pcontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext(::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: IWSDHttpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWSDHttpMessageParameters>,
            ::windows::core::GetTrustLevel,
            SetInboundHttpHeaders::<Impl, OFFSET>,
            GetInboundHttpHeaders::<Impl, OFFSET>,
            SetOutboundHttpHeaders::<Impl, OFFSET>,
            GetOutboundHttpHeaders::<Impl, OFFSET>,
            SetID::<Impl, OFFSET>,
            GetID::<Impl, OFFSET>,
            SetContext::<Impl, OFFSET>,
            GetContext::<Impl, OFFSET>,
            Clear::<Impl, OFFSET>,
        )
    }
}
pub trait IWSDInboundAttachmentImpl: Sized + IWSDAttachmentImpl {
    fn Read();
    fn Close();
}
impl ::windows::core::RuntimeName for IWSDInboundAttachment {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDInboundAttachment";
}
impl IWSDInboundAttachmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDInboundAttachmentImpl, const OFFSET: isize>() -> IWSDInboundAttachmentVtbl {
        unsafe extern "system" fn Read<Impl: IWSDInboundAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *mut u8, dwbytestoread: u32, pdwnumberofbytesread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Read(::core::mem::transmute_copy(&pbuffer), dwbytestoread, ::core::mem::transmute_copy(&pdwnumberofbytesread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWSDInboundAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDInboundAttachment>, ::windows::core::GetTrustLevel, Read::<Impl, OFFSET>, Close::<Impl, OFFSET>)
    }
}
pub trait IWSDMessageParametersImpl: Sized {
    fn GetLocalAddress();
    fn SetLocalAddress();
    fn GetRemoteAddress();
    fn SetRemoteAddress();
    fn GetLowerParameters();
}
impl ::windows::core::RuntimeName for IWSDMessageParameters {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDMessageParameters";
}
impl IWSDMessageParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMessageParametersImpl, const OFFSET: isize>() -> IWSDMessageParametersVtbl {
        unsafe extern "system" fn GetLocalAddress<Impl: IWSDMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalAddress(::core::mem::transmute_copy(&ppaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLocalAddress<Impl: IWSDMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLocalAddress(&*(&paddress as *const <IWSDAddress as ::windows::core::Abi>::Abi as *const <IWSDAddress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteAddress<Impl: IWSDMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppaddress: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteAddress(::core::mem::transmute_copy(&ppaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRemoteAddress<Impl: IWSDMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, paddress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRemoteAddress(&*(&paddress as *const <IWSDAddress as ::windows::core::Abi>::Abi as *const <IWSDAddress as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLowerParameters<Impl: IWSDMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptxparams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLowerParameters(::core::mem::transmute_copy(&pptxparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDMessageParameters>, ::windows::core::GetTrustLevel, GetLocalAddress::<Impl, OFFSET>, SetLocalAddress::<Impl, OFFSET>, GetRemoteAddress::<Impl, OFFSET>, SetRemoteAddress::<Impl, OFFSET>, GetLowerParameters::<Impl, OFFSET>)
    }
}
pub trait IWSDMetadataExchangeImpl: Sized {
    fn GetMetadata();
}
impl ::windows::core::RuntimeName for IWSDMetadataExchange {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDMetadataExchange";
}
impl IWSDMetadataExchangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDMetadataExchangeImpl, const OFFSET: isize>() -> IWSDMetadataExchangeVtbl {
        unsafe extern "system" fn GetMetadata<Impl: IWSDMetadataExchangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, metadataout: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadata(::core::mem::transmute_copy(&metadataout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDMetadataExchange>, ::windows::core::GetTrustLevel, GetMetadata::<Impl, OFFSET>)
    }
}
pub trait IWSDOutboundAttachmentImpl: Sized + IWSDAttachmentImpl {
    fn Write();
    fn Close();
    fn Abort();
}
impl ::windows::core::RuntimeName for IWSDOutboundAttachment {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDOutboundAttachment";
}
impl IWSDOutboundAttachmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDOutboundAttachmentImpl, const OFFSET: isize>() -> IWSDOutboundAttachmentVtbl {
        unsafe extern "system" fn Write<Impl: IWSDOutboundAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbuffer: *const u8, dwbytestowrite: u32, pdwnumberofbyteswritten: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(pbuffer, dwbytestowrite, ::core::mem::transmute_copy(&pdwnumberofbyteswritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IWSDOutboundAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IWSDOutboundAttachmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDOutboundAttachment>, ::windows::core::GetTrustLevel, Write::<Impl, OFFSET>, Close::<Impl, OFFSET>, Abort::<Impl, OFFSET>)
    }
}
pub trait IWSDSSLClientCertificateImpl: Sized {
    fn GetClientCertificate();
    fn GetMappedAccessToken();
}
impl ::windows::core::RuntimeName for IWSDSSLClientCertificate {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDSSLClientCertificate";
}
impl IWSDSSLClientCertificateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSSLClientCertificateImpl, const OFFSET: isize>() -> IWSDSSLClientCertificateVtbl {
        unsafe extern "system" fn GetClientCertificate<Impl: IWSDSSLClientCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcertcontext: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClientCertificate(::core::mem::transmute_copy(&ppcertcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMappedAccessToken<Impl: IWSDSSLClientCertificateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phtoken: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMappedAccessToken(::core::mem::transmute_copy(&phtoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDSSLClientCertificate>, ::windows::core::GetTrustLevel, GetClientCertificate::<Impl, OFFSET>, GetMappedAccessToken::<Impl, OFFSET>)
    }
}
pub trait IWSDScopeMatchingRuleImpl: Sized {
    fn GetScopeRule();
    fn MatchScopes();
}
impl ::windows::core::RuntimeName for IWSDScopeMatchingRule {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDScopeMatchingRule";
}
impl IWSDScopeMatchingRuleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDScopeMatchingRuleImpl, const OFFSET: isize>() -> IWSDScopeMatchingRuleVtbl {
        unsafe extern "system" fn GetScopeRule<Impl: IWSDScopeMatchingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszscopematchingrule: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScopeRule(::core::mem::transmute_copy(&ppszscopematchingrule)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchScopes<Impl: IWSDScopeMatchingRuleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszscope1: super::super::Foundation::PWSTR, pszscope2: super::super::Foundation::PWSTR, pfmatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchScopes(&*(&pszscope1 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszscope2 as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfmatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDScopeMatchingRule>, ::windows::core::GetTrustLevel, GetScopeRule::<Impl, OFFSET>, MatchScopes::<Impl, OFFSET>)
    }
}
pub trait IWSDServiceMessagingImpl: Sized {
    fn SendResponse();
    fn FaultRequest();
}
impl ::windows::core::RuntimeName for IWSDServiceMessaging {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDServiceMessaging";
}
impl IWSDServiceMessagingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceMessagingImpl, const OFFSET: isize>() -> IWSDServiceMessagingVtbl {
        unsafe extern "system" fn SendResponse<Impl: IWSDServiceMessagingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbody: *const ::core::ffi::c_void, poperation: *const WSD_OPERATION, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendResponse(&*(&pbody as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&poperation as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType), &*(&pmessageparameters as *const <IWSDMessageParameters as ::windows::core::Abi>::Abi as *const <IWSDMessageParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FaultRequest<Impl: IWSDServiceMessagingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, prequestheader: *const WSD_SOAP_HEADER, pmessageparameters: ::windows::core::RawPtr, pfault: *const WSD_SOAP_FAULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FaultRequest(&*(&prequestheader as *const <WSD_SOAP_HEADER as ::windows::core::Abi>::Abi as *const <WSD_SOAP_HEADER as ::windows::core::DefaultType>::DefaultType), &*(&pmessageparameters as *const <IWSDMessageParameters as ::windows::core::Abi>::Abi as *const <IWSDMessageParameters as ::windows::core::DefaultType>::DefaultType), &*(&pfault as *const <WSD_SOAP_FAULT as ::windows::core::Abi>::Abi as *const <WSD_SOAP_FAULT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDServiceMessaging>, ::windows::core::GetTrustLevel, SendResponse::<Impl, OFFSET>, FaultRequest::<Impl, OFFSET>)
    }
}
pub trait IWSDServiceProxyImpl: Sized + IWSDMetadataExchangeImpl {
    fn BeginGetMetadata();
    fn EndGetMetadata();
    fn GetServiceMetadata();
    fn SubscribeToOperation();
    fn UnsubscribeToOperation();
    fn SetEventingStatusCallback();
    fn GetEndpointProxy();
}
impl ::windows::core::RuntimeName for IWSDServiceProxy {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDServiceProxy";
}
impl IWSDServiceProxyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyImpl, const OFFSET: isize>() -> IWSDServiceProxyVtbl {
        unsafe extern "system" fn BeginGetMetadata<Impl: IWSDServiceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginGetMetadata(::core::mem::transmute_copy(&ppresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetMetadata<Impl: IWSDServiceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presult: ::windows::core::RawPtr, ppmetadata: *mut *mut WSD_METADATA_SECTION_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndGetMetadata(&*(&presult as *const <IWSDAsyncResult as ::windows::core::Abi>::Abi as *const <IWSDAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppmetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceMetadata<Impl: IWSDServiceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppservicemetadata: *mut *mut WSD_SERVICE_METADATA) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetServiceMetadata(::core::mem::transmute_copy(&ppservicemetadata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubscribeToOperation<Impl: IWSDServiceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION, punknown: *mut ::core::ffi::c_void, pany: *const WSDXML_ELEMENT, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscribeToOperation(
                &*(&poperation as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType),
                &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppany),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnsubscribeToOperation<Impl: IWSDServiceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperation: *const WSD_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnsubscribeToOperation(&*(&poperation as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventingStatusCallback<Impl: IWSDServiceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstatus: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventingStatusCallback(&*(&pstatus as *const <IWSDEventingStatus as ::windows::core::Abi>::Abi as *const <IWSDEventingStatus as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEndpointProxy<Impl: IWSDServiceProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppproxy: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointProxy(::core::mem::transmute_copy(&ppproxy)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWSDServiceProxy>,
            ::windows::core::GetTrustLevel,
            BeginGetMetadata::<Impl, OFFSET>,
            EndGetMetadata::<Impl, OFFSET>,
            GetServiceMetadata::<Impl, OFFSET>,
            SubscribeToOperation::<Impl, OFFSET>,
            UnsubscribeToOperation::<Impl, OFFSET>,
            SetEventingStatusCallback::<Impl, OFFSET>,
            GetEndpointProxy::<Impl, OFFSET>,
        )
    }
}
pub trait IWSDServiceProxyEventingImpl: Sized + IWSDServiceProxyImpl + IWSDMetadataExchangeImpl {
    fn SubscribeToMultipleOperations();
    fn BeginSubscribeToMultipleOperations();
    fn EndSubscribeToMultipleOperations();
    fn UnsubscribeToMultipleOperations();
    fn BeginUnsubscribeToMultipleOperations();
    fn EndUnsubscribeToMultipleOperations();
    fn RenewMultipleOperations();
    fn BeginRenewMultipleOperations();
    fn EndRenewMultipleOperations();
    fn GetStatusForMultipleOperations();
    fn BeginGetStatusForMultipleOperations();
    fn EndGetStatusForMultipleOperations();
}
impl ::windows::core::RuntimeName for IWSDServiceProxyEventing {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDServiceProxyEventing";
}
impl IWSDServiceProxyEventingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>() -> IWSDServiceProxyEventingVtbl {
        unsafe extern "system" fn SubscribeToMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscribeToMultipleOperations(
                &*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType),
                dwoperationcount,
                &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pexpires as *const <WSD_EVENTING_EXPIRES as ::windows::core::Abi>::Abi as *const <WSD_EVENTING_EXPIRES as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppexpires),
                ::core::mem::transmute_copy(&ppany),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginSubscribeToMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, punknown: *mut ::core::ffi::c_void, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginSubscribeToMultipleOperations(
                &*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType),
                dwoperationcount,
                &*(&punknown as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pexpires as *const <WSD_EVENTING_EXPIRES as ::windows::core::Abi>::Abi as *const <WSD_EVENTING_EXPIRES as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pasyncstate as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pasynccallback as *const <IWSDAsyncCallback as ::windows::core::Abi>::Abi as *const <IWSDAsyncCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndSubscribeToMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndSubscribeToMultipleOperations(&*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType), dwoperationcount, &*(&presult as *const <IWSDAsyncResult as ::windows::core::Abi>::Abi as *const <IWSDAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnsubscribeToMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnsubscribeToMultipleOperations(&*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType), dwoperationcount, &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginUnsubscribeToMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginUnsubscribeToMultipleOperations(
                &*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType),
                dwoperationcount,
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pasyncstate as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pasynccallback as *const <IWSDAsyncCallback as ::windows::core::Abi>::Abi as *const <IWSDAsyncCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndUnsubscribeToMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndUnsubscribeToMultipleOperations(&*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType), dwoperationcount, &*(&presult as *const <IWSDAsyncResult as ::windows::core::Abi>::Abi as *const <IWSDAsyncResult as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RenewMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RenewMultipleOperations(
                &*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType),
                dwoperationcount,
                &*(&pexpires as *const <WSD_EVENTING_EXPIRES as ::windows::core::Abi>::Abi as *const <WSD_EVENTING_EXPIRES as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppexpires),
                ::core::mem::transmute_copy(&ppany),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginRenewMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pexpires: *const WSD_EVENTING_EXPIRES, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginRenewMultipleOperations(
                &*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType),
                dwoperationcount,
                &*(&pexpires as *const <WSD_EVENTING_EXPIRES as ::windows::core::Abi>::Abi as *const <WSD_EVENTING_EXPIRES as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pasyncstate as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pasynccallback as *const <IWSDAsyncCallback as ::windows::core::Abi>::Abi as *const <IWSDAsyncCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndRenewMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndRenewMultipleOperations(&*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType), dwoperationcount, &*(&presult as *const <IWSDAsyncResult as ::windows::core::Abi>::Abi as *const <IWSDAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetStatusForMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetStatusForMultipleOperations(&*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType), dwoperationcount, &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginGetStatusForMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, pany: *const WSDXML_ELEMENT, pasyncstate: *mut ::core::ffi::c_void, pasynccallback: ::windows::core::RawPtr, ppresult: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginGetStatusForMultipleOperations(
                &*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType),
                dwoperationcount,
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pasyncstate as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&pasynccallback as *const <IWSDAsyncCallback as ::windows::core::Abi>::Abi as *const <IWSDAsyncCallback as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppresult),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndGetStatusForMultipleOperations<Impl: IWSDServiceProxyEventingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, poperations: *const WSD_OPERATION, dwoperationcount: u32, presult: ::windows::core::RawPtr, ppexpires: *mut *mut WSD_EVENTING_EXPIRES, ppany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndGetStatusForMultipleOperations(&*(&poperations as *const <WSD_OPERATION as ::windows::core::Abi>::Abi as *const <WSD_OPERATION as ::windows::core::DefaultType>::DefaultType), dwoperationcount, &*(&presult as *const <IWSDAsyncResult as ::windows::core::Abi>::Abi as *const <IWSDAsyncResult as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppexpires), ::core::mem::transmute_copy(&ppany)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWSDServiceProxyEventing>,
            ::windows::core::GetTrustLevel,
            SubscribeToMultipleOperations::<Impl, OFFSET>,
            BeginSubscribeToMultipleOperations::<Impl, OFFSET>,
            EndSubscribeToMultipleOperations::<Impl, OFFSET>,
            UnsubscribeToMultipleOperations::<Impl, OFFSET>,
            BeginUnsubscribeToMultipleOperations::<Impl, OFFSET>,
            EndUnsubscribeToMultipleOperations::<Impl, OFFSET>,
            RenewMultipleOperations::<Impl, OFFSET>,
            BeginRenewMultipleOperations::<Impl, OFFSET>,
            EndRenewMultipleOperations::<Impl, OFFSET>,
            GetStatusForMultipleOperations::<Impl, OFFSET>,
            BeginGetStatusForMultipleOperations::<Impl, OFFSET>,
            EndGetStatusForMultipleOperations::<Impl, OFFSET>,
        )
    }
}
pub trait IWSDSignaturePropertyImpl: Sized {
    fn IsMessageSigned();
    fn IsMessageSignatureTrusted();
    fn GetKeyInfo();
    fn GetSignature();
    fn GetSignedInfoHash();
}
impl ::windows::core::RuntimeName for IWSDSignatureProperty {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDSignatureProperty";
}
impl IWSDSignaturePropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDSignaturePropertyImpl, const OFFSET: isize>() -> IWSDSignaturePropertyVtbl {
        unsafe extern "system" fn IsMessageSigned<Impl: IWSDSignaturePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsigned: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMessageSigned(::core::mem::transmute_copy(&pbsigned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMessageSignatureTrusted<Impl: IWSDSignaturePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignaturetrusted: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMessageSignatureTrusted(::core::mem::transmute_copy(&pbsignaturetrusted)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetKeyInfo<Impl: IWSDSignaturePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbkeyinfo: *mut u8, pdwkeyinfosize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetKeyInfo(::core::mem::transmute_copy(&pbkeyinfo), pdwkeyinfosize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignature<Impl: IWSDSignaturePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignature: *mut u8, pdwsignaturesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignature(::core::mem::transmute_copy(&pbsignature), pdwsignaturesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSignedInfoHash<Impl: IWSDSignaturePropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsignedinfohash: *mut u8, pdwhashsize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSignedInfoHash(::core::mem::transmute_copy(&pbsignedinfohash), pdwhashsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDSignatureProperty>, ::windows::core::GetTrustLevel, IsMessageSigned::<Impl, OFFSET>, IsMessageSignatureTrusted::<Impl, OFFSET>, GetKeyInfo::<Impl, OFFSET>, GetSignature::<Impl, OFFSET>, GetSignedInfoHash::<Impl, OFFSET>)
    }
}
pub trait IWSDTransportAddressImpl: Sized + IWSDAddressImpl {
    fn GetPort();
    fn SetPort();
    fn GetTransportAddress();
    fn GetTransportAddressEx();
    fn SetTransportAddress();
}
impl ::windows::core::RuntimeName for IWSDTransportAddress {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDTransportAddress";
}
impl IWSDTransportAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDTransportAddressImpl, const OFFSET: isize>() -> IWSDTransportAddressVtbl {
        unsafe extern "system" fn GetPort<Impl: IWSDTransportAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwport: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPort(::core::mem::transmute_copy(&pwport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPort<Impl: IWSDTransportAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wport: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPort(wport) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransportAddress<Impl: IWSDTransportAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransportAddress(::core::mem::transmute_copy(&ppszaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransportAddressEx<Impl: IWSDTransportAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fsafe: super::super::Foundation::BOOL, ppszaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransportAddressEx(&*(&fsafe as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransportAddress<Impl: IWSDTransportAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaddress: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransportAddress(&*(&pszaddress as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDTransportAddress>, ::windows::core::GetTrustLevel, GetPort::<Impl, OFFSET>, SetPort::<Impl, OFFSET>, GetTransportAddress::<Impl, OFFSET>, GetTransportAddressEx::<Impl, OFFSET>, SetTransportAddress::<Impl, OFFSET>)
    }
}
pub trait IWSDUdpAddressImpl: Sized + IWSDTransportAddressImpl + IWSDAddressImpl {
    fn SetSockaddr();
    fn GetSockaddr();
    fn SetExclusive();
    fn GetExclusive();
    fn SetMessageType();
    fn GetMessageType();
    fn SetTTL();
    fn GetTTL();
    fn SetAlias();
    fn GetAlias();
}
impl ::windows::core::RuntimeName for IWSDUdpAddress {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDUdpAddress";
}
impl IWSDUdpAddressVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpAddressImpl, const OFFSET: isize>() -> IWSDUdpAddressVtbl {
        unsafe extern "system" fn SetSockaddr<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psockaddr: *const super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSockaddr(&*(&psockaddr as *const <super::super::Networking::WinSock::SOCKADDR_STORAGE as ::windows::core::Abi>::Abi as *const <super::super::Networking::WinSock::SOCKADDR_STORAGE as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSockaddr<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psockaddr: *mut super::super::Networking::WinSock::SOCKADDR_STORAGE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSockaddr(::core::mem::transmute_copy(&psockaddr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExclusive<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fexclusive: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExclusive(&*(&fexclusive as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExclusive<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMessageType<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, messagetype: WSDUdpMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMessageType(messagetype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMessageType<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmessagetype: *mut WSDUdpMessageType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMessageType(::core::mem::transmute_copy(&pmessagetype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTTL<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwttl: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTTL(dwttl) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTTL<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwttl: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTTL(::core::mem::transmute_copy(&pdwttl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAlias<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palias: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAlias(&*(&palias as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAlias<Impl: IWSDUdpAddressImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, palias: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAlias(::core::mem::transmute_copy(&palias)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWSDUdpAddress>,
            ::windows::core::GetTrustLevel,
            SetSockaddr::<Impl, OFFSET>,
            GetSockaddr::<Impl, OFFSET>,
            SetExclusive::<Impl, OFFSET>,
            GetExclusive::<Impl, OFFSET>,
            SetMessageType::<Impl, OFFSET>,
            GetMessageType::<Impl, OFFSET>,
            SetTTL::<Impl, OFFSET>,
            GetTTL::<Impl, OFFSET>,
            SetAlias::<Impl, OFFSET>,
            GetAlias::<Impl, OFFSET>,
        )
    }
}
pub trait IWSDUdpMessageParametersImpl: Sized + IWSDMessageParametersImpl {
    fn SetRetransmitParams();
    fn GetRetransmitParams();
}
impl ::windows::core::RuntimeName for IWSDUdpMessageParameters {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDUdpMessageParameters";
}
impl IWSDUdpMessageParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDUdpMessageParametersImpl, const OFFSET: isize>() -> IWSDUdpMessageParametersVtbl {
        unsafe extern "system" fn SetRetransmitParams<Impl: IWSDUdpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *const WSDUdpRetransmitParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRetransmitParams(&*(&pparams as *const <WSDUdpRetransmitParams as ::windows::core::Abi>::Abi as *const <WSDUdpRetransmitParams as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRetransmitParams<Impl: IWSDUdpMessageParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pparams: *mut WSDUdpRetransmitParams) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRetransmitParams(::core::mem::transmute_copy(&pparams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDUdpMessageParameters>, ::windows::core::GetTrustLevel, SetRetransmitParams::<Impl, OFFSET>, GetRetransmitParams::<Impl, OFFSET>)
    }
}
pub trait IWSDXMLContextImpl: Sized {
    fn AddNamespace();
    fn AddNameToNamespace();
    fn SetNamespaces();
    fn SetTypes();
}
impl ::windows::core::RuntimeName for IWSDXMLContext {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDXMLContext";
}
impl IWSDXMLContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDXMLContextImpl, const OFFSET: isize>() -> IWSDXMLContextVtbl {
        unsafe extern "system" fn AddNamespace<Impl: IWSDXMLContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuri: super::super::Foundation::PWSTR, pszsuggestedprefix: super::super::Foundation::PWSTR, ppnamespace: *mut *mut WSDXML_NAMESPACE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNamespace(&*(&pszuri as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszsuggestedprefix as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddNameToNamespace<Impl: IWSDXMLContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszuri: super::super::Foundation::PWSTR, pszname: super::super::Foundation::PWSTR, ppname: *mut *mut WSDXML_NAME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNameToNamespace(&*(&pszuri as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNamespaces<Impl: IWSDXMLContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnamespaces: *const *const WSDXML_NAMESPACE, wnamespacescount: u16, blayernumber: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetNamespaces(&*(&pnamespaces as *const <WSDXML_NAMESPACE as ::windows::core::Abi>::Abi as *const <WSDXML_NAMESPACE as ::windows::core::DefaultType>::DefaultType), wnamespacescount, blayernumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypes<Impl: IWSDXMLContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypes: *const *const WSDXML_TYPE, dwtypescount: u32, blayernumber: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTypes(&*(&ptypes as *const <WSDXML_TYPE as ::windows::core::Abi>::Abi as *const <WSDXML_TYPE as ::windows::core::DefaultType>::DefaultType), dwtypescount, blayernumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDXMLContext>, ::windows::core::GetTrustLevel, AddNamespace::<Impl, OFFSET>, AddNameToNamespace::<Impl, OFFSET>, SetNamespaces::<Impl, OFFSET>, SetTypes::<Impl, OFFSET>)
    }
}
pub trait IWSDiscoveredServiceImpl: Sized {
    fn GetEndpointReference();
    fn GetTypes();
    fn GetScopes();
    fn GetXAddrs();
    fn GetMetadataVersion();
    fn GetExtendedDiscoXML();
    fn GetProbeResolveTag();
    fn GetRemoteTransportAddress();
    fn GetLocalTransportAddress();
    fn GetLocalInterfaceGUID();
    fn GetInstanceId();
}
impl ::windows::core::RuntimeName for IWSDiscoveredService {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDiscoveredService";
}
impl IWSDiscoveredServiceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>() -> IWSDiscoveredServiceVtbl {
        unsafe extern "system" fn GetEndpointReference<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppendpointreference: *mut *mut WSD_ENDPOINT_REFERENCE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEndpointReference(::core::mem::transmute_copy(&ppendpointreference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTypes<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptypeslist: *mut *mut WSD_NAME_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTypes(::core::mem::transmute_copy(&pptypeslist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetScopes<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscopeslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetScopes(::core::mem::transmute_copy(&ppscopeslist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXAddrs<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppxaddrslist: *mut *mut WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXAddrs(::core::mem::transmute_copy(&ppxaddrslist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMetadataVersion<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullmetadataversion: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMetadataVersion(::core::mem::transmute_copy(&pullmetadataversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetExtendedDiscoXML<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppheaderany: *mut *mut WSDXML_ELEMENT, ppbodyany: *mut *mut WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetExtendedDiscoXML(::core::mem::transmute_copy(&ppheaderany), ::core::mem::transmute_copy(&ppbodyany)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProbeResolveTag<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsztag: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProbeResolveTag(::core::mem::transmute_copy(&ppsztag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRemoteTransportAddress<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszremotetransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRemoteTransportAddress(::core::mem::transmute_copy(&ppszremotetransportaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalTransportAddress<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszlocaltransportaddress: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalTransportAddress(::core::mem::transmute_copy(&ppszlocaltransportaddress)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLocalInterfaceGUID<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLocalInterfaceGUID(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceId<Impl: IWSDiscoveredServiceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullinstanceid: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceId(::core::mem::transmute_copy(&pullinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWSDiscoveredService>,
            ::windows::core::GetTrustLevel,
            GetEndpointReference::<Impl, OFFSET>,
            GetTypes::<Impl, OFFSET>,
            GetScopes::<Impl, OFFSET>,
            GetXAddrs::<Impl, OFFSET>,
            GetMetadataVersion::<Impl, OFFSET>,
            GetExtendedDiscoXML::<Impl, OFFSET>,
            GetProbeResolveTag::<Impl, OFFSET>,
            GetRemoteTransportAddress::<Impl, OFFSET>,
            GetLocalTransportAddress::<Impl, OFFSET>,
            GetLocalInterfaceGUID::<Impl, OFFSET>,
            GetInstanceId::<Impl, OFFSET>,
        )
    }
}
pub trait IWSDiscoveryProviderImpl: Sized {
    fn SetAddressFamily();
    fn Attach();
    fn Detach();
    fn SearchById();
    fn SearchByAddress();
    fn SearchByType();
    fn GetXMLContext();
}
impl ::windows::core::RuntimeName for IWSDiscoveryProvider {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDiscoveryProvider";
}
impl IWSDiscoveryProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProviderImpl, const OFFSET: isize>() -> IWSDiscoveryProviderVtbl {
        unsafe extern "system" fn SetAddressFamily<Impl: IWSDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAddressFamily(dwaddressfamily) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attach<Impl: IWSDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Attach(&*(&psink as *const <IWSDiscoveryProviderNotify as ::windows::core::Abi>::Abi as *const <IWSDiscoveryProviderNotify as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Detach<Impl: IWSDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Detach() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchById<Impl: IWSDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchById(&*(&pszid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&psztag as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchByAddress<Impl: IWSDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszaddress: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchByAddress(&*(&pszaddress as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&psztag as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchByType<Impl: IWSDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pszmatchby: super::super::Foundation::PWSTR, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchByType(
                &*(&ptypeslist as *const <WSD_NAME_LIST as ::windows::core::Abi>::Abi as *const <WSD_NAME_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pscopeslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pszmatchby as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&psztag as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLContext<Impl: IWSDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLContext(::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDiscoveryProvider>, ::windows::core::GetTrustLevel, SetAddressFamily::<Impl, OFFSET>, Attach::<Impl, OFFSET>, Detach::<Impl, OFFSET>, SearchById::<Impl, OFFSET>, SearchByAddress::<Impl, OFFSET>, SearchByType::<Impl, OFFSET>, GetXMLContext::<Impl, OFFSET>)
    }
}
pub trait IWSDiscoveryProviderNotifyImpl: Sized {
    fn Add();
    fn Remove();
    fn SearchFailed();
    fn SearchComplete();
}
impl ::windows::core::RuntimeName for IWSDiscoveryProviderNotify {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDiscoveryProviderNotify";
}
impl IWSDiscoveryProviderNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryProviderNotifyImpl, const OFFSET: isize>() -> IWSDiscoveryProviderNotifyVtbl {
        unsafe extern "system" fn Add<Impl: IWSDiscoveryProviderNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&pservice as *const <IWSDiscoveredService as ::windows::core::Abi>::Abi as *const <IWSDiscoveredService as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IWSDiscoveryProviderNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pservice: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&pservice as *const <IWSDiscoveredService as ::windows::core::Abi>::Abi as *const <IWSDiscoveredService as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchFailed<Impl: IWSDiscoveryProviderNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchFailed(hr, &*(&psztag as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SearchComplete<Impl: IWSDiscoveryProviderNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psztag: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SearchComplete(&*(&psztag as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDiscoveryProviderNotify>, ::windows::core::GetTrustLevel, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, SearchFailed::<Impl, OFFSET>, SearchComplete::<Impl, OFFSET>)
    }
}
pub trait IWSDiscoveryPublisherImpl: Sized {
    fn SetAddressFamily();
    fn RegisterNotificationSink();
    fn UnRegisterNotificationSink();
    fn Publish();
    fn UnPublish();
    fn MatchProbe();
    fn MatchResolve();
    fn PublishEx();
    fn MatchProbeEx();
    fn MatchResolveEx();
    fn RegisterScopeMatchingRule();
    fn UnRegisterScopeMatchingRule();
    fn GetXMLContext();
}
impl ::windows::core::RuntimeName for IWSDiscoveryPublisher {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDiscoveryPublisher";
}
impl IWSDiscoveryPublisherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>() -> IWSDiscoveryPublisherVtbl {
        unsafe extern "system" fn SetAddressFamily<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwaddressfamily: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAddressFamily(dwaddressfamily) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterNotificationSink<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterNotificationSink(&*(&psink as *const <IWSDiscoveryPublisherNotify as ::windows::core::Abi>::Abi as *const <IWSDiscoveryPublisherNotify as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnRegisterNotificationSink<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnRegisterNotificationSink(&*(&psink as *const <IWSDiscoveryPublisherNotify as ::windows::core::Abi>::Abi as *const <IWSDiscoveryPublisherNotify as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Publish<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Publish(
                &*(&pszid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptypeslist as *const <WSD_NAME_LIST as ::windows::core::Abi>::Abi as *const <WSD_NAME_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pscopeslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pxaddrslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnPublish<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnPublish(
                &*(&pszid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ullinstanceid,
                ullmessagenumber,
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchProbe<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchProbe(
                &*(&pprobemessage as *const <WSD_SOAP_MESSAGE as ::windows::core::Abi>::Abi as *const <WSD_SOAP_MESSAGE as ::windows::core::DefaultType>::DefaultType),
                &*(&pmessageparameters as *const <IWSDMessageParameters as ::windows::core::Abi>::Abi as *const <IWSDMessageParameters as ::windows::core::DefaultType>::DefaultType),
                &*(&pszid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptypeslist as *const <WSD_NAME_LIST as ::windows::core::Abi>::Abi as *const <WSD_NAME_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pscopeslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pxaddrslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchResolve<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchResolve(
                &*(&presolvemessage as *const <WSD_SOAP_MESSAGE as ::windows::core::Abi>::Abi as *const <WSD_SOAP_MESSAGE as ::windows::core::DefaultType>::DefaultType),
                &*(&pmessageparameters as *const <IWSDMessageParameters as ::windows::core::Abi>::Abi as *const <IWSDMessageParameters as ::windows::core::DefaultType>::DefaultType),
                &*(&pszid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptypeslist as *const <WSD_NAME_LIST as ::windows::core::Abi>::Abi as *const <WSD_NAME_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pscopeslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pxaddrslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PublishEx<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublishEx(
                &*(&pszid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptypeslist as *const <WSD_NAME_LIST as ::windows::core::Abi>::Abi as *const <WSD_NAME_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pscopeslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pxaddrslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pheaderany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&preferenceparameterany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&ppolicyany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pendpointreferenceany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchProbeEx<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprobemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchProbeEx(
                &*(&pprobemessage as *const <WSD_SOAP_MESSAGE as ::windows::core::Abi>::Abi as *const <WSD_SOAP_MESSAGE as ::windows::core::DefaultType>::DefaultType),
                &*(&pmessageparameters as *const <IWSDMessageParameters as ::windows::core::Abi>::Abi as *const <IWSDMessageParameters as ::windows::core::DefaultType>::DefaultType),
                &*(&pszid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptypeslist as *const <WSD_NAME_LIST as ::windows::core::Abi>::Abi as *const <WSD_NAME_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pscopeslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pxaddrslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pheaderany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&preferenceparameterany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&ppolicyany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pendpointreferenceany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MatchResolveEx<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, presolvemessage: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr, pszid: super::super::Foundation::PWSTR, ullmetadataversion: u64, ullinstanceid: u64, ullmessagenumber: u64, pszsessionid: super::super::Foundation::PWSTR, ptypeslist: *const WSD_NAME_LIST, pscopeslist: *const WSD_URI_LIST, pxaddrslist: *const WSD_URI_LIST, pheaderany: *const WSDXML_ELEMENT, preferenceparameterany: *const WSDXML_ELEMENT, ppolicyany: *const WSDXML_ELEMENT, pendpointreferenceany: *const WSDXML_ELEMENT, pany: *const WSDXML_ELEMENT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MatchResolveEx(
                &*(&presolvemessage as *const <WSD_SOAP_MESSAGE as ::windows::core::Abi>::Abi as *const <WSD_SOAP_MESSAGE as ::windows::core::DefaultType>::DefaultType),
                &*(&pmessageparameters as *const <IWSDMessageParameters as ::windows::core::Abi>::Abi as *const <IWSDMessageParameters as ::windows::core::DefaultType>::DefaultType),
                &*(&pszid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ullmetadataversion,
                ullinstanceid,
                ullmessagenumber,
                &*(&pszsessionid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ptypeslist as *const <WSD_NAME_LIST as ::windows::core::Abi>::Abi as *const <WSD_NAME_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pscopeslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pxaddrslist as *const <WSD_URI_LIST as ::windows::core::Abi>::Abi as *const <WSD_URI_LIST as ::windows::core::DefaultType>::DefaultType),
                &*(&pheaderany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&preferenceparameterany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&ppolicyany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pendpointreferenceany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
                &*(&pany as *const <WSDXML_ELEMENT as ::windows::core::Abi>::Abi as *const <WSDXML_ELEMENT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterScopeMatchingRule<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopematchingrule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterScopeMatchingRule(&*(&pscopematchingrule as *const <IWSDScopeMatchingRule as ::windows::core::Abi>::Abi as *const <IWSDScopeMatchingRule as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnRegisterScopeMatchingRule<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pscopematchingrule: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnRegisterScopeMatchingRule(&*(&pscopematchingrule as *const <IWSDScopeMatchingRule as ::windows::core::Abi>::Abi as *const <IWSDScopeMatchingRule as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXMLContext<Impl: IWSDiscoveryPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXMLContext(::core::mem::transmute_copy(&ppcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWSDiscoveryPublisher>,
            ::windows::core::GetTrustLevel,
            SetAddressFamily::<Impl, OFFSET>,
            RegisterNotificationSink::<Impl, OFFSET>,
            UnRegisterNotificationSink::<Impl, OFFSET>,
            Publish::<Impl, OFFSET>,
            UnPublish::<Impl, OFFSET>,
            MatchProbe::<Impl, OFFSET>,
            MatchResolve::<Impl, OFFSET>,
            PublishEx::<Impl, OFFSET>,
            MatchProbeEx::<Impl, OFFSET>,
            MatchResolveEx::<Impl, OFFSET>,
            RegisterScopeMatchingRule::<Impl, OFFSET>,
            UnRegisterScopeMatchingRule::<Impl, OFFSET>,
            GetXMLContext::<Impl, OFFSET>,
        )
    }
}
pub trait IWSDiscoveryPublisherNotifyImpl: Sized {
    fn ProbeHandler();
    fn ResolveHandler();
}
impl ::windows::core::RuntimeName for IWSDiscoveryPublisherNotify {
    const NAME: &'static str = "Windows.Win32.Devices.WebServicesOnDevices.IWSDiscoveryPublisherNotify";
}
impl IWSDiscoveryPublisherNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSDiscoveryPublisherNotifyImpl, const OFFSET: isize>() -> IWSDiscoveryPublisherNotifyVtbl {
        unsafe extern "system" fn ProbeHandler<Impl: IWSDiscoveryPublisherNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProbeHandler(&*(&psoap as *const <WSD_SOAP_MESSAGE as ::windows::core::Abi>::Abi as *const <WSD_SOAP_MESSAGE as ::windows::core::DefaultType>::DefaultType), &*(&pmessageparameters as *const <IWSDMessageParameters as ::windows::core::Abi>::Abi as *const <IWSDMessageParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResolveHandler<Impl: IWSDiscoveryPublisherNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psoap: *const WSD_SOAP_MESSAGE, pmessageparameters: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResolveHandler(&*(&psoap as *const <WSD_SOAP_MESSAGE as ::windows::core::Abi>::Abi as *const <WSD_SOAP_MESSAGE as ::windows::core::DefaultType>::DefaultType), &*(&pmessageparameters as *const <IWSDMessageParameters as ::windows::core::Abi>::Abi as *const <IWSDMessageParameters as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWSDiscoveryPublisherNotify>, ::windows::core::GetTrustLevel, ProbeHandler::<Impl, OFFSET>, ResolveHandler::<Impl, OFFSET>)
    }
}
