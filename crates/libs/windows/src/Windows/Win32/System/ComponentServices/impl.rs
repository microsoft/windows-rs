#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ContextInfoImpl: Sized + IDispatchImpl {
    fn IsInTransaction();
    fn GetTransaction();
    fn GetTransactionId();
    fn GetActivityId();
    fn GetContextId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ContextInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ContextInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ContextInfoVtbl {
        unsafe extern "system" fn IsInTransaction<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransaction<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptx: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransactionId<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActivityId<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstractivityid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContextId<Impl: ContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrctxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, IsInTransaction::<Impl, IMPL_OFFSET>, GetTransaction::<Impl, IMPL_OFFSET>, GetTransactionId::<Impl, IMPL_OFFSET>, GetActivityId::<Impl, IMPL_OFFSET>, GetContextId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ContextInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ContextInfo2Impl: Sized + ContextInfoImpl + IDispatchImpl {
    fn GetPartitionId();
    fn GetApplicationId();
    fn GetApplicationInstanceId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ContextInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ContextInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ContextInfo2Vtbl {
        unsafe extern "system" fn GetPartitionId<Impl: ContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20000: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplicationId<Impl: ContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20001: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplicationInstanceId<Impl: ContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20002: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            IsInTransaction::<Impl, IMPL_OFFSET>,
            GetTransaction::<Impl, IMPL_OFFSET>,
            GetTransactionId::<Impl, IMPL_OFFSET>,
            GetActivityId::<Impl, IMPL_OFFSET>,
            GetContextId::<Impl, IMPL_OFFSET>,
            GetPartitionId::<Impl, IMPL_OFFSET>,
            GetApplicationId::<Impl, IMPL_OFFSET>,
            GetApplicationInstanceId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ContextInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAppDomainHelperImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn DoCallback();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAppDomainHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDomainHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDomainHelperVtbl {
        unsafe extern "system" fn Initialize<Impl: IAppDomainHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoCallback<Impl: IAppDomainHelperImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, DoCallback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDomainHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAssemblyLocatorImpl: Sized + IDispatchImpl {
    fn GetModules();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAssemblyLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssemblyLocatorVtbl {
        unsafe extern "system" fn GetModules<Impl: IAssemblyLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, assemblyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetModules::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyLocator as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncErrorNotifyImpl: Sized {
    fn OnError();
}
impl IAsyncErrorNotifyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncErrorNotifyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncErrorNotifyVtbl {
        unsafe extern "system" fn OnError<Impl: IAsyncErrorNotifyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnError::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncErrorNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICOMAdminCatalogVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICOMAdminCatalogImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICOMAdminCatalogVtbl {
        unsafe extern "system" fn GetCollection<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Connect<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcatalogservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MajorVersion<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MinorVersion<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCollectionByQuery<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportComponent<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallComponent<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShutdownApplication<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportApplication<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallApplication<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopRouter<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshRouter<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartRouter<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved1<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved2<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallMultipleComponents<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RefreshComponents<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BackupREGDB<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreREGDB<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryApplicationFile<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartApplication<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ServiceCheck<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallEventClass<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventClassesForIID<Impl: ICOMAdminCatalogImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstriid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetCollection::<Impl, IMPL_OFFSET>,
            Connect::<Impl, IMPL_OFFSET>,
            MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion::<Impl, IMPL_OFFSET>,
            GetCollectionByQuery::<Impl, IMPL_OFFSET>,
            ImportComponent::<Impl, IMPL_OFFSET>,
            InstallComponent::<Impl, IMPL_OFFSET>,
            ShutdownApplication::<Impl, IMPL_OFFSET>,
            ExportApplication::<Impl, IMPL_OFFSET>,
            InstallApplication::<Impl, IMPL_OFFSET>,
            StopRouter::<Impl, IMPL_OFFSET>,
            RefreshRouter::<Impl, IMPL_OFFSET>,
            StartRouter::<Impl, IMPL_OFFSET>,
            Reserved1::<Impl, IMPL_OFFSET>,
            Reserved2::<Impl, IMPL_OFFSET>,
            InstallMultipleComponents::<Impl, IMPL_OFFSET>,
            GetMultipleComponentsInfo::<Impl, IMPL_OFFSET>,
            RefreshComponents::<Impl, IMPL_OFFSET>,
            BackupREGDB::<Impl, IMPL_OFFSET>,
            RestoreREGDB::<Impl, IMPL_OFFSET>,
            QueryApplicationFile::<Impl, IMPL_OFFSET>,
            StartApplication::<Impl, IMPL_OFFSET>,
            ServiceCheck::<Impl, IMPL_OFFSET>,
            InstallMultipleEventClasses::<Impl, IMPL_OFFSET>,
            InstallEventClass::<Impl, IMPL_OFFSET>,
            GetEventClassesForIID::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICOMAdminCatalog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICOMAdminCatalog2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICOMAdminCatalog2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICOMAdminCatalog2Vtbl {
        unsafe extern "system" fn GetCollectionByQuery2<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollectionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarquerystrings: *const super::Com::VARIANT, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PauseApplicationInstances<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResumeApplicationInstances<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecycleApplicationInstances<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, pvarboolpaused: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DumpApplicationInstance<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaximages: i32, pbstrdumpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbooldumpsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateServiceForApplication<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrstarttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrerrorcontrol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdependencies: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrunas: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bdesktopok: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteServiceForApplication<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPartitionID<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPartitionName<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCurrentPartition<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentPartitionID<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentPartitionName<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GlobalPartitionID<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrglobalpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlushPartitionCache<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyApplications<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourcepartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CopyComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AliasComponent<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsrcapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSafeToDelete<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportComponents<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Is64BitCatalogServer<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbis64bit: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExportPartition<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpartitionfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstallPartition<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryApplicationFile2<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfilesforimport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetComponentVersionCount<Impl: ICOMAdminCatalog2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plversioncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            GetCollection::<Impl, IMPL_OFFSET>,
            Connect::<Impl, IMPL_OFFSET>,
            MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion::<Impl, IMPL_OFFSET>,
            GetCollectionByQuery::<Impl, IMPL_OFFSET>,
            ImportComponent::<Impl, IMPL_OFFSET>,
            InstallComponent::<Impl, IMPL_OFFSET>,
            ShutdownApplication::<Impl, IMPL_OFFSET>,
            ExportApplication::<Impl, IMPL_OFFSET>,
            InstallApplication::<Impl, IMPL_OFFSET>,
            StopRouter::<Impl, IMPL_OFFSET>,
            RefreshRouter::<Impl, IMPL_OFFSET>,
            StartRouter::<Impl, IMPL_OFFSET>,
            Reserved1::<Impl, IMPL_OFFSET>,
            Reserved2::<Impl, IMPL_OFFSET>,
            InstallMultipleComponents::<Impl, IMPL_OFFSET>,
            GetMultipleComponentsInfo::<Impl, IMPL_OFFSET>,
            RefreshComponents::<Impl, IMPL_OFFSET>,
            BackupREGDB::<Impl, IMPL_OFFSET>,
            RestoreREGDB::<Impl, IMPL_OFFSET>,
            QueryApplicationFile::<Impl, IMPL_OFFSET>,
            StartApplication::<Impl, IMPL_OFFSET>,
            ServiceCheck::<Impl, IMPL_OFFSET>,
            InstallMultipleEventClasses::<Impl, IMPL_OFFSET>,
            InstallEventClass::<Impl, IMPL_OFFSET>,
            GetEventClassesForIID::<Impl, IMPL_OFFSET>,
            GetCollectionByQuery2::<Impl, IMPL_OFFSET>,
            GetApplicationInstanceIDFromProcessID::<Impl, IMPL_OFFSET>,
            ShutdownApplicationInstances::<Impl, IMPL_OFFSET>,
            PauseApplicationInstances::<Impl, IMPL_OFFSET>,
            ResumeApplicationInstances::<Impl, IMPL_OFFSET>,
            RecycleApplicationInstances::<Impl, IMPL_OFFSET>,
            AreApplicationInstancesPaused::<Impl, IMPL_OFFSET>,
            DumpApplicationInstance::<Impl, IMPL_OFFSET>,
            IsApplicationInstanceDumpSupported::<Impl, IMPL_OFFSET>,
            CreateServiceForApplication::<Impl, IMPL_OFFSET>,
            DeleteServiceForApplication::<Impl, IMPL_OFFSET>,
            GetPartitionID::<Impl, IMPL_OFFSET>,
            GetPartitionName::<Impl, IMPL_OFFSET>,
            SetCurrentPartition::<Impl, IMPL_OFFSET>,
            CurrentPartitionID::<Impl, IMPL_OFFSET>,
            CurrentPartitionName::<Impl, IMPL_OFFSET>,
            GlobalPartitionID::<Impl, IMPL_OFFSET>,
            FlushPartitionCache::<Impl, IMPL_OFFSET>,
            CopyApplications::<Impl, IMPL_OFFSET>,
            CopyComponents::<Impl, IMPL_OFFSET>,
            MoveComponents::<Impl, IMPL_OFFSET>,
            AliasComponent::<Impl, IMPL_OFFSET>,
            IsSafeToDelete::<Impl, IMPL_OFFSET>,
            ImportUnconfiguredComponents::<Impl, IMPL_OFFSET>,
            PromoteUnconfiguredComponents::<Impl, IMPL_OFFSET>,
            ImportComponents::<Impl, IMPL_OFFSET>,
            Is64BitCatalogServer::<Impl, IMPL_OFFSET>,
            ExportPartition::<Impl, IMPL_OFFSET>,
            InstallPartition::<Impl, IMPL_OFFSET>,
            QueryApplicationFile2::<Impl, IMPL_OFFSET>,
            GetComponentVersionCount::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICOMAdminCatalog2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICOMLBArgumentsImpl: Sized {
    fn GetCLSID();
    fn SetCLSID();
    fn GetMachineName();
    fn SetMachineName();
}
#[cfg(feature = "Win32_Foundation")]
impl ICOMLBArgumentsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICOMLBArgumentsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICOMLBArgumentsVtbl {
        unsafe extern "system" fn GetCLSID<Impl: ICOMLBArgumentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCLSID<Impl: ICOMLBArgumentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMachineName<Impl: ICOMLBArgumentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMachineName<Impl: ICOMLBArgumentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCLSID::<Impl, IMPL_OFFSET>, SetCLSID::<Impl, IMPL_OFFSET>, GetMachineName::<Impl, IMPL_OFFSET>, SetMachineName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICOMLBArguments as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICatalogCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICatalogCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppcatalogobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plobjectcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcatalogobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Populate<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveChanges<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchanges: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCollection<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varobjectkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarnamel: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddEnabled<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveEnabled<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUtilInterface<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DataStoreMajorVersion<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DataStoreMinorVersion<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversionl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopulateByKey<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PopulateByQuery<Impl: ICatalogCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquerystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lquerytype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            Add::<Impl, IMPL_OFFSET>,
            Populate::<Impl, IMPL_OFFSET>,
            SaveChanges::<Impl, IMPL_OFFSET>,
            GetCollection::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            AddEnabled::<Impl, IMPL_OFFSET>,
            RemoveEnabled::<Impl, IMPL_OFFSET>,
            GetUtilInterface::<Impl, IMPL_OFFSET>,
            DataStoreMajorVersion::<Impl, IMPL_OFFSET>,
            DataStoreMinorVersion::<Impl, IMPL_OFFSET>,
            PopulateByKey::<Impl, IMPL_OFFSET>,
            PopulateByQuery::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalogCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICatalogObjectImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
    fn Key();
    fn Name();
    fn IsPropertyReadOnly();
    fn Valid();
    fn IsPropertyWriteOnly();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICatalogObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICatalogObjectVtbl {
        unsafe extern "system" fn Value<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Key<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Name<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPropertyReadOnly<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Valid<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Impl: ICatalogObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Value::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
            Key::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            IsPropertyReadOnly::<Impl, IMPL_OFFSET>,
            Valid::<Impl, IMPL_OFFSET>,
            IsPropertyWriteOnly::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalogObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICheckSxsConfigImpl: Sized {
    fn IsSameSxsConfig();
}
#[cfg(feature = "Win32_Foundation")]
impl ICheckSxsConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICheckSxsConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICheckSxsConfigVtbl {
        unsafe extern "system" fn IsSameSxsConfig<Impl: ICheckSxsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsxsname: super::super::Foundation::PWSTR, wszsxsdirectory: super::super::Foundation::PWSTR, wszsxsappname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsSameSxsConfig::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICheckSxsConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComActivityEventsImpl: Sized {
    fn OnActivityCreate();
    fn OnActivityDestroy();
    fn OnActivityEnter();
    fn OnActivityTimeout();
    fn OnActivityReenter();
    fn OnActivityLeave();
    fn OnActivityLeaveSame();
}
#[cfg(feature = "Win32_Foundation")]
impl IComActivityEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComActivityEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComActivityEventsVtbl {
        unsafe extern "system" fn OnActivityCreate<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnActivityDestroy<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnActivityEnter<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnActivityTimeout<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnActivityReenter<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnActivityLeave<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidleft: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnActivityLeaveSame<Impl: IComActivityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwcalldepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnActivityCreate::<Impl, IMPL_OFFSET>, OnActivityDestroy::<Impl, IMPL_OFFSET>, OnActivityEnter::<Impl, IMPL_OFFSET>, OnActivityTimeout::<Impl, IMPL_OFFSET>, OnActivityReenter::<Impl, IMPL_OFFSET>, OnActivityLeave::<Impl, IMPL_OFFSET>, OnActivityLeaveSame::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComActivityEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComApp2EventsImpl: Sized {
    fn OnAppActivation2();
    fn OnAppShutdown2();
    fn OnAppForceShutdown2();
    fn OnAppPaused2();
    fn OnAppRecycle2();
}
#[cfg(feature = "Win32_Foundation")]
impl IComApp2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComApp2EventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComApp2EventsVtbl {
        unsafe extern "system" fn OnAppActivation2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAppShutdown2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAppForceShutdown2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAppPaused2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAppRecycle2<Impl: IComApp2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID, lreason: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnAppActivation2::<Impl, IMPL_OFFSET>, OnAppShutdown2::<Impl, IMPL_OFFSET>, OnAppForceShutdown2::<Impl, IMPL_OFFSET>, OnAppPaused2::<Impl, IMPL_OFFSET>, OnAppRecycle2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComApp2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComAppEventsImpl: Sized {
    fn OnAppActivation();
    fn OnAppShutdown();
    fn OnAppForceShutdown();
}
#[cfg(feature = "Win32_Foundation")]
impl IComAppEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComAppEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComAppEventsVtbl {
        unsafe extern "system" fn OnAppActivation<Impl: IComAppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAppShutdown<Impl: IComAppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAppForceShutdown<Impl: IComAppEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnAppActivation::<Impl, IMPL_OFFSET>, OnAppShutdown::<Impl, IMPL_OFFSET>, OnAppForceShutdown::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComAppEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IComCRMEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComCRMEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComCRMEventsVtbl {
        unsafe extern "system" fn OnCRMRecoveryStart<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMRecoveryDone<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMCheckpoint<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMBegin<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, guidactivity: ::windows::core::GUID, guidtx: ::windows::core::GUID, szprogidcompensator: super::super::Foundation::PWSTR, szdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMPrepare<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMCommit<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMAbort<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMIndoubt<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMDone<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMRelease<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMAnalyze<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMWrite<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMForget<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMForce<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnCRMDeliver<Impl: IComCRMEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            OnCRMRecoveryStart::<Impl, IMPL_OFFSET>,
            OnCRMRecoveryDone::<Impl, IMPL_OFFSET>,
            OnCRMCheckpoint::<Impl, IMPL_OFFSET>,
            OnCRMBegin::<Impl, IMPL_OFFSET>,
            OnCRMPrepare::<Impl, IMPL_OFFSET>,
            OnCRMCommit::<Impl, IMPL_OFFSET>,
            OnCRMAbort::<Impl, IMPL_OFFSET>,
            OnCRMIndoubt::<Impl, IMPL_OFFSET>,
            OnCRMDone::<Impl, IMPL_OFFSET>,
            OnCRMRelease::<Impl, IMPL_OFFSET>,
            OnCRMAnalyze::<Impl, IMPL_OFFSET>,
            OnCRMWrite::<Impl, IMPL_OFFSET>,
            OnCRMForget::<Impl, IMPL_OFFSET>,
            OnCRMForce::<Impl, IMPL_OFFSET>,
            OnCRMDeliver::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComCRMEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComExceptionEventsImpl: Sized {
    fn OnExceptionUser();
}
#[cfg(feature = "Win32_Foundation")]
impl IComExceptionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComExceptionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComExceptionEventsVtbl {
        unsafe extern "system" fn OnExceptionUser<Impl: IComExceptionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnExceptionUser::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComExceptionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComIdentityEventsImpl: Sized {
    fn OnIISRequestInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl IComIdentityEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComIdentityEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComIdentityEventsVtbl {
        unsafe extern "system" fn OnIISRequestInfo<Impl: IComIdentityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: super::super::Foundation::PWSTR, pszserverip: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnIISRequestInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComIdentityEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComInstance2EventsImpl: Sized {
    fn OnObjectCreate2();
    fn OnObjectDestroy2();
}
#[cfg(feature = "Win32_Foundation")]
impl IComInstance2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComInstance2EventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComInstance2EventsVtbl {
        unsafe extern "system" fn OnObjectCreate2<Impl: IComInstance2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjectDestroy2<Impl: IComInstance2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnObjectCreate2::<Impl, IMPL_OFFSET>, OnObjectDestroy2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComInstance2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComInstanceEventsImpl: Sized {
    fn OnObjectCreate();
    fn OnObjectDestroy();
}
#[cfg(feature = "Win32_Foundation")]
impl IComInstanceEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComInstanceEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComInstanceEventsVtbl {
        unsafe extern "system" fn OnObjectCreate<Impl: IComInstanceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjectDestroy<Impl: IComInstanceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnObjectCreate::<Impl, IMPL_OFFSET>, OnObjectDestroy::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComInstanceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComLTxEventsImpl: Sized {
    fn OnLtxTransactionStart();
    fn OnLtxTransactionPrepare();
    fn OnLtxTransactionAbort();
    fn OnLtxTransactionCommit();
    fn OnLtxTransactionPromote();
}
#[cfg(feature = "Win32_Foundation")]
impl IComLTxEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComLTxEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComLTxEventsVtbl {
        unsafe extern "system" fn OnLtxTransactionStart<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, tsid: ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLtxTransactionPrepare<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLtxTransactionAbort<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLtxTransactionCommit<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLtxTransactionPromote<Impl: IComLTxEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, txnid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnLtxTransactionStart::<Impl, IMPL_OFFSET>, OnLtxTransactionPrepare::<Impl, IMPL_OFFSET>, OnLtxTransactionAbort::<Impl, IMPL_OFFSET>, OnLtxTransactionCommit::<Impl, IMPL_OFFSET>, OnLtxTransactionPromote::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComLTxEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComMethod2EventsImpl: Sized {
    fn OnMethodCall2();
    fn OnMethodReturn2();
    fn OnMethodException2();
}
#[cfg(feature = "Win32_Foundation")]
impl IComMethod2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComMethod2EventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComMethod2EventsVtbl {
        unsafe extern "system" fn OnMethodCall2<Impl: IComMethod2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnMethodReturn2<Impl: IComMethod2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnMethodException2<Impl: IComMethod2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnMethodCall2::<Impl, IMPL_OFFSET>, OnMethodReturn2::<Impl, IMPL_OFFSET>, OnMethodException2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComMethod2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComMethodEventsImpl: Sized {
    fn OnMethodCall();
    fn OnMethodReturn();
    fn OnMethodException();
}
#[cfg(feature = "Win32_Foundation")]
impl IComMethodEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComMethodEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComMethodEventsVtbl {
        unsafe extern "system" fn OnMethodCall<Impl: IComMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnMethodReturn<Impl: IComMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnMethodException<Impl: IComMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnMethodCall::<Impl, IMPL_OFFSET>, OnMethodReturn::<Impl, IMPL_OFFSET>, OnMethodException::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComMethodEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComMtaThreadPoolKnobsImpl: Sized {
    fn MTASetMaxThreadCount();
    fn MTAGetMaxThreadCount();
    fn MTASetThrottleValue();
    fn MTAGetThrottleValue();
}
impl IComMtaThreadPoolKnobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComMtaThreadPoolKnobsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComMtaThreadPoolKnobsVtbl {
        unsafe extern "system" fn MTASetMaxThreadCount<Impl: IComMtaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MTAGetMaxThreadCount<Impl: IComMtaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MTASetThrottleValue<Impl: IComMtaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthrottle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MTAGetThrottleValue<Impl: IComMtaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthrottle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, MTASetMaxThreadCount::<Impl, IMPL_OFFSET>, MTAGetMaxThreadCount::<Impl, IMPL_OFFSET>, MTASetThrottleValue::<Impl, IMPL_OFFSET>, MTAGetThrottleValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComMtaThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectConstruction2EventsImpl: Sized {
    fn OnObjectConstruct2();
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectConstruction2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectConstruction2EventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectConstruction2EventsVtbl {
        unsafe extern "system" fn OnObjectConstruct2<Impl: IComObjectConstruction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnObjectConstruct2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectConstruction2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectConstructionEventsImpl: Sized {
    fn OnObjectConstruct();
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectConstructionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectConstructionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectConstructionEventsVtbl {
        unsafe extern "system" fn OnObjectConstruct<Impl: IComObjectConstructionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnObjectConstruct::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectConstructionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectEventsImpl: Sized {
    fn OnObjectActivate();
    fn OnObjectDeactivate();
    fn OnDisableCommit();
    fn OnEnableCommit();
    fn OnSetComplete();
    fn OnSetAbort();
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectEventsVtbl {
        unsafe extern "system" fn OnObjectActivate<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjectDeactivate<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDisableCommit<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEnableCommit<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSetComplete<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnSetAbort<Impl: IComObjectEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnObjectActivate::<Impl, IMPL_OFFSET>, OnObjectDeactivate::<Impl, IMPL_OFFSET>, OnDisableCommit::<Impl, IMPL_OFFSET>, OnEnableCommit::<Impl, IMPL_OFFSET>, OnSetComplete::<Impl, IMPL_OFFSET>, OnSetAbort::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectPool2EventsImpl: Sized {
    fn OnObjPoolPutObject2();
    fn OnObjPoolGetObject2();
    fn OnObjPoolRecycleToTx2();
    fn OnObjPoolGetFromTx2();
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectPool2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectPool2EventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectPool2EventsVtbl {
        unsafe extern "system" fn OnObjPoolPutObject2<Impl: IComObjectPool2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolGetObject2<Impl: IComObjectPool2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx2<Impl: IComObjectPool2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx2<Impl: IComObjectPool2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnObjPoolPutObject2::<Impl, IMPL_OFFSET>, OnObjPoolGetObject2::<Impl, IMPL_OFFSET>, OnObjPoolRecycleToTx2::<Impl, IMPL_OFFSET>, OnObjPoolGetFromTx2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectPool2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectPoolEventsImpl: Sized {
    fn OnObjPoolPutObject();
    fn OnObjPoolGetObject();
    fn OnObjPoolRecycleToTx();
    fn OnObjPoolGetFromTx();
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectPoolEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectPoolEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectPoolEventsVtbl {
        unsafe extern "system" fn OnObjPoolPutObject<Impl: IComObjectPoolEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolGetObject<Impl: IComObjectPoolEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx<Impl: IComObjectPoolEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx<Impl: IComObjectPoolEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnObjPoolPutObject::<Impl, IMPL_OFFSET>, OnObjPoolGetObject::<Impl, IMPL_OFFSET>, OnObjPoolRecycleToTx::<Impl, IMPL_OFFSET>, OnObjPoolGetFromTx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectPoolEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectPoolEvents2Impl: Sized {
    fn OnObjPoolCreateObject();
    fn OnObjPoolDestroyObject();
    fn OnObjPoolCreateDecision();
    fn OnObjPoolTimeout();
    fn OnObjPoolCreatePool();
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectPoolEvents2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectPoolEvents2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectPoolEvents2Vtbl {
        unsafe extern "system" fn OnObjPoolCreateObject<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolDestroyObject<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolCreateDecision<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolTimeout<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, guidactivity: *const ::windows::core::GUID, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnObjPoolCreatePool<Impl: IComObjectPoolEvents2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnObjPoolCreateObject::<Impl, IMPL_OFFSET>, OnObjPoolDestroyObject::<Impl, IMPL_OFFSET>, OnObjPoolCreateDecision::<Impl, IMPL_OFFSET>, OnObjPoolTimeout::<Impl, IMPL_OFFSET>, OnObjPoolCreatePool::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectPoolEvents2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComQCEventsImpl: Sized {
    fn OnQCRecord();
    fn OnQCQueueOpen();
    fn OnQCReceive();
    fn OnQCReceiveFail();
    fn OnQCMoveToReTryQueue();
    fn OnQCMoveToDeadQueue();
    fn OnQCPlayback();
}
#[cfg(feature = "Win32_Foundation")]
impl IComQCEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComQCEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComQCEventsVtbl {
        unsafe extern "system" fn OnQCRecord<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: super::super::Foundation::PWSTR, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnQCQueueOpen<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: super::super::Foundation::PWSTR, queueid: u64, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnQCReceive<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnQCReceiveFail<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnQCMoveToReTryQueue<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, retryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnQCMoveToDeadQueue<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnQCPlayback<Impl: IComQCEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnQCRecord::<Impl, IMPL_OFFSET>, OnQCQueueOpen::<Impl, IMPL_OFFSET>, OnQCReceive::<Impl, IMPL_OFFSET>, OnQCReceiveFail::<Impl, IMPL_OFFSET>, OnQCMoveToReTryQueue::<Impl, IMPL_OFFSET>, OnQCMoveToDeadQueue::<Impl, IMPL_OFFSET>, OnQCPlayback::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComQCEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComResourceEventsImpl: Sized {
    fn OnResourceCreate();
    fn OnResourceAllocate();
    fn OnResourceRecycle();
    fn OnResourceDestroy();
    fn OnResourceTrack();
}
#[cfg(feature = "Win32_Foundation")]
impl IComResourceEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComResourceEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComResourceEventsVtbl {
        unsafe extern "system" fn OnResourceCreate<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnResourceAllocate<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnResourceRecycle<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnResourceDestroy<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::core::HRESULT, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnResourceTrack<Impl: IComResourceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnResourceCreate::<Impl, IMPL_OFFSET>, OnResourceAllocate::<Impl, IMPL_OFFSET>, OnResourceRecycle::<Impl, IMPL_OFFSET>, OnResourceDestroy::<Impl, IMPL_OFFSET>, OnResourceTrack::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComResourceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComSecurityEventsImpl: Sized {
    fn OnAuthenticate();
    fn OnAuthenticateFail();
}
#[cfg(feature = "Win32_Foundation")]
impl IComSecurityEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComSecurityEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComSecurityEventsVtbl {
        unsafe extern "system" fn OnAuthenticate<Impl: IComSecurityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnAuthenticateFail<Impl: IComSecurityEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnAuthenticate::<Impl, IMPL_OFFSET>, OnAuthenticateFail::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComSecurityEvents as ::windows::core::Interface>::IID
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
impl IComStaThreadPoolKnobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComStaThreadPoolKnobsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComStaThreadPoolKnobsVtbl {
        unsafe extern "system" fn SetMinThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMinThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActivityPerThread<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActivityPerThread<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActivityRatio<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActivityRatio<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetThreadCount<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQueueDepth<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwqdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQueueDepth<Impl: IComStaThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqdepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetMinThreadCount::<Impl, IMPL_OFFSET>,
            GetMinThreadCount::<Impl, IMPL_OFFSET>,
            SetMaxThreadCount::<Impl, IMPL_OFFSET>,
            GetMaxThreadCount::<Impl, IMPL_OFFSET>,
            SetActivityPerThread::<Impl, IMPL_OFFSET>,
            GetActivityPerThread::<Impl, IMPL_OFFSET>,
            SetActivityRatio::<Impl, IMPL_OFFSET>,
            GetActivityRatio::<Impl, IMPL_OFFSET>,
            GetThreadCount::<Impl, IMPL_OFFSET>,
            GetQueueDepth::<Impl, IMPL_OFFSET>,
            SetQueueDepth::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IComStaThreadPoolKnobs2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComStaThreadPoolKnobs2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComStaThreadPoolKnobs2Vtbl {
        unsafe extern "system" fn GetMaxCPULoad<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxCPULoad<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCPUMetricEnabled<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCPUMetricEnabled<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCreateThreadsAggressively<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCreateThreadsAggressively<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxCSR<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcsr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxCSR<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcsr: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetWaitTimeForThreadCleanup<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWaitTimeForThreadCleanup<Impl: IComStaThreadPoolKnobs2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadcleanupwaittime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetMinThreadCount::<Impl, IMPL_OFFSET>,
            GetMinThreadCount::<Impl, IMPL_OFFSET>,
            SetMaxThreadCount::<Impl, IMPL_OFFSET>,
            GetMaxThreadCount::<Impl, IMPL_OFFSET>,
            SetActivityPerThread::<Impl, IMPL_OFFSET>,
            GetActivityPerThread::<Impl, IMPL_OFFSET>,
            SetActivityRatio::<Impl, IMPL_OFFSET>,
            GetActivityRatio::<Impl, IMPL_OFFSET>,
            GetThreadCount::<Impl, IMPL_OFFSET>,
            GetQueueDepth::<Impl, IMPL_OFFSET>,
            SetQueueDepth::<Impl, IMPL_OFFSET>,
            GetMaxCPULoad::<Impl, IMPL_OFFSET>,
            SetMaxCPULoad::<Impl, IMPL_OFFSET>,
            GetCPUMetricEnabled::<Impl, IMPL_OFFSET>,
            SetCPUMetricEnabled::<Impl, IMPL_OFFSET>,
            GetCreateThreadsAggressively::<Impl, IMPL_OFFSET>,
            SetCreateThreadsAggressively::<Impl, IMPL_OFFSET>,
            GetMaxCSR::<Impl, IMPL_OFFSET>,
            SetMaxCSR::<Impl, IMPL_OFFSET>,
            GetWaitTimeForThreadCleanup::<Impl, IMPL_OFFSET>,
            SetWaitTimeForThreadCleanup::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IComThreadEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComThreadEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComThreadEventsVtbl {
        unsafe extern "system" fn OnThreadStart<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadTerminate<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadBindToApartment<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadUnBind<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadWorkEnque<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadWorkPrivate<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadWorkPublic<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadWorkRedirect<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadWorkReject<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadAssignApartment<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, aptid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnThreadUnassignApartment<Impl: IComThreadEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            OnThreadStart::<Impl, IMPL_OFFSET>,
            OnThreadTerminate::<Impl, IMPL_OFFSET>,
            OnThreadBindToApartment::<Impl, IMPL_OFFSET>,
            OnThreadUnBind::<Impl, IMPL_OFFSET>,
            OnThreadWorkEnque::<Impl, IMPL_OFFSET>,
            OnThreadWorkPrivate::<Impl, IMPL_OFFSET>,
            OnThreadWorkPublic::<Impl, IMPL_OFFSET>,
            OnThreadWorkRedirect::<Impl, IMPL_OFFSET>,
            OnThreadWorkReject::<Impl, IMPL_OFFSET>,
            OnThreadAssignApartment::<Impl, IMPL_OFFSET>,
            OnThreadUnassignApartment::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComThreadEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComTrackingInfoCollectionImpl: Sized {
    fn Type();
    fn Count();
    fn Item();
}
impl IComTrackingInfoCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTrackingInfoCollectionVtbl {
        unsafe extern "system" fn Type<Impl: IComTrackingInfoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IComTrackingInfoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IComTrackingInfoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Type::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoCollection as ::windows::core::Interface>::IID
    }
}
pub trait IComTrackingInfoEventsImpl: Sized {
    fn OnNewTrackingInfo();
}
impl IComTrackingInfoEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTrackingInfoEventsVtbl {
        unsafe extern "system" fn OnNewTrackingInfo<Impl: IComTrackingInfoEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoplevelcollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnNewTrackingInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComTrackingInfoObjectImpl: Sized {
    fn GetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComTrackingInfoObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTrackingInfoObjectVtbl {
        unsafe extern "system" fn GetValue<Impl: IComTrackingInfoObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szpropertyname: super::super::Foundation::PWSTR, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComTrackingInfoPropertiesImpl: Sized {
    fn PropCount();
    fn GetPropName();
}
#[cfg(feature = "Win32_Foundation")]
impl IComTrackingInfoPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTrackingInfoPropertiesVtbl {
        unsafe extern "system" fn PropCount<Impl: IComTrackingInfoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropName<Impl: IComTrackingInfoPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppszpropname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, PropCount::<Impl, IMPL_OFFSET>, GetPropName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransaction2EventsImpl: Sized {
    fn OnTransactionStart2();
    fn OnTransactionPrepare2();
    fn OnTransactionAbort2();
    fn OnTransactionCommit2();
}
#[cfg(feature = "Win32_Foundation")]
impl IComTransaction2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTransaction2EventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTransaction2EventsVtbl {
        unsafe extern "system" fn OnTransactionStart2<Impl: IComTransaction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTransactionPrepare2<Impl: IComTransaction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTransactionAbort2<Impl: IComTransaction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTransactionCommit2<Impl: IComTransaction2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnTransactionStart2::<Impl, IMPL_OFFSET>, OnTransactionPrepare2::<Impl, IMPL_OFFSET>, OnTransactionAbort2::<Impl, IMPL_OFFSET>, OnTransactionCommit2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTransaction2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransactionEventsImpl: Sized {
    fn OnTransactionStart();
    fn OnTransactionPrepare();
    fn OnTransactionAbort();
    fn OnTransactionCommit();
}
#[cfg(feature = "Win32_Foundation")]
impl IComTransactionEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTransactionEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTransactionEventsVtbl {
        unsafe extern "system" fn OnTransactionStart<Impl: IComTransactionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTransactionPrepare<Impl: IComTransactionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTransactionAbort<Impl: IComTransactionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnTransactionCommit<Impl: IComTransactionEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnTransactionStart::<Impl, IMPL_OFFSET>, OnTransactionPrepare::<Impl, IMPL_OFFSET>, OnTransactionAbort::<Impl, IMPL_OFFSET>, OnTransactionCommit::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTransactionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComUserEventImpl: Sized {
    fn OnUserEvent();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComUserEventVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComUserEventImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComUserEventVtbl {
        unsafe extern "system" fn OnUserEvent<Impl: IComUserEventImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnUserEvent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComUserEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IContextPropertiesImpl: Sized {
    fn Count();
    fn GetProperty();
    fn EnumNames();
    fn SetProperty();
    fn RemoveProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IContextPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextPropertiesVtbl {
        unsafe extern "system" fn Count<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumNames<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperty<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveProperty<Impl: IContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Count::<Impl, IMPL_OFFSET>, GetProperty::<Impl, IMPL_OFFSET>, EnumNames::<Impl, IMPL_OFFSET>, SetProperty::<Impl, IMPL_OFFSET>, RemoveProperty::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContextSecurityPerimeterImpl: Sized {
    fn GetPerimeterFlag();
    fn SetPerimeterFlag();
}
#[cfg(feature = "Win32_Foundation")]
impl IContextSecurityPerimeterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextSecurityPerimeterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextSecurityPerimeterVtbl {
        unsafe extern "system" fn GetPerimeterFlag<Impl: IContextSecurityPerimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflag: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPerimeterFlag<Impl: IContextSecurityPerimeterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPerimeterFlag::<Impl, IMPL_OFFSET>, SetPerimeterFlag::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextSecurityPerimeter as ::windows::core::Interface>::IID
    }
}
pub trait IContextStateImpl: Sized {
    fn SetDeactivateOnReturn();
    fn GetDeactivateOnReturn();
    fn SetMyTransactionVote();
    fn GetMyTransactionVote();
}
impl IContextStateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextStateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextStateVtbl {
        unsafe extern "system" fn SetDeactivateOnReturn<Impl: IContextStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdeactivate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeactivateOnReturn<Impl: IContextStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeactivate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMyTransactionVote<Impl: IContextStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, txvote: TransactionVote) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMyTransactionVote<Impl: IContextStateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxvote: *mut TransactionVote) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetDeactivateOnReturn::<Impl, IMPL_OFFSET>, GetDeactivateOnReturn::<Impl, IMPL_OFFSET>, SetMyTransactionVote::<Impl, IMPL_OFFSET>, GetMyTransactionVote::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextState as ::windows::core::Interface>::IID
    }
}
pub trait ICreateWithLocalTransactionImpl: Sized {
    fn CreateInstanceWithSysTx();
}
impl ICreateWithLocalTransactionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateWithLocalTransactionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateWithLocalTransactionVtbl {
        unsafe extern "system" fn CreateInstanceWithSysTx<Impl: ICreateWithLocalTransactionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateInstanceWithSysTx::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateWithLocalTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICreateWithTipTransactionExImpl: Sized {
    fn CreateInstance();
}
#[cfg(feature = "Win32_Foundation")]
impl ICreateWithTipTransactionExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateWithTipTransactionExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateWithTipTransactionExVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICreateWithTipTransactionExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtipurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateWithTipTransactionEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ICreateWithTransactionExImpl: Sized {
    fn CreateInstance();
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ICreateWithTransactionExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateWithTransactionExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateWithTransactionExVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICreateWithTransactionExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateWithTransactionEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICrmCompensatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmCompensatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmCompensatorVtbl {
        unsafe extern "system" fn SetLogControl<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginPrepare<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrepareRecord<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndPrepare<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfoktoprepare: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginCommit<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitRecord<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndCommit<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginAbort<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbortRecord<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndAbort<Impl: ICrmCompensatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetLogControl::<Impl, IMPL_OFFSET>,
            BeginPrepare::<Impl, IMPL_OFFSET>,
            PrepareRecord::<Impl, IMPL_OFFSET>,
            EndPrepare::<Impl, IMPL_OFFSET>,
            BeginCommit::<Impl, IMPL_OFFSET>,
            CommitRecord::<Impl, IMPL_OFFSET>,
            EndCommit::<Impl, IMPL_OFFSET>,
            BeginAbort::<Impl, IMPL_OFFSET>,
            AbortRecord::<Impl, IMPL_OFFSET>,
            EndAbort::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmCompensator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmCompensatorVariantsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmCompensatorVariantsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmCompensatorVariantsVtbl {
        unsafe extern "system" fn SetLogControlVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginPrepareVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrepareRecordVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndPrepareVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboktoprepare: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginCommitVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitRecordVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndCommitVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BeginAbortVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AbortRecordVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndAbortVariants<Impl: ICrmCompensatorVariantsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetLogControlVariants::<Impl, IMPL_OFFSET>,
            BeginPrepareVariants::<Impl, IMPL_OFFSET>,
            PrepareRecordVariants::<Impl, IMPL_OFFSET>,
            EndPrepareVariants::<Impl, IMPL_OFFSET>,
            BeginCommitVariants::<Impl, IMPL_OFFSET>,
            CommitRecordVariants::<Impl, IMPL_OFFSET>,
            EndCommitVariants::<Impl, IMPL_OFFSET>,
            BeginAbortVariants::<Impl, IMPL_OFFSET>,
            AbortRecordVariants::<Impl, IMPL_OFFSET>,
            EndAbortVariants::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmCompensatorVariants as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmFormatLogRecordsImpl: Sized {
    fn GetColumnCount();
    fn GetColumnHeaders();
    fn GetColumn();
    fn GetColumnVariants();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmFormatLogRecordsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmFormatLogRecordsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmFormatLogRecordsVtbl {
        unsafe extern "system" fn GetColumnCount<Impl: ICrmFormatLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcolumncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnHeaders<Impl: ICrmFormatLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheaders: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumn<Impl: ICrmFormatLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetColumnVariants<Impl: ICrmFormatLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logrecord: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetColumnCount::<Impl, IMPL_OFFSET>, GetColumnHeaders::<Impl, IMPL_OFFSET>, GetColumn::<Impl, IMPL_OFFSET>, GetColumnVariants::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmFormatLogRecords as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmLogControlImpl: Sized {
    fn TransactionUOW();
    fn RegisterCompensator();
    fn WriteLogRecordVariants();
    fn ForceLog();
    fn ForgetLogRecord();
    fn ForceTransactionToAbort();
    fn WriteLogRecord();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmLogControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmLogControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmLogControlVtbl {
        unsafe extern "system" fn TransactionUOW<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterCompensator<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcwstrprogidcompensator: super::super::Foundation::PWSTR, lpcwstrdescription: super::super::Foundation::PWSTR, lcrmregflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteLogRecordVariants<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForceLog<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForgetLogRecord<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForceTransactionToAbort<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteLogRecord<Impl: ICrmLogControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, TransactionUOW::<Impl, IMPL_OFFSET>, RegisterCompensator::<Impl, IMPL_OFFSET>, WriteLogRecordVariants::<Impl, IMPL_OFFSET>, ForceLog::<Impl, IMPL_OFFSET>, ForgetLogRecord::<Impl, IMPL_OFFSET>, ForceTransactionToAbort::<Impl, IMPL_OFFSET>, WriteLogRecord::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmLogControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmMonitorImpl: Sized {
    fn GetClerks();
    fn HoldClerk();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmMonitorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmMonitorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmMonitorVtbl {
        unsafe extern "system" fn GetClerks<Impl: ICrmMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclerks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HoldClerk<Impl: ICrmMonitorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetClerks::<Impl, IMPL_OFFSET>, HoldClerk::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmMonitorClerksImpl: Sized + IDispatchImpl {
    fn Item();
    fn _NewEnum();
    fn Count();
    fn ProgIdCompensator();
    fn Description();
    fn TransactionUOW();
    fn ActivityId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmMonitorClerksVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmMonitorClerksImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmMonitorClerksVtbl {
        unsafe extern "system" fn Item<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProgIdCompensator<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionUOW<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActivityId<Impl: ICrmMonitorClerksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            ProgIdCompensator::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            TransactionUOW::<Impl, IMPL_OFFSET>,
            ActivityId::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmMonitorClerks as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmMonitorLogRecordsImpl: Sized {
    fn Count();
    fn TransactionState();
    fn StructuredRecords();
    fn GetLogRecord();
    fn GetLogRecordVariants();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmMonitorLogRecordsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmMonitorLogRecordsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmMonitorLogRecordsVtbl {
        unsafe extern "system" fn Count<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionState<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut CrmTransactionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StructuredRecords<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLogRecord<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLogRecordVariants<Impl: ICrmMonitorLogRecordsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexnumber: ::core::mem::ManuallyDrop<super::Com::VARIANT>, plogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Count::<Impl, IMPL_OFFSET>, TransactionState::<Impl, IMPL_OFFSET>, StructuredRecords::<Impl, IMPL_OFFSET>, GetLogRecord::<Impl, IMPL_OFFSET>, GetLogRecordVariants::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmMonitorLogRecords as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDispenserDriverImpl: Sized {
    fn CreateResource();
    fn RateResource();
    fn EnlistResource();
    fn ResetResource();
    fn DestroyResource();
    fn DestroyResourceS();
}
#[cfg(feature = "Win32_Foundation")]
impl IDispenserDriverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispenserDriverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispenserDriverVtbl {
        unsafe extern "system" fn CreateResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RateResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnlistResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize, transid: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyResource<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyResourceS<Impl: IDispenserDriverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateResource::<Impl, IMPL_OFFSET>, RateResource::<Impl, IMPL_OFFSET>, EnlistResource::<Impl, IMPL_OFFSET>, ResetResource::<Impl, IMPL_OFFSET>, DestroyResource::<Impl, IMPL_OFFSET>, DestroyResourceS::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispenserDriver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDispenserManagerImpl: Sized {
    fn RegisterDispenser();
    fn GetContext();
}
#[cfg(feature = "Win32_Foundation")]
impl IDispenserManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispenserManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispenserManagerVtbl {
        unsafe extern "system" fn RegisterDispenser<Impl: IDispenserManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0000: ::windows::core::RawPtr, szdispensername: super::super::Foundation::PWSTR, __midl__idispensermanager0001: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContext<Impl: IDispenserManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterDispenser::<Impl, IMPL_OFFSET>, GetContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispenserManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumNamesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumNamesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNamesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNamesVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNamesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNames as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEventServerTraceImpl: Sized + IDispatchImpl {
    fn StartTraceGuid();
    fn StopTraceGuid();
    fn EnumTraceGuid();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEventServerTraceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventServerTraceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventServerTraceVtbl {
        unsafe extern "system" fn StartTraceGuid<Impl: IEventServerTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopTraceGuid<Impl: IEventServerTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumTraceGuid<Impl: IEventServerTraceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, StartTraceGuid::<Impl, IMPL_OFFSET>, StopTraceGuid::<Impl, IMPL_OFFSET>, EnumTraceGuid::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventServerTrace as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetAppTrackerDataImpl: Sized {
    fn GetApplicationProcesses();
    fn GetApplicationProcessDetails();
    fn GetApplicationsInProcess();
    fn GetComponentsInProcess();
    fn GetComponentDetails();
    fn GetTrackerDataAsCollectionObject();
    fn GetSuggestedPollingInterval();
}
#[cfg(feature = "Win32_Foundation")]
impl IGetAppTrackerDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetAppTrackerDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetAppTrackerDataVtbl {
        unsafe extern "system" fn GetApplicationProcesses<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplicationProcessDetails<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplicationsInProcess<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetComponentsInProcess<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetComponentDetails<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, clsid: *const ::windows::core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrackerDataAsCollectionObject<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toplevelcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSuggestedPollingInterval<Impl: IGetAppTrackerDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pollingintervalinseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetApplicationProcesses::<Impl, IMPL_OFFSET>,
            GetApplicationProcessDetails::<Impl, IMPL_OFFSET>,
            GetApplicationsInProcess::<Impl, IMPL_OFFSET>,
            GetComponentsInProcess::<Impl, IMPL_OFFSET>,
            GetComponentDetails::<Impl, IMPL_OFFSET>,
            GetTrackerDataAsCollectionObject::<Impl, IMPL_OFFSET>,
            GetSuggestedPollingInterval::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetAppTrackerData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGetContextPropertiesImpl: Sized {
    fn Count();
    fn GetProperty();
    fn EnumNames();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGetContextPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetContextPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetContextPropertiesVtbl {
        unsafe extern "system" fn Count<Impl: IGetContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IGetContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumNames<Impl: IGetContextPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Count::<Impl, IMPL_OFFSET>, GetProperty::<Impl, IMPL_OFFSET>, EnumNames::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetContextProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGetSecurityCallContextImpl: Sized + IDispatchImpl {
    fn GetSecurityCallContext();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGetSecurityCallContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetSecurityCallContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetSecurityCallContextVtbl {
        unsafe extern "system" fn GetSecurityCallContext<Impl: IGetSecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetSecurityCallContext::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetSecurityCallContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IHolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolderVtbl {
        unsafe extern "system" fn AllocResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0002: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrackResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0003: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrackResourceS<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0004: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UntrackResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UntrackResourceS<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestDestroyResource<Impl: IHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0009: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AllocResource::<Impl, IMPL_OFFSET>, FreeResource::<Impl, IMPL_OFFSET>, TrackResource::<Impl, IMPL_OFFSET>, TrackResourceS::<Impl, IMPL_OFFSET>, UntrackResource::<Impl, IMPL_OFFSET>, UntrackResourceS::<Impl, IMPL_OFFSET>, Close::<Impl, IMPL_OFFSET>, RequestDestroyResource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILBEventsImpl: Sized {
    fn TargetUp();
    fn TargetDown();
    fn EngineDefined();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILBEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILBEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILBEventsVtbl {
        unsafe extern "system" fn TargetUp<Impl: ILBEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TargetDown<Impl: ILBEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EngineDefined<Impl: ILBEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varpropvalue: *const super::Com::VARIANT, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, TargetUp::<Impl, IMPL_OFFSET>, TargetDown::<Impl, IMPL_OFFSET>, EngineDefined::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILBEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMTSActivityImpl: Sized {
    fn SynchronousCall();
    fn AsyncCall();
    fn Reserved1();
    fn BindToCurrentThread();
    fn UnbindFromThread();
}
impl IMTSActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMTSActivityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMTSActivityVtbl {
        unsafe extern "system" fn SynchronousCall<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsyncCall<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved1<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindToCurrentThread<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnbindFromThread<Impl: IMTSActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SynchronousCall::<Impl, IMPL_OFFSET>, AsyncCall::<Impl, IMPL_OFFSET>, Reserved1::<Impl, IMPL_OFFSET>, BindToCurrentThread::<Impl, IMPL_OFFSET>, UnbindFromThread::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMTSActivity as ::windows::core::Interface>::IID
    }
}
pub trait IMTSCallImpl: Sized {
    fn OnCall();
}
impl IMTSCallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMTSCallImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMTSCallVtbl {
        unsafe extern "system" fn OnCall<Impl: IMTSCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnCall::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMTSCall as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMTSLocatorImpl: Sized + IDispatchImpl {
    fn GetEventDispatcher();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMTSLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMTSLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMTSLocatorVtbl {
        unsafe extern "system" fn GetEventDispatcher<Impl: IMTSLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetEventDispatcher::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMTSLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedActivationEventsImpl: Sized {
    fn CreateManagedStub();
    fn DestroyManagedStub();
}
#[cfg(feature = "Win32_Foundation")]
impl IManagedActivationEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedActivationEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManagedActivationEventsVtbl {
        unsafe extern "system" fn CreateManagedStub<Impl: IManagedActivationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, fdist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestroyManagedStub<Impl: IManagedActivationEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateManagedStub::<Impl, IMPL_OFFSET>, DestroyManagedStub::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedActivationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedObjectInfoImpl: Sized {
    fn GetIUnknown();
    fn GetIObjectControl();
    fn SetInPool();
    fn SetWrapperStrength();
}
#[cfg(feature = "Win32_Foundation")]
impl IManagedObjectInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedObjectInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManagedObjectInfoVtbl {
        unsafe extern "system" fn GetIUnknown<Impl: IManagedObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIObjectControl<Impl: IManagedObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctrl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInPool<Impl: IManagedObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binpool: super::super::Foundation::BOOL, ppooledobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWrapperStrength<Impl: IManagedObjectInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrong: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIUnknown::<Impl, IMPL_OFFSET>, GetIObjectControl::<Impl, IMPL_OFFSET>, SetInPool::<Impl, IMPL_OFFSET>, SetWrapperStrength::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedObjectInfo as ::windows::core::Interface>::IID
    }
}
pub trait IManagedPoolActionImpl: Sized {
    fn LastRelease();
}
impl IManagedPoolActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedPoolActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManagedPoolActionVtbl {
        unsafe extern "system" fn LastRelease<Impl: IManagedPoolActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, LastRelease::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedPoolAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedPooledObjImpl: Sized {
    fn SetHeld();
}
#[cfg(feature = "Win32_Foundation")]
impl IManagedPooledObjVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedPooledObjImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManagedPooledObjVtbl {
        unsafe extern "system" fn SetHeld<Impl: IManagedPooledObjImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, m_bheld: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetHeld::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedPooledObj as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMessageMoverImpl: Sized + IDispatchImpl {
    fn SourcePath();
    fn SetSourcePath();
    fn DestPath();
    fn SetDestPath();
    fn CommitBatchSize();
    fn SetCommitBatchSize();
    fn MoveMessages();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMessageMoverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageMoverImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageMoverVtbl {
        unsafe extern "system" fn SourcePath<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSourcePath<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DestPath<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDestPath<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommitBatchSize<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCommitBatchSize<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveMessages<Impl: IMessageMoverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagesmoved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            SourcePath::<Impl, IMPL_OFFSET>,
            SetSourcePath::<Impl, IMPL_OFFSET>,
            DestPath::<Impl, IMPL_OFFSET>,
            SetDestPath::<Impl, IMPL_OFFSET>,
            CommitBatchSize::<Impl, IMPL_OFFSET>,
            SetCommitBatchSize::<Impl, IMPL_OFFSET>,
            MoveMessages::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageMover as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMtsEventInfoImpl: Sized + IDispatchImpl {
    fn Names();
    fn DisplayName();
    fn EventID();
    fn Count();
    fn Value();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMtsEventInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMtsEventInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMtsEventInfoVtbl {
        unsafe extern "system" fn Names<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisplayName<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventID<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sguideventid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IMtsEventInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Names::<Impl, IMPL_OFFSET>, DisplayName::<Impl, IMPL_OFFSET>, EventID::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMtsEventInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMtsEventsImpl: Sized + IDispatchImpl {
    fn PackageName();
    fn PackageGuid();
    fn PostEvent();
    fn FireEvents();
    fn GetProcessID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMtsEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMtsEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMtsEventsVtbl {
        unsafe extern "system" fn PackageName<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PackageGuid<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostEvent<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FireEvents<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProcessID<Impl: IMtsEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, PackageName::<Impl, IMPL_OFFSET>, PackageGuid::<Impl, IMPL_OFFSET>, PostEvent::<Impl, IMPL_OFFSET>, FireEvents::<Impl, IMPL_OFFSET>, GetProcessID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMtsEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMtsGrpImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn Refresh();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMtsGrpVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMtsGrpImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMtsGrpVtbl {
        unsafe extern "system" fn Count<Impl: IMtsGrpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IMtsGrpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IMtsGrpImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Refresh::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMtsGrp as ::windows::core::Interface>::IID
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
impl IObjPoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjPoolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjPoolVtbl {
        unsafe extern "system" fn Reserved1<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved2<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved3<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved4<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutEndTx<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobj: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved5<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved6<Impl: IObjPoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Reserved1::<Impl, IMPL_OFFSET>, Reserved2::<Impl, IMPL_OFFSET>, Reserved3::<Impl, IMPL_OFFSET>, Reserved4::<Impl, IMPL_OFFSET>, PutEndTx::<Impl, IMPL_OFFSET>, Reserved5::<Impl, IMPL_OFFSET>, Reserved6::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjPool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstructImpl: Sized {
    fn Construct();
}
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstructVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectConstructImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectConstructVtbl {
        unsafe extern "system" fn Construct<Impl: IObjectConstructImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctorobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Construct::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectConstruct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IObjectConstructStringImpl: Sized + IDispatchImpl {
    fn ConstructString();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IObjectConstructStringVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectConstructStringImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectConstructStringVtbl {
        unsafe extern "system" fn ConstructString<Impl: IObjectConstructStringImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ConstructString::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectConstructString as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContextVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetComplete<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAbort<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableCommit<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableCommit<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsInTransaction<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCallerInRole<Impl: IObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>, SetComplete::<Impl, IMPL_OFFSET>, SetAbort::<Impl, IMPL_OFFSET>, EnableCommit::<Impl, IMPL_OFFSET>, DisableCommit::<Impl, IMPL_OFFSET>, IsInTransaction::<Impl, IMPL_OFFSET>, IsSecurityEnabled::<Impl, IMPL_OFFSET>, IsCallerInRole::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContext as ::windows::core::Interface>::IID
    }
}
pub trait IObjectContextActivityImpl: Sized {
    fn GetActivityId();
}
impl IObjectContextActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextActivityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContextActivityVtbl {
        unsafe extern "system" fn GetActivityId<Impl: IObjectContextActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetActivityId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextActivity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfoImpl: Sized {
    fn IsInTransaction();
    fn GetTransaction();
    fn GetTransactionId();
    fn GetActivityId();
    fn GetContextId();
}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContextInfoVtbl {
        unsafe extern "system" fn IsInTransaction<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransaction<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrans: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransactionId<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActivityId<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetContextId<Impl: IObjectContextInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsInTransaction::<Impl, IMPL_OFFSET>, GetTransaction::<Impl, IMPL_OFFSET>, GetTransactionId::<Impl, IMPL_OFFSET>, GetActivityId::<Impl, IMPL_OFFSET>, GetContextId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfo2Impl: Sized + IObjectContextInfoImpl {
    fn GetPartitionId();
    fn GetApplicationId();
    fn GetApplicationInstanceId();
}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextInfo2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextInfo2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContextInfo2Vtbl {
        unsafe extern "system" fn GetPartitionId<Impl: IObjectContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplicationId<Impl: IObjectContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetApplicationInstanceId<Impl: IObjectContextInfo2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsInTransaction::<Impl, IMPL_OFFSET>, GetTransaction::<Impl, IMPL_OFFSET>, GetTransactionId::<Impl, IMPL_OFFSET>, GetActivityId::<Impl, IMPL_OFFSET>, GetContextId::<Impl, IMPL_OFFSET>, GetPartitionId::<Impl, IMPL_OFFSET>, GetApplicationId::<Impl, IMPL_OFFSET>, GetApplicationInstanceId::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextTipImpl: Sized {
    fn GetTipUrl();
}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextTipVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextTipImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContextTipVtbl {
        unsafe extern "system" fn GetTipUrl<Impl: IObjectContextTipImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTipUrl::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextTip as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectControlImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn CanBePooled();
}
#[cfg(feature = "Win32_Foundation")]
impl IObjectControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectControlVtbl {
        unsafe extern "system" fn Activate<Impl: IObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deactivate<Impl: IObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanBePooled<Impl: IObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Activate::<Impl, IMPL_OFFSET>, Deactivate::<Impl, IMPL_OFFSET>, CanBePooled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectControl as ::windows::core::Interface>::IID
    }
}
pub trait IPlaybackControlImpl: Sized {
    fn FinalClientRetry();
    fn FinalServerRetry();
}
impl IPlaybackControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackControlVtbl {
        unsafe extern "system" fn FinalClientRetry<Impl: IPlaybackControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FinalServerRetry<Impl: IPlaybackControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FinalClientRetry::<Impl, IMPL_OFFSET>, FinalServerRetry::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPoolManagerImpl: Sized + IDispatchImpl {
    fn ShutdownPool();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPoolManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPoolManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPoolManagerVtbl {
        unsafe extern "system" fn ShutdownPool<Impl: IPoolManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, ShutdownPool::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPoolManager as ::windows::core::Interface>::IID
    }
}
pub trait IProcessInitializerImpl: Sized {
    fn Startup();
    fn Shutdown();
}
impl IProcessInitializerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessInitializerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessInitializerVtbl {
        unsafe extern "system" fn Startup<Impl: IProcessInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkprocesscontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IProcessInitializerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Startup::<Impl, IMPL_OFFSET>, Shutdown::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISecurityCallContextImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
    fn IsCallerInRole();
    fn IsSecurityEnabled();
    fn IsUserInRole();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISecurityCallContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityCallContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityCallContextVtbl {
        unsafe extern "system" fn Count<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCallerInRole<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsUserInRole<Impl: ISecurityCallContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *const super::Com::VARIANT, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            IsCallerInRole::<Impl, IMPL_OFFSET>,
            IsSecurityEnabled::<Impl, IMPL_OFFSET>,
            IsUserInRole::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityCallContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISecurityCallersCollImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISecurityCallersCollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityCallersCollImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityCallersCollVtbl {
        unsafe extern "system" fn Count<Impl: ISecurityCallersCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISecurityCallersCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ISecurityCallersCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityCallersColl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISecurityIdentityCollImpl: Sized + IDispatchImpl {
    fn Count();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISecurityIdentityCollVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityIdentityCollImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityIdentityCollVtbl {
        unsafe extern "system" fn Count<Impl: ISecurityIdentityCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISecurityIdentityCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ISecurityIdentityCollImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityIdentityColl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityPropertyImpl: Sized {
    fn GetDirectCreatorSID();
    fn GetOriginalCreatorSID();
    fn GetDirectCallerSID();
    fn GetOriginalCallerSID();
    fn ReleaseSID();
}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityPropertyVtbl {
        unsafe extern "system" fn GetDirectCreatorSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOriginalCreatorSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDirectCallerSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOriginalCallerSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseSID<Impl: ISecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDirectCreatorSID::<Impl, IMPL_OFFSET>, GetOriginalCreatorSID::<Impl, IMPL_OFFSET>, GetDirectCallerSID::<Impl, IMPL_OFFSET>, GetOriginalCallerSID::<Impl, IMPL_OFFSET>, ReleaseSID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityProperty as ::windows::core::Interface>::IID
    }
}
pub trait ISelectCOMLBServerImpl: Sized {
    fn Init();
    fn GetLBServer();
}
impl ISelectCOMLBServerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectCOMLBServerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectCOMLBServerVtbl {
        unsafe extern "system" fn Init<Impl: ISelectCOMLBServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLBServer<Impl: ISelectCOMLBServerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Init::<Impl, IMPL_OFFSET>, GetLBServer::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectCOMLBServer as ::windows::core::Interface>::IID
    }
}
pub trait ISendMethodEventsImpl: Sized {
    fn SendMethodCall();
    fn SendMethodReturn();
}
impl ISendMethodEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISendMethodEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISendMethodEventsVtbl {
        unsafe extern "system" fn SendMethodCall<Impl: ISendMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendMethodReturn<Impl: ISendMethodEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32, hrcall: ::windows::core::HRESULT, hrserver: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SendMethodCall::<Impl, IMPL_OFFSET>, SendMethodReturn::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISendMethodEvents as ::windows::core::Interface>::IID
    }
}
pub trait IServiceActivityImpl: Sized {
    fn SynchronousCall();
    fn AsynchronousCall();
    fn BindToCurrentThread();
    fn UnbindFromThread();
}
impl IServiceActivityVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceActivityImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceActivityVtbl {
        unsafe extern "system" fn SynchronousCall<Impl: IServiceActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AsynchronousCall<Impl: IServiceActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BindToCurrentThread<Impl: IServiceActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnbindFromThread<Impl: IServiceActivityImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SynchronousCall::<Impl, IMPL_OFFSET>, AsynchronousCall::<Impl, IMPL_OFFSET>, BindToCurrentThread::<Impl, IMPL_OFFSET>, UnbindFromThread::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceActivity as ::windows::core::Interface>::IID
    }
}
pub trait IServiceCallImpl: Sized {
    fn OnCall();
}
impl IServiceCallVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceCallImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceCallVtbl {
        unsafe extern "system" fn OnCall<Impl: IServiceCallImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnCall::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceCall as ::windows::core::Interface>::IID
    }
}
pub trait IServiceComTIIntrinsicsConfigImpl: Sized {
    fn ComTIIntrinsicsConfig();
}
impl IServiceComTIIntrinsicsConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceComTIIntrinsicsConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceComTIIntrinsicsConfigVtbl {
        unsafe extern "system" fn ComTIIntrinsicsConfig<Impl: IServiceComTIIntrinsicsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ComTIIntrinsicsConfig::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceComTIIntrinsicsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceIISIntrinsicsConfigImpl: Sized {
    fn IISIntrinsicsConfig();
}
impl IServiceIISIntrinsicsConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceIISIntrinsicsConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceIISIntrinsicsConfigVtbl {
        unsafe extern "system" fn IISIntrinsicsConfig<Impl: IServiceIISIntrinsicsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IISIntrinsicsConfig::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceIISIntrinsicsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceInheritanceConfigImpl: Sized {
    fn ContainingContextTreatment();
}
impl IServiceInheritanceConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceInheritanceConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceInheritanceConfigVtbl {
        unsafe extern "system" fn ContainingContextTreatment<Impl: IServiceInheritanceConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ContainingContextTreatment::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceInheritanceConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServicePartitionConfigImpl: Sized {
    fn PartitionConfig();
    fn PartitionID();
}
impl IServicePartitionConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServicePartitionConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServicePartitionConfigVtbl {
        unsafe extern "system" fn PartitionConfig<Impl: IServicePartitionConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PartitionID<Impl: IServicePartitionConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidpartitionid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, PartitionConfig::<Impl, IMPL_OFFSET>, PartitionID::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServicePartitionConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServicePoolImpl: Sized {
    fn Initialize();
    fn GetObject();
    fn Shutdown();
}
impl IServicePoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServicePoolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServicePoolVtbl {
        unsafe extern "system" fn Initialize<Impl: IServicePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoolconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetObject<Impl: IServicePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shutdown<Impl: IServicePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, GetObject::<Impl, IMPL_OFFSET>, Shutdown::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServicePool as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IServicePoolConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServicePoolConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServicePoolConfigVtbl {
        unsafe extern "system" fn SetMaxPoolSize<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxpool: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MaxPoolSize<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxpool: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinPoolSize<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminpool: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MinPoolSize<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminpool: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCreationTimeout<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreationtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreationTimeout<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcreationtimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTransactionAffinity<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftxaffinity: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionAffinity<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClassFactory<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClassFactory<Impl: IServicePoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            SetMaxPoolSize::<Impl, IMPL_OFFSET>,
            MaxPoolSize::<Impl, IMPL_OFFSET>,
            SetMinPoolSize::<Impl, IMPL_OFFSET>,
            MinPoolSize::<Impl, IMPL_OFFSET>,
            SetCreationTimeout::<Impl, IMPL_OFFSET>,
            CreationTimeout::<Impl, IMPL_OFFSET>,
            SetTransactionAffinity::<Impl, IMPL_OFFSET>,
            TransactionAffinity::<Impl, IMPL_OFFSET>,
            SetClassFactory::<Impl, IMPL_OFFSET>,
            ClassFactory::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServicePoolConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IServiceSxsConfigImpl: Sized {
    fn SxsConfig();
    fn SxsName();
    fn SxsDirectory();
}
#[cfg(feature = "Win32_Foundation")]
impl IServiceSxsConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceSxsConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceSxsConfigVtbl {
        unsafe extern "system" fn SxsConfig<Impl: IServiceSxsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scsconfig: CSC_SxsConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SxsName<Impl: IServiceSxsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SxsDirectory<Impl: IServiceSxsConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsdirectory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SxsConfig::<Impl, IMPL_OFFSET>, SxsName::<Impl, IMPL_OFFSET>, SxsDirectory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceSxsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceSynchronizationConfigImpl: Sized {
    fn ConfigureSynchronization();
}
impl IServiceSynchronizationConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceSynchronizationConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceSynchronizationConfigVtbl {
        unsafe extern "system" fn ConfigureSynchronization<Impl: IServiceSynchronizationConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ConfigureSynchronization::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceSynchronizationConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait IServiceSysTxnConfigImpl: Sized + IServiceTransactionConfigImpl + IServiceTransactionConfigBaseImpl {
    fn ConfigureBYOTSysTxn();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl IServiceSysTxnConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceSysTxnConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceSysTxnConfigVtbl {
        unsafe extern "system" fn ConfigureBYOTSysTxn<Impl: IServiceSysTxnConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxproxy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ConfigureTransaction::<Impl, IMPL_OFFSET>, IsolationLevel::<Impl, IMPL_OFFSET>, TransactionTimeout::<Impl, IMPL_OFFSET>, BringYourOwnTransaction::<Impl, IMPL_OFFSET>, NewTransactionDescription::<Impl, IMPL_OFFSET>, ConfigureBYOT::<Impl, IMPL_OFFSET>, ConfigureBYOTSysTxn::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceSysTxnConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceThreadPoolConfigImpl: Sized {
    fn SelectThreadPool();
    fn SetBindingInfo();
}
impl IServiceThreadPoolConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceThreadPoolConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceThreadPoolConfigVtbl {
        unsafe extern "system" fn SelectThreadPool<Impl: IServiceThreadPoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadpool: CSC_ThreadPool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBindingInfo<Impl: IServiceThreadPoolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: CSC_Binding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SelectThreadPool::<Impl, IMPL_OFFSET>, SetBindingInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceThreadPoolConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IServiceTrackerConfigImpl: Sized {
    fn TrackerConfig();
}
#[cfg(feature = "Win32_Foundation")]
impl IServiceTrackerConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceTrackerConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceTrackerConfigVtbl {
        unsafe extern "system" fn TrackerConfig<Impl: IServiceTrackerConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: super::super::Foundation::PWSTR, sztrackerctxname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, TrackerConfig::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceTrackerConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait IServiceTransactionConfigImpl: Sized + IServiceTransactionConfigBaseImpl {
    fn ConfigureBYOT();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl IServiceTransactionConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceTransactionConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceTransactionConfigVtbl {
        unsafe extern "system" fn ConfigureBYOT<Impl: IServiceTransactionConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitxbyot: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ConfigureTransaction::<Impl, IMPL_OFFSET>, IsolationLevel::<Impl, IMPL_OFFSET>, TransactionTimeout::<Impl, IMPL_OFFSET>, BringYourOwnTransaction::<Impl, IMPL_OFFSET>, NewTransactionDescription::<Impl, IMPL_OFFSET>, ConfigureBYOT::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceTransactionConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IServiceTransactionConfigBaseImpl: Sized {
    fn ConfigureTransaction();
    fn IsolationLevel();
    fn TransactionTimeout();
    fn BringYourOwnTransaction();
    fn NewTransactionDescription();
}
#[cfg(feature = "Win32_Foundation")]
impl IServiceTransactionConfigBaseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceTransactionConfigBaseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceTransactionConfigBaseVtbl {
        unsafe extern "system" fn ConfigureTransaction<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsolationLevel<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TransactionTimeout<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BringYourOwnTransaction<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztipurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NewTransactionDescription<Impl: IServiceTransactionConfigBaseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztxdesc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ConfigureTransaction::<Impl, IMPL_OFFSET>, IsolationLevel::<Impl, IMPL_OFFSET>, TransactionTimeout::<Impl, IMPL_OFFSET>, BringYourOwnTransaction::<Impl, IMPL_OFFSET>, NewTransactionDescription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceTransactionConfigBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISharedPropertyImpl: Sized + IDispatchImpl {
    fn Value();
    fn SetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISharedPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPropertyVtbl {
        unsafe extern "system" fn Value<Impl: ISharedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: ISharedPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISharedPropertyGroupImpl: Sized + IDispatchImpl {
    fn CreatePropertyByPosition();
    fn PropertyByPosition();
    fn CreateProperty();
    fn Property();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISharedPropertyGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPropertyGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPropertyGroupVtbl {
        unsafe extern "system" fn CreatePropertyByPosition<Impl: ISharedPropertyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, fexists: *mut i16, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PropertyByPosition<Impl: ISharedPropertyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateProperty<Impl: ISharedPropertyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fexists: *mut i16, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Property<Impl: ISharedPropertyGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CreatePropertyByPosition::<Impl, IMPL_OFFSET>, PropertyByPosition::<Impl, IMPL_OFFSET>, CreateProperty::<Impl, IMPL_OFFSET>, Property::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPropertyGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISharedPropertyGroupManagerImpl: Sized + IDispatchImpl {
    fn CreatePropertyGroup();
    fn Group();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISharedPropertyGroupManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPropertyGroupManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPropertyGroupManagerVtbl {
        unsafe extern "system" fn CreatePropertyGroup<Impl: ISharedPropertyGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Group<Impl: ISharedPropertyGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ISharedPropertyGroupManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CreatePropertyGroup::<Impl, IMPL_OFFSET>, Group::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPropertyGroupManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISystemAppEventDataImpl: Sized {
    fn Startup();
    fn OnDataChanged();
}
#[cfg(feature = "Win32_Foundation")]
impl ISystemAppEventDataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemAppEventDataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemAppEventDataVtbl {
        unsafe extern "system" fn Startup<Impl: ISystemAppEventDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDataChanged<Impl: ISystemAppEventDataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Startup::<Impl, IMPL_OFFSET>, OnDataChanged::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemAppEventData as ::windows::core::Interface>::IID
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
impl IThreadPoolKnobsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThreadPoolKnobsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThreadPoolKnobsVtbl {
        unsafe extern "system" fn GetMaxThreads<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxthreads: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentThreads<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentthreads: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxThreads<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxthreads: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeleteDelay<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsecdeletedelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDeleteDelay<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msecdeletedelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaxQueuedRequests<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCurrentQueuedRequests<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxQueuedRequests<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxqueuedrequests: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMinThreads<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcminthreads: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetQueueDepth<Impl: IThreadPoolKnobsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcqueuedepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetMaxThreads::<Impl, IMPL_OFFSET>,
            GetCurrentThreads::<Impl, IMPL_OFFSET>,
            SetMaxThreads::<Impl, IMPL_OFFSET>,
            GetDeleteDelay::<Impl, IMPL_OFFSET>,
            SetDeleteDelay::<Impl, IMPL_OFFSET>,
            GetMaxQueuedRequests::<Impl, IMPL_OFFSET>,
            GetCurrentQueuedRequests::<Impl, IMPL_OFFSET>,
            SetMaxQueuedRequests::<Impl, IMPL_OFFSET>,
            SetMinThreads::<Impl, IMPL_OFFSET>,
            SetQueueDepth::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITransactionContextImpl: Sized + IDispatchImpl {
    fn CreateInstance();
    fn Commit();
    fn Abort();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITransactionContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionContextVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITransactionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: ITransactionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abort<Impl: ITransactionContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, Abort::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionContext as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionContextExImpl: Sized {
    fn CreateInstance();
    fn Commit();
    fn Abort();
}
impl ITransactionContextExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionContextExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionContextExVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITransactionContextExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Commit<Impl: ITransactionContextExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abort<Impl: ITransactionContextExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>, Commit::<Impl, IMPL_OFFSET>, Abort::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionContextEx as ::windows::core::Interface>::IID
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
impl ITransactionPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionPropertyVtbl {
        unsafe extern "system" fn Reserved1<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved2<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved3<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved4<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved5<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved6<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved7<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved8<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved9<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransactionResourcePool<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptxpool: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved10<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved11<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved12<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved13<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved14<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved15<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved16<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reserved17<Impl: ITransactionPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Reserved1::<Impl, IMPL_OFFSET>,
            Reserved2::<Impl, IMPL_OFFSET>,
            Reserved3::<Impl, IMPL_OFFSET>,
            Reserved4::<Impl, IMPL_OFFSET>,
            Reserved5::<Impl, IMPL_OFFSET>,
            Reserved6::<Impl, IMPL_OFFSET>,
            Reserved7::<Impl, IMPL_OFFSET>,
            Reserved8::<Impl, IMPL_OFFSET>,
            Reserved9::<Impl, IMPL_OFFSET>,
            GetTransactionResourcePool::<Impl, IMPL_OFFSET>,
            Reserved10::<Impl, IMPL_OFFSET>,
            Reserved11::<Impl, IMPL_OFFSET>,
            Reserved12::<Impl, IMPL_OFFSET>,
            Reserved13::<Impl, IMPL_OFFSET>,
            Reserved14::<Impl, IMPL_OFFSET>,
            Reserved15::<Impl, IMPL_OFFSET>,
            Reserved16::<Impl, IMPL_OFFSET>,
            Reserved17::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait ITransactionProxyImpl: Sized {
    fn Commit();
    fn Abort();
    fn Promote();
    fn CreateVoter();
    fn GetIsolationLevel();
    fn GetIdentifier();
    fn IsReusable();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ITransactionProxyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionProxyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionProxyVtbl {
        unsafe extern "system" fn Commit<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Abort<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Promote<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateVoter<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxasync: ::windows::core::RawPtr, ppballot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIsolationLevel<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIdentifier<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstridentifier: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsReusable<Impl: ITransactionProxyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Commit::<Impl, IMPL_OFFSET>, Abort::<Impl, IMPL_OFFSET>, Promote::<Impl, IMPL_OFFSET>, CreateVoter::<Impl, IMPL_OFFSET>, GetIsolationLevel::<Impl, IMPL_OFFSET>, GetIdentifier::<Impl, IMPL_OFFSET>, IsReusable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionProxy as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionResourcePoolImpl: Sized {
    fn PutResource();
    fn GetResource();
}
impl ITransactionResourcePoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionResourcePoolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionResourcePoolVtbl {
        unsafe extern "system" fn PutResource<Impl: ITransactionResourcePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: ::windows::core::RawPtr, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetResource<Impl: ITransactionResourcePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: ::windows::core::RawPtr, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, PutResource::<Impl, IMPL_OFFSET>, GetResource::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionResourcePool as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionStatusImpl: Sized {
    fn SetTransactionStatus();
    fn GetTransactionStatus();
}
impl ITransactionStatusVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionStatusImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionStatusVtbl {
        unsafe extern "system" fn SetTransactionStatus<Impl: ITransactionStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTransactionStatus<Impl: ITransactionStatusImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetTransactionStatus::<Impl, IMPL_OFFSET>, GetTransactionStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionStatus as ::windows::core::Interface>::IID
    }
}
pub trait ITxProxyHolderImpl: Sized {
    fn GetIdentifier();
}
impl ITxProxyHolderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITxProxyHolderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITxProxyHolderVtbl {
        unsafe extern "system" fn GetIdentifier<Impl: ITxProxyHolderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidltx: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetIdentifier::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITxProxyHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
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
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ObjectContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ObjectContextImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ObjectContextVtbl {
        unsafe extern "system" fn CreateInstance<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetComplete<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAbort<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableCommit<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableCommit<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsInTransaction<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCallerInRole<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Security<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ContextInfo<Impl: ObjectContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontextinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetTypeInfoCount::<Impl, IMPL_OFFSET>,
            GetTypeInfo::<Impl, IMPL_OFFSET>,
            GetIDsOfNames::<Impl, IMPL_OFFSET>,
            Invoke::<Impl, IMPL_OFFSET>,
            CreateInstance::<Impl, IMPL_OFFSET>,
            SetComplete::<Impl, IMPL_OFFSET>,
            SetAbort::<Impl, IMPL_OFFSET>,
            EnableCommit::<Impl, IMPL_OFFSET>,
            DisableCommit::<Impl, IMPL_OFFSET>,
            IsInTransaction::<Impl, IMPL_OFFSET>,
            IsSecurityEnabled::<Impl, IMPL_OFFSET>,
            IsCallerInRole::<Impl, IMPL_OFFSET>,
            Count::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            Security::<Impl, IMPL_OFFSET>,
            ContextInfo::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ObjectContext as ::windows::core::Interface>::IID
    }
}
pub trait ObjectControlImpl: Sized {
    fn Activate();
    fn Deactivate();
    fn CanBePooled();
}
impl ObjectControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ObjectControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ObjectControlVtbl {
        unsafe extern "system" fn Activate<Impl: ObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Deactivate<Impl: ObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CanBePooled<Impl: ObjectControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpoolable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Activate::<Impl, IMPL_OFFSET>, Deactivate::<Impl, IMPL_OFFSET>, CanBePooled::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ObjectControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait SecurityPropertyImpl: Sized + IDispatchImpl {
    fn GetDirectCallerName();
    fn GetDirectCreatorName();
    fn GetOriginalCallerName();
    fn GetOriginalCreatorName();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl SecurityPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: SecurityPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> SecurityPropertyVtbl {
        unsafe extern "system" fn GetDirectCallerName<Impl: SecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDirectCreatorName<Impl: SecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOriginalCallerName<Impl: SecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOriginalCreatorName<Impl: SecurityPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, GetDirectCallerName::<Impl, IMPL_OFFSET>, GetDirectCreatorName::<Impl, IMPL_OFFSET>, GetOriginalCallerName::<Impl, IMPL_OFFSET>, GetOriginalCreatorName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<SecurityProperty as ::windows::core::Interface>::IID
    }
}
