#[cfg(feature = "Win32_System_Com")]
pub trait ContextInfoImpl: Sized + IDispatchImpl {
    fn IsInTransaction();
    fn GetTransaction();
    fn GetTransactionId();
    fn GetActivityId();
    fn GetContextId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ContextInfo {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ContextInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl ContextInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ContextInfoImpl, const OFFSET: isize>() -> ContextInfoVtbl {
        unsafe extern "system" fn IsInTransaction<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInTransaction(::core::mem::transmute_copy(&pbisintx)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransaction<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptx: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransaction(::core::mem::transmute_copy(&pptx)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionId(::core::mem::transmute_copy(&pbstrtxid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityId<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstractivityid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivityId(::core::mem::transmute_copy(&pbstractivityid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextId<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrctxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextId(::core::mem::transmute_copy(&pbstrctxid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ContextInfo>, ::windows::core::GetTrustLevel, IsInTransaction::<Impl, OFFSET>, GetTransaction::<Impl, OFFSET>, GetTransactionId::<Impl, OFFSET>, GetActivityId::<Impl, OFFSET>, GetContextId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ContextInfo2Impl: Sized + ContextInfoImpl + IDispatchImpl {
    fn GetPartitionId();
    fn GetApplicationId();
    fn GetApplicationInstanceId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ContextInfo2 {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ContextInfo2";
}
#[cfg(feature = "Win32_System_Com")]
impl ContextInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ContextInfo2Impl, const OFFSET: isize>() -> ContextInfo2Vtbl {
        unsafe extern "system" fn GetPartitionId<Impl: ContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20000: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartitionId(::core::mem::transmute_copy(&__midl__contextinfo20000)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationId<Impl: ContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20001: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationId(::core::mem::transmute_copy(&__midl__contextinfo20001)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceId<Impl: ContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20002: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationInstanceId(::core::mem::transmute_copy(&__midl__contextinfo20002)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ContextInfo2>, ::windows::core::GetTrustLevel, GetPartitionId::<Impl, OFFSET>, GetApplicationId::<Impl, OFFSET>, GetApplicationInstanceId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAppDomainHelperImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn DoCallback();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAppDomainHelper {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IAppDomainHelper";
}
#[cfg(feature = "Win32_System_Com")]
impl IAppDomainHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDomainHelperImpl, const OFFSET: isize>() -> IAppDomainHelperVtbl {
        unsafe extern "system" fn Initialize<Impl: IAppDomainHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&punkad as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), __midl__iappdomainhelper0000, &*(&ppool as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoCallback<Impl: IAppDomainHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoCallback(&*(&punkad as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), __midl__iappdomainhelper0001, &*(&ppool as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAppDomainHelper>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, DoCallback::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAssemblyLocatorImpl: Sized + IDispatchImpl {
    fn GetModules();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAssemblyLocator {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IAssemblyLocator";
}
#[cfg(feature = "Win32_System_Com")]
impl IAssemblyLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyLocatorImpl, const OFFSET: isize>() -> IAssemblyLocatorVtbl {
        unsafe extern "system" fn GetModules<Impl: IAssemblyLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, assemblyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModules(
                &*(&applicationdir as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&applicationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&assemblyname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pmodules),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAssemblyLocator>, ::windows::core::GetTrustLevel, GetModules::<Impl, OFFSET>)
    }
}
pub trait IAsyncErrorNotifyImpl: Sized {
    fn OnError();
}
impl ::windows::core::RuntimeName for IAsyncErrorNotify {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IAsyncErrorNotify";
}
impl IAsyncErrorNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncErrorNotifyImpl, const OFFSET: isize>() -> IAsyncErrorNotifyVtbl {
        unsafe extern "system" fn OnError<Impl: IAsyncErrorNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnError(hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAsyncErrorNotify>, ::windows::core::GetTrustLevel, OnError::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICOMAdminCatalogImpl: Sized + IDispatchImpl {
    fn GetCollection();
    fn Connect();
    fn MajorVersion();
    fn MinorVersion();
    fn GetCollectionByQuery();
    fn ImportComponent();
    fn InstallComponent();
    fn ShutdownApplication();
    fn ExportApplication();
    fn InstallApplication();
    fn StopRouter();
    fn RefreshRouter();
    fn StartRouter();
    fn Reserved1();
    fn Reserved2();
    fn InstallMultipleComponents();
    fn GetMultipleComponentsInfo();
    fn RefreshComponents();
    fn BackupREGDB();
    fn RestoreREGDB();
    fn QueryApplicationFile();
    fn StartApplication();
    fn ServiceCheck();
    fn InstallMultipleEventClasses();
    fn InstallEventClass();
    fn GetEventClassesForIID();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICOMAdminCatalog {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICOMAdminCatalog";
}
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICOMAdminCatalogImpl, const OFFSET: isize>() -> ICOMAdminCatalogVtbl {
        unsafe extern "system" fn GetCollection<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollection(&*(&bstrcollname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcatalogcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcatalogservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(&*(&bstrcatalogservername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcatalogcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion(::core::mem::transmute_copy(&plmajorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion(::core::mem::transmute_copy(&plminorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionByQuery<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollectionByQuery(&*(&bstrcollname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&ppsavarquery as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcatalogcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportComponent<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportComponent(&*(&bstrapplidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrclsidorprogid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallComponent<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallComponent(
                &*(&bstrapplidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdll as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrtlb as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpsdll as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownApplication<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShutdownApplication(&*(&bstrapplidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportApplication<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportApplication(&*(&bstrapplidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrapplicationfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), loptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallApplication<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallApplication(
                &*(&bstrapplicationfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdestinationdirectory as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                loptions,
                &*(&bstruserid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrrsn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopRouter<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopRouter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshRouter<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshRouter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartRouter<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartRouter() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved1<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reserved1() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved2<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reserved2() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallMultipleComponents<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallMultipleComponents(
                &*(&bstrapplidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppsavarfilenames as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType),
                &*(&ppsavarclsids as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMultipleComponentsInfo(
                &*(&bstrapplidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppsavarfilenames as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppsavarclsids),
                ::core::mem::transmute_copy(&ppsavarclassnames),
                ::core::mem::transmute_copy(&ppsavarfileflags),
                ::core::mem::transmute_copy(&ppsavarcomponentflags),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RefreshComponents<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RefreshComponents() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BackupREGDB<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BackupREGDB(&*(&bstrbackupfilepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreREGDB<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestoreREGDB(&*(&bstrbackupfilepath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryApplicationFile<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryApplicationFile(&*(&bstrapplicationfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrapplicationname), ::core::mem::transmute_copy(&pbstrapplicationdescription), ::core::mem::transmute_copy(&pbhasusers), ::core::mem::transmute_copy(&pbisproxy), ::core::mem::transmute_copy(&ppsavarfilenames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartApplication<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartApplication(&*(&bstrapplidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServiceCheck<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceCheck(lservice, ::core::mem::transmute_copy(&plstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallMultipleEventClasses(
                &*(&bstrapplidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&ppsavarfilenames as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType),
                &*(&ppsavarclsids as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallEventClass<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallEventClass(
                &*(&bstrapplidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdll as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrtlb as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpsdll as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventClassesForIID<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstriid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventClassesForIID(&*(&bstriid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsavarclsids), ::core::mem::transmute_copy(&ppsavarprogids), ::core::mem::transmute_copy(&ppsavardescriptions)) {
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
            ::windows::core::GetRuntimeClassName::<ICOMAdminCatalog>,
            ::windows::core::GetTrustLevel,
            GetCollection::<Impl, OFFSET>,
            Connect::<Impl, OFFSET>,
            MajorVersion::<Impl, OFFSET>,
            MinorVersion::<Impl, OFFSET>,
            GetCollectionByQuery::<Impl, OFFSET>,
            ImportComponent::<Impl, OFFSET>,
            InstallComponent::<Impl, OFFSET>,
            ShutdownApplication::<Impl, OFFSET>,
            ExportApplication::<Impl, OFFSET>,
            InstallApplication::<Impl, OFFSET>,
            StopRouter::<Impl, OFFSET>,
            RefreshRouter::<Impl, OFFSET>,
            StartRouter::<Impl, OFFSET>,
            Reserved1::<Impl, OFFSET>,
            Reserved2::<Impl, OFFSET>,
            InstallMultipleComponents::<Impl, OFFSET>,
            GetMultipleComponentsInfo::<Impl, OFFSET>,
            RefreshComponents::<Impl, OFFSET>,
            BackupREGDB::<Impl, OFFSET>,
            RestoreREGDB::<Impl, OFFSET>,
            QueryApplicationFile::<Impl, OFFSET>,
            StartApplication::<Impl, OFFSET>,
            ServiceCheck::<Impl, OFFSET>,
            InstallMultipleEventClasses::<Impl, OFFSET>,
            InstallEventClass::<Impl, OFFSET>,
            GetEventClassesForIID::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICOMAdminCatalog2Impl: Sized + ICOMAdminCatalogImpl + IDispatchImpl {
    fn GetCollectionByQuery2();
    fn GetApplicationInstanceIDFromProcessID();
    fn ShutdownApplicationInstances();
    fn PauseApplicationInstances();
    fn ResumeApplicationInstances();
    fn RecycleApplicationInstances();
    fn AreApplicationInstancesPaused();
    fn DumpApplicationInstance();
    fn IsApplicationInstanceDumpSupported();
    fn CreateServiceForApplication();
    fn DeleteServiceForApplication();
    fn GetPartitionID();
    fn GetPartitionName();
    fn SetCurrentPartition();
    fn CurrentPartitionID();
    fn CurrentPartitionName();
    fn GlobalPartitionID();
    fn FlushPartitionCache();
    fn CopyApplications();
    fn CopyComponents();
    fn MoveComponents();
    fn AliasComponent();
    fn IsSafeToDelete();
    fn ImportUnconfiguredComponents();
    fn PromoteUnconfiguredComponents();
    fn ImportComponents();
    fn Is64BitCatalogServer();
    fn ExportPartition();
    fn InstallPartition();
    fn QueryApplicationFile2();
    fn GetComponentVersionCount();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICOMAdminCatalog2 {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICOMAdminCatalog2";
}
#[cfg(feature = "Win32_System_Com")]
impl ICOMAdminCatalog2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>() -> ICOMAdminCatalog2Vtbl {
        unsafe extern "system" fn GetCollectionByQuery2<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollectionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarquerystrings: *const super::Com::VARIANT, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollectionByQuery2(&*(&bstrcollectionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pvarquerystrings as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcatalogcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationInstanceIDFromProcessID(lprocessid, ::core::mem::transmute_copy(&pbstrapplicationinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShutdownApplicationInstances(&*(&pvarapplicationinstanceid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PauseApplicationInstances<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PauseApplicationInstances(&*(&pvarapplicationinstanceid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResumeApplicationInstances<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResumeApplicationInstances(&*(&pvarapplicationinstanceid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecycleApplicationInstances<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecycleApplicationInstances(&*(&pvarapplicationinstanceid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), lreasoncode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, pvarboolpaused: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreApplicationInstancesPaused(&*(&pvarapplicationinstanceid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarboolpaused)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpApplicationInstance<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaximages: i32, pbstrdumpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DumpApplicationInstance(&*(&bstrapplicationinstanceid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrdirectory as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lmaximages, ::core::mem::transmute_copy(&pbstrdumpfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbooldumpsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsApplicationInstanceDumpSupported(::core::mem::transmute_copy(&pvarbooldumpsupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServiceForApplication<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrstarttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrerrorcontrol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdependencies: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrunas: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bdesktopok: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateServiceForApplication(
                &*(&bstrapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrservicename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrstarttype as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrerrorcontrol as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdependencies as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrrunas as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                bdesktopok,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteServiceForApplication<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteServiceForApplication(&*(&bstrapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartitionID<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartitionID(&*(&bstrapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrpartitionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartitionName<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartitionName(&*(&bstrapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrpartitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPartition<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCurrentPartition(&*(&bstrpartitionidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPartitionID<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPartitionID(::core::mem::transmute_copy(&pbstrpartitionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPartitionName<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPartitionName(::core::mem::transmute_copy(&pbstrpartitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalPartitionID<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrglobalpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalPartitionID(::core::mem::transmute_copy(&pbstrglobalpartitionid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushPartitionCache<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FlushPartitionCache() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyApplications<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourcepartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyApplications(
                &*(&bstrsourcepartitionidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarapplicationid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdestinationpartitionidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CopyComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CopyComponents(
                &*(&bstrsourceapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarclsidorprogid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdestinationapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveComponents(
                &*(&bstrsourceapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarclsidorprogid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdestinationapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AliasComponent<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsrcapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AliasComponent(
                &*(&bstrsrcapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrclsidorprogid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdestapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrnewprogid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrnewclsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSafeToDelete<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSafeToDelete(&*(&bstrdllname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcomadmininuse)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportUnconfiguredComponents(
                &*(&bstrapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarclsidorprogid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarcomponenttype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PromoteUnconfiguredComponents(
                &*(&bstrapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarclsidorprogid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarcomponenttype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportComponents(
                &*(&bstrapplicationidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarclsidorprogid as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pvarcomponenttype as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Is64BitCatalogServer<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbis64bit: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is64BitCatalogServer(::core::mem::transmute_copy(&pbis64bit)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPartition<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpartitionfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExportPartition(&*(&bstrpartitionidorname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrpartitionfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), loptions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallPartition<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstallPartition(
                &*(&bstrfilename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrdestdirectory as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                loptions,
                &*(&bstruserid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrrsn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryApplicationFile2<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfilesforimport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryApplicationFile2(&*(&bstrapplicationfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppfilesforimport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentVersionCount<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plversioncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentVersionCount(&*(&bstrclsidorprogid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plversioncount)) {
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
            ::windows::core::GetRuntimeClassName::<ICOMAdminCatalog2>,
            ::windows::core::GetTrustLevel,
            GetCollectionByQuery2::<Impl, OFFSET>,
            GetApplicationInstanceIDFromProcessID::<Impl, OFFSET>,
            ShutdownApplicationInstances::<Impl, OFFSET>,
            PauseApplicationInstances::<Impl, OFFSET>,
            ResumeApplicationInstances::<Impl, OFFSET>,
            RecycleApplicationInstances::<Impl, OFFSET>,
            AreApplicationInstancesPaused::<Impl, OFFSET>,
            DumpApplicationInstance::<Impl, OFFSET>,
            IsApplicationInstanceDumpSupported::<Impl, OFFSET>,
            CreateServiceForApplication::<Impl, OFFSET>,
            DeleteServiceForApplication::<Impl, OFFSET>,
            GetPartitionID::<Impl, OFFSET>,
            GetPartitionName::<Impl, OFFSET>,
            SetCurrentPartition::<Impl, OFFSET>,
            CurrentPartitionID::<Impl, OFFSET>,
            CurrentPartitionName::<Impl, OFFSET>,
            GlobalPartitionID::<Impl, OFFSET>,
            FlushPartitionCache::<Impl, OFFSET>,
            CopyApplications::<Impl, OFFSET>,
            CopyComponents::<Impl, OFFSET>,
            MoveComponents::<Impl, OFFSET>,
            AliasComponent::<Impl, OFFSET>,
            IsSafeToDelete::<Impl, OFFSET>,
            ImportUnconfiguredComponents::<Impl, OFFSET>,
            PromoteUnconfiguredComponents::<Impl, OFFSET>,
            ImportComponents::<Impl, OFFSET>,
            Is64BitCatalogServer::<Impl, OFFSET>,
            ExportPartition::<Impl, OFFSET>,
            InstallPartition::<Impl, OFFSET>,
            QueryApplicationFile2::<Impl, OFFSET>,
            GetComponentVersionCount::<Impl, OFFSET>,
        )
    }
}
pub trait ICOMLBArgumentsImpl: Sized {
    fn GetCLSID();
    fn SetCLSID();
    fn GetMachineName();
    fn SetMachineName();
}
impl ::windows::core::RuntimeName for ICOMLBArguments {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICOMLBArguments";
}
impl ICOMLBArgumentsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICOMLBArgumentsImpl, const OFFSET: isize>() -> ICOMLBArgumentsVtbl {
        unsafe extern "system" fn GetCLSID<Impl: ICOMLBArgumentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCLSID(&*(&pclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCLSID<Impl: ICOMLBArgumentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCLSID(&*(&pclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMachineName<Impl: ICOMLBArgumentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMachineName(cchsvr, ::core::mem::transmute_copy(&szservername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachineName<Impl: ICOMLBArgumentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMachineName(cchsvr, &*(&szservername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICOMLBArguments>, ::windows::core::GetTrustLevel, GetCLSID::<Impl, OFFSET>, SetCLSID::<Impl, OFFSET>, GetMachineName::<Impl, OFFSET>, SetMachineName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICatalogCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Remove();
    fn Add();
    fn Populate();
    fn SaveChanges();
    fn GetCollection();
    fn Name();
    fn AddEnabled();
    fn RemoveEnabled();
    fn GetUtilInterface();
    fn DataStoreMajorVersion();
    fn DataStoreMinorVersion();
    fn PopulateByKey();
    fn PopulateByQuery();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICatalogCollection {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICatalogCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl ICatalogCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogCollectionImpl, const OFFSET: isize>() -> ICatalogCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumvariant)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppcatalogobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(lindex, ::core::mem::transmute_copy(&ppcatalogobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plobjectcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plobjectcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(lindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcatalogobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(::core::mem::transmute_copy(&ppcatalogobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Populate<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Populate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveChanges<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchanges: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveChanges(::core::mem::transmute_copy(&pcchanges)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollection<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varobjectkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollection(&*(&bstrcollname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varobjectkey as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppcatalogcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarnamel: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pvarnamel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnabled<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddEnabled(::core::mem::transmute_copy(&pvarbool)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnabled<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveEnabled(::core::mem::transmute_copy(&pvarbool)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUtilInterface<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUtilInterface(::core::mem::transmute_copy(&ppidispatch)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMajorVersion<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataStoreMajorVersion(::core::mem::transmute_copy(&plmajorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMinorVersion<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversionl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataStoreMinorVersion(::core::mem::transmute_copy(&plminorversionl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopulateByKey<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PopulateByKey(&*(&psakeys as *const <super::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopulateByQuery<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquerystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lquerytype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PopulateByQuery(&*(&bstrquerystring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lquerytype) {
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
            ::windows::core::GetRuntimeClassName::<ICatalogCollection>,
            ::windows::core::GetTrustLevel,
            _NewEnum::<Impl, OFFSET>,
            Item::<Impl, OFFSET>,
            Count::<Impl, OFFSET>,
            Remove::<Impl, OFFSET>,
            Add::<Impl, OFFSET>,
            Populate::<Impl, OFFSET>,
            SaveChanges::<Impl, OFFSET>,
            GetCollection::<Impl, OFFSET>,
            Name::<Impl, OFFSET>,
            AddEnabled::<Impl, OFFSET>,
            RemoveEnabled::<Impl, OFFSET>,
            GetUtilInterface::<Impl, OFFSET>,
            DataStoreMajorVersion::<Impl, OFFSET>,
            DataStoreMinorVersion::<Impl, OFFSET>,
            PopulateByKey::<Impl, OFFSET>,
            PopulateByQuery::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICatalogObjectImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Key();
    fn Name();
    fn IsPropertyReadOnly();
    fn Valid();
    fn IsPropertyWriteOnly();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICatalogObject {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICatalogObject";
}
#[cfg(feature = "Win32_System_Com")]
impl ICatalogObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogObjectImpl, const OFFSET: isize>() -> ICatalogObjectVtbl {
        unsafe extern "system" fn Value<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(&*(&bstrpropname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&bstrpropname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&val as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Key<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key(::core::mem::transmute_copy(&pvarretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pvarretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyReadOnly<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertyReadOnly(&*(&bstrpropname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Valid(::core::mem::transmute_copy(&pbretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertyWriteOnly(&*(&bstrpropname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbretval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICatalogObject>, ::windows::core::GetTrustLevel, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>, Key::<Impl, OFFSET>, Name::<Impl, OFFSET>, IsPropertyReadOnly::<Impl, OFFSET>, Valid::<Impl, OFFSET>, IsPropertyWriteOnly::<Impl, OFFSET>)
    }
}
pub trait ICheckSxsConfigImpl: Sized {
    fn IsSameSxsConfig();
}
impl ::windows::core::RuntimeName for ICheckSxsConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICheckSxsConfig";
}
impl ICheckSxsConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICheckSxsConfigImpl, const OFFSET: isize>() -> ICheckSxsConfigVtbl {
        unsafe extern "system" fn IsSameSxsConfig<Impl: ICheckSxsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsxsname: super::super::Foundation::PWSTR, wszsxsdirectory: super::super::Foundation::PWSTR, wszsxsappname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSameSxsConfig(
                &*(&wszsxsname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszsxsdirectory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszsxsappname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICheckSxsConfig>, ::windows::core::GetTrustLevel, IsSameSxsConfig::<Impl, OFFSET>)
    }
}
pub trait IComActivityEventsImpl: Sized {
    fn OnActivityCreate();
    fn OnActivityDestroy();
    fn OnActivityEnter();
    fn OnActivityTimeout();
    fn OnActivityReenter();
    fn OnActivityLeave();
    fn OnActivityLeaveSame();
}
impl ::windows::core::RuntimeName for IComActivityEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComActivityEvents";
}
impl IComActivityEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComActivityEventsImpl, const OFFSET: isize>() -> IComActivityEventsVtbl {
        unsafe extern "system" fn OnActivityCreate<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivityCreate(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnActivityDestroy<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivityDestroy(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnActivityEnter<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivityEnter(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidcurrent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidentered as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwthread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnActivityTimeout<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivityTimeout(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidcurrent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidentered as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dwthread,
                dwtimeout,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnActivityReenter<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivityReenter(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidcurrent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwthread, dwcalldepth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnActivityLeave<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidleft: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivityLeave(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidcurrent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidleft as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnActivityLeaveSame<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwcalldepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnActivityLeaveSame(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidcurrent as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcalldepth) {
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
            ::windows::core::GetRuntimeClassName::<IComActivityEvents>,
            ::windows::core::GetTrustLevel,
            OnActivityCreate::<Impl, OFFSET>,
            OnActivityDestroy::<Impl, OFFSET>,
            OnActivityEnter::<Impl, OFFSET>,
            OnActivityTimeout::<Impl, OFFSET>,
            OnActivityReenter::<Impl, OFFSET>,
            OnActivityLeave::<Impl, OFFSET>,
            OnActivityLeaveSame::<Impl, OFFSET>,
        )
    }
}
pub trait IComApp2EventsImpl: Sized {
    fn OnAppActivation2();
    fn OnAppShutdown2();
    fn OnAppForceShutdown2();
    fn OnAppPaused2();
    fn OnAppRecycle2();
}
impl ::windows::core::RuntimeName for IComApp2Events {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComApp2Events";
}
impl IComApp2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComApp2EventsImpl, const OFFSET: isize>() -> IComApp2EventsVtbl {
        unsafe extern "system" fn OnAppActivation2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAppActivation2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidprocess as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAppShutdown2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAppShutdown2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAppForceShutdown2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAppForceShutdown2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAppPaused2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAppPaused2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&bpaused as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAppRecycle2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID, lreason: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAppRecycle2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidprocess as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), lreason) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComApp2Events>, ::windows::core::GetTrustLevel, OnAppActivation2::<Impl, OFFSET>, OnAppShutdown2::<Impl, OFFSET>, OnAppForceShutdown2::<Impl, OFFSET>, OnAppPaused2::<Impl, OFFSET>, OnAppRecycle2::<Impl, OFFSET>)
    }
}
pub trait IComAppEventsImpl: Sized {
    fn OnAppActivation();
    fn OnAppShutdown();
    fn OnAppForceShutdown();
}
impl ::windows::core::RuntimeName for IComAppEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComAppEvents";
}
impl IComAppEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComAppEventsImpl, const OFFSET: isize>() -> IComAppEventsVtbl {
        unsafe extern "system" fn OnAppActivation<Impl: IComAppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAppActivation(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAppShutdown<Impl: IComAppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAppShutdown(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAppForceShutdown<Impl: IComAppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAppForceShutdown(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComAppEvents>, ::windows::core::GetTrustLevel, OnAppActivation::<Impl, OFFSET>, OnAppShutdown::<Impl, OFFSET>, OnAppForceShutdown::<Impl, OFFSET>)
    }
}
pub trait IComCRMEventsImpl: Sized {
    fn OnCRMRecoveryStart();
    fn OnCRMRecoveryDone();
    fn OnCRMCheckpoint();
    fn OnCRMBegin();
    fn OnCRMPrepare();
    fn OnCRMCommit();
    fn OnCRMAbort();
    fn OnCRMIndoubt();
    fn OnCRMDone();
    fn OnCRMRelease();
    fn OnCRMAnalyze();
    fn OnCRMWrite();
    fn OnCRMForget();
    fn OnCRMForce();
    fn OnCRMDeliver();
}
impl ::windows::core::RuntimeName for IComCRMEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComCRMEvents";
}
impl IComCRMEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComCRMEventsImpl, const OFFSET: isize>() -> IComCRMEventsVtbl {
        unsafe extern "system" fn OnCRMRecoveryStart<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMRecoveryStart(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMRecoveryDone<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMRecoveryDone(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMCheckpoint<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMCheckpoint(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidapp as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMBegin<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, guidactivity: ::windows::core::GUID, guidtx: ::windows::core::GUID, szprogidcompensator: super::super::Foundation::PWSTR, szdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMBegin(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&szprogidcompensator as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&szdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMPrepare<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMPrepare(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMCommit<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMCommit(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMAbort<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMAbort(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMIndoubt<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMIndoubt(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMDone<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMDone(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMRelease<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMRelease(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMAnalyze<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMAnalyze(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwcrmrecordtype, dwrecordsize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMWrite<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMWrite(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&fvariants as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                dwrecordsize,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMForget<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMForget(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMForce<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMForce(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnCRMDeliver<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCRMDeliver(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidclerkclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&fvariants as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                dwrecordsize,
            ) {
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
            ::windows::core::GetRuntimeClassName::<IComCRMEvents>,
            ::windows::core::GetTrustLevel,
            OnCRMRecoveryStart::<Impl, OFFSET>,
            OnCRMRecoveryDone::<Impl, OFFSET>,
            OnCRMCheckpoint::<Impl, OFFSET>,
            OnCRMBegin::<Impl, OFFSET>,
            OnCRMPrepare::<Impl, OFFSET>,
            OnCRMCommit::<Impl, OFFSET>,
            OnCRMAbort::<Impl, OFFSET>,
            OnCRMIndoubt::<Impl, OFFSET>,
            OnCRMDone::<Impl, OFFSET>,
            OnCRMRelease::<Impl, OFFSET>,
            OnCRMAnalyze::<Impl, OFFSET>,
            OnCRMWrite::<Impl, OFFSET>,
            OnCRMForget::<Impl, OFFSET>,
            OnCRMForce::<Impl, OFFSET>,
            OnCRMDeliver::<Impl, OFFSET>,
        )
    }
}
pub trait IComExceptionEventsImpl: Sized {
    fn OnExceptionUser();
}
impl ::windows::core::RuntimeName for IComExceptionEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComExceptionEvents";
}
impl IComExceptionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComExceptionEventsImpl, const OFFSET: isize>() -> IComExceptionEventsVtbl {
        unsafe extern "system" fn OnExceptionUser<Impl: IComExceptionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnExceptionUser(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), code, address, &*(&pszstacktrace as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComExceptionEvents>, ::windows::core::GetTrustLevel, OnExceptionUser::<Impl, OFFSET>)
    }
}
pub trait IComIdentityEventsImpl: Sized {
    fn OnIISRequestInfo();
}
impl ::windows::core::RuntimeName for IComIdentityEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComIdentityEvents";
}
impl IComIdentityEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComIdentityEventsImpl, const OFFSET: isize>() -> IComIdentityEventsVtbl {
        unsafe extern "system" fn OnIISRequestInfo<Impl: IComIdentityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: super::super::Foundation::PWSTR, pszserverip: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnIISRequestInfo(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                objid,
                &*(&pszclientip as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszserverip as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComIdentityEvents>, ::windows::core::GetTrustLevel, OnIISRequestInfo::<Impl, OFFSET>)
    }
}
pub trait IComInstance2EventsImpl: Sized {
    fn OnObjectCreate2();
    fn OnObjectDestroy2();
}
impl ::windows::core::RuntimeName for IComInstance2Events {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComInstance2Events";
}
impl IComInstance2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComInstance2EventsImpl, const OFFSET: isize>() -> IComInstance2EventsVtbl {
        unsafe extern "system" fn OnObjectCreate2<Impl: IComInstance2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjectCreate2(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&tsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ctxtid,
                objectid,
                &*(&guidpartition as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjectDestroy2<Impl: IComInstance2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjectDestroy2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), ctxtid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComInstance2Events>, ::windows::core::GetTrustLevel, OnObjectCreate2::<Impl, OFFSET>, OnObjectDestroy2::<Impl, OFFSET>)
    }
}
pub trait IComInstanceEventsImpl: Sized {
    fn OnObjectCreate();
    fn OnObjectDestroy();
}
impl ::windows::core::RuntimeName for IComInstanceEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComInstanceEvents";
}
impl IComInstanceEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComInstanceEventsImpl, const OFFSET: isize>() -> IComInstanceEventsVtbl {
        unsafe extern "system" fn OnObjectCreate<Impl: IComInstanceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjectCreate(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&tsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ctxtid,
                objectid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjectDestroy<Impl: IComInstanceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjectDestroy(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), ctxtid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComInstanceEvents>, ::windows::core::GetTrustLevel, OnObjectCreate::<Impl, OFFSET>, OnObjectDestroy::<Impl, OFFSET>)
    }
}
pub trait IComLTxEventsImpl: Sized {
    fn OnLtxTransactionStart();
    fn OnLtxTransactionPrepare();
    fn OnLtxTransactionAbort();
    fn OnLtxTransactionCommit();
    fn OnLtxTransactionPromote();
}
impl ::windows::core::RuntimeName for IComLTxEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComLTxEvents";
}
impl IComLTxEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComLTxEventsImpl, const OFFSET: isize>() -> IComLTxEventsVtbl {
        unsafe extern "system" fn OnLtxTransactionStart<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, tsid: ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLtxTransactionStart(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidltx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&tsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&froot as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                nisolationlevel,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLtxTransactionPrepare<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLtxTransactionPrepare(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidltx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&fvote as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLtxTransactionAbort<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLtxTransactionAbort(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidltx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLtxTransactionCommit<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLtxTransactionCommit(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidltx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLtxTransactionPromote<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, txnid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLtxTransactionPromote(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidltx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&txnid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComLTxEvents>, ::windows::core::GetTrustLevel, OnLtxTransactionStart::<Impl, OFFSET>, OnLtxTransactionPrepare::<Impl, OFFSET>, OnLtxTransactionAbort::<Impl, OFFSET>, OnLtxTransactionCommit::<Impl, OFFSET>, OnLtxTransactionPromote::<Impl, OFFSET>)
    }
}
pub trait IComMethod2EventsImpl: Sized {
    fn OnMethodCall2();
    fn OnMethodReturn2();
    fn OnMethodException2();
}
impl ::windows::core::RuntimeName for IComMethod2Events {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComMethod2Events";
}
impl IComMethod2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComMethod2EventsImpl, const OFFSET: isize>() -> IComMethod2EventsVtbl {
        unsafe extern "system" fn OnMethodCall2<Impl: IComMethod2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMethodCall2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), oid, &*(&guidcid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidrid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwthread, imeth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnMethodReturn2<Impl: IComMethod2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMethodReturn2(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                oid,
                &*(&guidcid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidrid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dwthread,
                imeth,
                hresult,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnMethodException2<Impl: IComMethod2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMethodException2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), oid, &*(&guidcid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidrid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwthread, imeth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComMethod2Events>, ::windows::core::GetTrustLevel, OnMethodCall2::<Impl, OFFSET>, OnMethodReturn2::<Impl, OFFSET>, OnMethodException2::<Impl, OFFSET>)
    }
}
pub trait IComMethodEventsImpl: Sized {
    fn OnMethodCall();
    fn OnMethodReturn();
    fn OnMethodException();
}
impl ::windows::core::RuntimeName for IComMethodEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComMethodEvents";
}
impl IComMethodEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComMethodEventsImpl, const OFFSET: isize>() -> IComMethodEventsVtbl {
        unsafe extern "system" fn OnMethodCall<Impl: IComMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMethodCall(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), oid, &*(&guidcid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidrid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), imeth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnMethodReturn<Impl: IComMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMethodReturn(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), oid, &*(&guidcid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidrid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), imeth, hresult) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnMethodException<Impl: IComMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnMethodException(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), oid, &*(&guidcid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidrid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), imeth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComMethodEvents>, ::windows::core::GetTrustLevel, OnMethodCall::<Impl, OFFSET>, OnMethodReturn::<Impl, OFFSET>, OnMethodException::<Impl, OFFSET>)
    }
}
pub trait IComMtaThreadPoolKnobsImpl: Sized {
    fn MTASetMaxThreadCount();
    fn MTAGetMaxThreadCount();
    fn MTASetThrottleValue();
    fn MTAGetThrottleValue();
}
impl ::windows::core::RuntimeName for IComMtaThreadPoolKnobs {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComMtaThreadPoolKnobs";
}
impl IComMtaThreadPoolKnobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComMtaThreadPoolKnobsImpl, const OFFSET: isize>() -> IComMtaThreadPoolKnobsVtbl {
        unsafe extern "system" fn MTASetMaxThreadCount<Impl: IComMtaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MTASetMaxThreadCount(dwmaxthreads) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MTAGetMaxThreadCount<Impl: IComMtaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MTAGetMaxThreadCount(::core::mem::transmute_copy(&pdwmaxthreads)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MTASetThrottleValue<Impl: IComMtaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthrottle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MTASetThrottleValue(dwthrottle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MTAGetThrottleValue<Impl: IComMtaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthrottle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MTAGetThrottleValue(::core::mem::transmute_copy(&pdwthrottle)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComMtaThreadPoolKnobs>, ::windows::core::GetTrustLevel, MTASetMaxThreadCount::<Impl, OFFSET>, MTAGetMaxThreadCount::<Impl, OFFSET>, MTASetThrottleValue::<Impl, OFFSET>, MTAGetThrottleValue::<Impl, OFFSET>)
    }
}
pub trait IComObjectConstruction2EventsImpl: Sized {
    fn OnObjectConstruct2();
}
impl ::windows::core::RuntimeName for IComObjectConstruction2Events {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComObjectConstruction2Events";
}
impl IComObjectConstruction2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectConstruction2EventsImpl, const OFFSET: isize>() -> IComObjectConstruction2EventsVtbl {
        unsafe extern "system" fn OnObjectConstruct2<Impl: IComObjectConstruction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjectConstruct2(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&sconstructstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                oid,
                &*(&guidpartition as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComObjectConstruction2Events>, ::windows::core::GetTrustLevel, OnObjectConstruct2::<Impl, OFFSET>)
    }
}
pub trait IComObjectConstructionEventsImpl: Sized {
    fn OnObjectConstruct();
}
impl ::windows::core::RuntimeName for IComObjectConstructionEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComObjectConstructionEvents";
}
impl IComObjectConstructionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectConstructionEventsImpl, const OFFSET: isize>() -> IComObjectConstructionEventsVtbl {
        unsafe extern "system" fn OnObjectConstruct<Impl: IComObjectConstructionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjectConstruct(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&sconstructstring as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                oid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComObjectConstructionEvents>, ::windows::core::GetTrustLevel, OnObjectConstruct::<Impl, OFFSET>)
    }
}
pub trait IComObjectEventsImpl: Sized {
    fn OnObjectActivate();
    fn OnObjectDeactivate();
    fn OnDisableCommit();
    fn OnEnableCommit();
    fn OnSetComplete();
    fn OnSetAbort();
}
impl ::windows::core::RuntimeName for IComObjectEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComObjectEvents";
}
impl IComObjectEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectEventsImpl, const OFFSET: isize>() -> IComObjectEventsVtbl {
        unsafe extern "system" fn OnObjectActivate<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjectActivate(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), ctxtid, objectid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjectDeactivate<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjectDeactivate(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), ctxtid, objectid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDisableCommit<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDisableCommit(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), ctxtid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEnableCommit<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEnableCommit(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), ctxtid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetComplete<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSetComplete(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), ctxtid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnSetAbort<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSetAbort(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), ctxtid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComObjectEvents>, ::windows::core::GetTrustLevel, OnObjectActivate::<Impl, OFFSET>, OnObjectDeactivate::<Impl, OFFSET>, OnDisableCommit::<Impl, OFFSET>, OnEnableCommit::<Impl, OFFSET>, OnSetComplete::<Impl, OFFSET>, OnSetAbort::<Impl, OFFSET>)
    }
}
pub trait IComObjectPool2EventsImpl: Sized {
    fn OnObjPoolPutObject2();
    fn OnObjPoolGetObject2();
    fn OnObjPoolRecycleToTx2();
    fn OnObjPoolGetFromTx2();
}
impl ::windows::core::RuntimeName for IComObjectPool2Events {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComObjectPool2Events";
}
impl IComObjectPool2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectPool2EventsImpl, const OFFSET: isize>() -> IComObjectPool2EventsVtbl {
        unsafe extern "system" fn OnObjPoolPutObject2<Impl: IComObjectPool2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolPutObject2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), nreason, dwavailable, oid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolGetObject2<Impl: IComObjectPool2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolGetObject2(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                dwavailable,
                oid,
                &*(&guidpartition as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx2<Impl: IComObjectPool2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolRecycleToTx2(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                objid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolGetFromTx2<Impl: IComObjectPool2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolGetFromTx2(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                objid,
                &*(&guidpartition as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComObjectPool2Events>, ::windows::core::GetTrustLevel, OnObjPoolPutObject2::<Impl, OFFSET>, OnObjPoolGetObject2::<Impl, OFFSET>, OnObjPoolRecycleToTx2::<Impl, OFFSET>, OnObjPoolGetFromTx2::<Impl, OFFSET>)
    }
}
pub trait IComObjectPoolEventsImpl: Sized {
    fn OnObjPoolPutObject();
    fn OnObjPoolGetObject();
    fn OnObjPoolRecycleToTx();
    fn OnObjPoolGetFromTx();
}
impl ::windows::core::RuntimeName for IComObjectPoolEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComObjectPoolEvents";
}
impl IComObjectPoolEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectPoolEventsImpl, const OFFSET: isize>() -> IComObjectPoolEventsVtbl {
        unsafe extern "system" fn OnObjPoolPutObject<Impl: IComObjectPoolEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolPutObject(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), nreason, dwavailable, oid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolGetObject<Impl: IComObjectPoolEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolGetObject(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwavailable, oid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx<Impl: IComObjectPoolEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolRecycleToTx(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                objid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolGetFromTx<Impl: IComObjectPoolEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolGetFromTx(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                objid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComObjectPoolEvents>, ::windows::core::GetTrustLevel, OnObjPoolPutObject::<Impl, OFFSET>, OnObjPoolGetObject::<Impl, OFFSET>, OnObjPoolRecycleToTx::<Impl, OFFSET>, OnObjPoolGetFromTx::<Impl, OFFSET>)
    }
}
pub trait IComObjectPoolEvents2Impl: Sized {
    fn OnObjPoolCreateObject();
    fn OnObjPoolDestroyObject();
    fn OnObjPoolCreateDecision();
    fn OnObjPoolTimeout();
    fn OnObjPoolCreatePool();
}
impl ::windows::core::RuntimeName for IComObjectPoolEvents2 {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComObjectPoolEvents2";
}
impl IComObjectPoolEvents2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>() -> IComObjectPoolEvents2Vtbl {
        unsafe extern "system" fn OnObjPoolCreateObject<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolCreateObject(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwobjscreated, oid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolDestroyObject<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolDestroyObject(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwobjscreated, oid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolCreateDecision<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolCreateDecision(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), dwthreadswaiting, dwavail, dwcreated, dwmin, dwmax) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolTimeout<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, guidactivity: *const ::windows::core::GUID, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolTimeout(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwtimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnObjPoolCreatePool<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnObjPoolCreatePool(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidobject as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwmin, dwmax, dwtimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComObjectPoolEvents2>, ::windows::core::GetTrustLevel, OnObjPoolCreateObject::<Impl, OFFSET>, OnObjPoolDestroyObject::<Impl, OFFSET>, OnObjPoolCreateDecision::<Impl, OFFSET>, OnObjPoolTimeout::<Impl, OFFSET>, OnObjPoolCreatePool::<Impl, OFFSET>)
    }
}
pub trait IComQCEventsImpl: Sized {
    fn OnQCRecord();
    fn OnQCQueueOpen();
    fn OnQCReceive();
    fn OnQCReceiveFail();
    fn OnQCMoveToReTryQueue();
    fn OnQCMoveToDeadQueue();
    fn OnQCPlayback();
}
impl ::windows::core::RuntimeName for IComQCEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComQCEvents";
}
impl IComQCEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComQCEventsImpl, const OFFSET: isize>() -> IComQCEventsVtbl {
        unsafe extern "system" fn OnQCRecord<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: super::super::Foundation::PWSTR, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQCRecord(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                objid,
                &*(&szqueue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&guidmsgid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&guidworkflowid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                msmqhr,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnQCQueueOpen<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: super::super::Foundation::PWSTR, queueid: u64, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQCQueueOpen(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&szqueue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), queueid, hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnQCReceive<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQCReceive(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), queueid, &*(&guidmsgid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidworkflowid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnQCReceiveFail<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQCReceiveFail(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), queueid, msmqhr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnQCMoveToReTryQueue<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, retryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQCMoveToReTryQueue(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidmsgid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidworkflowid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), retryindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnQCMoveToDeadQueue<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQCMoveToDeadQueue(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidmsgid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidworkflowid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnQCPlayback<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnQCPlayback(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), objid, &*(&guidmsgid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&guidworkflowid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), hr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComQCEvents>, ::windows::core::GetTrustLevel, OnQCRecord::<Impl, OFFSET>, OnQCQueueOpen::<Impl, OFFSET>, OnQCReceive::<Impl, OFFSET>, OnQCReceiveFail::<Impl, OFFSET>, OnQCMoveToReTryQueue::<Impl, OFFSET>, OnQCMoveToDeadQueue::<Impl, OFFSET>, OnQCPlayback::<Impl, OFFSET>)
    }
}
pub trait IComResourceEventsImpl: Sized {
    fn OnResourceCreate();
    fn OnResourceAllocate();
    fn OnResourceRecycle();
    fn OnResourceDestroy();
    fn OnResourceTrack();
}
impl ::windows::core::RuntimeName for IComResourceEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComResourceEvents";
}
impl IComResourceEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComResourceEventsImpl, const OFFSET: isize>() -> IComResourceEventsVtbl {
        unsafe extern "system" fn OnResourceCreate<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnResourceCreate(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                objectid,
                &*(&psztype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                resid,
                &*(&enlisted as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResourceAllocate<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnResourceAllocate(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                objectid,
                &*(&psztype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                resid,
                &*(&enlisted as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                numrated,
                rating,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResourceRecycle<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnResourceRecycle(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), objectid, &*(&psztype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), resid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResourceDestroy<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::core::HRESULT, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnResourceDestroy(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), objectid, hr, &*(&psztype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), resid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResourceTrack<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnResourceTrack(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                objectid,
                &*(&psztype as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                resid,
                &*(&enlisted as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComResourceEvents>, ::windows::core::GetTrustLevel, OnResourceCreate::<Impl, OFFSET>, OnResourceAllocate::<Impl, OFFSET>, OnResourceRecycle::<Impl, OFFSET>, OnResourceDestroy::<Impl, OFFSET>, OnResourceTrack::<Impl, OFFSET>)
    }
}
pub trait IComSecurityEventsImpl: Sized {
    fn OnAuthenticate();
    fn OnAuthenticateFail();
}
impl ::windows::core::RuntimeName for IComSecurityEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComSecurityEvents";
}
impl IComSecurityEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComSecurityEventsImpl, const OFFSET: isize>() -> IComSecurityEventsVtbl {
        unsafe extern "system" fn OnAuthenticate<Impl: IComSecurityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAuthenticate(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                objectid,
                &*(&guidiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                imeth,
                cbbyteorig,
                psidoriginaluser,
                cbbytecur,
                psidcurrentuser,
                &*(&bcurrentuserinpersonatinginproc as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnAuthenticateFail<Impl: IComSecurityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnAuthenticateFail(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                objectid,
                &*(&guidiid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                imeth,
                cbbyteorig,
                psidoriginaluser,
                cbbytecur,
                psidcurrentuser,
                &*(&bcurrentuserinpersonatinginproc as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComSecurityEvents>, ::windows::core::GetTrustLevel, OnAuthenticate::<Impl, OFFSET>, OnAuthenticateFail::<Impl, OFFSET>)
    }
}
pub trait IComStaThreadPoolKnobsImpl: Sized {
    fn SetMinThreadCount();
    fn GetMinThreadCount();
    fn SetMaxThreadCount();
    fn GetMaxThreadCount();
    fn SetActivityPerThread();
    fn GetActivityPerThread();
    fn SetActivityRatio();
    fn GetActivityRatio();
    fn GetThreadCount();
    fn GetQueueDepth();
    fn SetQueueDepth();
}
impl ::windows::core::RuntimeName for IComStaThreadPoolKnobs {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComStaThreadPoolKnobs";
}
impl IComStaThreadPoolKnobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>() -> IComStaThreadPoolKnobsVtbl {
        unsafe extern "system" fn SetMinThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMinThreadCount(minthreads) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMinThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinThreadCount(::core::mem::transmute_copy(&minthreads)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxThreadCount(maxthreads) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxThreadCount(::core::mem::transmute_copy(&maxthreads)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityPerThread<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActivityPerThread(activitiesperthread) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityPerThread<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivityPerThread(::core::mem::transmute_copy(&activitiesperthread)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityRatio<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActivityRatio(activityratio) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityRatio<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivityRatio(::core::mem::transmute_copy(&activityratio)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThreadCount(::core::mem::transmute_copy(&pdwthreads)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueueDepth<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwqdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQueueDepth(::core::mem::transmute_copy(&pdwqdepth)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueueDepth<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqdepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetQueueDepth(dwqdepth) {
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
            ::windows::core::GetRuntimeClassName::<IComStaThreadPoolKnobs>,
            ::windows::core::GetTrustLevel,
            SetMinThreadCount::<Impl, OFFSET>,
            GetMinThreadCount::<Impl, OFFSET>,
            SetMaxThreadCount::<Impl, OFFSET>,
            GetMaxThreadCount::<Impl, OFFSET>,
            SetActivityPerThread::<Impl, OFFSET>,
            GetActivityPerThread::<Impl, OFFSET>,
            SetActivityRatio::<Impl, OFFSET>,
            GetActivityRatio::<Impl, OFFSET>,
            GetThreadCount::<Impl, OFFSET>,
            GetQueueDepth::<Impl, OFFSET>,
            SetQueueDepth::<Impl, OFFSET>,
        )
    }
}
pub trait IComStaThreadPoolKnobs2Impl: Sized + IComStaThreadPoolKnobsImpl {
    fn GetMaxCPULoad();
    fn SetMaxCPULoad();
    fn GetCPUMetricEnabled();
    fn SetCPUMetricEnabled();
    fn GetCreateThreadsAggressively();
    fn SetCreateThreadsAggressively();
    fn GetMaxCSR();
    fn SetMaxCSR();
    fn GetWaitTimeForThreadCleanup();
    fn SetWaitTimeForThreadCleanup();
}
impl ::windows::core::RuntimeName for IComStaThreadPoolKnobs2 {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComStaThreadPoolKnobs2";
}
impl IComStaThreadPoolKnobs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>() -> IComStaThreadPoolKnobs2Vtbl {
        unsafe extern "system" fn GetMaxCPULoad<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxCPULoad(::core::mem::transmute_copy(&pdwload)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCPULoad<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxCPULoad(pdwload) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCPUMetricEnabled<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCPUMetricEnabled(::core::mem::transmute_copy(&pbmetricenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCPUMetricEnabled<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCPUMetricEnabled(&*(&bmetricenabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCreateThreadsAggressively<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreateThreadsAggressively(::core::mem::transmute_copy(&pbmetricenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateThreadsAggressively<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCreateThreadsAggressively(&*(&bmetricenabled as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxCSR<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcsr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxCSR(::core::mem::transmute_copy(&pdwcsr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCSR<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcsr: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxCSR(dwcsr) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetWaitTimeForThreadCleanup<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWaitTimeForThreadCleanup(::core::mem::transmute_copy(&pdwthreadcleanupwaittime)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaitTimeForThreadCleanup<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadcleanupwaittime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWaitTimeForThreadCleanup(dwthreadcleanupwaittime) {
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
            ::windows::core::GetRuntimeClassName::<IComStaThreadPoolKnobs2>,
            ::windows::core::GetTrustLevel,
            GetMaxCPULoad::<Impl, OFFSET>,
            SetMaxCPULoad::<Impl, OFFSET>,
            GetCPUMetricEnabled::<Impl, OFFSET>,
            SetCPUMetricEnabled::<Impl, OFFSET>,
            GetCreateThreadsAggressively::<Impl, OFFSET>,
            SetCreateThreadsAggressively::<Impl, OFFSET>,
            GetMaxCSR::<Impl, OFFSET>,
            SetMaxCSR::<Impl, OFFSET>,
            GetWaitTimeForThreadCleanup::<Impl, OFFSET>,
            SetWaitTimeForThreadCleanup::<Impl, OFFSET>,
        )
    }
}
pub trait IComThreadEventsImpl: Sized {
    fn OnThreadStart();
    fn OnThreadTerminate();
    fn OnThreadBindToApartment();
    fn OnThreadUnBind();
    fn OnThreadWorkEnque();
    fn OnThreadWorkPrivate();
    fn OnThreadWorkPublic();
    fn OnThreadWorkRedirect();
    fn OnThreadWorkReject();
    fn OnThreadAssignApartment();
    fn OnThreadUnassignApartment();
}
impl ::windows::core::RuntimeName for IComThreadEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComThreadEvents";
}
impl IComThreadEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComThreadEventsImpl, const OFFSET: isize>() -> IComThreadEventsVtbl {
        unsafe extern "system" fn OnThreadStart<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadStart(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), threadid, dwthread, dwtheadcnt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadTerminate<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadTerminate(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), threadid, dwthread, dwtheadcnt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadBindToApartment<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadBindToApartment(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), threadid, aptid, dwactcnt, dwlowcnt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadUnBind<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadUnBind(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), threadid, aptid, dwactcnt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadWorkEnque<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadWorkEnque(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), threadid, msgworkid, queuelen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadWorkPrivate<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadWorkPrivate(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), threadid, msgworkid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadWorkPublic<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadWorkPublic(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), threadid, msgworkid, queuelen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadWorkRedirect<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadWorkRedirect(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), threadid, msgworkid, queuelen, threadnum) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadWorkReject<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadWorkReject(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), threadid, msgworkid, queuelen) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadAssignApartment<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, aptid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadAssignApartment(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidactivity as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), aptid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnThreadUnassignApartment<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnThreadUnassignApartment(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), aptid) {
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
            ::windows::core::GetRuntimeClassName::<IComThreadEvents>,
            ::windows::core::GetTrustLevel,
            OnThreadStart::<Impl, OFFSET>,
            OnThreadTerminate::<Impl, OFFSET>,
            OnThreadBindToApartment::<Impl, OFFSET>,
            OnThreadUnBind::<Impl, OFFSET>,
            OnThreadWorkEnque::<Impl, OFFSET>,
            OnThreadWorkPrivate::<Impl, OFFSET>,
            OnThreadWorkPublic::<Impl, OFFSET>,
            OnThreadWorkRedirect::<Impl, OFFSET>,
            OnThreadWorkReject::<Impl, OFFSET>,
            OnThreadAssignApartment::<Impl, OFFSET>,
            OnThreadUnassignApartment::<Impl, OFFSET>,
        )
    }
}
pub trait IComTrackingInfoCollectionImpl: Sized {
    fn Type();
    fn Count();
    fn Item();
}
impl ::windows::core::RuntimeName for IComTrackingInfoCollection {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComTrackingInfoCollection";
}
impl IComTrackingInfoCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoCollectionImpl, const OFFSET: isize>() -> IComTrackingInfoCollectionVtbl {
        unsafe extern "system" fn Type<Impl: IComTrackingInfoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&ptype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IComTrackingInfoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IComTrackingInfoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(ulindex, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComTrackingInfoCollection>, ::windows::core::GetTrustLevel, Type::<Impl, OFFSET>, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>)
    }
}
pub trait IComTrackingInfoEventsImpl: Sized {
    fn OnNewTrackingInfo();
}
impl ::windows::core::RuntimeName for IComTrackingInfoEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComTrackingInfoEvents";
}
impl IComTrackingInfoEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoEventsImpl, const OFFSET: isize>() -> IComTrackingInfoEventsVtbl {
        unsafe extern "system" fn OnNewTrackingInfo<Impl: IComTrackingInfoEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoplevelcollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnNewTrackingInfo(&*(&ptoplevelcollection as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComTrackingInfoEvents>, ::windows::core::GetTrustLevel, OnNewTrackingInfo::<Impl, OFFSET>)
    }
}
pub trait IComTrackingInfoObjectImpl: Sized {
    fn GetValue();
}
impl ::windows::core::RuntimeName for IComTrackingInfoObject {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComTrackingInfoObject";
}
impl IComTrackingInfoObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoObjectImpl, const OFFSET: isize>() -> IComTrackingInfoObjectVtbl {
        unsafe extern "system" fn GetValue<Impl: IComTrackingInfoObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szpropertyname: super::super::Foundation::PWSTR, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&szpropertyname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComTrackingInfoObject>, ::windows::core::GetTrustLevel, GetValue::<Impl, OFFSET>)
    }
}
pub trait IComTrackingInfoPropertiesImpl: Sized {
    fn PropCount();
    fn GetPropName();
}
impl ::windows::core::RuntimeName for IComTrackingInfoProperties {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComTrackingInfoProperties";
}
impl IComTrackingInfoPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoPropertiesImpl, const OFFSET: isize>() -> IComTrackingInfoPropertiesVtbl {
        unsafe extern "system" fn PropCount<Impl: IComTrackingInfoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropName<Impl: IComTrackingInfoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppszpropname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropName(ulindex, ::core::mem::transmute_copy(&ppszpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComTrackingInfoProperties>, ::windows::core::GetTrustLevel, PropCount::<Impl, OFFSET>, GetPropName::<Impl, OFFSET>)
    }
}
pub trait IComTransaction2EventsImpl: Sized {
    fn OnTransactionStart2();
    fn OnTransactionPrepare2();
    fn OnTransactionAbort2();
    fn OnTransactionCommit2();
}
impl ::windows::core::RuntimeName for IComTransaction2Events {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComTransaction2Events";
}
impl IComTransaction2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTransaction2EventsImpl, const OFFSET: isize>() -> IComTransaction2EventsVtbl {
        unsafe extern "system" fn OnTransactionStart2<Impl: IComTransaction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransactionStart2(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&tsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&froot as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                nisolationlevel,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTransactionPrepare2<Impl: IComTransaction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransactionPrepare2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&fvoteyes as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTransactionAbort2<Impl: IComTransaction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransactionAbort2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTransactionCommit2<Impl: IComTransaction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransactionCommit2(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComTransaction2Events>, ::windows::core::GetTrustLevel, OnTransactionStart2::<Impl, OFFSET>, OnTransactionPrepare2::<Impl, OFFSET>, OnTransactionAbort2::<Impl, OFFSET>, OnTransactionCommit2::<Impl, OFFSET>)
    }
}
pub trait IComTransactionEventsImpl: Sized {
    fn OnTransactionStart();
    fn OnTransactionPrepare();
    fn OnTransactionAbort();
    fn OnTransactionCommit();
}
impl ::windows::core::RuntimeName for IComTransactionEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComTransactionEvents";
}
impl IComTransactionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTransactionEventsImpl, const OFFSET: isize>() -> IComTransactionEventsVtbl {
        unsafe extern "system" fn OnTransactionStart<Impl: IComTransactionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransactionStart(
                &*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType),
                &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&tsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&froot as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTransactionPrepare<Impl: IComTransactionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransactionPrepare(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&fvoteyes as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTransactionAbort<Impl: IComTransactionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransactionAbort(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnTransactionCommit<Impl: IComTransactionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnTransactionCommit(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&guidtx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComTransactionEvents>, ::windows::core::GetTrustLevel, OnTransactionStart::<Impl, OFFSET>, OnTransactionPrepare::<Impl, OFFSET>, OnTransactionAbort::<Impl, OFFSET>, OnTransactionCommit::<Impl, OFFSET>)
    }
}
pub trait IComUserEventImpl: Sized {
    fn OnUserEvent();
}
impl ::windows::core::RuntimeName for IComUserEvent {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IComUserEvent";
}
impl IComUserEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComUserEventImpl, const OFFSET: isize>() -> IComUserEventVtbl {
        unsafe extern "system" fn OnUserEvent<Impl: IComUserEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUserEvent(&*(&pinfo as *const <COMSVCSEVENTINFO as ::windows::core::Abi>::Abi as *const <COMSVCSEVENTINFO as ::windows::core::DefaultType>::DefaultType), &*(&pvarevent as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComUserEvent>, ::windows::core::GetTrustLevel, OnUserEvent::<Impl, OFFSET>)
    }
}
pub trait IContextPropertiesImpl: Sized {
    fn Count();
    fn GetProperty();
    fn EnumNames();
    fn SetProperty();
    fn RemoveProperty();
}
impl ::windows::core::RuntimeName for IContextProperties {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IContextProperties";
}
impl IContextPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextPropertiesImpl, const OFFSET: isize>() -> IContextPropertiesVtbl {
        unsafe extern "system" fn Count<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pproperty as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumNames<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumNames(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&property as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProperty<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveProperty(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContextProperties>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>, EnumNames::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>, RemoveProperty::<Impl, OFFSET>)
    }
}
pub trait IContextSecurityPerimeterImpl: Sized {
    fn GetPerimeterFlag();
    fn SetPerimeterFlag();
}
impl ::windows::core::RuntimeName for IContextSecurityPerimeter {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IContextSecurityPerimeter";
}
impl IContextSecurityPerimeterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextSecurityPerimeterImpl, const OFFSET: isize>() -> IContextSecurityPerimeterVtbl {
        unsafe extern "system" fn GetPerimeterFlag<Impl: IContextSecurityPerimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflag: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPerimeterFlag(&*(&pflag as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerimeterFlag<Impl: IContextSecurityPerimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPerimeterFlag(&*(&fflag as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContextSecurityPerimeter>, ::windows::core::GetTrustLevel, GetPerimeterFlag::<Impl, OFFSET>, SetPerimeterFlag::<Impl, OFFSET>)
    }
}
pub trait IContextStateImpl: Sized {
    fn SetDeactivateOnReturn();
    fn GetDeactivateOnReturn();
    fn SetMyTransactionVote();
    fn GetMyTransactionVote();
}
impl ::windows::core::RuntimeName for IContextState {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IContextState";
}
impl IContextStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextStateImpl, const OFFSET: isize>() -> IContextStateVtbl {
        unsafe extern "system" fn SetDeactivateOnReturn<Impl: IContextStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdeactivate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDeactivateOnReturn(bdeactivate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeactivateOnReturn<Impl: IContextStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeactivate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeactivateOnReturn(pbdeactivate) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMyTransactionVote<Impl: IContextStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, txvote: TransactionVote) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMyTransactionVote(txvote) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMyTransactionVote<Impl: IContextStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxvote: *mut TransactionVote) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMyTransactionVote(ptxvote) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IContextState>, ::windows::core::GetTrustLevel, SetDeactivateOnReturn::<Impl, OFFSET>, GetDeactivateOnReturn::<Impl, OFFSET>, SetMyTransactionVote::<Impl, OFFSET>, GetMyTransactionVote::<Impl, OFFSET>)
    }
}
pub trait ICreateWithLocalTransactionImpl: Sized {
    fn CreateInstanceWithSysTx();
}
impl ::windows::core::RuntimeName for ICreateWithLocalTransaction {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICreateWithLocalTransaction";
}
impl ICreateWithLocalTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateWithLocalTransactionImpl, const OFFSET: isize>() -> ICreateWithLocalTransactionVtbl {
        unsafe extern "system" fn CreateInstanceWithSysTx<Impl: ICreateWithLocalTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceWithSysTx(
                &*(&ptransaction as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&pobject as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICreateWithLocalTransaction>, ::windows::core::GetTrustLevel, CreateInstanceWithSysTx::<Impl, OFFSET>)
    }
}
pub trait ICreateWithTipTransactionExImpl: Sized {
    fn CreateInstance();
}
impl ::windows::core::RuntimeName for ICreateWithTipTransactionEx {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICreateWithTipTransactionEx";
}
impl ICreateWithTipTransactionExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateWithTipTransactionExImpl, const OFFSET: isize>() -> ICreateWithTipTransactionExVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICreateWithTipTransactionExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtipurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(
                &*(&bstrtipurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pobject),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICreateWithTipTransactionEx>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
pub trait ICreateWithTransactionExImpl: Sized {
    fn CreateInstance();
}
impl ::windows::core::RuntimeName for ICreateWithTransactionEx {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICreateWithTransactionEx";
}
impl ICreateWithTransactionExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateWithTransactionExImpl, const OFFSET: isize>() -> ICreateWithTransactionExVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICreateWithTransactionExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(
                &*(&ptransaction as *const <super::DistributedTransactionCoordinator::ITransaction as ::windows::core::Abi>::Abi as *const <super::DistributedTransactionCoordinator::ITransaction as ::windows::core::DefaultType>::DefaultType),
                &*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pobject),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICreateWithTransactionEx>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>)
    }
}
pub trait ICrmCompensatorImpl: Sized {
    fn SetLogControl();
    fn BeginPrepare();
    fn PrepareRecord();
    fn EndPrepare();
    fn BeginCommit();
    fn CommitRecord();
    fn EndCommit();
    fn BeginAbort();
    fn AbortRecord();
    fn EndAbort();
}
impl ::windows::core::RuntimeName for ICrmCompensator {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICrmCompensator";
}
impl ICrmCompensatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmCompensatorImpl, const OFFSET: isize>() -> ICrmCompensatorVtbl {
        unsafe extern "system" fn SetLogControl<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLogControl(&*(&plogcontrol as *const <ICrmLogControl as ::windows::core::Abi>::Abi as *const <ICrmLogControl as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginPrepare<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginPrepare() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareRecord<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareRecord(&*(&crmlogrec as *const <CrmLogRecordRead as ::windows::core::Abi>::Abi as *const <CrmLogRecordRead as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfforget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepare<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfoktoprepare: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPrepare(::core::mem::transmute_copy(&pfoktoprepare)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommit<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginCommit(&*(&frecovery as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitRecord<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitRecord(&*(&crmlogrec as *const <CrmLogRecordRead as ::windows::core::Abi>::Abi as *const <CrmLogRecordRead as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfforget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommit<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginAbort<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginAbort(&*(&frecovery as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortRecord<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortRecord(&*(&crmlogrec as *const <CrmLogRecordRead as ::windows::core::Abi>::Abi as *const <CrmLogRecordRead as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfforget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbort<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndAbort() {
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
            ::windows::core::GetRuntimeClassName::<ICrmCompensator>,
            ::windows::core::GetTrustLevel,
            SetLogControl::<Impl, OFFSET>,
            BeginPrepare::<Impl, OFFSET>,
            PrepareRecord::<Impl, OFFSET>,
            EndPrepare::<Impl, OFFSET>,
            BeginCommit::<Impl, OFFSET>,
            CommitRecord::<Impl, OFFSET>,
            EndCommit::<Impl, OFFSET>,
            BeginAbort::<Impl, OFFSET>,
            AbortRecord::<Impl, OFFSET>,
            EndAbort::<Impl, OFFSET>,
        )
    }
}
pub trait ICrmCompensatorVariantsImpl: Sized {
    fn SetLogControlVariants();
    fn BeginPrepareVariants();
    fn PrepareRecordVariants();
    fn EndPrepareVariants();
    fn BeginCommitVariants();
    fn CommitRecordVariants();
    fn EndCommitVariants();
    fn BeginAbortVariants();
    fn AbortRecordVariants();
    fn EndAbortVariants();
}
impl ::windows::core::RuntimeName for ICrmCompensatorVariants {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICrmCompensatorVariants";
}
impl ICrmCompensatorVariantsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>() -> ICrmCompensatorVariantsVtbl {
        unsafe extern "system" fn SetLogControlVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLogControlVariants(&*(&plogcontrol as *const <ICrmLogControl as ::windows::core::Abi>::Abi as *const <ICrmLogControl as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginPrepareVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginPrepareVariants() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PrepareRecordVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareRecordVariants(&*(&plogrecord as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbforget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepareVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboktoprepare: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPrepareVariants(::core::mem::transmute_copy(&pboktoprepare)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommitVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginCommitVariants(brecovery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitRecordVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitRecordVariants(&*(&plogrecord as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbforget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommitVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndCommitVariants() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginAbortVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BeginAbortVariants(brecovery) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AbortRecordVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortRecordVariants(&*(&plogrecord as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbforget)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbortVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndAbortVariants() {
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
            ::windows::core::GetRuntimeClassName::<ICrmCompensatorVariants>,
            ::windows::core::GetTrustLevel,
            SetLogControlVariants::<Impl, OFFSET>,
            BeginPrepareVariants::<Impl, OFFSET>,
            PrepareRecordVariants::<Impl, OFFSET>,
            EndPrepareVariants::<Impl, OFFSET>,
            BeginCommitVariants::<Impl, OFFSET>,
            CommitRecordVariants::<Impl, OFFSET>,
            EndCommitVariants::<Impl, OFFSET>,
            BeginAbortVariants::<Impl, OFFSET>,
            AbortRecordVariants::<Impl, OFFSET>,
            EndAbortVariants::<Impl, OFFSET>,
        )
    }
}
pub trait ICrmFormatLogRecordsImpl: Sized {
    fn GetColumnCount();
    fn GetColumnHeaders();
    fn GetColumn();
    fn GetColumnVariants();
}
impl ::windows::core::RuntimeName for ICrmFormatLogRecords {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICrmFormatLogRecords";
}
impl ICrmFormatLogRecordsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmFormatLogRecordsImpl, const OFFSET: isize>() -> ICrmFormatLogRecordsVtbl {
        unsafe extern "system" fn GetColumnCount<Impl: ICrmFormatLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcolumncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnCount(::core::mem::transmute_copy(&plcolumncount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Impl: ICrmFormatLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheaders: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnHeaders(::core::mem::transmute_copy(&pheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Impl: ICrmFormatLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumn(&*(&crmlogrec as *const <CrmLogRecordRead as ::windows::core::Abi>::Abi as *const <CrmLogRecordRead as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pformattedlogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnVariants<Impl: ICrmFormatLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logrecord: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnVariants(&*(&logrecord as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pformattedlogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICrmFormatLogRecords>, ::windows::core::GetTrustLevel, GetColumnCount::<Impl, OFFSET>, GetColumnHeaders::<Impl, OFFSET>, GetColumn::<Impl, OFFSET>, GetColumnVariants::<Impl, OFFSET>)
    }
}
pub trait ICrmLogControlImpl: Sized {
    fn TransactionUOW();
    fn RegisterCompensator();
    fn WriteLogRecordVariants();
    fn ForceLog();
    fn ForgetLogRecord();
    fn ForceTransactionToAbort();
    fn WriteLogRecord();
}
impl ::windows::core::RuntimeName for ICrmLogControl {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICrmLogControl";
}
impl ICrmLogControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmLogControlImpl, const OFFSET: isize>() -> ICrmLogControlVtbl {
        unsafe extern "system" fn TransactionUOW<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionUOW(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCompensator<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcwstrprogidcompensator: super::super::Foundation::PWSTR, lpcwstrdescription: super::super::Foundation::PWSTR, lcrmregflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterCompensator(&*(&lpcwstrprogidcompensator as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&lpcwstrdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), lcrmregflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteLogRecordVariants<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteLogRecordVariants(&*(&plogrecord as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceLog<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceLog() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForgetLogRecord<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForgetLogRecord() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceTransactionToAbort<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceTransactionToAbort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteLogRecord<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteLogRecord(&*(&rgblob as *const <super::Com::BLOB as ::windows::core::Abi>::Abi as *const <super::Com::BLOB as ::windows::core::DefaultType>::DefaultType), cblob) {
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
            ::windows::core::GetRuntimeClassName::<ICrmLogControl>,
            ::windows::core::GetTrustLevel,
            TransactionUOW::<Impl, OFFSET>,
            RegisterCompensator::<Impl, OFFSET>,
            WriteLogRecordVariants::<Impl, OFFSET>,
            ForceLog::<Impl, OFFSET>,
            ForgetLogRecord::<Impl, OFFSET>,
            ForceTransactionToAbort::<Impl, OFFSET>,
            WriteLogRecord::<Impl, OFFSET>,
        )
    }
}
pub trait ICrmMonitorImpl: Sized {
    fn GetClerks();
    fn HoldClerk();
}
impl ::windows::core::RuntimeName for ICrmMonitor {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICrmMonitor";
}
impl ICrmMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmMonitorImpl, const OFFSET: isize>() -> ICrmMonitorVtbl {
        unsafe extern "system" fn GetClerks<Impl: ICrmMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclerks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClerks(::core::mem::transmute_copy(&pclerks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldClerk<Impl: ICrmMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldClerk(&*(&index as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICrmMonitor>, ::windows::core::GetTrustLevel, GetClerks::<Impl, OFFSET>, HoldClerk::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICrmMonitorClerksImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
    fn ProgIdCompensator();
    fn Description();
    fn TransactionUOW();
    fn ActivityId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ICrmMonitorClerks {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICrmMonitorClerks";
}
#[cfg(feature = "Win32_System_Com")]
impl ICrmMonitorClerksVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmMonitorClerksImpl, const OFFSET: isize>() -> ICrmMonitorClerksVtbl {
        unsafe extern "system" fn Item<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&index as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgIdCompensator<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgIdCompensator(&*(&index as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(&*(&index as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionUOW<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionUOW(&*(&index as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId(&*(&index as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICrmMonitorClerks>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, Count::<Impl, OFFSET>, ProgIdCompensator::<Impl, OFFSET>, Description::<Impl, OFFSET>, TransactionUOW::<Impl, OFFSET>, ActivityId::<Impl, OFFSET>)
    }
}
pub trait ICrmMonitorLogRecordsImpl: Sized {
    fn Count();
    fn TransactionState();
    fn StructuredRecords();
    fn GetLogRecord();
    fn GetLogRecordVariants();
}
impl ::windows::core::RuntimeName for ICrmMonitorLogRecords {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ICrmMonitorLogRecords";
}
impl ICrmMonitorLogRecordsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>() -> ICrmMonitorLogRecordsVtbl {
        unsafe extern "system" fn Count<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionState<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut CrmTransactionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionState(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StructuredRecords<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StructuredRecords(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogRecord<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogRecord(dwindex, &*(&pcrmlogrec as *const <CrmLogRecordRead as ::windows::core::Abi>::Abi as *const <CrmLogRecordRead as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogRecordVariants<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexnumber: ::core::mem::ManuallyDrop<super::Com::VARIANT>, plogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogRecordVariants(&*(&indexnumber as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ICrmMonitorLogRecords>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, TransactionState::<Impl, OFFSET>, StructuredRecords::<Impl, OFFSET>, GetLogRecord::<Impl, OFFSET>, GetLogRecordVariants::<Impl, OFFSET>)
    }
}
pub trait IDispenserDriverImpl: Sized {
    fn CreateResource();
    fn RateResource();
    fn EnlistResource();
    fn ResetResource();
    fn DestroyResource();
    fn DestroyResourceS();
}
impl ::windows::core::RuntimeName for IDispenserDriver {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IDispenserDriver";
}
impl IDispenserDriverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispenserDriverImpl, const OFFSET: isize>() -> IDispenserDriverVtbl {
        unsafe extern "system" fn CreateResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResource(restypid, presid, psecsfreebeforedestroy) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RateResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RateResource(restypid, resid, &*(&frequirestransactionenlistment as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), prating) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnlistResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize, transid: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnlistResource(resid, transid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResetResource(resid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyResource(resid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyResourceS<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyResourceS(resid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispenserDriver>, ::windows::core::GetTrustLevel, CreateResource::<Impl, OFFSET>, RateResource::<Impl, OFFSET>, EnlistResource::<Impl, OFFSET>, ResetResource::<Impl, OFFSET>, DestroyResource::<Impl, OFFSET>, DestroyResourceS::<Impl, OFFSET>)
    }
}
pub trait IDispenserManagerImpl: Sized {
    fn RegisterDispenser();
    fn GetContext();
}
impl ::windows::core::RuntimeName for IDispenserManager {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IDispenserManager";
}
impl IDispenserManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispenserManagerImpl, const OFFSET: isize>() -> IDispenserManagerVtbl {
        unsafe extern "system" fn RegisterDispenser<Impl: IDispenserManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0000: ::windows::core::RawPtr, szdispensername: super::super::Foundation::PWSTR, __midl__idispensermanager0001: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDispenser(&*(&__midl__idispensermanager0000 as *const <IDispenserDriver as ::windows::core::Abi>::Abi as *const <IDispenserDriver as ::windows::core::DefaultType>::DefaultType), &*(&szdispensername as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&__midl__idispensermanager0001)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: IDispenserManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContext(__midl__idispensermanager0002, __midl__idispensermanager0003) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDispenserManager>, ::windows::core::GetTrustLevel, RegisterDispenser::<Impl, OFFSET>, GetContext::<Impl, OFFSET>)
    }
}
pub trait IEnumNamesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumNames {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IEnumNames";
}
impl IEnumNamesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNamesImpl, const OFFSET: isize>() -> IEnumNamesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, &*(&rgname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), pceltfetched) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumNames>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEventServerTraceImpl: Sized + IDispatchImpl {
    fn StartTraceGuid();
    fn StopTraceGuid();
    fn EnumTraceGuid();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IEventServerTrace {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IEventServerTrace";
}
#[cfg(feature = "Win32_System_Com")]
impl IEventServerTraceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventServerTraceImpl, const OFFSET: isize>() -> IEventServerTraceVtbl {
        unsafe extern "system" fn StartTraceGuid<Impl: IEventServerTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartTraceGuid(&*(&bstrguidevent as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrguidfilter as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lpidfilter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StopTraceGuid<Impl: IEventServerTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StopTraceGuid(&*(&bstrguidevent as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrguidfilter as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lpidfilter) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumTraceGuid<Impl: IEventServerTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumTraceGuid(::core::mem::transmute_copy(&plcntguids), ::core::mem::transmute_copy(&pbstrguidlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEventServerTrace>, ::windows::core::GetTrustLevel, StartTraceGuid::<Impl, OFFSET>, StopTraceGuid::<Impl, OFFSET>, EnumTraceGuid::<Impl, OFFSET>)
    }
}
pub trait IGetAppTrackerDataImpl: Sized {
    fn GetApplicationProcesses();
    fn GetApplicationProcessDetails();
    fn GetApplicationsInProcess();
    fn GetComponentsInProcess();
    fn GetComponentDetails();
    fn GetTrackerDataAsCollectionObject();
    fn GetSuggestedPollingInterval();
}
impl ::windows::core::RuntimeName for IGetAppTrackerData {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IGetAppTrackerData";
}
impl IGetAppTrackerDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetAppTrackerDataImpl, const OFFSET: isize>() -> IGetAppTrackerDataVtbl {
        unsafe extern "system" fn GetApplicationProcesses<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationProcesses(&*(&partitionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&applicationid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&numapplicationprocesses), ::core::mem::transmute_copy(&applicationprocesses)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationProcessDetails<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationProcessDetails(&*(&applicationinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), processid, flags, ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&recycleinfo), ::core::mem::transmute_copy(&anycomponentshangmonitored)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationsInProcess<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationsInProcess(&*(&applicationinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), processid, &*(&partitionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&numapplicationsinprocess), ::core::mem::transmute_copy(&applications)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentsInProcess<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentsInProcess(
                &*(&applicationinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                processid,
                &*(&partitionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&applicationid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                flags,
                ::core::mem::transmute_copy(&numcomponentsinprocess),
                ::core::mem::transmute_copy(&components),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentDetails<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, clsid: *const ::windows::core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentDetails(&*(&applicationinstanceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), processid, &*(&clsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&hangmonitorinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrackerDataAsCollectionObject<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toplevelcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTrackerDataAsCollectionObject(::core::mem::transmute_copy(&toplevelcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSuggestedPollingInterval<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pollingintervalinseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSuggestedPollingInterval(::core::mem::transmute_copy(&pollingintervalinseconds)) {
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
            ::windows::core::GetRuntimeClassName::<IGetAppTrackerData>,
            ::windows::core::GetTrustLevel,
            GetApplicationProcesses::<Impl, OFFSET>,
            GetApplicationProcessDetails::<Impl, OFFSET>,
            GetApplicationsInProcess::<Impl, OFFSET>,
            GetComponentsInProcess::<Impl, OFFSET>,
            GetComponentDetails::<Impl, OFFSET>,
            GetTrackerDataAsCollectionObject::<Impl, OFFSET>,
            GetSuggestedPollingInterval::<Impl, OFFSET>,
        )
    }
}
pub trait IGetContextPropertiesImpl: Sized {
    fn Count();
    fn GetProperty();
    fn EnumNames();
}
impl ::windows::core::RuntimeName for IGetContextProperties {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IGetContextProperties";
}
impl IGetContextPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetContextPropertiesImpl, const OFFSET: isize>() -> IGetContextPropertiesVtbl {
        unsafe extern "system" fn Count<Impl: IGetContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(plcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IGetContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pproperty as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumNames<Impl: IGetContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumNames(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGetContextProperties>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, GetProperty::<Impl, OFFSET>, EnumNames::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IGetSecurityCallContextImpl: Sized + IDispatchImpl {
    fn GetSecurityCallContext();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IGetSecurityCallContext {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IGetSecurityCallContext";
}
#[cfg(feature = "Win32_System_Com")]
impl IGetSecurityCallContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetSecurityCallContextImpl, const OFFSET: isize>() -> IGetSecurityCallContextVtbl {
        unsafe extern "system" fn GetSecurityCallContext<Impl: IGetSecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityCallContext(::core::mem::transmute_copy(&ppobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IGetSecurityCallContext>, ::windows::core::GetTrustLevel, GetSecurityCallContext::<Impl, OFFSET>)
    }
}
pub trait IHolderImpl: Sized {
    fn AllocResource();
    fn FreeResource();
    fn TrackResource();
    fn TrackResourceS();
    fn UntrackResource();
    fn UntrackResourceS();
    fn Close();
    fn RequestDestroyResource();
}
impl ::windows::core::RuntimeName for IHolder {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IHolder";
}
impl IHolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolderImpl, const OFFSET: isize>() -> IHolderVtbl {
        unsafe extern "system" fn AllocResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllocResource(__midl__iholder0000, __midl__iholder0001) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0002: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeResource(__midl__iholder0002) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0003: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackResource(__midl__iholder0003) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackResourceS<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0004: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackResourceS(__midl__iholder0004) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UntrackResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UntrackResource(__midl__iholder0005, &*(&__midl__iholder0006 as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UntrackResourceS<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UntrackResourceS(__midl__iholder0007, &*(&__midl__iholder0008 as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn RequestDestroyResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0009: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestDestroyResource(__midl__iholder0009) {
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
            ::windows::core::GetRuntimeClassName::<IHolder>,
            ::windows::core::GetTrustLevel,
            AllocResource::<Impl, OFFSET>,
            FreeResource::<Impl, OFFSET>,
            TrackResource::<Impl, OFFSET>,
            TrackResourceS::<Impl, OFFSET>,
            UntrackResource::<Impl, OFFSET>,
            UntrackResourceS::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
            RequestDestroyResource::<Impl, OFFSET>,
        )
    }
}
pub trait ILBEventsImpl: Sized {
    fn TargetUp();
    fn TargetDown();
    fn EngineDefined();
}
impl ::windows::core::RuntimeName for ILBEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ILBEvents";
}
impl ILBEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILBEventsImpl, const OFFSET: isize>() -> ILBEventsVtbl {
        unsafe extern "system" fn TargetUp<Impl: ILBEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetUp(&*(&bstrservername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrclsideng as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetDown<Impl: ILBEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetDown(&*(&bstrservername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrclsideng as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EngineDefined<Impl: ILBEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varpropvalue: *const super::Com::VARIANT, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EngineDefined(
                &*(&bstrpropname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&varpropvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrclsideng as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILBEvents>, ::windows::core::GetTrustLevel, TargetUp::<Impl, OFFSET>, TargetDown::<Impl, OFFSET>, EngineDefined::<Impl, OFFSET>)
    }
}
pub trait IMTSActivityImpl: Sized {
    fn SynchronousCall();
    fn AsyncCall();
    fn Reserved1();
    fn BindToCurrentThread();
    fn UnbindFromThread();
}
impl ::windows::core::RuntimeName for IMTSActivity {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IMTSActivity";
}
impl IMTSActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMTSActivityImpl, const OFFSET: isize>() -> IMTSActivityVtbl {
        unsafe extern "system" fn SynchronousCall<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SynchronousCall(&*(&pcall as *const <IMTSCall as ::windows::core::Abi>::Abi as *const <IMTSCall as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsyncCall<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsyncCall(&*(&pcall as *const <IMTSCall as ::windows::core::Abi>::Abi as *const <IMTSCall as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved1<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved1().into()
        }
        unsafe extern "system" fn BindToCurrentThread<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindToCurrentThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnbindFromThread<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnbindFromThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMTSActivity>, ::windows::core::GetTrustLevel, SynchronousCall::<Impl, OFFSET>, AsyncCall::<Impl, OFFSET>, Reserved1::<Impl, OFFSET>, BindToCurrentThread::<Impl, OFFSET>, UnbindFromThread::<Impl, OFFSET>)
    }
}
pub trait IMTSCallImpl: Sized {
    fn OnCall();
}
impl ::windows::core::RuntimeName for IMTSCall {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IMTSCall";
}
impl IMTSCallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMTSCallImpl, const OFFSET: isize>() -> IMTSCallVtbl {
        unsafe extern "system" fn OnCall<Impl: IMTSCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMTSCall>, ::windows::core::GetTrustLevel, OnCall::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMTSLocatorImpl: Sized + IDispatchImpl {
    fn GetEventDispatcher();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMTSLocator {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IMTSLocator";
}
#[cfg(feature = "Win32_System_Com")]
impl IMTSLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMTSLocatorImpl, const OFFSET: isize>() -> IMTSLocatorVtbl {
        unsafe extern "system" fn GetEventDispatcher<Impl: IMTSLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventDispatcher(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMTSLocator>, ::windows::core::GetTrustLevel, GetEventDispatcher::<Impl, OFFSET>)
    }
}
pub trait IManagedActivationEventsImpl: Sized {
    fn CreateManagedStub();
    fn DestroyManagedStub();
}
impl ::windows::core::RuntimeName for IManagedActivationEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IManagedActivationEvents";
}
impl IManagedActivationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedActivationEventsImpl, const OFFSET: isize>() -> IManagedActivationEventsVtbl {
        unsafe extern "system" fn CreateManagedStub<Impl: IManagedActivationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, fdist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateManagedStub(&*(&pinfo as *const <IManagedObjectInfo as ::windows::core::Abi>::Abi as *const <IManagedObjectInfo as ::windows::core::DefaultType>::DefaultType), &*(&fdist as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestroyManagedStub<Impl: IManagedActivationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestroyManagedStub(&*(&pinfo as *const <IManagedObjectInfo as ::windows::core::Abi>::Abi as *const <IManagedObjectInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IManagedActivationEvents>, ::windows::core::GetTrustLevel, CreateManagedStub::<Impl, OFFSET>, DestroyManagedStub::<Impl, OFFSET>)
    }
}
pub trait IManagedObjectInfoImpl: Sized {
    fn GetIUnknown();
    fn GetIObjectControl();
    fn SetInPool();
    fn SetWrapperStrength();
}
impl ::windows::core::RuntimeName for IManagedObjectInfo {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IManagedObjectInfo";
}
impl IManagedObjectInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedObjectInfoImpl, const OFFSET: isize>() -> IManagedObjectInfoVtbl {
        unsafe extern "system" fn GetIUnknown<Impl: IManagedObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIUnknown(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIObjectControl<Impl: IManagedObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctrl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIObjectControl(::core::mem::transmute_copy(&pctrl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPool<Impl: IManagedObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binpool: super::super::Foundation::BOOL, ppooledobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInPool(&*(&binpool as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType), &*(&ppooledobj as *const <IManagedPooledObj as ::windows::core::Abi>::Abi as *const <IManagedPooledObj as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWrapperStrength<Impl: IManagedObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrong: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWrapperStrength(&*(&bstrong as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IManagedObjectInfo>, ::windows::core::GetTrustLevel, GetIUnknown::<Impl, OFFSET>, GetIObjectControl::<Impl, OFFSET>, SetInPool::<Impl, OFFSET>, SetWrapperStrength::<Impl, OFFSET>)
    }
}
pub trait IManagedPoolActionImpl: Sized {
    fn LastRelease();
}
impl ::windows::core::RuntimeName for IManagedPoolAction {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IManagedPoolAction";
}
impl IManagedPoolActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedPoolActionImpl, const OFFSET: isize>() -> IManagedPoolActionVtbl {
        unsafe extern "system" fn LastRelease<Impl: IManagedPoolActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastRelease() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IManagedPoolAction>, ::windows::core::GetTrustLevel, LastRelease::<Impl, OFFSET>)
    }
}
pub trait IManagedPooledObjImpl: Sized {
    fn SetHeld();
}
impl ::windows::core::RuntimeName for IManagedPooledObj {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IManagedPooledObj";
}
impl IManagedPooledObjVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedPooledObjImpl, const OFFSET: isize>() -> IManagedPooledObjVtbl {
        unsafe extern "system" fn SetHeld<Impl: IManagedPooledObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, m_bheld: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetHeld(&*(&m_bheld as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IManagedPooledObj>, ::windows::core::GetTrustLevel, SetHeld::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMessageMoverImpl: Sized + IDispatchImpl {
    fn SourcePath();
    fn SetSourcePath();
    fn DestPath();
    fn SetDestPath();
    fn CommitBatchSize();
    fn SetCommitBatchSize();
    fn MoveMessages();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMessageMover {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IMessageMover";
}
#[cfg(feature = "Win32_System_Com")]
impl IMessageMoverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageMoverImpl, const OFFSET: isize>() -> IMessageMoverVtbl {
        unsafe extern "system" fn SourcePath<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePath(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourcePath<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSourcePath(&*(&newval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DestPath<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestPath(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestPath<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDestPath(&*(&newval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommitBatchSize<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitBatchSize(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitBatchSize<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCommitBatchSize(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveMessages<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagesmoved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveMessages(::core::mem::transmute_copy(&plmessagesmoved)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMessageMover>, ::windows::core::GetTrustLevel, SourcePath::<Impl, OFFSET>, SetSourcePath::<Impl, OFFSET>, DestPath::<Impl, OFFSET>, SetDestPath::<Impl, OFFSET>, CommitBatchSize::<Impl, OFFSET>, SetCommitBatchSize::<Impl, OFFSET>, MoveMessages::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMtsEventInfoImpl: Sized + IDispatchImpl {
    fn Names();
    fn DisplayName();
    fn EventID();
    fn Count();
    fn Value();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMtsEventInfo {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IMtsEventInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl IMtsEventInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMtsEventInfoImpl, const OFFSET: isize>() -> IMtsEventInfoVtbl {
        unsafe extern "system" fn Names<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Names(::core::mem::transmute_copy(&punk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName(::core::mem::transmute_copy(&sdisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventID<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sguideventid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventID(::core::mem::transmute_copy(&sguideventid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&lcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(&*(&skey as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMtsEventInfo>, ::windows::core::GetTrustLevel, Names::<Impl, OFFSET>, DisplayName::<Impl, OFFSET>, EventID::<Impl, OFFSET>, Count::<Impl, OFFSET>, Value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMtsEventsImpl: Sized + IDispatchImpl {
    fn PackageName();
    fn PackageGuid();
    fn PostEvent();
    fn FireEvents();
    fn GetProcessID();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMtsEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IMtsEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl IMtsEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMtsEventsImpl, const OFFSET: isize>() -> IMtsEventsVtbl {
        unsafe extern "system" fn PackageName<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageGuid<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageGuid(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostEvent<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostEvent(&*(&vevent as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FireEvents<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FireEvents(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessID<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProcessID(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMtsEvents>, ::windows::core::GetTrustLevel, PackageName::<Impl, OFFSET>, PackageGuid::<Impl, OFFSET>, PostEvent::<Impl, OFFSET>, FireEvents::<Impl, OFFSET>, GetProcessID::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMtsGrpImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn Refresh();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMtsGrp {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IMtsGrp";
}
#[cfg(feature = "Win32_System_Com")]
impl IMtsGrpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMtsGrpImpl, const OFFSET: isize>() -> IMtsGrpVtbl {
        unsafe extern "system" fn Count<Impl: IMtsGrpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IMtsGrpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(lindex, ::core::mem::transmute_copy(&ppunkdispatcher)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMtsGrpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Refresh() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMtsGrp>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, Refresh::<Impl, OFFSET>)
    }
}
pub trait IObjPoolImpl: Sized {
    fn Reserved1();
    fn Reserved2();
    fn Reserved3();
    fn Reserved4();
    fn PutEndTx();
    fn Reserved5();
    fn Reserved6();
}
impl ::windows::core::RuntimeName for IObjPool {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IObjPool";
}
impl IObjPoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjPoolImpl, const OFFSET: isize>() -> IObjPoolVtbl {
        unsafe extern "system" fn Reserved1<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved1().into()
        }
        unsafe extern "system" fn Reserved2<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved2().into()
        }
        unsafe extern "system" fn Reserved3<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved3().into()
        }
        unsafe extern "system" fn Reserved4<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved4().into()
        }
        unsafe extern "system" fn PutEndTx<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobj: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutEndTx(&*(&pobj as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn Reserved5<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved5().into()
        }
        unsafe extern "system" fn Reserved6<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved6().into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjPool>, ::windows::core::GetTrustLevel, Reserved1::<Impl, OFFSET>, Reserved2::<Impl, OFFSET>, Reserved3::<Impl, OFFSET>, Reserved4::<Impl, OFFSET>, PutEndTx::<Impl, OFFSET>, Reserved5::<Impl, OFFSET>, Reserved6::<Impl, OFFSET>)
    }
}
pub trait IObjectConstructImpl: Sized {
    fn Construct();
}
impl ::windows::core::RuntimeName for IObjectConstruct {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IObjectConstruct";
}
impl IObjectConstructVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectConstructImpl, const OFFSET: isize>() -> IObjectConstructVtbl {
        unsafe extern "system" fn Construct<Impl: IObjectConstructImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctorobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Construct(&*(&pctorobj as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectConstruct>, ::windows::core::GetTrustLevel, Construct::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstructStringImpl: Sized + IDispatchImpl {
    fn ConstructString();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IObjectConstructString {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IObjectConstructString";
}
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstructStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectConstructStringImpl, const OFFSET: isize>() -> IObjectConstructStringVtbl {
        unsafe extern "system" fn ConstructString<Impl: IObjectConstructStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConstructString(&*(&pval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectConstructString>, ::windows::core::GetTrustLevel, ConstructString::<Impl, OFFSET>)
    }
}
pub trait IObjectContextImpl: Sized {
    fn CreateInstance();
    fn SetComplete();
    fn SetAbort();
    fn EnableCommit();
    fn DisableCommit();
    fn IsInTransaction();
    fn IsSecurityEnabled();
    fn IsCallerInRole();
}
impl ::windows::core::RuntimeName for IObjectContext {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IObjectContext";
}
impl IObjectContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextImpl, const OFFSET: isize>() -> IObjectContextVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComplete<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAbort<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAbort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableCommit<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableCommit<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInTransaction<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecurityEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCallerInRole(&*(&bstrrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pfisinrole as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<IObjectContext>,
            ::windows::core::GetTrustLevel,
            CreateInstance::<Impl, OFFSET>,
            SetComplete::<Impl, OFFSET>,
            SetAbort::<Impl, OFFSET>,
            EnableCommit::<Impl, OFFSET>,
            DisableCommit::<Impl, OFFSET>,
            IsInTransaction::<Impl, OFFSET>,
            IsSecurityEnabled::<Impl, OFFSET>,
            IsCallerInRole::<Impl, OFFSET>,
        )
    }
}
pub trait IObjectContextActivityImpl: Sized {
    fn GetActivityId();
}
impl ::windows::core::RuntimeName for IObjectContextActivity {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IObjectContextActivity";
}
impl IObjectContextActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextActivityImpl, const OFFSET: isize>() -> IObjectContextActivityVtbl {
        unsafe extern "system" fn GetActivityId<Impl: IObjectContextActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivityId(&*(&pguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectContextActivity>, ::windows::core::GetTrustLevel, GetActivityId::<Impl, OFFSET>)
    }
}
pub trait IObjectContextInfoImpl: Sized {
    fn IsInTransaction();
    fn GetTransaction();
    fn GetTransactionId();
    fn GetActivityId();
    fn GetContextId();
}
impl ::windows::core::RuntimeName for IObjectContextInfo {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IObjectContextInfo";
}
impl IObjectContextInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextInfoImpl, const OFFSET: isize>() -> IObjectContextInfoVtbl {
        unsafe extern "system" fn IsInTransaction<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransaction<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrans: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransaction(::core::mem::transmute_copy(&pptrans)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionId(&*(&pguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityId<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivityId(&*(&pguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextId<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextId(&*(&pguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectContextInfo>, ::windows::core::GetTrustLevel, IsInTransaction::<Impl, OFFSET>, GetTransaction::<Impl, OFFSET>, GetTransactionId::<Impl, OFFSET>, GetActivityId::<Impl, OFFSET>, GetContextId::<Impl, OFFSET>)
    }
}
pub trait IObjectContextInfo2Impl: Sized + IObjectContextInfoImpl {
    fn GetPartitionId();
    fn GetApplicationId();
    fn GetApplicationInstanceId();
}
impl ::windows::core::RuntimeName for IObjectContextInfo2 {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IObjectContextInfo2";
}
impl IObjectContextInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextInfo2Impl, const OFFSET: isize>() -> IObjectContextInfo2Vtbl {
        unsafe extern "system" fn GetPartitionId<Impl: IObjectContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartitionId(&*(&pguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationId<Impl: IObjectContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationId(&*(&pguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceId<Impl: IObjectContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationInstanceId(&*(&pguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectContextInfo2>, ::windows::core::GetTrustLevel, GetPartitionId::<Impl, OFFSET>, GetApplicationId::<Impl, OFFSET>, GetApplicationInstanceId::<Impl, OFFSET>)
    }
}
pub trait IObjectContextTipImpl: Sized {
    fn GetTipUrl();
}
impl ::windows::core::RuntimeName for IObjectContextTip {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IObjectContextTip";
}
impl IObjectContextTipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextTipImpl, const OFFSET: isize>() -> IObjectContextTipVtbl {
        unsafe extern "system" fn GetTipUrl<Impl: IObjectContextTipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTipUrl(&*(&ptipurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectContextTip>, ::windows::core::GetTrustLevel, GetTipUrl::<Impl, OFFSET>)
    }
}
pub trait IObjectControlImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn CanBePooled();
}
impl ::windows::core::RuntimeName for IObjectControl {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IObjectControl";
}
impl IObjectControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectControlImpl, const OFFSET: isize>() -> IObjectControlVtbl {
        unsafe extern "system" fn Activate<Impl: IObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: IObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate().into()
        }
        unsafe extern "system" fn CanBePooled<Impl: IObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanBePooled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IObjectControl>, ::windows::core::GetTrustLevel, Activate::<Impl, OFFSET>, Deactivate::<Impl, OFFSET>, CanBePooled::<Impl, OFFSET>)
    }
}
pub trait IPlaybackControlImpl: Sized {
    fn FinalClientRetry();
    fn FinalServerRetry();
}
impl ::windows::core::RuntimeName for IPlaybackControl {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IPlaybackControl";
}
impl IPlaybackControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackControlImpl, const OFFSET: isize>() -> IPlaybackControlVtbl {
        unsafe extern "system" fn FinalClientRetry<Impl: IPlaybackControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalClientRetry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FinalServerRetry<Impl: IPlaybackControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FinalServerRetry() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPlaybackControl>, ::windows::core::GetTrustLevel, FinalClientRetry::<Impl, OFFSET>, FinalServerRetry::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPoolManagerImpl: Sized + IDispatchImpl {
    fn ShutdownPool();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IPoolManager {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IPoolManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IPoolManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPoolManagerImpl, const OFFSET: isize>() -> IPoolManagerVtbl {
        unsafe extern "system" fn ShutdownPool<Impl: IPoolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ShutdownPool(&*(&clsidorprogid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPoolManager>, ::windows::core::GetTrustLevel, ShutdownPool::<Impl, OFFSET>)
    }
}
pub trait IProcessInitializerImpl: Sized {
    fn Startup();
    fn Shutdown();
}
impl ::windows::core::RuntimeName for IProcessInitializer {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IProcessInitializer";
}
impl IProcessInitializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessInitializerImpl, const OFFSET: isize>() -> IProcessInitializerVtbl {
        unsafe extern "system" fn Startup<Impl: IProcessInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkprocesscontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Startup(&*(&punkprocesscontrol as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: IProcessInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProcessInitializer>, ::windows::core::GetTrustLevel, Startup::<Impl, OFFSET>, Shutdown::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityCallContextImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn IsCallerInRole();
    fn IsSecurityEnabled();
    fn IsUserInRole();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISecurityCallContext {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISecurityCallContext";
}
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityCallContextImpl, const OFFSET: isize>() -> ISecurityCallContextVtbl {
        unsafe extern "system" fn Count<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCallerInRole(&*(&bstrrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfinrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecurityEnabled(::core::mem::transmute_copy(&pfisenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserInRole<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *const super::Com::VARIANT, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserInRole(&*(&puser as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&bstrrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pfinrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISecurityCallContext>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>, IsCallerInRole::<Impl, OFFSET>, IsSecurityEnabled::<Impl, OFFSET>, IsUserInRole::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityCallersCollImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISecurityCallersColl {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISecurityCallersColl";
}
#[cfg(feature = "Win32_System_Com")]
impl ISecurityCallersCollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityCallersCollImpl, const OFFSET: isize>() -> ISecurityCallersCollVtbl {
        unsafe extern "system" fn Count<Impl: ISecurityCallersCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISecurityCallersCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(lindex, ::core::mem::transmute_copy(&pobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISecurityCallersCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISecurityCallersColl>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISecurityIdentityCollImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISecurityIdentityColl {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISecurityIdentityColl";
}
#[cfg(feature = "Win32_System_Com")]
impl ISecurityIdentityCollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityIdentityCollImpl, const OFFSET: isize>() -> ISecurityIdentityCollVtbl {
        unsafe extern "system" fn Count<Impl: ISecurityIdentityCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISecurityIdentityCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISecurityIdentityCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISecurityIdentityColl>, ::windows::core::GetTrustLevel, Count::<Impl, OFFSET>, Item::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
pub trait ISecurityPropertyImpl: Sized {
    fn GetDirectCreatorSID();
    fn GetOriginalCreatorSID();
    fn GetDirectCallerSID();
    fn GetOriginalCallerSID();
    fn ReleaseSID();
}
impl ::windows::core::RuntimeName for ISecurityProperty {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISecurityProperty";
}
impl ISecurityPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityPropertyImpl, const OFFSET: isize>() -> ISecurityPropertyVtbl {
        unsafe extern "system" fn GetDirectCreatorSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDirectCreatorSID(&*(&psid as *const <super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCreatorSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginalCreatorSID(&*(&psid as *const <super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirectCallerSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDirectCallerSID(&*(&psid as *const <super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCallerSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginalCallerSID(&*(&psid as *const <super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseSID(&*(&psid as *const <super::super::Foundation::PSID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PSID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISecurityProperty>, ::windows::core::GetTrustLevel, GetDirectCreatorSID::<Impl, OFFSET>, GetOriginalCreatorSID::<Impl, OFFSET>, GetDirectCallerSID::<Impl, OFFSET>, GetOriginalCallerSID::<Impl, OFFSET>, ReleaseSID::<Impl, OFFSET>)
    }
}
pub trait ISelectCOMLBServerImpl: Sized {
    fn Init();
    fn GetLBServer();
}
impl ::windows::core::RuntimeName for ISelectCOMLBServer {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISelectCOMLBServer";
}
impl ISelectCOMLBServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectCOMLBServerImpl, const OFFSET: isize>() -> ISelectCOMLBServerVtbl {
        unsafe extern "system" fn Init<Impl: ISelectCOMLBServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Init() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLBServer<Impl: ISelectCOMLBServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLBServer(&*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISelectCOMLBServer>, ::windows::core::GetTrustLevel, Init::<Impl, OFFSET>, GetLBServer::<Impl, OFFSET>)
    }
}
pub trait ISendMethodEventsImpl: Sized {
    fn SendMethodCall();
    fn SendMethodReturn();
}
impl ::windows::core::RuntimeName for ISendMethodEvents {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISendMethodEvents";
}
impl ISendMethodEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISendMethodEventsImpl, const OFFSET: isize>() -> ISendMethodEventsVtbl {
        unsafe extern "system" fn SendMethodCall<Impl: ISendMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMethodCall(&*(&pidentity as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwmeth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendMethodReturn<Impl: ISendMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32, hrcall: ::windows::core::HRESULT, hrserver: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendMethodReturn(&*(&pidentity as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), dwmeth, hrcall, hrserver) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISendMethodEvents>, ::windows::core::GetTrustLevel, SendMethodCall::<Impl, OFFSET>, SendMethodReturn::<Impl, OFFSET>)
    }
}
pub trait IServiceActivityImpl: Sized {
    fn SynchronousCall();
    fn AsynchronousCall();
    fn BindToCurrentThread();
    fn UnbindFromThread();
}
impl ::windows::core::RuntimeName for IServiceActivity {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceActivity";
}
impl IServiceActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceActivityImpl, const OFFSET: isize>() -> IServiceActivityVtbl {
        unsafe extern "system" fn SynchronousCall<Impl: IServiceActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SynchronousCall(&*(&piservicecall as *const <IServiceCall as ::windows::core::Abi>::Abi as *const <IServiceCall as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AsynchronousCall<Impl: IServiceActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AsynchronousCall(&*(&piservicecall as *const <IServiceCall as ::windows::core::Abi>::Abi as *const <IServiceCall as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BindToCurrentThread<Impl: IServiceActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindToCurrentThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnbindFromThread<Impl: IServiceActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnbindFromThread() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceActivity>, ::windows::core::GetTrustLevel, SynchronousCall::<Impl, OFFSET>, AsynchronousCall::<Impl, OFFSET>, BindToCurrentThread::<Impl, OFFSET>, UnbindFromThread::<Impl, OFFSET>)
    }
}
pub trait IServiceCallImpl: Sized {
    fn OnCall();
}
impl ::windows::core::RuntimeName for IServiceCall {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceCall";
}
impl IServiceCallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceCallImpl, const OFFSET: isize>() -> IServiceCallVtbl {
        unsafe extern "system" fn OnCall<Impl: IServiceCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCall() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceCall>, ::windows::core::GetTrustLevel, OnCall::<Impl, OFFSET>)
    }
}
pub trait IServiceComTIIntrinsicsConfigImpl: Sized {
    fn ComTIIntrinsicsConfig();
}
impl ::windows::core::RuntimeName for IServiceComTIIntrinsicsConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceComTIIntrinsicsConfig";
}
impl IServiceComTIIntrinsicsConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceComTIIntrinsicsConfigImpl, const OFFSET: isize>() -> IServiceComTIIntrinsicsConfigVtbl {
        unsafe extern "system" fn ComTIIntrinsicsConfig<Impl: IServiceComTIIntrinsicsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ComTIIntrinsicsConfig(comtiintrinsicsconfig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceComTIIntrinsicsConfig>, ::windows::core::GetTrustLevel, ComTIIntrinsicsConfig::<Impl, OFFSET>)
    }
}
pub trait IServiceIISIntrinsicsConfigImpl: Sized {
    fn IISIntrinsicsConfig();
}
impl ::windows::core::RuntimeName for IServiceIISIntrinsicsConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceIISIntrinsicsConfig";
}
impl IServiceIISIntrinsicsConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceIISIntrinsicsConfigImpl, const OFFSET: isize>() -> IServiceIISIntrinsicsConfigVtbl {
        unsafe extern "system" fn IISIntrinsicsConfig<Impl: IServiceIISIntrinsicsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IISIntrinsicsConfig(iisintrinsicsconfig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceIISIntrinsicsConfig>, ::windows::core::GetTrustLevel, IISIntrinsicsConfig::<Impl, OFFSET>)
    }
}
pub trait IServiceInheritanceConfigImpl: Sized {
    fn ContainingContextTreatment();
}
impl ::windows::core::RuntimeName for IServiceInheritanceConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceInheritanceConfig";
}
impl IServiceInheritanceConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceInheritanceConfigImpl, const OFFSET: isize>() -> IServiceInheritanceConfigVtbl {
        unsafe extern "system" fn ContainingContextTreatment<Impl: IServiceInheritanceConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContainingContextTreatment(inheritanceconfig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceInheritanceConfig>, ::windows::core::GetTrustLevel, ContainingContextTreatment::<Impl, OFFSET>)
    }
}
pub trait IServicePartitionConfigImpl: Sized {
    fn PartitionConfig();
    fn PartitionID();
}
impl ::windows::core::RuntimeName for IServicePartitionConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServicePartitionConfig";
}
impl IServicePartitionConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServicePartitionConfigImpl, const OFFSET: isize>() -> IServicePartitionConfigVtbl {
        unsafe extern "system" fn PartitionConfig<Impl: IServicePartitionConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartitionConfig(partitionconfig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PartitionID<Impl: IServicePartitionConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidpartitionid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PartitionID(&*(&guidpartitionid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServicePartitionConfig>, ::windows::core::GetTrustLevel, PartitionConfig::<Impl, OFFSET>, PartitionID::<Impl, OFFSET>)
    }
}
pub trait IServicePoolImpl: Sized {
    fn Initialize();
    fn GetObject();
    fn Shutdown();
}
impl ::windows::core::RuntimeName for IServicePool {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServicePool";
}
impl IServicePoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServicePoolImpl, const OFFSET: isize>() -> IServicePoolVtbl {
        unsafe extern "system" fn Initialize<Impl: IServicePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoolconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&ppoolconfig as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetObject<Impl: IServicePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetObject(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&ppv as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Shutdown<Impl: IServicePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Shutdown() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServicePool>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, GetObject::<Impl, OFFSET>, Shutdown::<Impl, OFFSET>)
    }
}
pub trait IServicePoolConfigImpl: Sized {
    fn SetMaxPoolSize();
    fn MaxPoolSize();
    fn SetMinPoolSize();
    fn MinPoolSize();
    fn SetCreationTimeout();
    fn CreationTimeout();
    fn SetTransactionAffinity();
    fn TransactionAffinity();
    fn SetClassFactory();
    fn ClassFactory();
}
impl ::windows::core::RuntimeName for IServicePoolConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServicePoolConfig";
}
impl IServicePoolConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServicePoolConfigImpl, const OFFSET: isize>() -> IServicePoolConfigVtbl {
        unsafe extern "system" fn SetMaxPoolSize<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxpool: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxPoolSize(dwmaxpool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxPoolSize<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxpool: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxPoolSize(pdwmaxpool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinPoolSize<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminpool: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMinPoolSize(dwminpool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinPoolSize<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminpool: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinPoolSize(pdwminpool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreationTimeout<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreationtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCreationTimeout(dwcreationtimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTimeout<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcreationtimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreationTimeout(pdwcreationtimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTransactionAffinity<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftxaffinity: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransactionAffinity(&*(&ftxaffinity as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionAffinity<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionAffinity(&*(&pftxaffinity as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClassFactory<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClassFactory(&*(&pfactory as *const <super::Com::IClassFactory as ::windows::core::Abi>::Abi as *const <super::Com::IClassFactory as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClassFactory<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassFactory(::core::mem::transmute_copy(&pfactory)) {
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
            ::windows::core::GetRuntimeClassName::<IServicePoolConfig>,
            ::windows::core::GetTrustLevel,
            SetMaxPoolSize::<Impl, OFFSET>,
            MaxPoolSize::<Impl, OFFSET>,
            SetMinPoolSize::<Impl, OFFSET>,
            MinPoolSize::<Impl, OFFSET>,
            SetCreationTimeout::<Impl, OFFSET>,
            CreationTimeout::<Impl, OFFSET>,
            SetTransactionAffinity::<Impl, OFFSET>,
            TransactionAffinity::<Impl, OFFSET>,
            SetClassFactory::<Impl, OFFSET>,
            ClassFactory::<Impl, OFFSET>,
        )
    }
}
pub trait IServiceSxsConfigImpl: Sized {
    fn SxsConfig();
    fn SxsName();
    fn SxsDirectory();
}
impl ::windows::core::RuntimeName for IServiceSxsConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceSxsConfig";
}
impl IServiceSxsConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceSxsConfigImpl, const OFFSET: isize>() -> IServiceSxsConfigVtbl {
        unsafe extern "system" fn SxsConfig<Impl: IServiceSxsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scsconfig: CSC_SxsConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SxsConfig(scsconfig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SxsName<Impl: IServiceSxsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SxsName(&*(&szsxsname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SxsDirectory<Impl: IServiceSxsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsdirectory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SxsDirectory(&*(&szsxsdirectory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceSxsConfig>, ::windows::core::GetTrustLevel, SxsConfig::<Impl, OFFSET>, SxsName::<Impl, OFFSET>, SxsDirectory::<Impl, OFFSET>)
    }
}
pub trait IServiceSynchronizationConfigImpl: Sized {
    fn ConfigureSynchronization();
}
impl ::windows::core::RuntimeName for IServiceSynchronizationConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceSynchronizationConfig";
}
impl IServiceSynchronizationConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceSynchronizationConfigImpl, const OFFSET: isize>() -> IServiceSynchronizationConfigVtbl {
        unsafe extern "system" fn ConfigureSynchronization<Impl: IServiceSynchronizationConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureSynchronization(synchconfig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceSynchronizationConfig>, ::windows::core::GetTrustLevel, ConfigureSynchronization::<Impl, OFFSET>)
    }
}
pub trait IServiceSysTxnConfigImpl: Sized + IServiceTransactionConfigImpl + IServiceTransactionConfigBaseImpl {
    fn ConfigureBYOTSysTxn();
}
impl ::windows::core::RuntimeName for IServiceSysTxnConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceSysTxnConfig";
}
impl IServiceSysTxnConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceSysTxnConfigImpl, const OFFSET: isize>() -> IServiceSysTxnConfigVtbl {
        unsafe extern "system" fn ConfigureBYOTSysTxn<Impl: IServiceSysTxnConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxproxy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureBYOTSysTxn(&*(&ptxproxy as *const <ITransactionProxy as ::windows::core::Abi>::Abi as *const <ITransactionProxy as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceSysTxnConfig>, ::windows::core::GetTrustLevel, ConfigureBYOTSysTxn::<Impl, OFFSET>)
    }
}
pub trait IServiceThreadPoolConfigImpl: Sized {
    fn SelectThreadPool();
    fn SetBindingInfo();
}
impl ::windows::core::RuntimeName for IServiceThreadPoolConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceThreadPoolConfig";
}
impl IServiceThreadPoolConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceThreadPoolConfigImpl, const OFFSET: isize>() -> IServiceThreadPoolConfigVtbl {
        unsafe extern "system" fn SelectThreadPool<Impl: IServiceThreadPoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadpool: CSC_ThreadPool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectThreadPool(threadpool) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBindingInfo<Impl: IServiceThreadPoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: CSC_Binding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBindingInfo(binding) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceThreadPoolConfig>, ::windows::core::GetTrustLevel, SelectThreadPool::<Impl, OFFSET>, SetBindingInfo::<Impl, OFFSET>)
    }
}
pub trait IServiceTrackerConfigImpl: Sized {
    fn TrackerConfig();
}
impl ::windows::core::RuntimeName for IServiceTrackerConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceTrackerConfig";
}
impl IServiceTrackerConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceTrackerConfigImpl, const OFFSET: isize>() -> IServiceTrackerConfigVtbl {
        unsafe extern "system" fn TrackerConfig<Impl: IServiceTrackerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: super::super::Foundation::PWSTR, sztrackerctxname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackerConfig(trackerconfig, &*(&sztrackerappname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&sztrackerctxname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceTrackerConfig>, ::windows::core::GetTrustLevel, TrackerConfig::<Impl, OFFSET>)
    }
}
pub trait IServiceTransactionConfigImpl: Sized + IServiceTransactionConfigBaseImpl {
    fn ConfigureBYOT();
}
impl ::windows::core::RuntimeName for IServiceTransactionConfig {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceTransactionConfig";
}
impl IServiceTransactionConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceTransactionConfigImpl, const OFFSET: isize>() -> IServiceTransactionConfigVtbl {
        unsafe extern "system" fn ConfigureBYOT<Impl: IServiceTransactionConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitxbyot: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureBYOT(&*(&pitxbyot as *const <super::DistributedTransactionCoordinator::ITransaction as ::windows::core::Abi>::Abi as *const <super::DistributedTransactionCoordinator::ITransaction as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceTransactionConfig>, ::windows::core::GetTrustLevel, ConfigureBYOT::<Impl, OFFSET>)
    }
}
pub trait IServiceTransactionConfigBaseImpl: Sized {
    fn ConfigureTransaction();
    fn IsolationLevel();
    fn TransactionTimeout();
    fn BringYourOwnTransaction();
    fn NewTransactionDescription();
}
impl ::windows::core::RuntimeName for IServiceTransactionConfigBase {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IServiceTransactionConfigBase";
}
impl IServiceTransactionConfigBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>() -> IServiceTransactionConfigBaseVtbl {
        unsafe extern "system" fn ConfigureTransaction<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigureTransaction(transactionconfig) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsolationLevel<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsolationLevel(option) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionTimeout<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionTimeout(ultimeoutsec) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BringYourOwnTransaction<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztipurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BringYourOwnTransaction(&*(&sztipurl as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewTransactionDescription<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztxdesc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewTransactionDescription(&*(&sztxdesc as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IServiceTransactionConfigBase>, ::windows::core::GetTrustLevel, ConfigureTransaction::<Impl, OFFSET>, IsolationLevel::<Impl, OFFSET>, TransactionTimeout::<Impl, OFFSET>, BringYourOwnTransaction::<Impl, OFFSET>, NewTransactionDescription::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISharedPropertyImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISharedProperty {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISharedProperty";
}
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPropertyImpl, const OFFSET: isize>() -> ISharedPropertyVtbl {
        unsafe extern "system" fn Value<Impl: ISharedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISharedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(&*(&val as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISharedProperty>, ::windows::core::GetTrustLevel, Value::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISharedPropertyGroupImpl: Sized + IDispatchImpl {
    fn CreatePropertyByPosition();
    fn PropertyByPosition();
    fn CreateProperty();
    fn Property();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISharedPropertyGroup {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISharedPropertyGroup";
}
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPropertyGroupImpl, const OFFSET: isize>() -> ISharedPropertyGroupVtbl {
        unsafe extern "system" fn CreatePropertyByPosition<Impl: ISharedPropertyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, fexists: *mut i16, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyByPosition(index, ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PropertyByPosition<Impl: ISharedPropertyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyByPosition(index, ::core::mem::transmute_copy(&ppproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperty<Impl: ISharedPropertyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fexists: *mut i16, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProperty(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Property<Impl: ISharedPropertyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISharedPropertyGroup>, ::windows::core::GetTrustLevel, CreatePropertyByPosition::<Impl, OFFSET>, PropertyByPosition::<Impl, OFFSET>, CreateProperty::<Impl, OFFSET>, Property::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISharedPropertyGroupManagerImpl: Sized + IDispatchImpl {
    fn CreatePropertyGroup();
    fn Group();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISharedPropertyGroupManager {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISharedPropertyGroupManager";
}
#[cfg(feature = "Win32_System_Com")]
impl ISharedPropertyGroupManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPropertyGroupManagerImpl, const OFFSET: isize>() -> ISharedPropertyGroupManagerVtbl {
        unsafe extern "system" fn CreatePropertyGroup<Impl: ISharedPropertyGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyGroup(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwisomode, dwrelmode, ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Impl: ISharedPropertyGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISharedPropertyGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&retval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISharedPropertyGroupManager>, ::windows::core::GetTrustLevel, CreatePropertyGroup::<Impl, OFFSET>, Group::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
pub trait ISystemAppEventDataImpl: Sized {
    fn Startup();
    fn OnDataChanged();
}
impl ::windows::core::RuntimeName for ISystemAppEventData {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ISystemAppEventData";
}
impl ISystemAppEventDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemAppEventDataImpl, const OFFSET: isize>() -> ISystemAppEventDataVtbl {
        unsafe extern "system" fn Startup<Impl: ISystemAppEventDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Startup() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataChanged<Impl: ISystemAppEventDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDataChanged(dwpid, dwmask, dwnumbersinks, &*(&bstrdwmethodmask as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), dwreason, u64tracehandle) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISystemAppEventData>, ::windows::core::GetTrustLevel, Startup::<Impl, OFFSET>, OnDataChanged::<Impl, OFFSET>)
    }
}
pub trait IThreadPoolKnobsImpl: Sized {
    fn GetMaxThreads();
    fn GetCurrentThreads();
    fn SetMaxThreads();
    fn GetDeleteDelay();
    fn SetDeleteDelay();
    fn GetMaxQueuedRequests();
    fn GetCurrentQueuedRequests();
    fn SetMaxQueuedRequests();
    fn SetMinThreads();
    fn SetQueueDepth();
}
impl ::windows::core::RuntimeName for IThreadPoolKnobs {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.IThreadPoolKnobs";
}
impl IThreadPoolKnobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThreadPoolKnobsImpl, const OFFSET: isize>() -> IThreadPoolKnobsVtbl {
        unsafe extern "system" fn GetMaxThreads<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxthreads: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxThreads(plcmaxthreads) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentThreads<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentthreads: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentThreads(plccurrentthreads) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxThreads<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxthreads: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxThreads(lcmaxthreads) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeleteDelay<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsecdeletedelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeleteDelay(pmsecdeletedelay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDeleteDelay<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msecdeletedelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDeleteDelay(msecdeletedelay) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaxQueuedRequests<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxQueuedRequests(plcmaxqueuedrequests) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCurrentQueuedRequests<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCurrentQueuedRequests(plccurrentqueuedrequests) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxQueuedRequests<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxqueuedrequests: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxQueuedRequests(lcmaxqueuedrequests) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMinThreads<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcminthreads: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMinThreads(lcminthreads) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueueDepth<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcqueuedepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetQueueDepth(lcqueuedepth) {
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
            ::windows::core::GetRuntimeClassName::<IThreadPoolKnobs>,
            ::windows::core::GetTrustLevel,
            GetMaxThreads::<Impl, OFFSET>,
            GetCurrentThreads::<Impl, OFFSET>,
            SetMaxThreads::<Impl, OFFSET>,
            GetDeleteDelay::<Impl, OFFSET>,
            SetDeleteDelay::<Impl, OFFSET>,
            GetMaxQueuedRequests::<Impl, OFFSET>,
            GetCurrentQueuedRequests::<Impl, OFFSET>,
            SetMaxQueuedRequests::<Impl, OFFSET>,
            SetMinThreads::<Impl, OFFSET>,
            SetQueueDepth::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITransactionContextImpl: Sized + IDispatchImpl {
    fn CreateInstance();
    fn Commit();
    fn Abort();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ITransactionContext {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ITransactionContext";
}
#[cfg(feature = "Win32_System_Com")]
impl ITransactionContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionContextImpl, const OFFSET: isize>() -> ITransactionContextVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITransactionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&pszprogid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ITransactionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: ITransactionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransactionContext>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>, Commit::<Impl, OFFSET>, Abort::<Impl, OFFSET>)
    }
}
pub trait ITransactionContextExImpl: Sized {
    fn CreateInstance();
    fn Commit();
    fn Abort();
}
impl ::windows::core::RuntimeName for ITransactionContextEx {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ITransactionContextEx";
}
impl ITransactionContextExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionContextExImpl, const OFFSET: isize>() -> ITransactionContextExVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITransactionContextExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&rclsid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ITransactionContextExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: ITransactionContextExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransactionContextEx>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>, Commit::<Impl, OFFSET>, Abort::<Impl, OFFSET>)
    }
}
pub trait ITransactionPropertyImpl: Sized {
    fn Reserved1();
    fn Reserved2();
    fn Reserved3();
    fn Reserved4();
    fn Reserved5();
    fn Reserved6();
    fn Reserved7();
    fn Reserved8();
    fn Reserved9();
    fn GetTransactionResourcePool();
    fn Reserved10();
    fn Reserved11();
    fn Reserved12();
    fn Reserved13();
    fn Reserved14();
    fn Reserved15();
    fn Reserved16();
    fn Reserved17();
}
impl ::windows::core::RuntimeName for ITransactionProperty {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ITransactionProperty";
}
impl ITransactionPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionPropertyImpl, const OFFSET: isize>() -> ITransactionPropertyVtbl {
        unsafe extern "system" fn Reserved1<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved1().into()
        }
        unsafe extern "system" fn Reserved2<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved2().into()
        }
        unsafe extern "system" fn Reserved3<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved3().into()
        }
        unsafe extern "system" fn Reserved4<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved4().into()
        }
        unsafe extern "system" fn Reserved5<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved5().into()
        }
        unsafe extern "system" fn Reserved6<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved6().into()
        }
        unsafe extern "system" fn Reserved7<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved7().into()
        }
        unsafe extern "system" fn Reserved8<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved8().into()
        }
        unsafe extern "system" fn Reserved9<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved9().into()
        }
        unsafe extern "system" fn GetTransactionResourcePool<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptxpool: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionResourcePool(::core::mem::transmute_copy(&pptxpool)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved10<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved10().into()
        }
        unsafe extern "system" fn Reserved11<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved11().into()
        }
        unsafe extern "system" fn Reserved12<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved12().into()
        }
        unsafe extern "system" fn Reserved13<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved13().into()
        }
        unsafe extern "system" fn Reserved14<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved14().into()
        }
        unsafe extern "system" fn Reserved15<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved15().into()
        }
        unsafe extern "system" fn Reserved16<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved16().into()
        }
        unsafe extern "system" fn Reserved17<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved17().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<ITransactionProperty>,
            ::windows::core::GetTrustLevel,
            Reserved1::<Impl, OFFSET>,
            Reserved2::<Impl, OFFSET>,
            Reserved3::<Impl, OFFSET>,
            Reserved4::<Impl, OFFSET>,
            Reserved5::<Impl, OFFSET>,
            Reserved6::<Impl, OFFSET>,
            Reserved7::<Impl, OFFSET>,
            Reserved8::<Impl, OFFSET>,
            Reserved9::<Impl, OFFSET>,
            GetTransactionResourcePool::<Impl, OFFSET>,
            Reserved10::<Impl, OFFSET>,
            Reserved11::<Impl, OFFSET>,
            Reserved12::<Impl, OFFSET>,
            Reserved13::<Impl, OFFSET>,
            Reserved14::<Impl, OFFSET>,
            Reserved15::<Impl, OFFSET>,
            Reserved16::<Impl, OFFSET>,
            Reserved17::<Impl, OFFSET>,
        )
    }
}
pub trait ITransactionProxyImpl: Sized {
    fn Commit();
    fn Abort();
    fn Promote();
    fn CreateVoter();
    fn GetIsolationLevel();
    fn GetIdentifier();
    fn IsReusable();
}
impl ::windows::core::RuntimeName for ITransactionProxy {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ITransactionProxy";
}
impl ITransactionProxyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionProxyImpl, const OFFSET: isize>() -> ITransactionProxyVtbl {
        unsafe extern "system" fn Commit<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Commit(&*(&guid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Promote<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Promote(::core::mem::transmute_copy(&ptransaction)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVoter<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxasync: ::windows::core::RawPtr, ppballot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVoter(&*(&ptxasync as *const <super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2 as ::windows::core::Abi>::Abi as *const <super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppballot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsolationLevel<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIsolationLevel(__midl__itransactionproxy0000) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIdentifier<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstridentifier: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIdentifier(&*(&pbstridentifier as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReusable<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReusable(&*(&pfisreusable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransactionProxy>, ::windows::core::GetTrustLevel, Commit::<Impl, OFFSET>, Abort::<Impl, OFFSET>, Promote::<Impl, OFFSET>, CreateVoter::<Impl, OFFSET>, GetIsolationLevel::<Impl, OFFSET>, GetIdentifier::<Impl, OFFSET>, IsReusable::<Impl, OFFSET>)
    }
}
pub trait ITransactionResourcePoolImpl: Sized {
    fn PutResource();
    fn GetResource();
}
impl ::windows::core::RuntimeName for ITransactionResourcePool {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ITransactionResourcePool";
}
impl ITransactionResourcePoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionResourcePoolImpl, const OFFSET: isize>() -> ITransactionResourcePoolVtbl {
        unsafe extern "system" fn PutResource<Impl: ITransactionResourcePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: ::windows::core::RawPtr, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PutResource(&*(&ppool as *const <IObjPool as ::windows::core::Abi>::Abi as *const <IObjPool as ::windows::core::DefaultType>::DefaultType), &*(&punk as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResource<Impl: ITransactionResourcePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: ::windows::core::RawPtr, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource(&*(&ppool as *const <IObjPool as ::windows::core::Abi>::Abi as *const <IObjPool as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransactionResourcePool>, ::windows::core::GetTrustLevel, PutResource::<Impl, OFFSET>, GetResource::<Impl, OFFSET>)
    }
}
pub trait ITransactionStatusImpl: Sized {
    fn SetTransactionStatus();
    fn GetTransactionStatus();
}
impl ::windows::core::RuntimeName for ITransactionStatus {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ITransactionStatus";
}
impl ITransactionStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionStatusImpl, const OFFSET: isize>() -> ITransactionStatusVtbl {
        unsafe extern "system" fn SetTransactionStatus<Impl: ITransactionStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetTransactionStatus(hrstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionStatus<Impl: ITransactionStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionStatus(phrstatus) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITransactionStatus>, ::windows::core::GetTrustLevel, SetTransactionStatus::<Impl, OFFSET>, GetTransactionStatus::<Impl, OFFSET>)
    }
}
pub trait ITxProxyHolderImpl: Sized {
    fn GetIdentifier();
}
impl ::windows::core::RuntimeName for ITxProxyHolder {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ITxProxyHolder";
}
impl ITxProxyHolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITxProxyHolderImpl, const OFFSET: isize>() -> ITxProxyHolderVtbl {
        unsafe extern "system" fn GetIdentifier<Impl: ITxProxyHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidltx: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdentifier(&*(&pguidltx as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ITxProxyHolder>, ::windows::core::GetTrustLevel, GetIdentifier::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ObjectContextImpl: Sized + IDispatchImpl {
    fn CreateInstance();
    fn SetComplete();
    fn SetAbort();
    fn EnableCommit();
    fn DisableCommit();
    fn IsInTransaction();
    fn IsSecurityEnabled();
    fn IsCallerInRole();
    fn Count();
    fn Item();
    fn _NewEnum();
    fn Security();
    fn ContextInfo();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ObjectContext {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ObjectContext";
}
#[cfg(feature = "Win32_System_Com")]
impl ObjectContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ObjectContextImpl, const OFFSET: isize>() -> ObjectContextVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&bstrprogid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComplete<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetComplete() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAbort<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAbort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableCommit<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableCommit<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableCommit() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInTransaction<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInTransaction(::core::mem::transmute_copy(&pbisintx)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecurityEnabled(::core::mem::transmute_copy(&pbisenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCallerInRole(&*(&bstrrole as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbinrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security(::core::mem::transmute_copy(&ppsecurityproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextInfo<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontextinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextInfo(::core::mem::transmute_copy(&ppcontextinfo)) {
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
            ::windows::core::GetRuntimeClassName::<ObjectContext>,
            ::windows::core::GetTrustLevel,
            CreateInstance::<Impl, OFFSET>,
            SetComplete::<Impl, OFFSET>,
            SetAbort::<Impl, OFFSET>,
            EnableCommit::<Impl, OFFSET>,
            DisableCommit::<Impl, OFFSET>,
            IsInTransaction::<Impl, OFFSET>,
            IsSecurityEnabled::<Impl, OFFSET>,
            IsCallerInRole::<Impl, OFFSET>,
            Count::<Impl, OFFSET>,
            Item::<Impl, OFFSET>,
            _NewEnum::<Impl, OFFSET>,
            Security::<Impl, OFFSET>,
            ContextInfo::<Impl, OFFSET>,
        )
    }
}
pub trait ObjectControlImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn CanBePooled();
}
impl ::windows::core::RuntimeName for ObjectControl {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.ObjectControl";
}
impl ObjectControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ObjectControlImpl, const OFFSET: isize>() -> ObjectControlVtbl {
        unsafe extern "system" fn Activate<Impl: ObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Activate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: ObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Deactivate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanBePooled<Impl: ObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpoolable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanBePooled(pbpoolable) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ObjectControl>, ::windows::core::GetTrustLevel, Activate::<Impl, OFFSET>, Deactivate::<Impl, OFFSET>, CanBePooled::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait SecurityPropertyImpl: Sized + IDispatchImpl {
    fn GetDirectCallerName();
    fn GetDirectCreatorName();
    fn GetOriginalCallerName();
    fn GetOriginalCreatorName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for SecurityProperty {
    const NAME: &'static str = "Windows.Win32.System.ComponentServices.SecurityProperty";
}
#[cfg(feature = "Win32_System_Com")]
impl SecurityPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: SecurityPropertyImpl, const OFFSET: isize>() -> SecurityPropertyVtbl {
        unsafe extern "system" fn GetDirectCallerName<Impl: SecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDirectCallerName(::core::mem::transmute_copy(&bstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirectCreatorName<Impl: SecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDirectCreatorName(::core::mem::transmute_copy(&bstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCallerName<Impl: SecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginalCallerName(::core::mem::transmute_copy(&bstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCreatorName<Impl: SecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginalCreatorName(::core::mem::transmute_copy(&bstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<SecurityProperty>, ::windows::core::GetTrustLevel, GetDirectCallerName::<Impl, OFFSET>, GetDirectCreatorName::<Impl, OFFSET>, GetOriginalCallerName::<Impl, OFFSET>, GetOriginalCreatorName::<Impl, OFFSET>)
    }
}
