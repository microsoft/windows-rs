#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ContextInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn IsInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn GetTransaction(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetTransactionId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetActivityId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetContextId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ContextInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ContextInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ContextInfo_Vtbl {
        unsafe extern "system" fn IsInTransaction<Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisintx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransaction<Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptx: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pptx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtxid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityId<Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstractivityid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstractivityid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextId<Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrctxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetContextId() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrctxid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsInTransaction: IsInTransaction::<Impl, IMPL_OFFSET>,
            GetTransaction: GetTransaction::<Impl, IMPL_OFFSET>,
            GetTransactionId: GetTransactionId::<Impl, IMPL_OFFSET>,
            GetActivityId: GetActivityId::<Impl, IMPL_OFFSET>,
            GetContextId: GetContextId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ContextInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ContextInfo2_Impl: Sized + super::Com::IDispatch_Impl + ContextInfo_Impl {
    fn GetPartitionId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetApplicationId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetApplicationInstanceId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ContextInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ContextInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ContextInfo2_Vtbl {
        unsafe extern "system" fn GetPartitionId<Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20000: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartitionId() {
                ::core::result::Result::Ok(ok__) => {
                    *__midl__contextinfo20000 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationId<Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20001: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationId() {
                ::core::result::Result::Ok(ok__) => {
                    *__midl__contextinfo20001 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceId<Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20002: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    *__midl__contextinfo20002 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ContextInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPartitionId: GetPartitionId::<Impl, IMPL_OFFSET>,
            GetApplicationId: GetApplicationId::<Impl, IMPL_OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ContextInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAppDomainHelper_Impl: Sized + super::Com::IDispatch_Impl {
    fn Initialize(&mut self, punkad: ::core::option::Option<::windows::core::IUnknown>, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DoCallback(&mut self, punkad: ::core::option::Option<::windows::core::IUnknown>, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAppDomainHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAppDomainHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAppDomainHelper_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&punkad), ::core::mem::transmute_copy(&__midl__iappdomainhelper0000), ::core::mem::transmute_copy(&ppool)).into()
        }
        unsafe extern "system" fn DoCallback<Impl: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DoCallback(::core::mem::transmute(&punkad), ::core::mem::transmute_copy(&__midl__iappdomainhelper0001), ::core::mem::transmute_copy(&ppool)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            DoCallback: DoCallback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDomainHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAssemblyLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetModules(&mut self, applicationdir: super::super::Foundation::BSTR, applicationname: super::super::Foundation::BSTR, assemblyname: super::super::Foundation::BSTR) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAssemblyLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAssemblyLocator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAssemblyLocator_Vtbl {
        unsafe extern "system" fn GetModules<Impl: IAssemblyLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, assemblyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModules(::core::mem::transmute_copy(&applicationdir), ::core::mem::transmute_copy(&applicationname), ::core::mem::transmute_copy(&assemblyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pmodules = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetModules: GetModules::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyLocator as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncErrorNotify_Impl: Sized {
    fn OnError(&mut self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IAsyncErrorNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAsyncErrorNotify_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAsyncErrorNotify_Vtbl {
        unsafe extern "system" fn OnError<Impl: IAsyncErrorNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnError(::core::mem::transmute_copy(&hr)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnError: OnError::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncErrorNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICOMAdminCatalog_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetCollection(&mut self, bstrcollname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Connect(&mut self, bstrcatalogservername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn MajorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn GetCollectionByQuery(&mut self, bstrcollname: super::super::Foundation::BSTR, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<super::Com::IDispatch>;
    fn ImportComponent(&mut self, bstrapplidorname: super::super::Foundation::BSTR, bstrclsidorprogid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InstallComponent(&mut self, bstrapplidorname: super::super::Foundation::BSTR, bstrdll: super::super::Foundation::BSTR, bstrtlb: super::super::Foundation::BSTR, bstrpsdll: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ShutdownApplication(&mut self, bstrapplidorname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExportApplication(&mut self, bstrapplidorname: super::super::Foundation::BSTR, bstrapplicationfile: super::super::Foundation::BSTR, loptions: COMAdminApplicationExportOptions) -> ::windows::core::Result<()>;
    fn InstallApplication(&mut self, bstrapplicationfile: super::super::Foundation::BSTR, bstrdestinationdirectory: super::super::Foundation::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR, bstrrsn: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StopRouter(&mut self) -> ::windows::core::Result<()>;
    fn RefreshRouter(&mut self) -> ::windows::core::Result<()>;
    fn StartRouter(&mut self) -> ::windows::core::Result<()>;
    fn Reserved1(&mut self) -> ::windows::core::Result<()>;
    fn Reserved2(&mut self) -> ::windows::core::Result<()>;
    fn InstallMultipleComponents(&mut self, bstrapplidorname: super::super::Foundation::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GetMultipleComponentsInfo(&mut self, bstrapplidorname: super::super::Foundation::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RefreshComponents(&mut self) -> ::windows::core::Result<()>;
    fn BackupREGDB(&mut self, bstrbackupfilepath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RestoreREGDB(&mut self, bstrbackupfilepath: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn QueryApplicationFile(&mut self, bstrapplicationfile: super::super::Foundation::BSTR, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn StartApplication(&mut self, bstrapplidorname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceCheck(&mut self, lservice: i32) -> ::windows::core::Result<i32>;
    fn InstallMultipleEventClasses(&mut self, bstrapplidorname: super::super::Foundation::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn InstallEventClass(&mut self, bstrapplidorname: super::super::Foundation::BSTR, bstrdll: super::super::Foundation::BSTR, bstrtlb: super::super::Foundation::BSTR, bstrpsdll: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetEventClassesForIID(&mut self, bstriid: super::super::Foundation::BSTR, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICOMAdminCatalog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICOMAdminCatalog_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICOMAdminCatalog_Vtbl {
        unsafe extern "system" fn GetCollection<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollection(::core::mem::transmute_copy(&bstrcollname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcatalogservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Connect(::core::mem::transmute_copy(&bstrcatalogservername)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plmajorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plminorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionByQuery<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollectionByQuery(::core::mem::transmute_copy(&bstrcollname), ::core::mem::transmute_copy(&ppsavarquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportComponent<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportComponent(::core::mem::transmute_copy(&bstrapplidorname), ::core::mem::transmute_copy(&bstrclsidorprogid)).into()
        }
        unsafe extern "system" fn InstallComponent<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallComponent(::core::mem::transmute_copy(&bstrapplidorname), ::core::mem::transmute_copy(&bstrdll), ::core::mem::transmute_copy(&bstrtlb), ::core::mem::transmute_copy(&bstrpsdll)).into()
        }
        unsafe extern "system" fn ShutdownApplication<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutdownApplication(::core::mem::transmute_copy(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ExportApplication<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExportApplication(::core::mem::transmute_copy(&bstrapplidorname), ::core::mem::transmute_copy(&bstrapplicationfile), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallApplication<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallApplication(::core::mem::transmute_copy(&bstrapplicationfile), ::core::mem::transmute_copy(&bstrdestinationdirectory), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute_copy(&bstruserid), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&bstrrsn)).into()
        }
        unsafe extern "system" fn StopRouter<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopRouter().into()
        }
        unsafe extern "system" fn RefreshRouter<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshRouter().into()
        }
        unsafe extern "system" fn StartRouter<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartRouter().into()
        }
        unsafe extern "system" fn Reserved1<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved1().into()
        }
        unsafe extern "system" fn Reserved2<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved2().into()
        }
        unsafe extern "system" fn InstallMultipleComponents<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallMultipleComponents(::core::mem::transmute_copy(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMultipleComponentsInfo(::core::mem::transmute_copy(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids), ::core::mem::transmute_copy(&ppsavarclassnames), ::core::mem::transmute_copy(&ppsavarfileflags), ::core::mem::transmute_copy(&ppsavarcomponentflags)).into()
        }
        unsafe extern "system" fn RefreshComponents<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RefreshComponents().into()
        }
        unsafe extern "system" fn BackupREGDB<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BackupREGDB(::core::mem::transmute_copy(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn RestoreREGDB<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreREGDB(::core::mem::transmute_copy(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn QueryApplicationFile<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryApplicationFile(::core::mem::transmute_copy(&bstrapplicationfile), ::core::mem::transmute_copy(&pbstrapplicationname), ::core::mem::transmute_copy(&pbstrapplicationdescription), ::core::mem::transmute_copy(&pbhasusers), ::core::mem::transmute_copy(&pbisproxy), ::core::mem::transmute_copy(&ppsavarfilenames)).into()
        }
        unsafe extern "system" fn StartApplication<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartApplication(::core::mem::transmute_copy(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ServiceCheck<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ServiceCheck(::core::mem::transmute_copy(&lservice)) {
                ::core::result::Result::Ok(ok__) => {
                    *plstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallMultipleEventClasses(::core::mem::transmute_copy(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn InstallEventClass<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallEventClass(::core::mem::transmute_copy(&bstrapplidorname), ::core::mem::transmute_copy(&bstrdll), ::core::mem::transmute_copy(&bstrtlb), ::core::mem::transmute_copy(&bstrpsdll)).into()
        }
        unsafe extern "system" fn GetEventClassesForIID<Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstriid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEventClassesForIID(::core::mem::transmute_copy(&bstriid), ::core::mem::transmute_copy(&ppsavarclsids), ::core::mem::transmute_copy(&ppsavarprogids), ::core::mem::transmute_copy(&ppsavardescriptions)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCollection: GetCollection::<Impl, IMPL_OFFSET>,
            Connect: Connect::<Impl, IMPL_OFFSET>,
            MajorVersion: MajorVersion::<Impl, IMPL_OFFSET>,
            MinorVersion: MinorVersion::<Impl, IMPL_OFFSET>,
            GetCollectionByQuery: GetCollectionByQuery::<Impl, IMPL_OFFSET>,
            ImportComponent: ImportComponent::<Impl, IMPL_OFFSET>,
            InstallComponent: InstallComponent::<Impl, IMPL_OFFSET>,
            ShutdownApplication: ShutdownApplication::<Impl, IMPL_OFFSET>,
            ExportApplication: ExportApplication::<Impl, IMPL_OFFSET>,
            InstallApplication: InstallApplication::<Impl, IMPL_OFFSET>,
            StopRouter: StopRouter::<Impl, IMPL_OFFSET>,
            RefreshRouter: RefreshRouter::<Impl, IMPL_OFFSET>,
            StartRouter: StartRouter::<Impl, IMPL_OFFSET>,
            Reserved1: Reserved1::<Impl, IMPL_OFFSET>,
            Reserved2: Reserved2::<Impl, IMPL_OFFSET>,
            InstallMultipleComponents: InstallMultipleComponents::<Impl, IMPL_OFFSET>,
            GetMultipleComponentsInfo: GetMultipleComponentsInfo::<Impl, IMPL_OFFSET>,
            RefreshComponents: RefreshComponents::<Impl, IMPL_OFFSET>,
            BackupREGDB: BackupREGDB::<Impl, IMPL_OFFSET>,
            RestoreREGDB: RestoreREGDB::<Impl, IMPL_OFFSET>,
            QueryApplicationFile: QueryApplicationFile::<Impl, IMPL_OFFSET>,
            StartApplication: StartApplication::<Impl, IMPL_OFFSET>,
            ServiceCheck: ServiceCheck::<Impl, IMPL_OFFSET>,
            InstallMultipleEventClasses: InstallMultipleEventClasses::<Impl, IMPL_OFFSET>,
            InstallEventClass: InstallEventClass::<Impl, IMPL_OFFSET>,
            GetEventClassesForIID: GetEventClassesForIID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICOMAdminCatalog as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICOMAdminCatalog2_Impl: Sized + super::Com::IDispatch_Impl + ICOMAdminCatalog_Impl {
    fn GetCollectionByQuery2(&mut self, bstrcollectionname: super::super::Foundation::BSTR, pvarquerystrings: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::IDispatch>;
    fn GetApplicationInstanceIDFromProcessID(&mut self, lprocessid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ShutdownApplicationInstances(&mut self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PauseApplicationInstances(&mut self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ResumeApplicationInstances(&mut self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RecycleApplicationInstances(&mut self, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows::core::Result<()>;
    fn AreApplicationInstancesPaused(&mut self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn DumpApplicationInstance(&mut self, bstrapplicationinstanceid: super::super::Foundation::BSTR, bstrdirectory: super::super::Foundation::BSTR, lmaximages: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsApplicationInstanceDumpSupported(&mut self) -> ::windows::core::Result<i16>;
    fn CreateServiceForApplication(&mut self, bstrapplicationidorname: super::super::Foundation::BSTR, bstrservicename: super::super::Foundation::BSTR, bstrstarttype: super::super::Foundation::BSTR, bstrerrorcontrol: super::super::Foundation::BSTR, bstrdependencies: super::super::Foundation::BSTR, bstrrunas: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR, bdesktopok: i16) -> ::windows::core::Result<()>;
    fn DeleteServiceForApplication(&mut self, bstrapplicationidorname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPartitionID(&mut self, bstrapplicationidorname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPartitionName(&mut self, bstrapplicationidorname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCurrentPartition(&mut self, bstrpartitionidorname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CurrentPartitionID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentPartitionName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GlobalPartitionID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FlushPartitionCache(&mut self) -> ::windows::core::Result<()>;
    fn CopyApplications(&mut self, bstrsourcepartitionidorname: super::super::Foundation::BSTR, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CopyComponents(&mut self, bstrsourceapplicationidorname: super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MoveComponents(&mut self, bstrsourceapplicationidorname: super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AliasComponent(&mut self, bstrsrcapplicationidorname: super::super::Foundation::BSTR, bstrclsidorprogid: super::super::Foundation::BSTR, bstrdestapplicationidorname: super::super::Foundation::BSTR, bstrnewprogid: super::super::Foundation::BSTR, bstrnewclsid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsSafeToDelete(&mut self, bstrdllname: super::super::Foundation::BSTR) -> ::windows::core::Result<COMAdminInUse>;
    fn ImportUnconfiguredComponents(&mut self, bstrapplicationidorname: super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PromoteUnconfiguredComponents(&mut self, bstrapplicationidorname: super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ImportComponents(&mut self, bstrapplicationidorname: super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Is64BitCatalogServer(&mut self) -> ::windows::core::Result<i16>;
    fn ExportPartition(&mut self, bstrpartitionidorname: super::super::Foundation::BSTR, bstrpartitionfilename: super::super::Foundation::BSTR, loptions: COMAdminApplicationExportOptions) -> ::windows::core::Result<()>;
    fn InstallPartition(&mut self, bstrfilename: super::super::Foundation::BSTR, bstrdestdirectory: super::super::Foundation::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: super::super::Foundation::BSTR, bstrpassword: super::super::Foundation::BSTR, bstrrsn: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn QueryApplicationFile2(&mut self, bstrapplicationfile: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn GetComponentVersionCount(&mut self, bstrclsidorprogid: super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICOMAdminCatalog2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICOMAdminCatalog2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICOMAdminCatalog2_Vtbl {
        unsafe extern "system" fn GetCollectionByQuery2<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollectionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarquerystrings: *const super::Com::VARIANT, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollectionByQuery2(::core::mem::transmute_copy(&bstrcollectionname), ::core::mem::transmute_copy(&pvarquerystrings)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetApplicationInstanceIDFromProcessID(::core::mem::transmute_copy(&lprocessid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationinstanceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutdownApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn PauseApplicationInstances<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PauseApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn ResumeApplicationInstances<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn RecycleApplicationInstances<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecycleApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid), ::core::mem::transmute_copy(&lreasoncode)).into()
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, pvarboolpaused: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AreApplicationInstancesPaused(::core::mem::transmute_copy(&pvarapplicationinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarboolpaused = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpApplicationInstance<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaximages: i32, pbstrdumpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DumpApplicationInstance(::core::mem::transmute_copy(&bstrapplicationinstanceid), ::core::mem::transmute_copy(&bstrdirectory), ::core::mem::transmute_copy(&lmaximages)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdumpfile = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbooldumpsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsApplicationInstanceDumpSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbooldumpsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServiceForApplication<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrstarttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrerrorcontrol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdependencies: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrunas: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bdesktopok: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateServiceForApplication(::core::mem::transmute_copy(&bstrapplicationidorname), ::core::mem::transmute_copy(&bstrservicename), ::core::mem::transmute_copy(&bstrstarttype), ::core::mem::transmute_copy(&bstrerrorcontrol), ::core::mem::transmute_copy(&bstrdependencies), ::core::mem::transmute_copy(&bstrrunas), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&bdesktopok)).into()
        }
        unsafe extern "system" fn DeleteServiceForApplication<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteServiceForApplication(::core::mem::transmute_copy(&bstrapplicationidorname)).into()
        }
        unsafe extern "system" fn GetPartitionID<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartitionID(::core::mem::transmute_copy(&bstrapplicationidorname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpartitionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartitionName<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPartitionName(::core::mem::transmute_copy(&bstrapplicationidorname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpartitionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPartition<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCurrentPartition(::core::mem::transmute_copy(&bstrpartitionidorname)).into()
        }
        unsafe extern "system" fn CurrentPartitionID<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPartitionID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpartitionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPartitionName<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPartitionName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpartitionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalPartitionID<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrglobalpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GlobalPartitionID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrglobalpartitionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushPartitionCache<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FlushPartitionCache().into()
        }
        unsafe extern "system" fn CopyApplications<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourcepartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyApplications(::core::mem::transmute_copy(&bstrsourcepartitionidorname), ::core::mem::transmute_copy(&pvarapplicationid), ::core::mem::transmute_copy(&bstrdestinationpartitionidorname)).into()
        }
        unsafe extern "system" fn CopyComponents<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CopyComponents(::core::mem::transmute_copy(&bstrsourceapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn MoveComponents<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveComponents(::core::mem::transmute_copy(&bstrsourceapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn AliasComponent<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsrcapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AliasComponent(::core::mem::transmute_copy(&bstrsrcapplicationidorname), ::core::mem::transmute_copy(&bstrclsidorprogid), ::core::mem::transmute_copy(&bstrdestapplicationidorname), ::core::mem::transmute_copy(&bstrnewprogid), ::core::mem::transmute_copy(&bstrnewclsid)).into()
        }
        unsafe extern "system" fn IsSafeToDelete<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSafeToDelete(::core::mem::transmute_copy(&bstrdllname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcomadmininuse = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportUnconfiguredComponents(::core::mem::transmute_copy(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PromoteUnconfiguredComponents(::core::mem::transmute_copy(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn ImportComponents<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportComponents(::core::mem::transmute_copy(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn Is64BitCatalogServer<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbis64bit: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Is64BitCatalogServer() {
                ::core::result::Result::Ok(ok__) => {
                    *pbis64bit = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPartition<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpartitionfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExportPartition(::core::mem::transmute_copy(&bstrpartitionidorname), ::core::mem::transmute_copy(&bstrpartitionfilename), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallPartition<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstallPartition(::core::mem::transmute_copy(&bstrfilename), ::core::mem::transmute_copy(&bstrdestdirectory), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute_copy(&bstruserid), ::core::mem::transmute_copy(&bstrpassword), ::core::mem::transmute_copy(&bstrrsn)).into()
        }
        unsafe extern "system" fn QueryApplicationFile2<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfilesforimport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryApplicationFile2(::core::mem::transmute_copy(&bstrapplicationfile)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppfilesforimport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentVersionCount<Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plversioncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetComponentVersionCount(::core::mem::transmute_copy(&bstrclsidorprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    *plversioncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ICOMAdminCatalog_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetCollectionByQuery2: GetCollectionByQuery2::<Impl, IMPL_OFFSET>,
            GetApplicationInstanceIDFromProcessID: GetApplicationInstanceIDFromProcessID::<Impl, IMPL_OFFSET>,
            ShutdownApplicationInstances: ShutdownApplicationInstances::<Impl, IMPL_OFFSET>,
            PauseApplicationInstances: PauseApplicationInstances::<Impl, IMPL_OFFSET>,
            ResumeApplicationInstances: ResumeApplicationInstances::<Impl, IMPL_OFFSET>,
            RecycleApplicationInstances: RecycleApplicationInstances::<Impl, IMPL_OFFSET>,
            AreApplicationInstancesPaused: AreApplicationInstancesPaused::<Impl, IMPL_OFFSET>,
            DumpApplicationInstance: DumpApplicationInstance::<Impl, IMPL_OFFSET>,
            IsApplicationInstanceDumpSupported: IsApplicationInstanceDumpSupported::<Impl, IMPL_OFFSET>,
            CreateServiceForApplication: CreateServiceForApplication::<Impl, IMPL_OFFSET>,
            DeleteServiceForApplication: DeleteServiceForApplication::<Impl, IMPL_OFFSET>,
            GetPartitionID: GetPartitionID::<Impl, IMPL_OFFSET>,
            GetPartitionName: GetPartitionName::<Impl, IMPL_OFFSET>,
            SetCurrentPartition: SetCurrentPartition::<Impl, IMPL_OFFSET>,
            CurrentPartitionID: CurrentPartitionID::<Impl, IMPL_OFFSET>,
            CurrentPartitionName: CurrentPartitionName::<Impl, IMPL_OFFSET>,
            GlobalPartitionID: GlobalPartitionID::<Impl, IMPL_OFFSET>,
            FlushPartitionCache: FlushPartitionCache::<Impl, IMPL_OFFSET>,
            CopyApplications: CopyApplications::<Impl, IMPL_OFFSET>,
            CopyComponents: CopyComponents::<Impl, IMPL_OFFSET>,
            MoveComponents: MoveComponents::<Impl, IMPL_OFFSET>,
            AliasComponent: AliasComponent::<Impl, IMPL_OFFSET>,
            IsSafeToDelete: IsSafeToDelete::<Impl, IMPL_OFFSET>,
            ImportUnconfiguredComponents: ImportUnconfiguredComponents::<Impl, IMPL_OFFSET>,
            PromoteUnconfiguredComponents: PromoteUnconfiguredComponents::<Impl, IMPL_OFFSET>,
            ImportComponents: ImportComponents::<Impl, IMPL_OFFSET>,
            Is64BitCatalogServer: Is64BitCatalogServer::<Impl, IMPL_OFFSET>,
            ExportPartition: ExportPartition::<Impl, IMPL_OFFSET>,
            InstallPartition: InstallPartition::<Impl, IMPL_OFFSET>,
            QueryApplicationFile2: QueryApplicationFile2::<Impl, IMPL_OFFSET>,
            GetComponentVersionCount: GetComponentVersionCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICOMAdminCatalog2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICOMLBArguments_Impl: Sized {
    fn GetCLSID(&mut self, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetCLSID(&mut self, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMachineName(&mut self, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SetMachineName(&mut self, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICOMLBArguments_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICOMLBArguments_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICOMLBArguments_Vtbl {
        unsafe extern "system" fn GetCLSID<Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCLSID(::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetCLSID<Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCLSID(::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn GetMachineName<Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMachineName(::core::mem::transmute_copy(&cchsvr), ::core::mem::transmute_copy(&szservername)).into()
        }
        unsafe extern "system" fn SetMachineName<Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMachineName(::core::mem::transmute_copy(&cchsvr), ::core::mem::transmute_copy(&szservername)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCLSID: GetCLSID::<Impl, IMPL_OFFSET>,
            SetCLSID: SetCLSID::<Impl, IMPL_OFFSET>,
            GetMachineName: GetMachineName::<Impl, IMPL_OFFSET>,
            SetMachineName: SetMachineName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICOMLBArguments as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICatalogCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Remove(&mut self, lindex: i32) -> ::windows::core::Result<()>;
    fn Add(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Populate(&mut self) -> ::windows::core::Result<()>;
    fn SaveChanges(&mut self) -> ::windows::core::Result<i32>;
    fn GetCollection(&mut self, bstrcollname: super::super::Foundation::BSTR, varobjectkey: super::Com::VARIANT) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Name(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn AddEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn RemoveEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn GetUtilInterface(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn DataStoreMajorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn DataStoreMinorVersion(&mut self) -> ::windows::core::Result<i32>;
    fn PopulateByKey(&mut self, psakeys: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn PopulateByQuery(&mut self, bstrquerystring: super::super::Foundation::BSTR, lquerytype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICatalogCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICatalogCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumvariant = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppcatalogobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plobjectcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plobjectcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn Add<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcatalogobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Populate<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Populate().into()
        }
        unsafe extern "system" fn SaveChanges<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchanges: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *pcchanges = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollection<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varobjectkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppcatalogcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCollection(::core::mem::transmute_copy(&bstrcollname), ::core::mem::transmute_copy(&varobjectkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcatalogcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarnamel: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarnamel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnabled<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbool = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnabled<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarbool = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUtilInterface<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidispatch: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUtilInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *ppidispatch = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMajorVersion<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataStoreMajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plmajorversion = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMinorVersion<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversionl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataStoreMinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    *plminorversionl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopulateByKey<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopulateByKey(::core::mem::transmute_copy(&psakeys)).into()
        }
        unsafe extern "system" fn PopulateByQuery<Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquerystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lquerytype: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopulateByQuery(::core::mem::transmute_copy(&bstrquerystring), ::core::mem::transmute_copy(&lquerytype)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Populate: Populate::<Impl, IMPL_OFFSET>,
            SaveChanges: SaveChanges::<Impl, IMPL_OFFSET>,
            GetCollection: GetCollection::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            AddEnabled: AddEnabled::<Impl, IMPL_OFFSET>,
            RemoveEnabled: RemoveEnabled::<Impl, IMPL_OFFSET>,
            GetUtilInterface: GetUtilInterface::<Impl, IMPL_OFFSET>,
            DataStoreMajorVersion: DataStoreMajorVersion::<Impl, IMPL_OFFSET>,
            DataStoreMinorVersion: DataStoreMinorVersion::<Impl, IMPL_OFFSET>,
            PopulateByKey: PopulateByKey::<Impl, IMPL_OFFSET>,
            PopulateByQuery: PopulateByQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalogCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICatalogObject_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&mut self, bstrpropname: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&mut self, bstrpropname: super::super::Foundation::BSTR, val: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Key(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Name(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsPropertyReadOnly(&mut self, bstrpropname: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn Valid(&mut self) -> ::windows::core::Result<i16>;
    fn IsPropertyWriteOnly(&mut self, bstrpropname: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICatalogObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICatalogObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICatalogObject_Vtbl {
        unsafe extern "system" fn Value<Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&bstrpropname), ::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn Key<Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Key() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyReadOnly<Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertyReadOnly(::core::mem::transmute_copy(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Valid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertyWriteOnly(::core::mem::transmute_copy(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbretval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
            Key: Key::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            IsPropertyReadOnly: IsPropertyReadOnly::<Impl, IMPL_OFFSET>,
            Valid: Valid::<Impl, IMPL_OFFSET>,
            IsPropertyWriteOnly: IsPropertyWriteOnly::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalogObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICheckSxsConfig_Impl: Sized {
    fn IsSameSxsConfig(&mut self, wszsxsname: super::super::Foundation::PWSTR, wszsxsdirectory: super::super::Foundation::PWSTR, wszsxsappname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICheckSxsConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICheckSxsConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICheckSxsConfig_Vtbl {
        unsafe extern "system" fn IsSameSxsConfig<Impl: ICheckSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsxsname: super::super::Foundation::PWSTR, wszsxsdirectory: super::super::Foundation::PWSTR, wszsxsappname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSameSxsConfig(::core::mem::transmute_copy(&wszsxsname), ::core::mem::transmute_copy(&wszsxsdirectory), ::core::mem::transmute_copy(&wszsxsappname)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IsSameSxsConfig: IsSameSxsConfig::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICheckSxsConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComActivityEvents_Impl: Sized {
    fn OnActivityCreate(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnActivityDestroy(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnActivityEnter(&mut self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32) -> ::windows::core::Result<()>;
    fn OnActivityTimeout(&mut self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::core::Result<()>;
    fn OnActivityReenter(&mut self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::core::Result<()>;
    fn OnActivityLeave(&mut self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidleft: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnActivityLeaveSame(&mut self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwcalldepth: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComActivityEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComActivityEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComActivityEvents_Vtbl {
        unsafe extern "system" fn OnActivityCreate<Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivityCreate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityDestroy<Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivityDestroy(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityEnter<Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivityEnter(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidentered), ::core::mem::transmute_copy(&dwthread)).into()
        }
        unsafe extern "system" fn OnActivityTimeout<Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivityTimeout(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidentered), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnActivityReenter<Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivityReenter(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwcalldepth)).into()
        }
        unsafe extern "system" fn OnActivityLeave<Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidleft: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivityLeave(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidleft)).into()
        }
        unsafe extern "system" fn OnActivityLeaveSame<Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwcalldepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivityLeaveSame(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&dwcalldepth)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnActivityCreate: OnActivityCreate::<Impl, IMPL_OFFSET>,
            OnActivityDestroy: OnActivityDestroy::<Impl, IMPL_OFFSET>,
            OnActivityEnter: OnActivityEnter::<Impl, IMPL_OFFSET>,
            OnActivityTimeout: OnActivityTimeout::<Impl, IMPL_OFFSET>,
            OnActivityReenter: OnActivityReenter::<Impl, IMPL_OFFSET>,
            OnActivityLeave: OnActivityLeave::<Impl, IMPL_OFFSET>,
            OnActivityLeaveSame: OnActivityLeaveSame::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComActivityEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComApp2Events_Impl: Sized {
    fn OnAppActivation2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppShutdown2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppForceShutdown2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppPaused2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnAppRecycle2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID, lreason: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComApp2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComApp2Events_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComApp2Events_Vtbl {
        unsafe extern "system" fn OnAppActivation2<Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAppActivation2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp), ::core::mem::transmute_copy(&guidprocess)).into()
        }
        unsafe extern "system" fn OnAppShutdown2<Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAppShutdown2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown2<Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAppForceShutdown2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppPaused2<Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAppPaused2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp), ::core::mem::transmute_copy(&bpaused)).into()
        }
        unsafe extern "system" fn OnAppRecycle2<Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID, lreason: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAppRecycle2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp), ::core::mem::transmute_copy(&guidprocess), ::core::mem::transmute_copy(&lreason)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnAppActivation2: OnAppActivation2::<Impl, IMPL_OFFSET>,
            OnAppShutdown2: OnAppShutdown2::<Impl, IMPL_OFFSET>,
            OnAppForceShutdown2: OnAppForceShutdown2::<Impl, IMPL_OFFSET>,
            OnAppPaused2: OnAppPaused2::<Impl, IMPL_OFFSET>,
            OnAppRecycle2: OnAppRecycle2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComApp2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComAppEvents_Impl: Sized {
    fn OnAppActivation(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppShutdown(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppForceShutdown(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComAppEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComAppEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComAppEvents_Vtbl {
        unsafe extern "system" fn OnAppActivation<Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAppActivation(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppShutdown<Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAppShutdown(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown<Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAppForceShutdown(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnAppActivation: OnAppActivation::<Impl, IMPL_OFFSET>,
            OnAppShutdown: OnAppShutdown::<Impl, IMPL_OFFSET>,
            OnAppForceShutdown: OnAppForceShutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComAppEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComCRMEvents_Impl: Sized {
    fn OnCRMRecoveryStart(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMRecoveryDone(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMCheckpoint(&mut self, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMBegin(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, guidactivity: ::windows::core::GUID, guidtx: ::windows::core::GUID, szprogidcompensator: super::super::Foundation::PWSTR, szdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnCRMPrepare(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMCommit(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMAbort(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMIndoubt(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMDone(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMRelease(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMAnalyze(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::core::Result<()>;
    fn OnCRMWrite(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::Result<()>;
    fn OnCRMForget(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMForce(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMDeliver(&mut self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComCRMEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComCRMEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComCRMEvents_Vtbl {
        unsafe extern "system" fn OnCRMRecoveryStart<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMRecoveryStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMRecoveryDone<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMRecoveryDone(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMCheckpoint<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMCheckpoint(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMBegin<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, guidactivity: ::windows::core::GUID, guidtx: ::windows::core::GUID, szprogidcompensator: super::super::Foundation::PWSTR, szdescription: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMBegin(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&szprogidcompensator), ::core::mem::transmute_copy(&szdescription)).into()
        }
        unsafe extern "system" fn OnCRMPrepare<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMPrepare(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMCommit<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAbort<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMIndoubt<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMIndoubt(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDone<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMDone(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMRelease<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMRelease(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAnalyze<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMAnalyze(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid), ::core::mem::transmute_copy(&dwcrmrecordtype), ::core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMWrite<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMWrite(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid), ::core::mem::transmute_copy(&fvariants), ::core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMForget<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMForget(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMForce<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMForce(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDeliver<Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCRMDeliver(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidclerkclsid), ::core::mem::transmute_copy(&fvariants), ::core::mem::transmute_copy(&dwrecordsize)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnCRMRecoveryStart: OnCRMRecoveryStart::<Impl, IMPL_OFFSET>,
            OnCRMRecoveryDone: OnCRMRecoveryDone::<Impl, IMPL_OFFSET>,
            OnCRMCheckpoint: OnCRMCheckpoint::<Impl, IMPL_OFFSET>,
            OnCRMBegin: OnCRMBegin::<Impl, IMPL_OFFSET>,
            OnCRMPrepare: OnCRMPrepare::<Impl, IMPL_OFFSET>,
            OnCRMCommit: OnCRMCommit::<Impl, IMPL_OFFSET>,
            OnCRMAbort: OnCRMAbort::<Impl, IMPL_OFFSET>,
            OnCRMIndoubt: OnCRMIndoubt::<Impl, IMPL_OFFSET>,
            OnCRMDone: OnCRMDone::<Impl, IMPL_OFFSET>,
            OnCRMRelease: OnCRMRelease::<Impl, IMPL_OFFSET>,
            OnCRMAnalyze: OnCRMAnalyze::<Impl, IMPL_OFFSET>,
            OnCRMWrite: OnCRMWrite::<Impl, IMPL_OFFSET>,
            OnCRMForget: OnCRMForget::<Impl, IMPL_OFFSET>,
            OnCRMForce: OnCRMForce::<Impl, IMPL_OFFSET>,
            OnCRMDeliver: OnCRMDeliver::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComCRMEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComExceptionEvents_Impl: Sized {
    fn OnExceptionUser(&mut self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComExceptionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComExceptionEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComExceptionEvents_Vtbl {
        unsafe extern "system" fn OnExceptionUser<Impl: IComExceptionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnExceptionUser(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&code), ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&pszstacktrace)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnExceptionUser: OnExceptionUser::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComExceptionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComIdentityEvents_Impl: Sized {
    fn OnIISRequestInfo(&mut self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: super::super::Foundation::PWSTR, pszserverip: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComIdentityEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComIdentityEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComIdentityEvents_Vtbl {
        unsafe extern "system" fn OnIISRequestInfo<Impl: IComIdentityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: super::super::Foundation::PWSTR, pszserverip: super::super::Foundation::PWSTR, pszurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnIISRequestInfo(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&pszclientip), ::core::mem::transmute_copy(&pszserverip), ::core::mem::transmute_copy(&pszurl)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnIISRequestInfo: OnIISRequestInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComIdentityEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComInstance2Events_Impl: Sized {
    fn OnObjectCreate2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnObjectDestroy2(&mut self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComInstance2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComInstance2Events_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComInstance2Events_Vtbl {
        unsafe extern "system" fn OnObjectCreate2<Impl: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjectCreate2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjectDestroy2<Impl: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjectDestroy2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnObjectCreate2: OnObjectCreate2::<Impl, IMPL_OFFSET>,
            OnObjectDestroy2: OnObjectDestroy2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComInstance2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComInstanceEvents_Impl: Sized {
    fn OnObjectCreate(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64) -> ::windows::core::Result<()>;
    fn OnObjectDestroy(&mut self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComInstanceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComInstanceEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComInstanceEvents_Vtbl {
        unsafe extern "system" fn OnObjectCreate<Impl: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjectCreate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDestroy<Impl: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjectDestroy(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnObjectCreate: OnObjectCreate::<Impl, IMPL_OFFSET>,
            OnObjectDestroy: OnObjectDestroy::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComInstanceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComLTxEvents_Impl: Sized {
    fn OnLtxTransactionStart(&mut self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, tsid: ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::Result<()>;
    fn OnLtxTransactionPrepare(&mut self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnLtxTransactionAbort(&mut self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnLtxTransactionCommit(&mut self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnLtxTransactionPromote(&mut self, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, txnid: ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComLTxEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComLTxEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComLTxEvents_Vtbl {
        unsafe extern "system" fn OnLtxTransactionStart<Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, tsid: ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLtxTransactionStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidltx), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&froot), ::core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPrepare<Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLtxTransactionPrepare(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidltx), ::core::mem::transmute_copy(&fvote)).into()
        }
        unsafe extern "system" fn OnLtxTransactionAbort<Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLtxTransactionAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionCommit<Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLtxTransactionCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPromote<Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, txnid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLtxTransactionPromote(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidltx), ::core::mem::transmute_copy(&txnid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnLtxTransactionStart: OnLtxTransactionStart::<Impl, IMPL_OFFSET>,
            OnLtxTransactionPrepare: OnLtxTransactionPrepare::<Impl, IMPL_OFFSET>,
            OnLtxTransactionAbort: OnLtxTransactionAbort::<Impl, IMPL_OFFSET>,
            OnLtxTransactionCommit: OnLtxTransactionCommit::<Impl, IMPL_OFFSET>,
            OnLtxTransactionPromote: OnLtxTransactionPromote::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComLTxEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComMethod2Events_Impl: Sized {
    fn OnMethodCall2(&mut self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::Result<()>;
    fn OnMethodReturn2(&mut self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnMethodException2(&mut self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComMethod2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComMethod2Events_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComMethod2Events_Vtbl {
        unsafe extern "system" fn OnMethodCall2<Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMethodCall2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn2<Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMethodReturn2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException2<Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMethodException2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnMethodCall2: OnMethodCall2::<Impl, IMPL_OFFSET>,
            OnMethodReturn2: OnMethodReturn2::<Impl, IMPL_OFFSET>,
            OnMethodException2: OnMethodException2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComMethod2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComMethodEvents_Impl: Sized {
    fn OnMethodCall(&mut self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::Result<()>;
    fn OnMethodReturn(&mut self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnMethodException(&mut self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComMethodEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComMethodEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComMethodEvents_Vtbl {
        unsafe extern "system" fn OnMethodCall<Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMethodCall(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn<Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMethodReturn(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException<Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnMethodException(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnMethodCall: OnMethodCall::<Impl, IMPL_OFFSET>,
            OnMethodReturn: OnMethodReturn::<Impl, IMPL_OFFSET>,
            OnMethodException: OnMethodException::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComMethodEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComMtaThreadPoolKnobs_Impl: Sized {
    fn MTASetMaxThreadCount(&mut self, dwmaxthreads: u32) -> ::windows::core::Result<()>;
    fn MTAGetMaxThreadCount(&mut self) -> ::windows::core::Result<u32>;
    fn MTASetThrottleValue(&mut self, dwthrottle: u32) -> ::windows::core::Result<()>;
    fn MTAGetThrottleValue(&mut self) -> ::windows::core::Result<u32>;
}
impl IComMtaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComMtaThreadPoolKnobs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComMtaThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn MTASetMaxThreadCount<Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MTASetMaxThreadCount(::core::mem::transmute_copy(&dwmaxthreads)).into()
        }
        unsafe extern "system" fn MTAGetMaxThreadCount<Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MTAGetMaxThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwmaxthreads = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MTASetThrottleValue<Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthrottle: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MTASetThrottleValue(::core::mem::transmute_copy(&dwthrottle)).into()
        }
        unsafe extern "system" fn MTAGetThrottleValue<Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthrottle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MTAGetThrottleValue() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwthrottle = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            MTASetMaxThreadCount: MTASetMaxThreadCount::<Impl, IMPL_OFFSET>,
            MTAGetMaxThreadCount: MTAGetMaxThreadCount::<Impl, IMPL_OFFSET>,
            MTASetThrottleValue: MTASetThrottleValue::<Impl, IMPL_OFFSET>,
            MTAGetThrottleValue: MTAGetThrottleValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComMtaThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectConstruction2Events_Impl: Sized {
    fn OnObjectConstruct2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectConstruction2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectConstruction2Events_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectConstruction2Events_Vtbl {
        unsafe extern "system" fn OnObjectConstruct2<Impl: IComObjectConstruction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjectConstruct2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&sconstructstring), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnObjectConstruct2: OnObjectConstruct2::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectConstruction2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectConstructionEvents_Impl: Sized {
    fn OnObjectConstruct(&mut self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectConstructionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectConstructionEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectConstructionEvents_Vtbl {
        unsafe extern "system" fn OnObjectConstruct<Impl: IComObjectConstructionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: super::super::Foundation::PWSTR, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjectConstruct(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&sconstructstring), ::core::mem::transmute_copy(&oid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnObjectConstruct: OnObjectConstruct::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectConstructionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectEvents_Impl: Sized {
    fn OnObjectActivate(&mut self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::Result<()>;
    fn OnObjectDeactivate(&mut self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::Result<()>;
    fn OnDisableCommit(&mut self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
    fn OnEnableCommit(&mut self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
    fn OnSetComplete(&mut self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
    fn OnSetAbort(&mut self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectEvents_Vtbl {
        unsafe extern "system" fn OnObjectActivate<Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjectActivate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDeactivate<Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjectDeactivate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnDisableCommit<Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisableCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnEnableCommit<Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEnableCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetComplete<Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetComplete(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetAbort<Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSetAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnObjectActivate: OnObjectActivate::<Impl, IMPL_OFFSET>,
            OnObjectDeactivate: OnObjectDeactivate::<Impl, IMPL_OFFSET>,
            OnDisableCommit: OnDisableCommit::<Impl, IMPL_OFFSET>,
            OnEnableCommit: OnEnableCommit::<Impl, IMPL_OFFSET>,
            OnSetComplete: OnSetComplete::<Impl, IMPL_OFFSET>,
            OnSetAbort: OnSetAbort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectPool2Events_Impl: Sized {
    fn OnObjPoolPutObject2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolGetObject2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnObjPoolRecycleToTx2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolGetFromTx2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectPool2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectPool2Events_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectPool2Events_Vtbl {
        unsafe extern "system" fn OnObjPoolPutObject2<Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolPutObject2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&nreason), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject2<Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolGetObject2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx2<Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolRecycleToTx2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx2<Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolGetFromTx2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnObjPoolPutObject2: OnObjPoolPutObject2::<Impl, IMPL_OFFSET>,
            OnObjPoolGetObject2: OnObjPoolGetObject2::<Impl, IMPL_OFFSET>,
            OnObjPoolRecycleToTx2: OnObjPoolRecycleToTx2::<Impl, IMPL_OFFSET>,
            OnObjPoolGetFromTx2: OnObjPoolGetFromTx2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectPool2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectPoolEvents_Impl: Sized {
    fn OnObjPoolPutObject(&mut self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolGetObject(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolRecycleToTx(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolGetFromTx(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectPoolEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectPoolEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectPoolEvents_Vtbl {
        unsafe extern "system" fn OnObjPoolPutObject<Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolPutObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&nreason), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject<Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolGetObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx<Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolRecycleToTx(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx<Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolGetFromTx(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnObjPoolPutObject: OnObjPoolPutObject::<Impl, IMPL_OFFSET>,
            OnObjPoolGetObject: OnObjPoolGetObject::<Impl, IMPL_OFFSET>,
            OnObjPoolRecycleToTx: OnObjPoolRecycleToTx::<Impl, IMPL_OFFSET>,
            OnObjPoolGetFromTx: OnObjPoolGetFromTx::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectPoolEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComObjectPoolEvents2_Impl: Sized {
    fn OnObjPoolCreateObject(&mut self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolDestroyObject(&mut self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolCreateDecision(&mut self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::core::Result<()>;
    fn OnObjPoolTimeout(&mut self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, guidactivity: *const ::windows::core::GUID, dwtimeout: u32) -> ::windows::core::Result<()>;
    fn OnObjPoolCreatePool(&mut self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComObjectPoolEvents2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComObjectPoolEvents2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComObjectPoolEvents2_Vtbl {
        unsafe extern "system" fn OnObjPoolCreateObject<Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolCreateObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwobjscreated), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolDestroyObject<Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolDestroyObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwobjscreated), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolCreateDecision<Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolCreateDecision(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&dwthreadswaiting), ::core::mem::transmute_copy(&dwavail), ::core::mem::transmute_copy(&dwcreated), ::core::mem::transmute_copy(&dwmin), ::core::mem::transmute_copy(&dwmax)).into()
        }
        unsafe extern "system" fn OnObjPoolTimeout<Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, guidactivity: *const ::windows::core::GUID, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolTimeout(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnObjPoolCreatePool<Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnObjPoolCreatePool(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwmin), ::core::mem::transmute_copy(&dwmax), ::core::mem::transmute_copy(&dwtimeout)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnObjPoolCreateObject: OnObjPoolCreateObject::<Impl, IMPL_OFFSET>,
            OnObjPoolDestroyObject: OnObjPoolDestroyObject::<Impl, IMPL_OFFSET>,
            OnObjPoolCreateDecision: OnObjPoolCreateDecision::<Impl, IMPL_OFFSET>,
            OnObjPoolTimeout: OnObjPoolTimeout::<Impl, IMPL_OFFSET>,
            OnObjPoolCreatePool: OnObjPoolCreatePool::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectPoolEvents2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComQCEvents_Impl: Sized {
    fn OnQCRecord(&mut self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: super::super::Foundation::PWSTR, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, msmqhr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnQCQueueOpen(&mut self, pinfo: *const COMSVCSEVENTINFO, szqueue: super::super::Foundation::PWSTR, queueid: u64, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnQCReceive(&mut self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnQCReceiveFail(&mut self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnQCMoveToReTryQueue(&mut self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, retryindex: u32) -> ::windows::core::Result<()>;
    fn OnQCMoveToDeadQueue(&mut self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnQCPlayback(&mut self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComQCEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComQCEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComQCEvents_Vtbl {
        unsafe extern "system" fn OnQCRecord<Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: super::super::Foundation::PWSTR, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQCRecord(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&szqueue), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCQueueOpen<Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: super::super::Foundation::PWSTR, queueid: u64, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQCQueueOpen(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&szqueue), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceive<Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQCReceive(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceiveFail<Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQCReceiveFail(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCMoveToReTryQueue<Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, retryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQCMoveToReTryQueue(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&retryindex)).into()
        }
        unsafe extern "system" fn OnQCMoveToDeadQueue<Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQCMoveToDeadQueue(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid)).into()
        }
        unsafe extern "system" fn OnQCPlayback<Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnQCPlayback(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnQCRecord: OnQCRecord::<Impl, IMPL_OFFSET>,
            OnQCQueueOpen: OnQCQueueOpen::<Impl, IMPL_OFFSET>,
            OnQCReceive: OnQCReceive::<Impl, IMPL_OFFSET>,
            OnQCReceiveFail: OnQCReceiveFail::<Impl, IMPL_OFFSET>,
            OnQCMoveToReTryQueue: OnQCMoveToReTryQueue::<Impl, IMPL_OFFSET>,
            OnQCMoveToDeadQueue: OnQCMoveToDeadQueue::<Impl, IMPL_OFFSET>,
            OnQCPlayback: OnQCPlayback::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComQCEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComResourceEvents_Impl: Sized {
    fn OnResourceCreate(&mut self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnResourceAllocate(&mut self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows::core::Result<()>;
    fn OnResourceRecycle(&mut self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::core::Result<()>;
    fn OnResourceDestroy(&mut self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::core::HRESULT, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::core::Result<()>;
    fn OnResourceTrack(&mut self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComResourceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComResourceEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComResourceEvents_Vtbl {
        unsafe extern "system" fn OnResourceCreate<Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnResourceCreate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted)).into()
        }
        unsafe extern "system" fn OnResourceAllocate<Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnResourceAllocate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted), ::core::mem::transmute_copy(&numrated), ::core::mem::transmute_copy(&rating)).into()
        }
        unsafe extern "system" fn OnResourceRecycle<Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnResourceRecycle(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&psztype), ::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceDestroy<Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::core::HRESULT, psztype: super::super::Foundation::PWSTR, resid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnResourceDestroy(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&psztype), ::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceTrack<Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: super::super::Foundation::PWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnResourceTrack(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnResourceCreate: OnResourceCreate::<Impl, IMPL_OFFSET>,
            OnResourceAllocate: OnResourceAllocate::<Impl, IMPL_OFFSET>,
            OnResourceRecycle: OnResourceRecycle::<Impl, IMPL_OFFSET>,
            OnResourceDestroy: OnResourceDestroy::<Impl, IMPL_OFFSET>,
            OnResourceTrack: OnResourceTrack::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComResourceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComSecurityEvents_Impl: Sized {
    fn OnAuthenticate(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnAuthenticateFail(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComSecurityEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComSecurityEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComSecurityEvents_Vtbl {
        unsafe extern "system" fn OnAuthenticate<Impl: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAuthenticate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidiid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&cbbyteorig), ::core::mem::transmute_copy(&psidoriginaluser), ::core::mem::transmute_copy(&cbbytecur), ::core::mem::transmute_copy(&psidcurrentuser), ::core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        unsafe extern "system" fn OnAuthenticateFail<Impl: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnAuthenticateFail(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidiid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&cbbyteorig), ::core::mem::transmute_copy(&psidoriginaluser), ::core::mem::transmute_copy(&cbbytecur), ::core::mem::transmute_copy(&psidcurrentuser), ::core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnAuthenticate: OnAuthenticate::<Impl, IMPL_OFFSET>,
            OnAuthenticateFail: OnAuthenticateFail::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComSecurityEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComStaThreadPoolKnobs_Impl: Sized {
    fn SetMinThreadCount(&mut self, minthreads: u32) -> ::windows::core::Result<()>;
    fn GetMinThreadCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxThreadCount(&mut self, maxthreads: u32) -> ::windows::core::Result<()>;
    fn GetMaxThreadCount(&mut self) -> ::windows::core::Result<u32>;
    fn SetActivityPerThread(&mut self, activitiesperthread: u32) -> ::windows::core::Result<()>;
    fn GetActivityPerThread(&mut self) -> ::windows::core::Result<u32>;
    fn SetActivityRatio(&mut self, activityratio: f64) -> ::windows::core::Result<()>;
    fn GetActivityRatio(&mut self) -> ::windows::core::Result<f64>;
    fn GetThreadCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetQueueDepth(&mut self) -> ::windows::core::Result<u32>;
    fn SetQueueDepth(&mut self, dwqdepth: i32) -> ::windows::core::Result<()>;
}
impl IComStaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComStaThreadPoolKnobs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComStaThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn SetMinThreadCount<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinThreadCount(::core::mem::transmute_copy(&minthreads)).into()
        }
        unsafe extern "system" fn GetMinThreadCount<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMinThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    *minthreads = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxThreadCount<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxThreadCount(::core::mem::transmute_copy(&maxthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreadCount<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    *maxthreads = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityPerThread<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivityPerThread(::core::mem::transmute_copy(&activitiesperthread)).into()
        }
        unsafe extern "system" fn GetActivityPerThread<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivityPerThread() {
                ::core::result::Result::Ok(ok__) => {
                    *activitiesperthread = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityRatio<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActivityRatio(::core::mem::transmute_copy(&activityratio)).into()
        }
        unsafe extern "system" fn GetActivityRatio<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActivityRatio() {
                ::core::result::Result::Ok(ok__) => {
                    *activityratio = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadCount<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwthreads = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueueDepth<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwqdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQueueDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwqdepth = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueueDepth<Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqdepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQueueDepth(::core::mem::transmute_copy(&dwqdepth)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMinThreadCount: SetMinThreadCount::<Impl, IMPL_OFFSET>,
            GetMinThreadCount: GetMinThreadCount::<Impl, IMPL_OFFSET>,
            SetMaxThreadCount: SetMaxThreadCount::<Impl, IMPL_OFFSET>,
            GetMaxThreadCount: GetMaxThreadCount::<Impl, IMPL_OFFSET>,
            SetActivityPerThread: SetActivityPerThread::<Impl, IMPL_OFFSET>,
            GetActivityPerThread: GetActivityPerThread::<Impl, IMPL_OFFSET>,
            SetActivityRatio: SetActivityRatio::<Impl, IMPL_OFFSET>,
            GetActivityRatio: GetActivityRatio::<Impl, IMPL_OFFSET>,
            GetThreadCount: GetThreadCount::<Impl, IMPL_OFFSET>,
            GetQueueDepth: GetQueueDepth::<Impl, IMPL_OFFSET>,
            SetQueueDepth: SetQueueDepth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComStaThreadPoolKnobs2_Impl: Sized + IComStaThreadPoolKnobs_Impl {
    fn GetMaxCPULoad(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxCPULoad(&mut self, pdwload: i32) -> ::windows::core::Result<()>;
    fn GetCPUMetricEnabled(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetCPUMetricEnabled(&mut self, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCreateThreadsAggressively(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetCreateThreadsAggressively(&mut self, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMaxCSR(&mut self) -> ::windows::core::Result<u32>;
    fn SetMaxCSR(&mut self, dwcsr: i32) -> ::windows::core::Result<()>;
    fn GetWaitTimeForThreadCleanup(&mut self) -> ::windows::core::Result<u32>;
    fn SetWaitTimeForThreadCleanup(&mut self, dwthreadcleanupwaittime: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComStaThreadPoolKnobs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComStaThreadPoolKnobs2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComStaThreadPoolKnobs2_Vtbl {
        unsafe extern "system" fn GetMaxCPULoad<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxCPULoad() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwload = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCPULoad<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxCPULoad(::core::mem::transmute_copy(&pdwload)).into()
        }
        unsafe extern "system" fn GetCPUMetricEnabled<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCPUMetricEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmetricenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCPUMetricEnabled<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCPUMetricEnabled(::core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetCreateThreadsAggressively<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCreateThreadsAggressively() {
                ::core::result::Result::Ok(ok__) => {
                    *pbmetricenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateThreadsAggressively<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreateThreadsAggressively(::core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetMaxCSR<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcsr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaxCSR() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcsr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCSR<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcsr: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxCSR(::core::mem::transmute_copy(&dwcsr)).into()
        }
        unsafe extern "system" fn GetWaitTimeForThreadCleanup<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetWaitTimeForThreadCleanup() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwthreadcleanupwaittime = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaitTimeForThreadCleanup<Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadcleanupwaittime: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWaitTimeForThreadCleanup(::core::mem::transmute_copy(&dwthreadcleanupwaittime)).into()
        }
        Self {
            base: IComStaThreadPoolKnobs_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetMaxCPULoad: GetMaxCPULoad::<Impl, IMPL_OFFSET>,
            SetMaxCPULoad: SetMaxCPULoad::<Impl, IMPL_OFFSET>,
            GetCPUMetricEnabled: GetCPUMetricEnabled::<Impl, IMPL_OFFSET>,
            SetCPUMetricEnabled: SetCPUMetricEnabled::<Impl, IMPL_OFFSET>,
            GetCreateThreadsAggressively: GetCreateThreadsAggressively::<Impl, IMPL_OFFSET>,
            SetCreateThreadsAggressively: SetCreateThreadsAggressively::<Impl, IMPL_OFFSET>,
            GetMaxCSR: GetMaxCSR::<Impl, IMPL_OFFSET>,
            SetMaxCSR: SetMaxCSR::<Impl, IMPL_OFFSET>,
            GetWaitTimeForThreadCleanup: GetWaitTimeForThreadCleanup::<Impl, IMPL_OFFSET>,
            SetWaitTimeForThreadCleanup: SetWaitTimeForThreadCleanup::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComThreadEvents_Impl: Sized {
    fn OnThreadStart(&mut self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::Result<()>;
    fn OnThreadTerminate(&mut self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::Result<()>;
    fn OnThreadBindToApartment(&mut self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::core::Result<()>;
    fn OnThreadUnBind(&mut self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::core::Result<()>;
    fn OnThreadWorkEnque(&mut self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::Result<()>;
    fn OnThreadWorkPrivate(&mut self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::core::Result<()>;
    fn OnThreadWorkPublic(&mut self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::Result<()>;
    fn OnThreadWorkRedirect(&mut self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::core::Result<()>;
    fn OnThreadWorkReject(&mut self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::Result<()>;
    fn OnThreadAssignApartment(&mut self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, aptid: u64) -> ::windows::core::Result<()>;
    fn OnThreadUnassignApartment(&mut self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComThreadEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComThreadEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComThreadEvents_Vtbl {
        unsafe extern "system" fn OnThreadStart<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadTerminate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadBindToApartment<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadBindToApartment(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&aptid), ::core::mem::transmute_copy(&dwactcnt), ::core::mem::transmute_copy(&dwlowcnt)).into()
        }
        unsafe extern "system" fn OnThreadUnBind<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadUnBind(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&aptid), ::core::mem::transmute_copy(&dwactcnt)).into()
        }
        unsafe extern "system" fn OnThreadWorkEnque<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadWorkEnque(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkPrivate<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadWorkPrivate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid)).into()
        }
        unsafe extern "system" fn OnThreadWorkPublic<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadWorkPublic(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkRedirect<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadWorkRedirect(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen), ::core::mem::transmute_copy(&threadnum)).into()
        }
        unsafe extern "system" fn OnThreadWorkReject<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadWorkReject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadAssignApartment<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, aptid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadAssignApartment(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&aptid)).into()
        }
        unsafe extern "system" fn OnThreadUnassignApartment<Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnThreadUnassignApartment(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&aptid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnThreadStart: OnThreadStart::<Impl, IMPL_OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Impl, IMPL_OFFSET>,
            OnThreadBindToApartment: OnThreadBindToApartment::<Impl, IMPL_OFFSET>,
            OnThreadUnBind: OnThreadUnBind::<Impl, IMPL_OFFSET>,
            OnThreadWorkEnque: OnThreadWorkEnque::<Impl, IMPL_OFFSET>,
            OnThreadWorkPrivate: OnThreadWorkPrivate::<Impl, IMPL_OFFSET>,
            OnThreadWorkPublic: OnThreadWorkPublic::<Impl, IMPL_OFFSET>,
            OnThreadWorkRedirect: OnThreadWorkRedirect::<Impl, IMPL_OFFSET>,
            OnThreadWorkReject: OnThreadWorkReject::<Impl, IMPL_OFFSET>,
            OnThreadAssignApartment: OnThreadAssignApartment::<Impl, IMPL_OFFSET>,
            OnThreadUnassignApartment: OnThreadUnassignApartment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComThreadEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComTrackingInfoCollection_Impl: Sized {
    fn Type(&mut self) -> ::windows::core::Result<TRACKING_COLL_TYPE>;
    fn Count(&mut self) -> ::windows::core::Result<u32>;
    fn Item(&mut self, ulindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IComTrackingInfoCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTrackingInfoCollection_Vtbl {
        unsafe extern "system" fn Type<Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *ptype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Item(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Type: Type::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoCollection as ::windows::core::Interface>::IID
    }
}
pub trait IComTrackingInfoEvents_Impl: Sized {
    fn OnNewTrackingInfo(&mut self, ptoplevelcollection: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl IComTrackingInfoEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTrackingInfoEvents_Vtbl {
        unsafe extern "system" fn OnNewTrackingInfo<Impl: IComTrackingInfoEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoplevelcollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnNewTrackingInfo(::core::mem::transmute(&ptoplevelcollection)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnNewTrackingInfo: OnNewTrackingInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComTrackingInfoObject_Impl: Sized {
    fn GetValue(&mut self, szpropertyname: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComTrackingInfoObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoObject_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTrackingInfoObject_Vtbl {
        unsafe extern "system" fn GetValue<Impl: IComTrackingInfoObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szpropertyname: super::super::Foundation::PWSTR, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute_copy(&szpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarout = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetValue: GetValue::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComTrackingInfoProperties_Impl: Sized {
    fn PropCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetPropName(&mut self, ulindex: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComTrackingInfoProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTrackingInfoProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTrackingInfoProperties_Vtbl {
        unsafe extern "system" fn PropCount<Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropName<Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppszpropname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropName(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszpropname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PropCount: PropCount::<Impl, IMPL_OFFSET>,
            GetPropName: GetPropName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransaction2Events_Impl: Sized {
    fn OnTransactionStart2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::Result<()>;
    fn OnTransactionPrepare2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTransactionAbort2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnTransactionCommit2(&mut self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComTransaction2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTransaction2Events_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTransaction2Events_Vtbl {
        unsafe extern "system" fn OnTransactionStart2<Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTransactionStart2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&froot), ::core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare2<Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTransactionPrepare2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort2<Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTransactionAbort2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit2<Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTransactionCommit2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnTransactionStart2: OnTransactionStart2::<Impl, IMPL_OFFSET>,
            OnTransactionPrepare2: OnTransactionPrepare2::<Impl, IMPL_OFFSET>,
            OnTransactionAbort2: OnTransactionAbort2::<Impl, IMPL_OFFSET>,
            OnTransactionCommit2: OnTransactionCommit2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTransaction2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransactionEvents_Impl: Sized {
    fn OnTransactionStart(&mut self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTransactionPrepare(&mut self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTransactionAbort(&mut self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnTransactionCommit(&mut self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IComTransactionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComTransactionEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComTransactionEvents_Vtbl {
        unsafe extern "system" fn OnTransactionStart<Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTransactionStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&froot)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare<Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTransactionPrepare(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort<Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTransactionAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit<Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnTransactionCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnTransactionStart: OnTransactionStart::<Impl, IMPL_OFFSET>,
            OnTransactionPrepare: OnTransactionPrepare::<Impl, IMPL_OFFSET>,
            OnTransactionAbort: OnTransactionAbort::<Impl, IMPL_OFFSET>,
            OnTransactionCommit: OnTransactionCommit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTransactionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComUserEvent_Impl: Sized {
    fn OnUserEvent(&mut self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComUserEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComUserEvent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComUserEvent_Vtbl {
        unsafe extern "system" fn OnUserEvent<Impl: IComUserEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUserEvent(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pvarevent)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnUserEvent: OnUserEvent::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComUserEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IContextProperties_Impl: Sized {
    fn Count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, name: super::super::Foundation::BSTR, pproperty: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EnumNames(&mut self) -> ::windows::core::Result<IEnumNames>;
    fn SetProperty(&mut self, name: super::super::Foundation::BSTR, property: super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RemoveProperty(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IContextProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextProperties_Vtbl {
        unsafe extern "system" fn Count<Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumNames() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&property)).into()
        }
        unsafe extern "system" fn RemoveProperty<Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProperty(::core::mem::transmute_copy(&name)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            EnumNames: EnumNames::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            RemoveProperty: RemoveProperty::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContextSecurityPerimeter_Impl: Sized {
    fn GetPerimeterFlag(&mut self, pflag: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetPerimeterFlag(&mut self, fflag: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IContextSecurityPerimeter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextSecurityPerimeter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextSecurityPerimeter_Vtbl {
        unsafe extern "system" fn GetPerimeterFlag<Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflag: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPerimeterFlag(::core::mem::transmute_copy(&pflag)).into()
        }
        unsafe extern "system" fn SetPerimeterFlag<Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPerimeterFlag(::core::mem::transmute_copy(&fflag)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetPerimeterFlag: GetPerimeterFlag::<Impl, IMPL_OFFSET>,
            SetPerimeterFlag: SetPerimeterFlag::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextSecurityPerimeter as ::windows::core::Interface>::IID
    }
}
pub trait IContextState_Impl: Sized {
    fn SetDeactivateOnReturn(&mut self, bdeactivate: i16) -> ::windows::core::Result<()>;
    fn GetDeactivateOnReturn(&mut self, pbdeactivate: *mut i16) -> ::windows::core::Result<()>;
    fn SetMyTransactionVote(&mut self, txvote: TransactionVote) -> ::windows::core::Result<()>;
    fn GetMyTransactionVote(&mut self, ptxvote: *mut TransactionVote) -> ::windows::core::Result<()>;
}
impl IContextState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IContextState_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IContextState_Vtbl {
        unsafe extern "system" fn SetDeactivateOnReturn<Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdeactivate: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeactivateOnReturn(::core::mem::transmute_copy(&bdeactivate)).into()
        }
        unsafe extern "system" fn GetDeactivateOnReturn<Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeactivate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeactivateOnReturn(::core::mem::transmute_copy(&pbdeactivate)).into()
        }
        unsafe extern "system" fn SetMyTransactionVote<Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, txvote: TransactionVote) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMyTransactionVote(::core::mem::transmute_copy(&txvote)).into()
        }
        unsafe extern "system" fn GetMyTransactionVote<Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxvote: *mut TransactionVote) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMyTransactionVote(::core::mem::transmute_copy(&ptxvote)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetDeactivateOnReturn: SetDeactivateOnReturn::<Impl, IMPL_OFFSET>,
            GetDeactivateOnReturn: GetDeactivateOnReturn::<Impl, IMPL_OFFSET>,
            SetMyTransactionVote: SetMyTransactionVote::<Impl, IMPL_OFFSET>,
            GetMyTransactionVote: GetMyTransactionVote::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextState as ::windows::core::Interface>::IID
    }
}
pub trait ICreateWithLocalTransaction_Impl: Sized {
    fn CreateInstanceWithSysTx(&mut self, ptransaction: ::core::option::Option<::windows::core::IUnknown>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ICreateWithLocalTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateWithLocalTransaction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateWithLocalTransaction_Vtbl {
        unsafe extern "system" fn CreateInstanceWithSysTx<Impl: ICreateWithLocalTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstanceWithSysTx(::core::mem::transmute(&ptransaction), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateInstanceWithSysTx: CreateInstanceWithSysTx::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateWithLocalTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICreateWithTipTransactionEx_Impl: Sized {
    fn CreateInstance(&mut self, bstrtipurl: super::super::Foundation::BSTR, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ICreateWithTipTransactionEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateWithTipTransactionEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateWithTipTransactionEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICreateWithTipTransactionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtipurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstance(::core::mem::transmute_copy(&bstrtipurl), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateWithTipTransactionEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ICreateWithTransactionEx_Impl: Sized {
    fn CreateInstance(&mut self, ptransaction: ::core::option::Option<super::DistributedTransactionCoordinator::ITransaction>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ICreateWithTransactionEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICreateWithTransactionEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICreateWithTransactionEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ICreateWithTransactionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: ::windows::core::RawPtr, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstance(::core::mem::transmute(&ptransaction), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateInstance: CreateInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateWithTransactionEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICrmCompensator_Impl: Sized {
    fn SetLogControl(&mut self, plogcontrol: ::core::option::Option<ICrmLogControl>) -> ::windows::core::Result<()>;
    fn BeginPrepare(&mut self) -> ::windows::core::Result<()>;
    fn PrepareRecord(&mut self, crmlogrec: CrmLogRecordRead) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EndPrepare(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BeginCommit(&mut self, frecovery: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CommitRecord(&mut self, crmlogrec: CrmLogRecordRead) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EndCommit(&mut self) -> ::windows::core::Result<()>;
    fn BeginAbort(&mut self, frecovery: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AbortRecord(&mut self, crmlogrec: CrmLogRecordRead) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EndAbort(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICrmCompensator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmCompensator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmCompensator_Vtbl {
        unsafe extern "system" fn SetLogControl<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogControl(::core::mem::transmute(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepare<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginPrepare().into()
        }
        unsafe extern "system" fn PrepareRecord<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareRecord(::core::mem::transmute_copy(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfforget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepare<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfoktoprepare: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPrepare() {
                ::core::result::Result::Ok(ok__) => {
                    *pfoktoprepare = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommit<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginCommit(::core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn CommitRecord<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitRecord(::core::mem::transmute_copy(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfforget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommit<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndCommit().into()
        }
        unsafe extern "system" fn BeginAbort<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginAbort(::core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn AbortRecord<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortRecord(::core::mem::transmute_copy(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfforget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbort<Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndAbort().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetLogControl: SetLogControl::<Impl, IMPL_OFFSET>,
            BeginPrepare: BeginPrepare::<Impl, IMPL_OFFSET>,
            PrepareRecord: PrepareRecord::<Impl, IMPL_OFFSET>,
            EndPrepare: EndPrepare::<Impl, IMPL_OFFSET>,
            BeginCommit: BeginCommit::<Impl, IMPL_OFFSET>,
            CommitRecord: CommitRecord::<Impl, IMPL_OFFSET>,
            EndCommit: EndCommit::<Impl, IMPL_OFFSET>,
            BeginAbort: BeginAbort::<Impl, IMPL_OFFSET>,
            AbortRecord: AbortRecord::<Impl, IMPL_OFFSET>,
            EndAbort: EndAbort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmCompensator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmCompensatorVariants_Impl: Sized {
    fn SetLogControlVariants(&mut self, plogcontrol: ::core::option::Option<ICrmLogControl>) -> ::windows::core::Result<()>;
    fn BeginPrepareVariants(&mut self) -> ::windows::core::Result<()>;
    fn PrepareRecordVariants(&mut self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn EndPrepareVariants(&mut self) -> ::windows::core::Result<i16>;
    fn BeginCommitVariants(&mut self, brecovery: i16) -> ::windows::core::Result<()>;
    fn CommitRecordVariants(&mut self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn EndCommitVariants(&mut self) -> ::windows::core::Result<()>;
    fn BeginAbortVariants(&mut self, brecovery: i16) -> ::windows::core::Result<()>;
    fn AbortRecordVariants(&mut self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn EndAbortVariants(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmCompensatorVariants_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmCompensatorVariants_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmCompensatorVariants_Vtbl {
        unsafe extern "system" fn SetLogControlVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLogControlVariants(::core::mem::transmute(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepareVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginPrepareVariants().into()
        }
        unsafe extern "system" fn PrepareRecordVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareRecordVariants(::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbforget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepareVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboktoprepare: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndPrepareVariants() {
                ::core::result::Result::Ok(ok__) => {
                    *pboktoprepare = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommitVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginCommitVariants(::core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn CommitRecordVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitRecordVariants(::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbforget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommitVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndCommitVariants().into()
        }
        unsafe extern "system" fn BeginAbortVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BeginAbortVariants(::core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn AbortRecordVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AbortRecordVariants(::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbforget = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbortVariants<Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndAbortVariants().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetLogControlVariants: SetLogControlVariants::<Impl, IMPL_OFFSET>,
            BeginPrepareVariants: BeginPrepareVariants::<Impl, IMPL_OFFSET>,
            PrepareRecordVariants: PrepareRecordVariants::<Impl, IMPL_OFFSET>,
            EndPrepareVariants: EndPrepareVariants::<Impl, IMPL_OFFSET>,
            BeginCommitVariants: BeginCommitVariants::<Impl, IMPL_OFFSET>,
            CommitRecordVariants: CommitRecordVariants::<Impl, IMPL_OFFSET>,
            EndCommitVariants: EndCommitVariants::<Impl, IMPL_OFFSET>,
            BeginAbortVariants: BeginAbortVariants::<Impl, IMPL_OFFSET>,
            AbortRecordVariants: AbortRecordVariants::<Impl, IMPL_OFFSET>,
            EndAbortVariants: EndAbortVariants::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmCompensatorVariants as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmFormatLogRecords_Impl: Sized {
    fn GetColumnCount(&mut self) -> ::windows::core::Result<i32>;
    fn GetColumnHeaders(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetColumn(&mut self, crmlogrec: CrmLogRecordRead) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetColumnVariants(&mut self, logrecord: super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmFormatLogRecords_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmFormatLogRecords_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmFormatLogRecords_Vtbl {
        unsafe extern "system" fn GetColumnCount<Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcolumncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    *plcolumncount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheaders: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pheaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumn(::core::mem::transmute_copy(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    *pformattedlogrecord = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnVariants<Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logrecord: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetColumnVariants(::core::mem::transmute_copy(&logrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    *pformattedlogrecord = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetColumnCount: GetColumnCount::<Impl, IMPL_OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Impl, IMPL_OFFSET>,
            GetColumn: GetColumn::<Impl, IMPL_OFFSET>,
            GetColumnVariants: GetColumnVariants::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmFormatLogRecords as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmLogControl_Impl: Sized {
    fn TransactionUOW(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RegisterCompensator(&mut self, lpcwstrprogidcompensator: super::super::Foundation::PWSTR, lpcwstrdescription: super::super::Foundation::PWSTR, lcrmregflags: i32) -> ::windows::core::Result<()>;
    fn WriteLogRecordVariants(&mut self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ForceLog(&mut self) -> ::windows::core::Result<()>;
    fn ForgetLogRecord(&mut self) -> ::windows::core::Result<()>;
    fn ForceTransactionToAbort(&mut self) -> ::windows::core::Result<()>;
    fn WriteLogRecord(&mut self, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmLogControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmLogControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmLogControl_Vtbl {
        unsafe extern "system" fn TransactionUOW<Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionUOW() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCompensator<Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcwstrprogidcompensator: super::super::Foundation::PWSTR, lpcwstrdescription: super::super::Foundation::PWSTR, lcrmregflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RegisterCompensator(::core::mem::transmute_copy(&lpcwstrprogidcompensator), ::core::mem::transmute_copy(&lpcwstrdescription), ::core::mem::transmute_copy(&lcrmregflags)).into()
        }
        unsafe extern "system" fn WriteLogRecordVariants<Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteLogRecordVariants(::core::mem::transmute_copy(&plogrecord)).into()
        }
        unsafe extern "system" fn ForceLog<Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForceLog().into()
        }
        unsafe extern "system" fn ForgetLogRecord<Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForgetLogRecord().into()
        }
        unsafe extern "system" fn ForceTransactionToAbort<Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ForceTransactionToAbort().into()
        }
        unsafe extern "system" fn WriteLogRecord<Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteLogRecord(::core::mem::transmute_copy(&rgblob), ::core::mem::transmute_copy(&cblob)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TransactionUOW: TransactionUOW::<Impl, IMPL_OFFSET>,
            RegisterCompensator: RegisterCompensator::<Impl, IMPL_OFFSET>,
            WriteLogRecordVariants: WriteLogRecordVariants::<Impl, IMPL_OFFSET>,
            ForceLog: ForceLog::<Impl, IMPL_OFFSET>,
            ForgetLogRecord: ForgetLogRecord::<Impl, IMPL_OFFSET>,
            ForceTransactionToAbort: ForceTransactionToAbort::<Impl, IMPL_OFFSET>,
            WriteLogRecord: WriteLogRecord::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmLogControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmMonitor_Impl: Sized {
    fn GetClerks(&mut self) -> ::windows::core::Result<ICrmMonitorClerks>;
    fn HoldClerk(&mut self, index: super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmMonitor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmMonitor_Vtbl {
        unsafe extern "system" fn GetClerks<Impl: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclerks: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClerks() {
                ::core::result::Result::Ok(ok__) => {
                    *pclerks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldClerk<Impl: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HoldClerk(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetClerks: GetClerks::<Impl, IMPL_OFFSET>,
            HoldClerk: HoldClerk::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmMonitorClerks_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&mut self, index: super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn ProgIdCompensator(&mut self, index: super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Description(&mut self, index: super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn TransactionUOW(&mut self, index: super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ActivityId(&mut self, index: super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmMonitorClerks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmMonitorClerks_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmMonitorClerks_Vtbl {
        unsafe extern "system" fn Item<Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgIdCompensator<Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgIdCompensator(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionUOW<Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionUOW(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivityId(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            ProgIdCompensator: ProgIdCompensator::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            TransactionUOW: TransactionUOW::<Impl, IMPL_OFFSET>,
            ActivityId: ActivityId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmMonitorClerks as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmMonitorLogRecords_Impl: Sized {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn TransactionState(&mut self) -> ::windows::core::Result<CrmTransactionState>;
    fn StructuredRecords(&mut self) -> ::windows::core::Result<i16>;
    fn GetLogRecord(&mut self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::core::Result<()>;
    fn GetLogRecordVariants(&mut self, indexnumber: super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmMonitorLogRecords_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ICrmMonitorLogRecords_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ICrmMonitorLogRecords_Vtbl {
        unsafe extern "system" fn Count<Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionState<Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut CrmTransactionState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TransactionState() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StructuredRecords<Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StructuredRecords() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogRecord<Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLogRecord(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcrmlogrec)).into()
        }
        unsafe extern "system" fn GetLogRecordVariants<Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexnumber: ::core::mem::ManuallyDrop<super::Com::VARIANT>, plogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLogRecordVariants(::core::mem::transmute_copy(&indexnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *plogrecord = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            TransactionState: TransactionState::<Impl, IMPL_OFFSET>,
            StructuredRecords: StructuredRecords::<Impl, IMPL_OFFSET>,
            GetLogRecord: GetLogRecord::<Impl, IMPL_OFFSET>,
            GetLogRecordVariants: GetLogRecordVariants::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmMonitorLogRecords as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDispenserDriver_Impl: Sized {
    fn CreateResource(&mut self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::core::Result<()>;
    fn RateResource(&mut self, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows::core::Result<()>;
    fn EnlistResource(&mut self, resid: usize, transid: usize) -> ::windows::core::Result<()>;
    fn ResetResource(&mut self, resid: usize) -> ::windows::core::Result<()>;
    fn DestroyResource(&mut self, resid: usize) -> ::windows::core::Result<()>;
    fn DestroyResourceS(&mut self, resid: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDispenserDriver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispenserDriver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispenserDriver_Vtbl {
        unsafe extern "system" fn CreateResource<Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateResource(::core::mem::transmute_copy(&restypid), ::core::mem::transmute_copy(&presid), ::core::mem::transmute_copy(&psecsfreebeforedestroy)).into()
        }
        unsafe extern "system" fn RateResource<Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RateResource(::core::mem::transmute_copy(&restypid), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&frequirestransactionenlistment), ::core::mem::transmute_copy(&prating)).into()
        }
        unsafe extern "system" fn EnlistResource<Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize, transid: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnlistResource(::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&transid)).into()
        }
        unsafe extern "system" fn ResetResource<Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetResource(::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResource<Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyResource(::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResourceS<Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyResourceS(::core::mem::transmute_copy(&resid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateResource: CreateResource::<Impl, IMPL_OFFSET>,
            RateResource: RateResource::<Impl, IMPL_OFFSET>,
            EnlistResource: EnlistResource::<Impl, IMPL_OFFSET>,
            ResetResource: ResetResource::<Impl, IMPL_OFFSET>,
            DestroyResource: DestroyResource::<Impl, IMPL_OFFSET>,
            DestroyResourceS: DestroyResourceS::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispenserDriver as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDispenserManager_Impl: Sized {
    fn RegisterDispenser(&mut self, __midl__idispensermanager0000: ::core::option::Option<IDispenserDriver>, szdispensername: super::super::Foundation::PWSTR) -> ::windows::core::Result<IHolder>;
    fn GetContext(&mut self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDispenserManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDispenserManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDispenserManager_Vtbl {
        unsafe extern "system" fn RegisterDispenser<Impl: IDispenserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0000: ::windows::core::RawPtr, szdispensername: super::super::Foundation::PWSTR, __midl__idispensermanager0001: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterDispenser(::core::mem::transmute(&__midl__idispensermanager0000), ::core::mem::transmute_copy(&szdispensername)) {
                ::core::result::Result::Ok(ok__) => {
                    *__midl__idispensermanager0001 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Impl: IDispenserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContext(::core::mem::transmute_copy(&__midl__idispensermanager0002), ::core::mem::transmute_copy(&__midl__idispensermanager0003)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            RegisterDispenser: RegisterDispenser::<Impl, IMPL_OFFSET>,
            GetContext: GetContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispenserManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumNames_Impl: Sized {
    fn Next(&mut self, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNames>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumNames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNames_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNames_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgname), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNames as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEventServerTrace_Impl: Sized + super::Com::IDispatch_Impl {
    fn StartTraceGuid(&mut self, bstrguidevent: super::super::Foundation::BSTR, bstrguidfilter: super::super::Foundation::BSTR, lpidfilter: i32) -> ::windows::core::Result<()>;
    fn StopTraceGuid(&mut self, bstrguidevent: super::super::Foundation::BSTR, bstrguidfilter: super::super::Foundation::BSTR, lpidfilter: i32) -> ::windows::core::Result<()>;
    fn EnumTraceGuid(&mut self, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEventServerTrace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventServerTrace_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventServerTrace_Vtbl {
        unsafe extern "system" fn StartTraceGuid<Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StartTraceGuid(::core::mem::transmute_copy(&bstrguidevent), ::core::mem::transmute_copy(&bstrguidfilter), ::core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn StopTraceGuid<Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).StopTraceGuid(::core::mem::transmute_copy(&bstrguidevent), ::core::mem::transmute_copy(&bstrguidfilter), ::core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn EnumTraceGuid<Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnumTraceGuid(::core::mem::transmute_copy(&plcntguids), ::core::mem::transmute_copy(&pbstrguidlist)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartTraceGuid: StartTraceGuid::<Impl, IMPL_OFFSET>,
            StopTraceGuid: StopTraceGuid::<Impl, IMPL_OFFSET>,
            EnumTraceGuid: EnumTraceGuid::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventServerTrace as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetAppTrackerData_Impl: Sized {
    fn GetApplicationProcesses(&mut self, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::core::Result<()>;
    fn GetApplicationProcessDetails(&mut self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetApplicationsInProcess(&mut self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::core::Result<()>;
    fn GetComponentsInProcess(&mut self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::core::Result<()>;
    fn GetComponentDetails(&mut self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, clsid: *const ::windows::core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::core::Result<()>;
    fn GetTrackerDataAsCollectionObject(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSuggestedPollingInterval(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IGetAppTrackerData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetAppTrackerData_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetAppTrackerData_Vtbl {
        unsafe extern "system" fn GetApplicationProcesses<Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetApplicationProcesses(::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&applicationid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numapplicationprocesses), ::core::mem::transmute_copy(&applicationprocesses)).into()
        }
        unsafe extern "system" fn GetApplicationProcessDetails<Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetApplicationProcessDetails(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&recycleinfo), ::core::mem::transmute_copy(&anycomponentshangmonitored)).into()
        }
        unsafe extern "system" fn GetApplicationsInProcess<Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetApplicationsInProcess(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numapplicationsinprocess), ::core::mem::transmute_copy(&applications)).into()
        }
        unsafe extern "system" fn GetComponentsInProcess<Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetComponentsInProcess(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&applicationid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numcomponentsinprocess), ::core::mem::transmute_copy(&components)).into()
        }
        unsafe extern "system" fn GetComponentDetails<Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, clsid: *const ::windows::core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetComponentDetails(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&hangmonitorinfo)).into()
        }
        unsafe extern "system" fn GetTrackerDataAsCollectionObject<Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toplevelcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTrackerDataAsCollectionObject() {
                ::core::result::Result::Ok(ok__) => {
                    *toplevelcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSuggestedPollingInterval<Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pollingintervalinseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSuggestedPollingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    *pollingintervalinseconds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetApplicationProcesses: GetApplicationProcesses::<Impl, IMPL_OFFSET>,
            GetApplicationProcessDetails: GetApplicationProcessDetails::<Impl, IMPL_OFFSET>,
            GetApplicationsInProcess: GetApplicationsInProcess::<Impl, IMPL_OFFSET>,
            GetComponentsInProcess: GetComponentsInProcess::<Impl, IMPL_OFFSET>,
            GetComponentDetails: GetComponentDetails::<Impl, IMPL_OFFSET>,
            GetTrackerDataAsCollectionObject: GetTrackerDataAsCollectionObject::<Impl, IMPL_OFFSET>,
            GetSuggestedPollingInterval: GetSuggestedPollingInterval::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetAppTrackerData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGetContextProperties_Impl: Sized {
    fn Count(&mut self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&mut self, name: super::super::Foundation::BSTR, pproperty: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EnumNames(&mut self) -> ::windows::core::Result<IEnumNames>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGetContextProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetContextProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetContextProperties_Vtbl {
        unsafe extern "system" fn Count<Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumNames() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            EnumNames: EnumNames::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetContextProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGetSecurityCallContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetSecurityCallContext(&mut self) -> ::windows::core::Result<ISecurityCallContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGetSecurityCallContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IGetSecurityCallContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IGetSecurityCallContext_Vtbl {
        unsafe extern "system" fn GetSecurityCallContext<Impl: IGetSecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSecurityCallContext() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetSecurityCallContext: GetSecurityCallContext::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetSecurityCallContext as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHolder_Impl: Sized {
    fn AllocResource(&mut self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::core::Result<()>;
    fn FreeResource(&mut self, __midl__iholder0002: usize) -> ::windows::core::Result<()>;
    fn TrackResource(&mut self, __midl__iholder0003: usize) -> ::windows::core::Result<()>;
    fn TrackResourceS(&mut self, __midl__iholder0004: *mut u16) -> ::windows::core::Result<()>;
    fn UntrackResource(&mut self, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UntrackResourceS(&mut self, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
    fn RequestDestroyResource(&mut self, __midl__iholder0009: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IHolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IHolder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IHolder_Vtbl {
        unsafe extern "system" fn AllocResource<Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AllocResource(::core::mem::transmute_copy(&__midl__iholder0000), ::core::mem::transmute_copy(&__midl__iholder0001)).into()
        }
        unsafe extern "system" fn FreeResource<Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0002: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FreeResource(::core::mem::transmute_copy(&__midl__iholder0002)).into()
        }
        unsafe extern "system" fn TrackResource<Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0003: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TrackResource(::core::mem::transmute_copy(&__midl__iholder0003)).into()
        }
        unsafe extern "system" fn TrackResourceS<Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0004: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TrackResourceS(::core::mem::transmute_copy(&__midl__iholder0004)).into()
        }
        unsafe extern "system" fn UntrackResource<Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UntrackResource(::core::mem::transmute_copy(&__midl__iholder0005), ::core::mem::transmute_copy(&__midl__iholder0006)).into()
        }
        unsafe extern "system" fn UntrackResourceS<Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UntrackResourceS(::core::mem::transmute_copy(&__midl__iholder0007), ::core::mem::transmute_copy(&__midl__iholder0008)).into()
        }
        unsafe extern "system" fn Close<Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        unsafe extern "system" fn RequestDestroyResource<Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0009: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestDestroyResource(::core::mem::transmute_copy(&__midl__iholder0009)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AllocResource: AllocResource::<Impl, IMPL_OFFSET>,
            FreeResource: FreeResource::<Impl, IMPL_OFFSET>,
            TrackResource: TrackResource::<Impl, IMPL_OFFSET>,
            TrackResourceS: TrackResourceS::<Impl, IMPL_OFFSET>,
            UntrackResource: UntrackResource::<Impl, IMPL_OFFSET>,
            UntrackResourceS: UntrackResourceS::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
            RequestDestroyResource: RequestDestroyResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILBEvents_Impl: Sized {
    fn TargetUp(&mut self, bstrservername: super::super::Foundation::BSTR, bstrclsideng: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TargetDown(&mut self, bstrservername: super::super::Foundation::BSTR, bstrclsideng: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EngineDefined(&mut self, bstrpropname: super::super::Foundation::BSTR, varpropvalue: *const super::Com::VARIANT, bstrclsideng: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILBEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILBEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILBEvents_Vtbl {
        unsafe extern "system" fn TargetUp<Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TargetUp(::core::mem::transmute_copy(&bstrservername), ::core::mem::transmute_copy(&bstrclsideng)).into()
        }
        unsafe extern "system" fn TargetDown<Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TargetDown(::core::mem::transmute_copy(&bstrservername), ::core::mem::transmute_copy(&bstrclsideng)).into()
        }
        unsafe extern "system" fn EngineDefined<Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varpropvalue: *const super::Com::VARIANT, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EngineDefined(::core::mem::transmute_copy(&bstrpropname), ::core::mem::transmute_copy(&varpropvalue), ::core::mem::transmute_copy(&bstrclsideng)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            TargetUp: TargetUp::<Impl, IMPL_OFFSET>,
            TargetDown: TargetDown::<Impl, IMPL_OFFSET>,
            EngineDefined: EngineDefined::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILBEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMTSActivity_Impl: Sized {
    fn SynchronousCall(&mut self, pcall: ::core::option::Option<IMTSCall>) -> ::windows::core::Result<()>;
    fn AsyncCall(&mut self, pcall: ::core::option::Option<IMTSCall>) -> ::windows::core::Result<()>;
    fn Reserved1(&mut self);
    fn BindToCurrentThread(&mut self) -> ::windows::core::Result<()>;
    fn UnbindFromThread(&mut self) -> ::windows::core::Result<()>;
}
impl IMTSActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMTSActivity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMTSActivity_Vtbl {
        unsafe extern "system" fn SynchronousCall<Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SynchronousCall(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn AsyncCall<Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsyncCall(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn Reserved1<Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved1()
        }
        unsafe extern "system" fn BindToCurrentThread<Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindToCurrentThread().into()
        }
        unsafe extern "system" fn UnbindFromThread<Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnbindFromThread().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SynchronousCall: SynchronousCall::<Impl, IMPL_OFFSET>,
            AsyncCall: AsyncCall::<Impl, IMPL_OFFSET>,
            Reserved1: Reserved1::<Impl, IMPL_OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Impl, IMPL_OFFSET>,
            UnbindFromThread: UnbindFromThread::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMTSActivity as ::windows::core::Interface>::IID
    }
}
pub trait IMTSCall_Impl: Sized {
    fn OnCall(&mut self) -> ::windows::core::Result<()>;
}
impl IMTSCall_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMTSCall_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMTSCall_Vtbl {
        unsafe extern "system" fn OnCall<Impl: IMTSCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCall().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCall: OnCall::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMTSCall as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMTSLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetEventDispatcher(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMTSLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMTSLocator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMTSLocator_Vtbl {
        unsafe extern "system" fn GetEventDispatcher<Impl: IMTSLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventDispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), GetEventDispatcher: GetEventDispatcher::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMTSLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedActivationEvents_Impl: Sized {
    fn CreateManagedStub(&mut self, pinfo: ::core::option::Option<IManagedObjectInfo>, fdist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DestroyManagedStub(&mut self, pinfo: ::core::option::Option<IManagedObjectInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IManagedActivationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedActivationEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManagedActivationEvents_Vtbl {
        unsafe extern "system" fn CreateManagedStub<Impl: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr, fdist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateManagedStub(::core::mem::transmute(&pinfo), ::core::mem::transmute_copy(&fdist)).into()
        }
        unsafe extern "system" fn DestroyManagedStub<Impl: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DestroyManagedStub(::core::mem::transmute(&pinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateManagedStub: CreateManagedStub::<Impl, IMPL_OFFSET>,
            DestroyManagedStub: DestroyManagedStub::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedActivationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedObjectInfo_Impl: Sized {
    fn GetIUnknown(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetIObjectControl(&mut self) -> ::windows::core::Result<IObjectControl>;
    fn SetInPool(&mut self, binpool: super::super::Foundation::BOOL, ppooledobj: ::core::option::Option<IManagedPooledObj>) -> ::windows::core::Result<()>;
    fn SetWrapperStrength(&mut self, bstrong: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IManagedObjectInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedObjectInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManagedObjectInfo_Vtbl {
        unsafe extern "system" fn GetIUnknown<Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIObjectControl<Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctrl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetIObjectControl() {
                ::core::result::Result::Ok(ok__) => {
                    *pctrl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPool<Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binpool: super::super::Foundation::BOOL, ppooledobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInPool(::core::mem::transmute_copy(&binpool), ::core::mem::transmute(&ppooledobj)).into()
        }
        unsafe extern "system" fn SetWrapperStrength<Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrong: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWrapperStrength(::core::mem::transmute_copy(&bstrong)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetIUnknown: GetIUnknown::<Impl, IMPL_OFFSET>,
            GetIObjectControl: GetIObjectControl::<Impl, IMPL_OFFSET>,
            SetInPool: SetInPool::<Impl, IMPL_OFFSET>,
            SetWrapperStrength: SetWrapperStrength::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedObjectInfo as ::windows::core::Interface>::IID
    }
}
pub trait IManagedPoolAction_Impl: Sized {
    fn LastRelease(&mut self) -> ::windows::core::Result<()>;
}
impl IManagedPoolAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedPoolAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManagedPoolAction_Vtbl {
        unsafe extern "system" fn LastRelease<Impl: IManagedPoolAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LastRelease().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), LastRelease: LastRelease::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedPoolAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedPooledObj_Impl: Sized {
    fn SetHeld(&mut self, m_bheld: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IManagedPooledObj_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IManagedPooledObj_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IManagedPooledObj_Vtbl {
        unsafe extern "system" fn SetHeld<Impl: IManagedPooledObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, m_bheld: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetHeld(::core::mem::transmute_copy(&m_bheld)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SetHeld: SetHeld::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedPooledObj as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMessageMover_Impl: Sized + super::Com::IDispatch_Impl {
    fn SourcePath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSourcePath(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DestPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDestPath(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CommitBatchSize(&mut self) -> ::windows::core::Result<i32>;
    fn SetCommitBatchSize(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn MoveMessages(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMessageMover_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMessageMover_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMessageMover_Vtbl {
        unsafe extern "system" fn SourcePath<Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SourcePath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourcePath<Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSourcePath(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn DestPath<Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DestPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestPath<Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDestPath(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn CommitBatchSize<Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommitBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitBatchSize<Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCommitBatchSize(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MoveMessages<Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagesmoved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveMessages() {
                ::core::result::Result::Ok(ok__) => {
                    *plmessagesmoved = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SourcePath: SourcePath::<Impl, IMPL_OFFSET>,
            SetSourcePath: SetSourcePath::<Impl, IMPL_OFFSET>,
            DestPath: DestPath::<Impl, IMPL_OFFSET>,
            SetDestPath: SetDestPath::<Impl, IMPL_OFFSET>,
            CommitBatchSize: CommitBatchSize::<Impl, IMPL_OFFSET>,
            SetCommitBatchSize: SetCommitBatchSize::<Impl, IMPL_OFFSET>,
            MoveMessages: MoveMessages::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageMover as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMtsEventInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Names(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn DisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EventID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Value(&mut self, skey: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMtsEventInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMtsEventInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMtsEventInfo_Vtbl {
        unsafe extern "system" fn Names<Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Names() {
                ::core::result::Result::Ok(ok__) => {
                    *punk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *sdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventID<Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sguideventid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventID() {
                ::core::result::Result::Ok(ok__) => {
                    *sguideventid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *lcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Value<Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value(::core::mem::transmute_copy(&skey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Names: Names::<Impl, IMPL_OFFSET>,
            DisplayName: DisplayName::<Impl, IMPL_OFFSET>,
            EventID: EventID::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMtsEventInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMtsEvents_Impl: Sized + super::Com::IDispatch_Impl {
    fn PackageName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PackageGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PostEvent(&mut self, vevent: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn FireEvents(&mut self) -> ::windows::core::Result<i16>;
    fn GetProcessID(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMtsEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMtsEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMtsEvents_Vtbl {
        unsafe extern "system" fn PackageName<Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageGuid<Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PackageGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostEvent<Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PostEvent(::core::mem::transmute_copy(&vevent)).into()
        }
        unsafe extern "system" fn FireEvents<Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FireEvents() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessID<Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProcessID() {
                ::core::result::Result::Ok(ok__) => {
                    *id = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PackageName: PackageName::<Impl, IMPL_OFFSET>,
            PackageGuid: PackageGuid::<Impl, IMPL_OFFSET>,
            PostEvent: PostEvent::<Impl, IMPL_OFFSET>,
            FireEvents: FireEvents::<Impl, IMPL_OFFSET>,
            GetProcessID: GetProcessID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMtsEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMtsGrp_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMtsGrp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMtsGrp_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMtsGrp_Vtbl {
        unsafe extern "system" fn Count<Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkdispatcher = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Refresh().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Refresh: Refresh::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMtsGrp as ::windows::core::Interface>::IID
    }
}
pub trait IObjPool_Impl: Sized {
    fn Reserved1(&mut self);
    fn Reserved2(&mut self);
    fn Reserved3(&mut self);
    fn Reserved4(&mut self);
    fn PutEndTx(&mut self, pobj: ::core::option::Option<::windows::core::IUnknown>);
    fn Reserved5(&mut self);
    fn Reserved6(&mut self);
}
impl IObjPool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjPool_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjPool_Vtbl {
        unsafe extern "system" fn Reserved1<Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved1()
        }
        unsafe extern "system" fn Reserved2<Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved2()
        }
        unsafe extern "system" fn Reserved3<Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved3()
        }
        unsafe extern "system" fn Reserved4<Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved4()
        }
        unsafe extern "system" fn PutEndTx<Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobj: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutEndTx(::core::mem::transmute(&pobj))
        }
        unsafe extern "system" fn Reserved5<Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved5()
        }
        unsafe extern "system" fn Reserved6<Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved6()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Reserved1: Reserved1::<Impl, IMPL_OFFSET>,
            Reserved2: Reserved2::<Impl, IMPL_OFFSET>,
            Reserved3: Reserved3::<Impl, IMPL_OFFSET>,
            Reserved4: Reserved4::<Impl, IMPL_OFFSET>,
            PutEndTx: PutEndTx::<Impl, IMPL_OFFSET>,
            Reserved5: Reserved5::<Impl, IMPL_OFFSET>,
            Reserved6: Reserved6::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjPool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstruct_Impl: Sized {
    fn Construct(&mut self, pctorobj: ::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstruct_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectConstruct_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectConstruct_Vtbl {
        unsafe extern "system" fn Construct<Impl: IObjectConstruct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctorobj: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Construct(::core::mem::transmute(&pctorobj)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Construct: Construct::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectConstruct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IObjectConstructString_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConstructString(&mut self, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IObjectConstructString_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectConstructString_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectConstructString_Vtbl {
        unsafe extern "system" fn ConstructString<Impl: IObjectConstructString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConstructString(::core::mem::transmute_copy(&pval)).into()
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ConstructString: ConstructString::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectConstructString as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContext_Impl: Sized {
    fn CreateInstance(&mut self, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetComplete(&mut self) -> ::windows::core::Result<()>;
    fn SetAbort(&mut self) -> ::windows::core::Result<()>;
    fn EnableCommit(&mut self) -> ::windows::core::Result<()>;
    fn DisableCommit(&mut self) -> ::windows::core::Result<()>;
    fn IsInTransaction(&mut self) -> super::super::Foundation::BOOL;
    fn IsSecurityEnabled(&mut self) -> super::super::Foundation::BOOL;
    fn IsCallerInRole(&mut self, bstrrole: super::super::Foundation::BSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstance(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn SetComplete<Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComplete().into()
        }
        unsafe extern "system" fn SetAbort<Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAbort().into()
        }
        unsafe extern "system" fn EnableCommit<Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableCommit().into()
        }
        unsafe extern "system" fn DisableCommit<Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableCommit().into()
        }
        unsafe extern "system" fn IsInTransaction<Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsInTransaction()
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSecurityEnabled()
        }
        unsafe extern "system" fn IsCallerInRole<Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsCallerInRole(::core::mem::transmute_copy(&bstrrole), ::core::mem::transmute_copy(&pfisinrole)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            SetComplete: SetComplete::<Impl, IMPL_OFFSET>,
            SetAbort: SetAbort::<Impl, IMPL_OFFSET>,
            EnableCommit: EnableCommit::<Impl, IMPL_OFFSET>,
            DisableCommit: DisableCommit::<Impl, IMPL_OFFSET>,
            IsInTransaction: IsInTransaction::<Impl, IMPL_OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Impl, IMPL_OFFSET>,
            IsCallerInRole: IsCallerInRole::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContext as ::windows::core::Interface>::IID
    }
}
pub trait IObjectContextActivity_Impl: Sized {
    fn GetActivityId(&mut self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IObjectContextActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextActivity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContextActivity_Vtbl {
        unsafe extern "system" fn GetActivityId<Impl: IObjectContextActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetActivityId(::core::mem::transmute_copy(&pguid)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetActivityId: GetActivityId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextActivity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfo_Impl: Sized {
    fn IsInTransaction(&mut self) -> super::super::Foundation::BOOL;
    fn GetTransaction(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetTransactionId(&mut self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetActivityId(&mut self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetContextId(&mut self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContextInfo_Vtbl {
        unsafe extern "system" fn IsInTransaction<Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsInTransaction()
        }
        unsafe extern "system" fn GetTransaction<Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrans: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pptrans = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTransactionId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetActivityId<Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetActivityId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetContextId<Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetContextId(::core::mem::transmute_copy(&pguid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsInTransaction: IsInTransaction::<Impl, IMPL_OFFSET>,
            GetTransaction: GetTransaction::<Impl, IMPL_OFFSET>,
            GetTransactionId: GetTransactionId::<Impl, IMPL_OFFSET>,
            GetActivityId: GetActivityId::<Impl, IMPL_OFFSET>,
            GetContextId: GetContextId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfo2_Impl: Sized + IObjectContextInfo_Impl {
    fn GetPartitionId(&mut self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetApplicationId(&mut self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetApplicationInstanceId(&mut self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextInfo2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContextInfo2_Vtbl {
        unsafe extern "system" fn GetPartitionId<Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPartitionId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationId<Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetApplicationId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationInstanceId<Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetApplicationInstanceId(::core::mem::transmute_copy(&pguid)).into()
        }
        Self {
            base: IObjectContextInfo_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPartitionId: GetPartitionId::<Impl, IMPL_OFFSET>,
            GetApplicationId: GetApplicationId::<Impl, IMPL_OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextInfo2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextTip_Impl: Sized {
    fn GetTipUrl(&mut self, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextTip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectContextTip_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectContextTip_Vtbl {
        unsafe extern "system" fn GetTipUrl<Impl: IObjectContextTip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTipUrl(::core::mem::transmute_copy(&ptipurl)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetTipUrl: GetTipUrl::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextTip as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectControl_Impl: Sized {
    fn Activate(&mut self) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self);
    fn CanBePooled(&mut self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl IObjectControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IObjectControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IObjectControl_Vtbl {
        unsafe extern "system" fn Activate<Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate().into()
        }
        unsafe extern "system" fn Deactivate<Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate()
        }
        unsafe extern "system" fn CanBePooled<Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CanBePooled()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
            CanBePooled: CanBePooled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectControl as ::windows::core::Interface>::IID
    }
}
pub trait IPlaybackControl_Impl: Sized {
    fn FinalClientRetry(&mut self) -> ::windows::core::Result<()>;
    fn FinalServerRetry(&mut self) -> ::windows::core::Result<()>;
}
impl IPlaybackControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPlaybackControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPlaybackControl_Vtbl {
        unsafe extern "system" fn FinalClientRetry<Impl: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinalClientRetry().into()
        }
        unsafe extern "system" fn FinalServerRetry<Impl: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FinalServerRetry().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FinalClientRetry: FinalClientRetry::<Impl, IMPL_OFFSET>,
            FinalServerRetry: FinalServerRetry::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPoolManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn ShutdownPool(&mut self, clsidorprogid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPoolManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPoolManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPoolManager_Vtbl {
        unsafe extern "system" fn ShutdownPool<Impl: IPoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ShutdownPool(::core::mem::transmute_copy(&clsidorprogid)).into()
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ShutdownPool: ShutdownPool::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPoolManager as ::windows::core::Interface>::IID
    }
}
pub trait IProcessInitializer_Impl: Sized {
    fn Startup(&mut self, punkprocesscontrol: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
}
impl IProcessInitializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProcessInitializer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProcessInitializer_Vtbl {
        unsafe extern "system" fn Startup<Impl: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkprocesscontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Startup(::core::mem::transmute(&punkprocesscontrol)).into()
        }
        unsafe extern "system" fn Shutdown<Impl: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Startup: Startup::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISecurityCallContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn IsCallerInRole(&mut self, bstrrole: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn IsSecurityEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn IsUserInRole(&mut self, puser: *const super::Com::VARIANT, bstrrole: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISecurityCallContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityCallContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityCallContext_Vtbl {
        unsafe extern "system" fn Count<Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCallerInRole(::core::mem::transmute_copy(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecurityEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfisenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserInRole<Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *const super::Com::VARIANT, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUserInRole(::core::mem::transmute_copy(&puser), ::core::mem::transmute_copy(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *pfinrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            IsCallerInRole: IsCallerInRole::<Impl, IMPL_OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Impl, IMPL_OFFSET>,
            IsUserInRole: IsUserInRole::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityCallContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISecurityCallersColl_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, lindex: i32) -> ::windows::core::Result<ISecurityIdentityColl>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISecurityCallersColl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityCallersColl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityCallersColl_Vtbl {
        unsafe extern "system" fn Count<Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pobj = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityCallersColl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISecurityIdentityColl_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISecurityIdentityColl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityIdentityColl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityIdentityColl_Vtbl {
        unsafe extern "system" fn Count<Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityIdentityColl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityProperty_Impl: Sized {
    fn GetDirectCreatorSID(&mut self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()>;
    fn GetOriginalCreatorSID(&mut self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()>;
    fn GetDirectCallerSID(&mut self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()>;
    fn GetOriginalCallerSID(&mut self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()>;
    fn ReleaseSID(&mut self, psid: super::super::Foundation::PSID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISecurityProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISecurityProperty_Vtbl {
        unsafe extern "system" fn GetDirectCreatorSID<Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDirectCreatorSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCreatorSID<Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOriginalCreatorSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetDirectCallerSID<Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDirectCallerSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCallerSID<Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetOriginalCallerSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn ReleaseSID<Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseSID(::core::mem::transmute_copy(&psid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDirectCreatorSID: GetDirectCreatorSID::<Impl, IMPL_OFFSET>,
            GetOriginalCreatorSID: GetOriginalCreatorSID::<Impl, IMPL_OFFSET>,
            GetDirectCallerSID: GetDirectCallerSID::<Impl, IMPL_OFFSET>,
            GetOriginalCallerSID: GetOriginalCallerSID::<Impl, IMPL_OFFSET>,
            ReleaseSID: ReleaseSID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityProperty as ::windows::core::Interface>::IID
    }
}
pub trait ISelectCOMLBServer_Impl: Sized {
    fn Init(&mut self) -> ::windows::core::Result<()>;
    fn GetLBServer(&mut self, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ISelectCOMLBServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISelectCOMLBServer_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISelectCOMLBServer_Vtbl {
        unsafe extern "system" fn Init<Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init().into()
        }
        unsafe extern "system" fn GetLBServer<Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetLBServer(::core::mem::transmute(&punk)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            GetLBServer: GetLBServer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectCOMLBServer as ::windows::core::Interface>::IID
    }
}
pub trait ISendMethodEvents_Impl: Sized {
    fn SendMethodCall(&mut self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32) -> ::windows::core::Result<()>;
    fn SendMethodReturn(&mut self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32, hrcall: ::windows::core::HRESULT, hrserver: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ISendMethodEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISendMethodEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISendMethodEvents_Vtbl {
        unsafe extern "system" fn SendMethodCall<Impl: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMethodCall(::core::mem::transmute_copy(&pidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwmeth)).into()
        }
        unsafe extern "system" fn SendMethodReturn<Impl: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32, hrcall: ::windows::core::HRESULT, hrserver: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendMethodReturn(::core::mem::transmute_copy(&pidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwmeth), ::core::mem::transmute_copy(&hrcall), ::core::mem::transmute_copy(&hrserver)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SendMethodCall: SendMethodCall::<Impl, IMPL_OFFSET>,
            SendMethodReturn: SendMethodReturn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISendMethodEvents as ::windows::core::Interface>::IID
    }
}
pub trait IServiceActivity_Impl: Sized {
    fn SynchronousCall(&mut self, piservicecall: ::core::option::Option<IServiceCall>) -> ::windows::core::Result<()>;
    fn AsynchronousCall(&mut self, piservicecall: ::core::option::Option<IServiceCall>) -> ::windows::core::Result<()>;
    fn BindToCurrentThread(&mut self) -> ::windows::core::Result<()>;
    fn UnbindFromThread(&mut self) -> ::windows::core::Result<()>;
}
impl IServiceActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceActivity_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceActivity_Vtbl {
        unsafe extern "system" fn SynchronousCall<Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SynchronousCall(::core::mem::transmute(&piservicecall)).into()
        }
        unsafe extern "system" fn AsynchronousCall<Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AsynchronousCall(::core::mem::transmute(&piservicecall)).into()
        }
        unsafe extern "system" fn BindToCurrentThread<Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindToCurrentThread().into()
        }
        unsafe extern "system" fn UnbindFromThread<Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnbindFromThread().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SynchronousCall: SynchronousCall::<Impl, IMPL_OFFSET>,
            AsynchronousCall: AsynchronousCall::<Impl, IMPL_OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Impl, IMPL_OFFSET>,
            UnbindFromThread: UnbindFromThread::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceActivity as ::windows::core::Interface>::IID
    }
}
pub trait IServiceCall_Impl: Sized {
    fn OnCall(&mut self) -> ::windows::core::Result<()>;
}
impl IServiceCall_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceCall_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceCall_Vtbl {
        unsafe extern "system" fn OnCall<Impl: IServiceCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCall().into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnCall: OnCall::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceCall as ::windows::core::Interface>::IID
    }
}
pub trait IServiceComTIIntrinsicsConfig_Impl: Sized {
    fn ComTIIntrinsicsConfig(&mut self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::core::Result<()>;
}
impl IServiceComTIIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceComTIIntrinsicsConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceComTIIntrinsicsConfig_Vtbl {
        unsafe extern "system" fn ComTIIntrinsicsConfig<Impl: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ComTIIntrinsicsConfig(::core::mem::transmute_copy(&comtiintrinsicsconfig)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ComTIIntrinsicsConfig: ComTIIntrinsicsConfig::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceComTIIntrinsicsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceIISIntrinsicsConfig_Impl: Sized {
    fn IISIntrinsicsConfig(&mut self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::core::Result<()>;
}
impl IServiceIISIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceIISIntrinsicsConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceIISIntrinsicsConfig_Vtbl {
        unsafe extern "system" fn IISIntrinsicsConfig<Impl: IServiceIISIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IISIntrinsicsConfig(::core::mem::transmute_copy(&iisintrinsicsconfig)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), IISIntrinsicsConfig: IISIntrinsicsConfig::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceIISIntrinsicsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceInheritanceConfig_Impl: Sized {
    fn ContainingContextTreatment(&mut self, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::core::Result<()>;
}
impl IServiceInheritanceConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceInheritanceConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceInheritanceConfig_Vtbl {
        unsafe extern "system" fn ContainingContextTreatment<Impl: IServiceInheritanceConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ContainingContextTreatment(::core::mem::transmute_copy(&inheritanceconfig)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ContainingContextTreatment: ContainingContextTreatment::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceInheritanceConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServicePartitionConfig_Impl: Sized {
    fn PartitionConfig(&mut self, partitionconfig: CSC_PartitionConfig) -> ::windows::core::Result<()>;
    fn PartitionID(&mut self, guidpartitionid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IServicePartitionConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServicePartitionConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServicePartitionConfig_Vtbl {
        unsafe extern "system" fn PartitionConfig<Impl: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PartitionConfig(::core::mem::transmute_copy(&partitionconfig)).into()
        }
        unsafe extern "system" fn PartitionID<Impl: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidpartitionid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PartitionID(::core::mem::transmute_copy(&guidpartitionid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PartitionConfig: PartitionConfig::<Impl, IMPL_OFFSET>,
            PartitionID: PartitionID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServicePartitionConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServicePool_Impl: Sized {
    fn Initialize(&mut self, ppoolconfig: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetObject(&mut self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Shutdown(&mut self) -> ::windows::core::Result<()>;
}
impl IServicePool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServicePool_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServicePool_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoolconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&ppoolconfig)).into()
        }
        unsafe extern "system" fn GetObject<Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Shutdown<Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Shutdown().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            GetObject: GetObject::<Impl, IMPL_OFFSET>,
            Shutdown: Shutdown::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServicePool as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IServicePoolConfig_Impl: Sized {
    fn SetMaxPoolSize(&mut self, dwmaxpool: u32) -> ::windows::core::Result<()>;
    fn MaxPoolSize(&mut self, pdwmaxpool: *mut u32) -> ::windows::core::Result<()>;
    fn SetMinPoolSize(&mut self, dwminpool: u32) -> ::windows::core::Result<()>;
    fn MinPoolSize(&mut self, pdwminpool: *mut u32) -> ::windows::core::Result<()>;
    fn SetCreationTimeout(&mut self, dwcreationtimeout: u32) -> ::windows::core::Result<()>;
    fn CreationTimeout(&mut self, pdwcreationtimeout: *mut u32) -> ::windows::core::Result<()>;
    fn SetTransactionAffinity(&mut self, ftxaffinity: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn TransactionAffinity(&mut self, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetClassFactory(&mut self, pfactory: ::core::option::Option<super::Com::IClassFactory>) -> ::windows::core::Result<()>;
    fn ClassFactory(&mut self) -> ::windows::core::Result<super::Com::IClassFactory>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IServicePoolConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServicePoolConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServicePoolConfig_Vtbl {
        unsafe extern "system" fn SetMaxPoolSize<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxpool: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxPoolSize(::core::mem::transmute_copy(&dwmaxpool)).into()
        }
        unsafe extern "system" fn MaxPoolSize<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxpool: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MaxPoolSize(::core::mem::transmute_copy(&pdwmaxpool)).into()
        }
        unsafe extern "system" fn SetMinPoolSize<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminpool: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinPoolSize(::core::mem::transmute_copy(&dwminpool)).into()
        }
        unsafe extern "system" fn MinPoolSize<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminpool: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MinPoolSize(::core::mem::transmute_copy(&pdwminpool)).into()
        }
        unsafe extern "system" fn SetCreationTimeout<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreationtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreationTimeout(::core::mem::transmute_copy(&dwcreationtimeout)).into()
        }
        unsafe extern "system" fn CreationTimeout<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcreationtimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreationTimeout(::core::mem::transmute_copy(&pdwcreationtimeout)).into()
        }
        unsafe extern "system" fn SetTransactionAffinity<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftxaffinity: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransactionAffinity(::core::mem::transmute_copy(&ftxaffinity)).into()
        }
        unsafe extern "system" fn TransactionAffinity<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransactionAffinity(::core::mem::transmute_copy(&pftxaffinity)).into()
        }
        unsafe extern "system" fn SetClassFactory<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClassFactory(::core::mem::transmute(&pfactory)).into()
        }
        unsafe extern "system" fn ClassFactory<Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClassFactory() {
                ::core::result::Result::Ok(ok__) => {
                    *pfactory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMaxPoolSize: SetMaxPoolSize::<Impl, IMPL_OFFSET>,
            MaxPoolSize: MaxPoolSize::<Impl, IMPL_OFFSET>,
            SetMinPoolSize: SetMinPoolSize::<Impl, IMPL_OFFSET>,
            MinPoolSize: MinPoolSize::<Impl, IMPL_OFFSET>,
            SetCreationTimeout: SetCreationTimeout::<Impl, IMPL_OFFSET>,
            CreationTimeout: CreationTimeout::<Impl, IMPL_OFFSET>,
            SetTransactionAffinity: SetTransactionAffinity::<Impl, IMPL_OFFSET>,
            TransactionAffinity: TransactionAffinity::<Impl, IMPL_OFFSET>,
            SetClassFactory: SetClassFactory::<Impl, IMPL_OFFSET>,
            ClassFactory: ClassFactory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServicePoolConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IServiceSxsConfig_Impl: Sized {
    fn SxsConfig(&mut self, scsconfig: CSC_SxsConfig) -> ::windows::core::Result<()>;
    fn SxsName(&mut self, szsxsname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn SxsDirectory(&mut self, szsxsdirectory: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IServiceSxsConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceSxsConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceSxsConfig_Vtbl {
        unsafe extern "system" fn SxsConfig<Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scsconfig: CSC_SxsConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SxsConfig(::core::mem::transmute_copy(&scsconfig)).into()
        }
        unsafe extern "system" fn SxsName<Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SxsName(::core::mem::transmute_copy(&szsxsname)).into()
        }
        unsafe extern "system" fn SxsDirectory<Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsdirectory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SxsDirectory(::core::mem::transmute_copy(&szsxsdirectory)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SxsConfig: SxsConfig::<Impl, IMPL_OFFSET>,
            SxsName: SxsName::<Impl, IMPL_OFFSET>,
            SxsDirectory: SxsDirectory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceSxsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceSynchronizationConfig_Impl: Sized {
    fn ConfigureSynchronization(&mut self, synchconfig: CSC_SynchronizationConfig) -> ::windows::core::Result<()>;
}
impl IServiceSynchronizationConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceSynchronizationConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceSynchronizationConfig_Vtbl {
        unsafe extern "system" fn ConfigureSynchronization<Impl: IServiceSynchronizationConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureSynchronization(::core::mem::transmute_copy(&synchconfig)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), ConfigureSynchronization: ConfigureSynchronization::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceSynchronizationConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait IServiceSysTxnConfig_Impl: Sized + IServiceTransactionConfigBase_Impl + IServiceTransactionConfig_Impl {
    fn ConfigureBYOTSysTxn(&mut self, ptxproxy: ::core::option::Option<ITransactionProxy>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl IServiceSysTxnConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceSysTxnConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceSysTxnConfig_Vtbl {
        unsafe extern "system" fn ConfigureBYOTSysTxn<Impl: IServiceSysTxnConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxproxy: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureBYOTSysTxn(::core::mem::transmute(&ptxproxy)).into()
        }
        Self {
            base: IServiceTransactionConfig_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ConfigureBYOTSysTxn: ConfigureBYOTSysTxn::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceSysTxnConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceThreadPoolConfig_Impl: Sized {
    fn SelectThreadPool(&mut self, threadpool: CSC_ThreadPool) -> ::windows::core::Result<()>;
    fn SetBindingInfo(&mut self, binding: CSC_Binding) -> ::windows::core::Result<()>;
}
impl IServiceThreadPoolConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceThreadPoolConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceThreadPoolConfig_Vtbl {
        unsafe extern "system" fn SelectThreadPool<Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadpool: CSC_ThreadPool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SelectThreadPool(::core::mem::transmute_copy(&threadpool)).into()
        }
        unsafe extern "system" fn SetBindingInfo<Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: CSC_Binding) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBindingInfo(::core::mem::transmute_copy(&binding)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SelectThreadPool: SelectThreadPool::<Impl, IMPL_OFFSET>,
            SetBindingInfo: SetBindingInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceThreadPoolConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IServiceTrackerConfig_Impl: Sized {
    fn TrackerConfig(&mut self, trackerconfig: CSC_TrackerConfig, sztrackerappname: super::super::Foundation::PWSTR, sztrackerctxname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IServiceTrackerConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceTrackerConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceTrackerConfig_Vtbl {
        unsafe extern "system" fn TrackerConfig<Impl: IServiceTrackerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: super::super::Foundation::PWSTR, sztrackerctxname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TrackerConfig(::core::mem::transmute_copy(&trackerconfig), ::core::mem::transmute_copy(&sztrackerappname), ::core::mem::transmute_copy(&sztrackerctxname)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), TrackerConfig: TrackerConfig::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceTrackerConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait IServiceTransactionConfig_Impl: Sized + IServiceTransactionConfigBase_Impl {
    fn ConfigureBYOT(&mut self, pitxbyot: ::core::option::Option<super::DistributedTransactionCoordinator::ITransaction>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl IServiceTransactionConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceTransactionConfig_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceTransactionConfig_Vtbl {
        unsafe extern "system" fn ConfigureBYOT<Impl: IServiceTransactionConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitxbyot: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureBYOT(::core::mem::transmute(&pitxbyot)).into()
        }
        Self { base: IServiceTransactionConfigBase_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ConfigureBYOT: ConfigureBYOT::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceTransactionConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IServiceTransactionConfigBase_Impl: Sized {
    fn ConfigureTransaction(&mut self, transactionconfig: CSC_TransactionConfig) -> ::windows::core::Result<()>;
    fn IsolationLevel(&mut self, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::Result<()>;
    fn TransactionTimeout(&mut self, ultimeoutsec: u32) -> ::windows::core::Result<()>;
    fn BringYourOwnTransaction(&mut self, sztipurl: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn NewTransactionDescription(&mut self, sztxdesc: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IServiceTransactionConfigBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IServiceTransactionConfigBase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IServiceTransactionConfigBase_Vtbl {
        unsafe extern "system" fn ConfigureTransaction<Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ConfigureTransaction(::core::mem::transmute_copy(&transactionconfig)).into()
        }
        unsafe extern "system" fn IsolationLevel<Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsolationLevel(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn TransactionTimeout<Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TransactionTimeout(::core::mem::transmute_copy(&ultimeoutsec)).into()
        }
        unsafe extern "system" fn BringYourOwnTransaction<Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztipurl: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BringYourOwnTransaction(::core::mem::transmute_copy(&sztipurl)).into()
        }
        unsafe extern "system" fn NewTransactionDescription<Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztxdesc: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NewTransactionDescription(::core::mem::transmute_copy(&sztxdesc)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ConfigureTransaction: ConfigureTransaction::<Impl, IMPL_OFFSET>,
            IsolationLevel: IsolationLevel::<Impl, IMPL_OFFSET>,
            TransactionTimeout: TransactionTimeout::<Impl, IMPL_OFFSET>,
            BringYourOwnTransaction: BringYourOwnTransaction::<Impl, IMPL_OFFSET>,
            NewTransactionDescription: NewTransactionDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceTransactionConfigBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISharedProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&mut self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&mut self, val: super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISharedProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedProperty_Vtbl {
        unsafe extern "system" fn Value<Impl: ISharedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: ISharedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&val)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISharedPropertyGroup_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreatePropertyByPosition(&mut self, index: i32, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows::core::Result<()>;
    fn PropertyByPosition(&mut self, index: i32) -> ::windows::core::Result<ISharedProperty>;
    fn CreateProperty(&mut self, name: super::super::Foundation::BSTR, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows::core::Result<()>;
    fn Property(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<ISharedProperty>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISharedPropertyGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPropertyGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPropertyGroup_Vtbl {
        unsafe extern "system" fn CreatePropertyByPosition<Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, fexists: *mut i16, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePropertyByPosition(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn PropertyByPosition<Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PropertyByPosition(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperty<Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fexists: *mut i16, ppprop: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateProperty(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn Property<Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Property(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreatePropertyByPosition: CreatePropertyByPosition::<Impl, IMPL_OFFSET>,
            PropertyByPosition: PropertyByPosition::<Impl, IMPL_OFFSET>,
            CreateProperty: CreateProperty::<Impl, IMPL_OFFSET>,
            Property: Property::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPropertyGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISharedPropertyGroupManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreatePropertyGroup(&mut self, name: super::super::Foundation::BSTR, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::core::option::Option<ISharedPropertyGroup>) -> ::windows::core::Result<()>;
    fn Group(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<ISharedPropertyGroup>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISharedPropertyGroupManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISharedPropertyGroupManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISharedPropertyGroupManager_Vtbl {
        unsafe extern "system" fn CreatePropertyGroup<Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreatePropertyGroup(::core::mem::transmute_copy(&name), ::core::mem::transmute_copy(&dwisomode), ::core::mem::transmute_copy(&dwrelmode), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppgroup)).into()
        }
        unsafe extern "system" fn Group<Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Group(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *retval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreatePropertyGroup: CreatePropertyGroup::<Impl, IMPL_OFFSET>,
            Group: Group::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPropertyGroupManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISystemAppEventData_Impl: Sized {
    fn Startup(&mut self) -> ::windows::core::Result<()>;
    fn OnDataChanged(&mut self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: super::super::Foundation::BSTR, dwreason: u32, u64tracehandle: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISystemAppEventData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISystemAppEventData_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISystemAppEventData_Vtbl {
        unsafe extern "system" fn Startup<Impl: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Startup().into()
        }
        unsafe extern "system" fn OnDataChanged<Impl: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataChanged(::core::mem::transmute_copy(&dwpid), ::core::mem::transmute_copy(&dwmask), ::core::mem::transmute_copy(&dwnumbersinks), ::core::mem::transmute_copy(&bstrdwmethodmask), ::core::mem::transmute_copy(&dwreason), ::core::mem::transmute_copy(&u64tracehandle)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Startup: Startup::<Impl, IMPL_OFFSET>,
            OnDataChanged: OnDataChanged::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemAppEventData as ::windows::core::Interface>::IID
    }
}
pub trait IThreadPoolKnobs_Impl: Sized {
    fn GetMaxThreads(&mut self, plcmaxthreads: *mut i32) -> ::windows::core::Result<()>;
    fn GetCurrentThreads(&mut self, plccurrentthreads: *mut i32) -> ::windows::core::Result<()>;
    fn SetMaxThreads(&mut self, lcmaxthreads: i32) -> ::windows::core::Result<()>;
    fn GetDeleteDelay(&mut self, pmsecdeletedelay: *mut i32) -> ::windows::core::Result<()>;
    fn SetDeleteDelay(&mut self, msecdeletedelay: i32) -> ::windows::core::Result<()>;
    fn GetMaxQueuedRequests(&mut self, plcmaxqueuedrequests: *mut i32) -> ::windows::core::Result<()>;
    fn GetCurrentQueuedRequests(&mut self, plccurrentqueuedrequests: *mut i32) -> ::windows::core::Result<()>;
    fn SetMaxQueuedRequests(&mut self, lcmaxqueuedrequests: i32) -> ::windows::core::Result<()>;
    fn SetMinThreads(&mut self, lcminthreads: i32) -> ::windows::core::Result<()>;
    fn SetQueueDepth(&mut self, lcqueuedepth: i32) -> ::windows::core::Result<()>;
}
impl IThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IThreadPoolKnobs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn GetMaxThreads<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxthreads: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMaxThreads(::core::mem::transmute_copy(&plcmaxthreads)).into()
        }
        unsafe extern "system" fn GetCurrentThreads<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentthreads: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentThreads(::core::mem::transmute_copy(&plccurrentthreads)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxthreads: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxThreads(::core::mem::transmute_copy(&lcmaxthreads)).into()
        }
        unsafe extern "system" fn GetDeleteDelay<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsecdeletedelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeleteDelay(::core::mem::transmute_copy(&pmsecdeletedelay)).into()
        }
        unsafe extern "system" fn SetDeleteDelay<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msecdeletedelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDeleteDelay(::core::mem::transmute_copy(&msecdeletedelay)).into()
        }
        unsafe extern "system" fn GetMaxQueuedRequests<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetMaxQueuedRequests(::core::mem::transmute_copy(&plcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn GetCurrentQueuedRequests<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCurrentQueuedRequests(::core::mem::transmute_copy(&plccurrentqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMaxQueuedRequests<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxqueuedrequests: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxQueuedRequests(::core::mem::transmute_copy(&lcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMinThreads<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcminthreads: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMinThreads(::core::mem::transmute_copy(&lcminthreads)).into()
        }
        unsafe extern "system" fn SetQueueDepth<Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcqueuedepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetQueueDepth(::core::mem::transmute_copy(&lcqueuedepth)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetMaxThreads: GetMaxThreads::<Impl, IMPL_OFFSET>,
            GetCurrentThreads: GetCurrentThreads::<Impl, IMPL_OFFSET>,
            SetMaxThreads: SetMaxThreads::<Impl, IMPL_OFFSET>,
            GetDeleteDelay: GetDeleteDelay::<Impl, IMPL_OFFSET>,
            SetDeleteDelay: SetDeleteDelay::<Impl, IMPL_OFFSET>,
            GetMaxQueuedRequests: GetMaxQueuedRequests::<Impl, IMPL_OFFSET>,
            GetCurrentQueuedRequests: GetCurrentQueuedRequests::<Impl, IMPL_OFFSET>,
            SetMaxQueuedRequests: SetMaxQueuedRequests::<Impl, IMPL_OFFSET>,
            SetMinThreads: SetMinThreads::<Impl, IMPL_OFFSET>,
            SetQueueDepth: SetQueueDepth::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITransactionContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateInstance(&mut self, pszprogid: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITransactionContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&pszprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn Abort<Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionContext as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionContextEx_Impl: Sized {
    fn CreateInstance(&mut self, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Commit(&mut self) -> ::windows::core::Result<()>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
}
impl ITransactionContextEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionContextEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionContextEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstance(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        unsafe extern "system" fn Commit<Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit().into()
        }
        unsafe extern "system" fn Abort<Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionContextEx as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionProperty_Impl: Sized {
    fn Reserved1(&mut self);
    fn Reserved2(&mut self);
    fn Reserved3(&mut self);
    fn Reserved4(&mut self);
    fn Reserved5(&mut self);
    fn Reserved6(&mut self);
    fn Reserved7(&mut self);
    fn Reserved8(&mut self);
    fn Reserved9(&mut self);
    fn GetTransactionResourcePool(&mut self) -> ::windows::core::Result<ITransactionResourcePool>;
    fn Reserved10(&mut self);
    fn Reserved11(&mut self);
    fn Reserved12(&mut self);
    fn Reserved13(&mut self);
    fn Reserved14(&mut self);
    fn Reserved15(&mut self);
    fn Reserved16(&mut self);
    fn Reserved17(&mut self);
}
impl ITransactionProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionProperty_Vtbl {
        unsafe extern "system" fn Reserved1<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved1()
        }
        unsafe extern "system" fn Reserved2<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved2()
        }
        unsafe extern "system" fn Reserved3<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved3()
        }
        unsafe extern "system" fn Reserved4<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved4()
        }
        unsafe extern "system" fn Reserved5<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved5()
        }
        unsafe extern "system" fn Reserved6<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved6()
        }
        unsafe extern "system" fn Reserved7<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved7()
        }
        unsafe extern "system" fn Reserved8<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved8()
        }
        unsafe extern "system" fn Reserved9<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved9()
        }
        unsafe extern "system" fn GetTransactionResourcePool<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptxpool: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTransactionResourcePool() {
                ::core::result::Result::Ok(ok__) => {
                    *pptxpool = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved10<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved10()
        }
        unsafe extern "system" fn Reserved11<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved11()
        }
        unsafe extern "system" fn Reserved12<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved12()
        }
        unsafe extern "system" fn Reserved13<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved13()
        }
        unsafe extern "system" fn Reserved14<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved14()
        }
        unsafe extern "system" fn Reserved15<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved15()
        }
        unsafe extern "system" fn Reserved16<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved16()
        }
        unsafe extern "system" fn Reserved17<Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reserved17()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Reserved1: Reserved1::<Impl, IMPL_OFFSET>,
            Reserved2: Reserved2::<Impl, IMPL_OFFSET>,
            Reserved3: Reserved3::<Impl, IMPL_OFFSET>,
            Reserved4: Reserved4::<Impl, IMPL_OFFSET>,
            Reserved5: Reserved5::<Impl, IMPL_OFFSET>,
            Reserved6: Reserved6::<Impl, IMPL_OFFSET>,
            Reserved7: Reserved7::<Impl, IMPL_OFFSET>,
            Reserved8: Reserved8::<Impl, IMPL_OFFSET>,
            Reserved9: Reserved9::<Impl, IMPL_OFFSET>,
            GetTransactionResourcePool: GetTransactionResourcePool::<Impl, IMPL_OFFSET>,
            Reserved10: Reserved10::<Impl, IMPL_OFFSET>,
            Reserved11: Reserved11::<Impl, IMPL_OFFSET>,
            Reserved12: Reserved12::<Impl, IMPL_OFFSET>,
            Reserved13: Reserved13::<Impl, IMPL_OFFSET>,
            Reserved14: Reserved14::<Impl, IMPL_OFFSET>,
            Reserved15: Reserved15::<Impl, IMPL_OFFSET>,
            Reserved16: Reserved16::<Impl, IMPL_OFFSET>,
            Reserved17: Reserved17::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait ITransactionProxy_Impl: Sized {
    fn Commit(&mut self, guid: ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Abort(&mut self) -> ::windows::core::Result<()>;
    fn Promote(&mut self) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransaction>;
    fn CreateVoter(&mut self, ptxasync: ::core::option::Option<super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>;
    fn GetIsolationLevel(&mut self, __midl__itransactionproxy0000: *mut i32) -> ::windows::core::Result<()>;
    fn GetIdentifier(&mut self, pbstridentifier: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn IsReusable(&mut self, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ITransactionProxy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionProxy_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionProxy_Vtbl {
        unsafe extern "system" fn Commit<Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Commit(::core::mem::transmute_copy(&guid)).into()
        }
        unsafe extern "system" fn Abort<Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Abort().into()
        }
        unsafe extern "system" fn Promote<Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Promote() {
                ::core::result::Result::Ok(ok__) => {
                    *ptransaction = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVoter<Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxasync: ::windows::core::RawPtr, ppballot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateVoter(::core::mem::transmute(&ptxasync)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppballot = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsolationLevel<Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIsolationLevel(::core::mem::transmute_copy(&__midl__itransactionproxy0000)).into()
        }
        unsafe extern "system" fn GetIdentifier<Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstridentifier: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdentifier(::core::mem::transmute_copy(&pbstridentifier)).into()
        }
        unsafe extern "system" fn IsReusable<Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsReusable(::core::mem::transmute_copy(&pfisreusable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Commit: Commit::<Impl, IMPL_OFFSET>,
            Abort: Abort::<Impl, IMPL_OFFSET>,
            Promote: Promote::<Impl, IMPL_OFFSET>,
            CreateVoter: CreateVoter::<Impl, IMPL_OFFSET>,
            GetIsolationLevel: GetIsolationLevel::<Impl, IMPL_OFFSET>,
            GetIdentifier: GetIdentifier::<Impl, IMPL_OFFSET>,
            IsReusable: IsReusable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionProxy as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionResourcePool_Impl: Sized {
    fn PutResource(&mut self, ppool: ::core::option::Option<IObjPool>, punk: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetResource(&mut self, ppool: ::core::option::Option<IObjPool>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ITransactionResourcePool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionResourcePool_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionResourcePool_Vtbl {
        unsafe extern "system" fn PutResource<Impl: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: ::windows::core::RawPtr, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutResource(::core::mem::transmute(&ppool), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetResource<Impl: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: ::windows::core::RawPtr, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetResource(::core::mem::transmute(&ppool)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            PutResource: PutResource::<Impl, IMPL_OFFSET>,
            GetResource: GetResource::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionResourcePool as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionStatus_Impl: Sized {
    fn SetTransactionStatus(&mut self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetTransactionStatus(&mut self, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ITransactionStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITransactionStatus_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITransactionStatus_Vtbl {
        unsafe extern "system" fn SetTransactionStatus<Impl: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTransactionStatus(::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn GetTransactionStatus<Impl: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTransactionStatus(::core::mem::transmute_copy(&phrstatus)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetTransactionStatus: SetTransactionStatus::<Impl, IMPL_OFFSET>,
            GetTransactionStatus: GetTransactionStatus::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionStatus as ::windows::core::Interface>::IID
    }
}
pub trait ITxProxyHolder_Impl: Sized {
    fn GetIdentifier(&mut self, pguidltx: *mut ::windows::core::GUID);
}
impl ITxProxyHolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITxProxyHolder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITxProxyHolder_Vtbl {
        unsafe extern "system" fn GetIdentifier<Impl: ITxProxyHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidltx: *mut ::windows::core::GUID) {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetIdentifier(::core::mem::transmute_copy(&pguidltx))
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetIdentifier: GetIdentifier::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITxProxyHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ObjectContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateInstance(&mut self, bstrprogid: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetComplete(&mut self) -> ::windows::core::Result<()>;
    fn SetAbort(&mut self) -> ::windows::core::Result<()>;
    fn EnableCommit(&mut self) -> ::windows::core::Result<()>;
    fn DisableCommit(&mut self) -> ::windows::core::Result<()>;
    fn IsInTransaction(&mut self) -> ::windows::core::Result<i16>;
    fn IsSecurityEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn IsCallerInRole(&mut self, bstrrole: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Item(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Security(&mut self) -> ::windows::core::Result<SecurityProperty>;
    fn ContextInfo(&mut self) -> ::windows::core::Result<ContextInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ObjectContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ObjectContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ObjectContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&bstrprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pobject = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComplete<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetComplete().into()
        }
        unsafe extern "system" fn SetAbort<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAbort().into()
        }
        unsafe extern "system" fn EnableCommit<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableCommit().into()
        }
        unsafe extern "system" fn DisableCommit<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableCommit().into()
        }
        unsafe extern "system" fn IsInTransaction<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisintx = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSecurityEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCallerInRole(::core::mem::transmute_copy(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbinrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityproperty: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Security() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsecurityproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextInfo<Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontextinfo: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContextInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcontextinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            SetComplete: SetComplete::<Impl, IMPL_OFFSET>,
            SetAbort: SetAbort::<Impl, IMPL_OFFSET>,
            EnableCommit: EnableCommit::<Impl, IMPL_OFFSET>,
            DisableCommit: DisableCommit::<Impl, IMPL_OFFSET>,
            IsInTransaction: IsInTransaction::<Impl, IMPL_OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Impl, IMPL_OFFSET>,
            IsCallerInRole: IsCallerInRole::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Security: Security::<Impl, IMPL_OFFSET>,
            ContextInfo: ContextInfo::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ObjectContext as ::windows::core::Interface>::IID
    }
}
pub trait ObjectControl_Impl: Sized {
    fn Activate(&mut self) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self) -> ::windows::core::Result<()>;
    fn CanBePooled(&mut self, pbpoolable: *mut i16) -> ::windows::core::Result<()>;
}
impl ObjectControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ObjectControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ObjectControl_Vtbl {
        unsafe extern "system" fn Activate<Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate().into()
        }
        unsafe extern "system" fn Deactivate<Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate().into()
        }
        unsafe extern "system" fn CanBePooled<Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpoolable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CanBePooled(::core::mem::transmute_copy(&pbpoolable)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
            CanBePooled: CanBePooled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ObjectControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait SecurityProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetDirectCallerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDirectCreatorName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetOriginalCallerName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetOriginalCreatorName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl SecurityProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: SecurityProperty_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> SecurityProperty_Vtbl {
        unsafe extern "system" fn GetDirectCallerName<Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDirectCallerName() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirectCreatorName<Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDirectCreatorName() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCallerName<Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginalCallerName() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCreatorName<Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOriginalCreatorName() {
                ::core::result::Result::Ok(ok__) => {
                    *bstrusername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDirectCallerName: GetDirectCallerName::<Impl, IMPL_OFFSET>,
            GetDirectCreatorName: GetDirectCreatorName::<Impl, IMPL_OFFSET>,
            GetOriginalCallerName: GetOriginalCallerName::<Impl, IMPL_OFFSET>,
            GetOriginalCreatorName: GetOriginalCreatorName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<SecurityProperty as ::windows::core::Interface>::IID
    }
}
