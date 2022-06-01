#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ContextInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn IsInTransaction(&self) -> ::windows::core::Result<i16>;
    fn GetTransaction(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetTransactionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetActivityId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetContextId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ContextInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ContextInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>() -> ContextInfo_Vtbl {
        unsafe extern "system" fn IsInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisintx, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptx: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptx, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtxid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstractivityid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstractivityid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrctxid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContextId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrctxid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            GetTransaction: GetTransaction::<Identity, Impl, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, Impl, OFFSET>,
            GetActivityId: GetActivityId::<Identity, Impl, OFFSET>,
            GetContextId: GetContextId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ContextInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ContextInfo2_Impl: Sized + super::Com::IDispatch_Impl + ContextInfo_Impl {
    fn GetPartitionId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetApplicationId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetApplicationInstanceId(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ContextInfo2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ContextInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>() -> ContextInfo2_Vtbl {
        unsafe extern "system" fn GetPartitionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20000: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartitionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__contextinfo20000, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20001: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplicationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__contextinfo20001, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20002: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplicationInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__contextinfo20002, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ContextInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPartitionId: GetPartitionId::<Identity, Impl, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, Impl, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ContextInfo2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ContextInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAppDomainHelper_Impl: Sized + super::Com::IDispatch_Impl {
    fn Initialize(&self, punkad: &::core::option::Option<::windows::core::IUnknown>, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn DoCallback(&self, punkad: &::core::option::Option<::windows::core::IUnknown>, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAppDomainHelper {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAppDomainHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: isize>() -> IAppDomainHelper_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&punkad), ::core::mem::transmute_copy(&__midl__iappdomainhelper0000), ::core::mem::transmute_copy(&ppool)).into()
        }
        unsafe extern "system" fn DoCallback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoCallback(::core::mem::transmute(&punkad), ::core::mem::transmute_copy(&__midl__iappdomainhelper0001), ::core::mem::transmute_copy(&ppool)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            DoCallback: DoCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAppDomainHelper as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAssemblyLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetModules(&self, applicationdir: &super::super::Foundation::BSTR, applicationname: &super::super::Foundation::BSTR, assemblyname: &super::super::Foundation::BSTR) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAssemblyLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAssemblyLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyLocator_Impl, const OFFSET: isize>() -> IAssemblyLocator_Vtbl {
        unsafe extern "system" fn GetModules<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationdir: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, applicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, assemblyname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetModules(::core::mem::transmute(&applicationdir), ::core::mem::transmute(&applicationname), ::core::mem::transmute(&assemblyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmodules, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetModules: GetModules::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAssemblyLocator as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IAsyncErrorNotify_Impl: Sized {
    fn OnError(&self, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IAsyncErrorNotify {}
impl IAsyncErrorNotify_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncErrorNotify_Impl, const OFFSET: isize>() -> IAsyncErrorNotify_Vtbl {
        unsafe extern "system" fn OnError<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAsyncErrorNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnError(::core::mem::transmute_copy(&hr)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnError: OnError::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAsyncErrorNotify as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICOMAdminCatalog_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Connect(&self, bstrcatalogservername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn MajorVersion(&self) -> ::windows::core::Result<i32>;
    fn MinorVersion(&self) -> ::windows::core::Result<i32>;
    fn GetCollectionByQuery(&self, bstrcollname: &super::super::Foundation::BSTR, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<super::Com::IDispatch>;
    fn ImportComponent(&self, bstrapplidorname: &super::super::Foundation::BSTR, bstrclsidorprogid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn InstallComponent(&self, bstrapplidorname: &super::super::Foundation::BSTR, bstrdll: &super::super::Foundation::BSTR, bstrtlb: &super::super::Foundation::BSTR, bstrpsdll: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ShutdownApplication(&self, bstrapplidorname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ExportApplication(&self, bstrapplidorname: &super::super::Foundation::BSTR, bstrapplicationfile: &super::super::Foundation::BSTR, loptions: COMAdminApplicationExportOptions) -> ::windows::core::Result<()>;
    fn InstallApplication(&self, bstrapplicationfile: &super::super::Foundation::BSTR, bstrdestinationdirectory: &super::super::Foundation::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, bstrrsn: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn StopRouter(&self) -> ::windows::core::Result<()>;
    fn RefreshRouter(&self) -> ::windows::core::Result<()>;
    fn StartRouter(&self) -> ::windows::core::Result<()>;
    fn Reserved1(&self) -> ::windows::core::Result<()>;
    fn Reserved2(&self) -> ::windows::core::Result<()>;
    fn InstallMultipleComponents(&self, bstrapplidorname: &super::super::Foundation::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn GetMultipleComponentsInfo(&self, bstrapplidorname: &super::super::Foundation::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn RefreshComponents(&self) -> ::windows::core::Result<()>;
    fn BackupREGDB(&self, bstrbackupfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RestoreREGDB(&self, bstrbackupfilepath: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn QueryApplicationFile(&self, bstrapplicationfile: &super::super::Foundation::BSTR, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn StartApplication(&self, bstrapplidorname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ServiceCheck(&self, lservice: i32) -> ::windows::core::Result<i32>;
    fn InstallMultipleEventClasses(&self, bstrapplidorname: &super::super::Foundation::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn InstallEventClass(&self, bstrapplidorname: &super::super::Foundation::BSTR, bstrdll: &super::super::Foundation::BSTR, bstrtlb: &super::super::Foundation::BSTR, bstrpsdll: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetEventClassesForIID(&self, bstriid: &super::super::Foundation::BSTR, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICOMAdminCatalog {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICOMAdminCatalog_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>() -> ICOMAdminCatalog_Vtbl {
        unsafe extern "system" fn GetCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollection(::core::mem::transmute(&bstrcollname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcatalogservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Connect(::core::mem::transmute(&bstrcatalogservername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionByQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollectionByQuery(::core::mem::transmute(&bstrcollname), ::core::mem::transmute_copy(&ppsavarquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportComponent(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrclsidorprogid)).into()
        }
        unsafe extern "system" fn InstallComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallComponent(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrdll), ::core::mem::transmute(&bstrtlb), ::core::mem::transmute(&bstrpsdll)).into()
        }
        unsafe extern "system" fn ShutdownApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownApplication(::core::mem::transmute(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ExportApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportApplication(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrapplicationfile), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestinationdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallApplication(::core::mem::transmute(&bstrapplicationfile), ::core::mem::transmute(&bstrdestinationdirectory), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute(&bstruserid), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute(&bstrrsn)).into()
        }
        unsafe extern "system" fn StopRouter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopRouter().into()
        }
        unsafe extern "system" fn RefreshRouter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshRouter().into()
        }
        unsafe extern "system" fn StartRouter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartRouter().into()
        }
        unsafe extern "system" fn Reserved1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved1().into()
        }
        unsafe extern "system" fn Reserved2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved2().into()
        }
        unsafe extern "system" fn InstallMultipleComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallMultipleComponents(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMultipleComponentsInfo(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids), ::core::mem::transmute_copy(&ppsavarclassnames), ::core::mem::transmute_copy(&ppsavarfileflags), ::core::mem::transmute_copy(&ppsavarcomponentflags)).into()
        }
        unsafe extern "system" fn RefreshComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshComponents().into()
        }
        unsafe extern "system" fn BackupREGDB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackupREGDB(::core::mem::transmute(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn RestoreREGDB<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreREGDB(::core::mem::transmute(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn QueryApplicationFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrapplicationname: *mut super::super::Foundation::BSTR, pbstrapplicationdescription: *mut super::super::Foundation::BSTR, pbhasusers: *mut i16, pbisproxy: *mut i16, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryApplicationFile(::core::mem::transmute(&bstrapplicationfile), ::core::mem::transmute_copy(&pbstrapplicationname), ::core::mem::transmute_copy(&pbstrapplicationdescription), ::core::mem::transmute_copy(&pbhasusers), ::core::mem::transmute_copy(&pbisproxy), ::core::mem::transmute_copy(&ppsavarfilenames)).into()
        }
        unsafe extern "system" fn StartApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartApplication(::core::mem::transmute(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ServiceCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceCheck(::core::mem::transmute_copy(&lservice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallMultipleEventClasses(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn InstallEventClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrtlb: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpsdll: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallEventClass(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrdll), ::core::mem::transmute(&bstrtlb), ::core::mem::transmute(&bstrpsdll)).into()
        }
        unsafe extern "system" fn GetEventClassesForIID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstriid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEventClassesForIID(::core::mem::transmute(&bstriid), ::core::mem::transmute_copy(&ppsavarclsids), ::core::mem::transmute_copy(&ppsavarprogids), ::core::mem::transmute_copy(&ppsavardescriptions)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCollection: GetCollection::<Identity, Impl, OFFSET>,
            Connect: Connect::<Identity, Impl, OFFSET>,
            MajorVersion: MajorVersion::<Identity, Impl, OFFSET>,
            MinorVersion: MinorVersion::<Identity, Impl, OFFSET>,
            GetCollectionByQuery: GetCollectionByQuery::<Identity, Impl, OFFSET>,
            ImportComponent: ImportComponent::<Identity, Impl, OFFSET>,
            InstallComponent: InstallComponent::<Identity, Impl, OFFSET>,
            ShutdownApplication: ShutdownApplication::<Identity, Impl, OFFSET>,
            ExportApplication: ExportApplication::<Identity, Impl, OFFSET>,
            InstallApplication: InstallApplication::<Identity, Impl, OFFSET>,
            StopRouter: StopRouter::<Identity, Impl, OFFSET>,
            RefreshRouter: RefreshRouter::<Identity, Impl, OFFSET>,
            StartRouter: StartRouter::<Identity, Impl, OFFSET>,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            InstallMultipleComponents: InstallMultipleComponents::<Identity, Impl, OFFSET>,
            GetMultipleComponentsInfo: GetMultipleComponentsInfo::<Identity, Impl, OFFSET>,
            RefreshComponents: RefreshComponents::<Identity, Impl, OFFSET>,
            BackupREGDB: BackupREGDB::<Identity, Impl, OFFSET>,
            RestoreREGDB: RestoreREGDB::<Identity, Impl, OFFSET>,
            QueryApplicationFile: QueryApplicationFile::<Identity, Impl, OFFSET>,
            StartApplication: StartApplication::<Identity, Impl, OFFSET>,
            ServiceCheck: ServiceCheck::<Identity, Impl, OFFSET>,
            InstallMultipleEventClasses: InstallMultipleEventClasses::<Identity, Impl, OFFSET>,
            InstallEventClass: InstallEventClass::<Identity, Impl, OFFSET>,
            GetEventClassesForIID: GetEventClassesForIID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICOMAdminCatalog as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICOMAdminCatalog2_Impl: Sized + super::Com::IDispatch_Impl + ICOMAdminCatalog_Impl {
    fn GetCollectionByQuery2(&self, bstrcollectionname: &super::super::Foundation::BSTR, pvarquerystrings: *const super::Com::VARIANT) -> ::windows::core::Result<super::Com::IDispatch>;
    fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows::core::Result<()>;
    fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn DumpApplicationInstance(&self, bstrapplicationinstanceid: &super::super::Foundation::BSTR, bstrdirectory: &super::super::Foundation::BSTR, lmaximages: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IsApplicationInstanceDumpSupported(&self) -> ::windows::core::Result<i16>;
    fn CreateServiceForApplication(&self, bstrapplicationidorname: &super::super::Foundation::BSTR, bstrservicename: &super::super::Foundation::BSTR, bstrstarttype: &super::super::Foundation::BSTR, bstrerrorcontrol: &super::super::Foundation::BSTR, bstrdependencies: &super::super::Foundation::BSTR, bstrrunas: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, bdesktopok: i16) -> ::windows::core::Result<()>;
    fn DeleteServiceForApplication(&self, bstrapplicationidorname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPartitionID(&self, bstrapplicationidorname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPartitionName(&self, bstrapplicationidorname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCurrentPartition(&self, bstrpartitionidorname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CurrentPartitionID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CurrentPartitionName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GlobalPartitionID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FlushPartitionCache(&self) -> ::windows::core::Result<()>;
    fn CopyApplications(&self, bstrsourcepartitionidorname: &super::super::Foundation::BSTR, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CopyComponents(&self, bstrsourceapplicationidorname: &super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MoveComponents(&self, bstrsourceapplicationidorname: &super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AliasComponent(&self, bstrsrcapplicationidorname: &super::super::Foundation::BSTR, bstrclsidorprogid: &super::super::Foundation::BSTR, bstrdestapplicationidorname: &super::super::Foundation::BSTR, bstrnewprogid: &super::super::Foundation::BSTR, bstrnewclsid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsSafeToDelete(&self, bstrdllname: &super::super::Foundation::BSTR) -> ::windows::core::Result<COMAdminInUse>;
    fn ImportUnconfiguredComponents(&self, bstrapplicationidorname: &super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PromoteUnconfiguredComponents(&self, bstrapplicationidorname: &super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ImportComponents(&self, bstrapplicationidorname: &super::super::Foundation::BSTR, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Is64BitCatalogServer(&self) -> ::windows::core::Result<i16>;
    fn ExportPartition(&self, bstrpartitionidorname: &super::super::Foundation::BSTR, bstrpartitionfilename: &super::super::Foundation::BSTR, loptions: COMAdminApplicationExportOptions) -> ::windows::core::Result<()>;
    fn InstallPartition(&self, bstrfilename: &super::super::Foundation::BSTR, bstrdestdirectory: &super::super::Foundation::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &super::super::Foundation::BSTR, bstrpassword: &super::super::Foundation::BSTR, bstrrsn: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn QueryApplicationFile2(&self, bstrapplicationfile: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn GetComponentVersionCount(&self, bstrclsidorprogid: &super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICOMAdminCatalog2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICOMAdminCatalog2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>() -> ICOMAdminCatalog2_Vtbl {
        unsafe extern "system" fn GetCollectionByQuery2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollectionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarquerystrings: *const super::Com::VARIANT, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollectionByQuery2(::core::mem::transmute(&bstrcollectionname), ::core::mem::transmute_copy(&pvarquerystrings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplicationInstanceIDFromProcessID(::core::mem::transmute_copy(&lprocessid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationinstanceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn PauseApplicationInstances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PauseApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn ResumeApplicationInstances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResumeApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn RecycleApplicationInstances<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, lreasoncode: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecycleApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid), ::core::mem::transmute_copy(&lreasoncode)).into()
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Com::VARIANT, pvarboolpaused: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreApplicationInstancesPaused(::core::mem::transmute_copy(&pvarapplicationinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarboolpaused, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpApplicationInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationinstanceid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lmaximages: i32, pbstrdumpfile: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DumpApplicationInstance(::core::mem::transmute(&bstrapplicationinstanceid), ::core::mem::transmute(&bstrdirectory), ::core::mem::transmute_copy(&lmaximages)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdumpfile, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbooldumpsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsApplicationInstanceDumpSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbooldumpsupported, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServiceForApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrstarttype: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrerrorcontrol: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdependencies: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrunas: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bdesktopok: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateServiceForApplication(::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute(&bstrservicename), ::core::mem::transmute(&bstrstarttype), ::core::mem::transmute(&bstrerrorcontrol), ::core::mem::transmute(&bstrdependencies), ::core::mem::transmute(&bstrrunas), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&bdesktopok)).into()
        }
        unsafe extern "system" fn DeleteServiceForApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteServiceForApplication(::core::mem::transmute(&bstrapplicationidorname)).into()
        }
        unsafe extern "system" fn GetPartitionID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartitionID(::core::mem::transmute(&bstrapplicationidorname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartitionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartitionName(::core::mem::transmute(&bstrapplicationidorname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPartition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentPartition(::core::mem::transmute(&bstrpartitionidorname)).into()
        }
        unsafe extern "system" fn CurrentPartitionID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPartitionID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPartitionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPartitionName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalPartitionID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrglobalpartitionid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GlobalPartitionID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrglobalpartitionid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushPartitionCache<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushPartitionCache().into()
        }
        unsafe extern "system" fn CopyApplications<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourcepartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarapplicationid: *const super::Com::VARIANT, bstrdestinationpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyApplications(::core::mem::transmute(&bstrsourcepartitionidorname), ::core::mem::transmute_copy(&pvarapplicationid), ::core::mem::transmute(&bstrdestinationpartitionidorname)).into()
        }
        unsafe extern "system" fn CopyComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyComponents(::core::mem::transmute(&bstrsourceapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn MoveComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, bstrdestinationapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveComponents(::core::mem::transmute(&bstrsourceapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn AliasComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsrcapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrnewclsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AliasComponent(::core::mem::transmute(&bstrsrcapplicationidorname), ::core::mem::transmute(&bstrclsidorprogid), ::core::mem::transmute(&bstrdestapplicationidorname), ::core::mem::transmute(&bstrnewprogid), ::core::mem::transmute(&bstrnewclsid)).into()
        }
        unsafe extern "system" fn IsSafeToDelete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSafeToDelete(::core::mem::transmute(&bstrdllname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcomadmininuse, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportUnconfiguredComponents(::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PromoteUnconfiguredComponents(::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn ImportComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarclsidorprogid: *const super::Com::VARIANT, pvarcomponenttype: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportComponents(::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn Is64BitCatalogServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbis64bit: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Is64BitCatalogServer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbis64bit, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPartition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpartitionfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportPartition(::core::mem::transmute(&bstrpartitionidorname), ::core::mem::transmute(&bstrpartitionfilename), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallPartition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrdestdirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrsn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallPartition(::core::mem::transmute(&bstrfilename), ::core::mem::transmute(&bstrdestdirectory), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute(&bstruserid), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute(&bstrrsn)).into()
        }
        unsafe extern "system" fn QueryApplicationFile2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppfilesforimport: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryApplicationFile2(::core::mem::transmute(&bstrapplicationfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfilesforimport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentVersionCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, plversioncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetComponentVersionCount(::core::mem::transmute(&bstrclsidorprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plversioncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ICOMAdminCatalog_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetCollectionByQuery2: GetCollectionByQuery2::<Identity, Impl, OFFSET>,
            GetApplicationInstanceIDFromProcessID: GetApplicationInstanceIDFromProcessID::<Identity, Impl, OFFSET>,
            ShutdownApplicationInstances: ShutdownApplicationInstances::<Identity, Impl, OFFSET>,
            PauseApplicationInstances: PauseApplicationInstances::<Identity, Impl, OFFSET>,
            ResumeApplicationInstances: ResumeApplicationInstances::<Identity, Impl, OFFSET>,
            RecycleApplicationInstances: RecycleApplicationInstances::<Identity, Impl, OFFSET>,
            AreApplicationInstancesPaused: AreApplicationInstancesPaused::<Identity, Impl, OFFSET>,
            DumpApplicationInstance: DumpApplicationInstance::<Identity, Impl, OFFSET>,
            IsApplicationInstanceDumpSupported: IsApplicationInstanceDumpSupported::<Identity, Impl, OFFSET>,
            CreateServiceForApplication: CreateServiceForApplication::<Identity, Impl, OFFSET>,
            DeleteServiceForApplication: DeleteServiceForApplication::<Identity, Impl, OFFSET>,
            GetPartitionID: GetPartitionID::<Identity, Impl, OFFSET>,
            GetPartitionName: GetPartitionName::<Identity, Impl, OFFSET>,
            SetCurrentPartition: SetCurrentPartition::<Identity, Impl, OFFSET>,
            CurrentPartitionID: CurrentPartitionID::<Identity, Impl, OFFSET>,
            CurrentPartitionName: CurrentPartitionName::<Identity, Impl, OFFSET>,
            GlobalPartitionID: GlobalPartitionID::<Identity, Impl, OFFSET>,
            FlushPartitionCache: FlushPartitionCache::<Identity, Impl, OFFSET>,
            CopyApplications: CopyApplications::<Identity, Impl, OFFSET>,
            CopyComponents: CopyComponents::<Identity, Impl, OFFSET>,
            MoveComponents: MoveComponents::<Identity, Impl, OFFSET>,
            AliasComponent: AliasComponent::<Identity, Impl, OFFSET>,
            IsSafeToDelete: IsSafeToDelete::<Identity, Impl, OFFSET>,
            ImportUnconfiguredComponents: ImportUnconfiguredComponents::<Identity, Impl, OFFSET>,
            PromoteUnconfiguredComponents: PromoteUnconfiguredComponents::<Identity, Impl, OFFSET>,
            ImportComponents: ImportComponents::<Identity, Impl, OFFSET>,
            Is64BitCatalogServer: Is64BitCatalogServer::<Identity, Impl, OFFSET>,
            ExportPartition: ExportPartition::<Identity, Impl, OFFSET>,
            InstallPartition: InstallPartition::<Identity, Impl, OFFSET>,
            QueryApplicationFile2: QueryApplicationFile2::<Identity, Impl, OFFSET>,
            GetComponentVersionCount: GetComponentVersionCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICOMAdminCatalog2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ICOMAdminCatalog as ::windows::core::Interface>::IID
    }
}
pub trait ICOMLBArguments_Impl: Sized {
    fn GetCLSID(&self, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn SetCLSID(&self, pclsid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetMachineName(&self, cchsvr: u32, szservername: ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn SetMachineName(&self, cchsvr: u32, szservername: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICOMLBArguments {}
impl ICOMLBArguments_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>() -> ICOMLBArguments_Vtbl {
        unsafe extern "system" fn GetCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCLSID(::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLSID(::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn GetMachineName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMachineName(::core::mem::transmute_copy(&cchsvr), ::core::mem::transmute_copy(&szservername)).into()
        }
        unsafe extern "system" fn SetMachineName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMachineName(::core::mem::transmute_copy(&cchsvr), ::core::mem::transmute(&szservername)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCLSID: GetCLSID::<Identity, Impl, OFFSET>,
            SetCLSID: SetCLSID::<Identity, Impl, OFFSET>,
            GetMachineName: GetMachineName::<Identity, Impl, OFFSET>,
            SetMachineName: SetMachineName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICOMLBArguments as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICatalogCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Remove(&self, lindex: i32) -> ::windows::core::Result<()>;
    fn Add(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Populate(&self) -> ::windows::core::Result<()>;
    fn SaveChanges(&self) -> ::windows::core::Result<i32>;
    fn GetCollection(&self, bstrcollname: &super::super::Foundation::BSTR, varobjectkey: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Name(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn AddEnabled(&self) -> ::windows::core::Result<i16>;
    fn RemoveEnabled(&self) -> ::windows::core::Result<i16>;
    fn GetUtilInterface(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn DataStoreMajorVersion(&self) -> ::windows::core::Result<i32>;
    fn DataStoreMinorVersion(&self) -> ::windows::core::Result<i32>;
    fn PopulateByKey(&self, psakeys: *const super::Com::SAFEARRAY) -> ::windows::core::Result<()>;
    fn PopulateByQuery(&self, bstrquerystring: &super::super::Foundation::BSTR, lquerytype: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICatalogCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICatalogCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>() -> ICatalogCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plobjectcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plobjectcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Populate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Populate().into()
        }
        unsafe extern "system" fn SaveChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchanges: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveChanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchanges, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varobjectkey: ::core::mem::ManuallyDrop<super::Com::VARIANT>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollection(::core::mem::transmute(&bstrcollname), ::core::mem::transmute(&varobjectkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarnamel: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarnamel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbool, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoveEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbool, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUtilInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidispatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUtilInterface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidispatch, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMajorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataStoreMajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMinorVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversionl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataStoreMinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversionl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopulateByKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopulateByKey(::core::mem::transmute_copy(&psakeys)).into()
        }
        unsafe extern "system" fn PopulateByQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquerystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lquerytype: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopulateByQuery(::core::mem::transmute(&bstrquerystring), ::core::mem::transmute_copy(&lquerytype)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Populate: Populate::<Identity, Impl, OFFSET>,
            SaveChanges: SaveChanges::<Identity, Impl, OFFSET>,
            GetCollection: GetCollection::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            AddEnabled: AddEnabled::<Identity, Impl, OFFSET>,
            RemoveEnabled: RemoveEnabled::<Identity, Impl, OFFSET>,
            GetUtilInterface: GetUtilInterface::<Identity, Impl, OFFSET>,
            DataStoreMajorVersion: DataStoreMajorVersion::<Identity, Impl, OFFSET>,
            DataStoreMinorVersion: DataStoreMinorVersion::<Identity, Impl, OFFSET>,
            PopulateByKey: PopulateByKey::<Identity, Impl, OFFSET>,
            PopulateByQuery: PopulateByQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalogCollection as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICatalogObject_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Value(&self, bstrpropname: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn put_Value(&self, bstrpropname: &super::super::Foundation::BSTR, val: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Key(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Name(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn IsPropertyReadOnly(&self, bstrpropname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn Valid(&self) -> ::windows::core::Result<i16>;
    fn IsPropertyWriteOnly(&self, bstrpropname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICatalogObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICatalogObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>() -> ICatalogObject_Vtbl {
        unsafe extern "system" fn get_Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Value(::core::mem::transmute(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_Value(::core::mem::transmute(&bstrpropname), ::core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn Key<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Key() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyReadOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPropertyReadOnly(::core::mem::transmute(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Valid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbretval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPropertyWriteOnly(::core::mem::transmute(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbretval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Value: get_Value::<Identity, Impl, OFFSET>,
            put_Value: put_Value::<Identity, Impl, OFFSET>,
            Key: Key::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
            IsPropertyReadOnly: IsPropertyReadOnly::<Identity, Impl, OFFSET>,
            Valid: Valid::<Identity, Impl, OFFSET>,
            IsPropertyWriteOnly: IsPropertyWriteOnly::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICatalogObject as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait ICheckSxsConfig_Impl: Sized {
    fn IsSameSxsConfig(&self, wszsxsname: &::windows::core::PCWSTR, wszsxsdirectory: &::windows::core::PCWSTR, wszsxsappname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICheckSxsConfig {}
impl ICheckSxsConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICheckSxsConfig_Impl, const OFFSET: isize>() -> ICheckSxsConfig_Vtbl {
        unsafe extern "system" fn IsSameSxsConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICheckSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsxsname: ::windows::core::PCWSTR, wszsxsdirectory: ::windows::core::PCWSTR, wszsxsappname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSameSxsConfig(::core::mem::transmute(&wszsxsname), ::core::mem::transmute(&wszsxsdirectory), ::core::mem::transmute(&wszsxsappname)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IsSameSxsConfig: IsSameSxsConfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICheckSxsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IComActivityEvents_Impl: Sized {
    fn OnActivityCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnActivityDestroy(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnActivityEnter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32) -> ::windows::core::Result<()>;
    fn OnActivityTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::core::Result<()>;
    fn OnActivityReenter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::core::Result<()>;
    fn OnActivityLeave(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidleft: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnActivityLeaveSame(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwcalldepth: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComActivityEvents {}
impl IComActivityEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>() -> IComActivityEvents_Vtbl {
        unsafe extern "system" fn OnActivityCreate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityCreate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityDestroy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityDestroy(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityEnter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityEnter(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidentered), ::core::mem::transmute_copy(&dwthread)).into()
        }
        unsafe extern "system" fn OnActivityTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidentered: *const ::windows::core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityTimeout(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidentered), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnActivityReenter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityReenter(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwcalldepth)).into()
        }
        unsafe extern "system" fn OnActivityLeave<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, guidleft: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityLeave(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidleft)).into()
        }
        unsafe extern "system" fn OnActivityLeaveSame<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows::core::GUID, dwcalldepth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityLeaveSame(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&dwcalldepth)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnActivityCreate: OnActivityCreate::<Identity, Impl, OFFSET>,
            OnActivityDestroy: OnActivityDestroy::<Identity, Impl, OFFSET>,
            OnActivityEnter: OnActivityEnter::<Identity, Impl, OFFSET>,
            OnActivityTimeout: OnActivityTimeout::<Identity, Impl, OFFSET>,
            OnActivityReenter: OnActivityReenter::<Identity, Impl, OFFSET>,
            OnActivityLeave: OnActivityLeave::<Identity, Impl, OFFSET>,
            OnActivityLeaveSame: OnActivityLeaveSame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComActivityEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComApp2Events_Impl: Sized {
    fn OnAppActivation2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID, guidprocess: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppForceShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppPaused2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnAppRecycle2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID, guidprocess: &::windows::core::GUID, lreason: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IComApp2Events {}
#[cfg(feature = "Win32_Foundation")]
impl IComApp2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>() -> IComApp2Events_Vtbl {
        unsafe extern "system" fn OnAppActivation2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppActivation2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp), ::core::mem::transmute(&guidprocess)).into()
        }
        unsafe extern "system" fn OnAppShutdown2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppShutdown2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppForceShutdown2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppPaused2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppPaused2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp), ::core::mem::transmute_copy(&bpaused)).into()
        }
        unsafe extern "system" fn OnAppRecycle2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID, guidprocess: ::windows::core::GUID, lreason: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppRecycle2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp), ::core::mem::transmute(&guidprocess), ::core::mem::transmute_copy(&lreason)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnAppActivation2: OnAppActivation2::<Identity, Impl, OFFSET>,
            OnAppShutdown2: OnAppShutdown2::<Identity, Impl, OFFSET>,
            OnAppForceShutdown2: OnAppForceShutdown2::<Identity, Impl, OFFSET>,
            OnAppPaused2: OnAppPaused2::<Identity, Impl, OFFSET>,
            OnAppRecycle2: OnAppRecycle2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComApp2Events as ::windows::core::Interface>::IID
    }
}
pub trait IComAppEvents_Impl: Sized {
    fn OnAppActivation(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnAppForceShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComAppEvents {}
impl IComAppEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>() -> IComAppEvents_Vtbl {
        unsafe extern "system" fn OnAppActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppActivation(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppShutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppShutdown(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppForceShutdown(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnAppActivation: OnAppActivation::<Identity, Impl, OFFSET>,
            OnAppShutdown: OnAppShutdown::<Identity, Impl, OFFSET>,
            OnAppForceShutdown: OnAppForceShutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComAppEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComCRMEvents_Impl: Sized {
    fn OnCRMRecoveryStart(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMRecoveryDone(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMCheckpoint(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMBegin(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID, guidactivity: &::windows::core::GUID, guidtx: &::windows::core::GUID, szprogidcompensator: &::windows::core::PCWSTR, szdescription: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnCRMPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMIndoubt(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMDone(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMRelease(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMAnalyze(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::core::Result<()>;
    fn OnCRMWrite(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::Result<()>;
    fn OnCRMForget(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMForce(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnCRMDeliver(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IComCRMEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComCRMEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>() -> IComCRMEvents_Vtbl {
        unsafe extern "system" fn OnCRMRecoveryStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMRecoveryStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMRecoveryDone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMRecoveryDone(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMCheckpoint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMCheckpoint(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMBegin<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, guidactivity: ::windows::core::GUID, guidtx: ::windows::core::GUID, szprogidcompensator: ::windows::core::PCWSTR, szdescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMBegin(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute(&guidactivity), ::core::mem::transmute(&guidtx), ::core::mem::transmute(&szprogidcompensator), ::core::mem::transmute(&szdescription)).into()
        }
        unsafe extern "system" fn OnCRMPrepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMPrepare(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMIndoubt<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMIndoubt(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMDone(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMRelease<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMRelease(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAnalyze<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMAnalyze(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute_copy(&dwcrmrecordtype), ::core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMWrite<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMWrite(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute_copy(&fvariants), ::core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMForget<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMForget(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMForce<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMForce(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDeliver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows::core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMDeliver(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute_copy(&fvariants), ::core::mem::transmute_copy(&dwrecordsize)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnCRMRecoveryStart: OnCRMRecoveryStart::<Identity, Impl, OFFSET>,
            OnCRMRecoveryDone: OnCRMRecoveryDone::<Identity, Impl, OFFSET>,
            OnCRMCheckpoint: OnCRMCheckpoint::<Identity, Impl, OFFSET>,
            OnCRMBegin: OnCRMBegin::<Identity, Impl, OFFSET>,
            OnCRMPrepare: OnCRMPrepare::<Identity, Impl, OFFSET>,
            OnCRMCommit: OnCRMCommit::<Identity, Impl, OFFSET>,
            OnCRMAbort: OnCRMAbort::<Identity, Impl, OFFSET>,
            OnCRMIndoubt: OnCRMIndoubt::<Identity, Impl, OFFSET>,
            OnCRMDone: OnCRMDone::<Identity, Impl, OFFSET>,
            OnCRMRelease: OnCRMRelease::<Identity, Impl, OFFSET>,
            OnCRMAnalyze: OnCRMAnalyze::<Identity, Impl, OFFSET>,
            OnCRMWrite: OnCRMWrite::<Identity, Impl, OFFSET>,
            OnCRMForget: OnCRMForget::<Identity, Impl, OFFSET>,
            OnCRMForce: OnCRMForce::<Identity, Impl, OFFSET>,
            OnCRMDeliver: OnCRMDeliver::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComCRMEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComExceptionEvents_Impl: Sized {
    fn OnExceptionUser(&self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComExceptionEvents {}
impl IComExceptionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComExceptionEvents_Impl, const OFFSET: isize>() -> IComExceptionEvents_Vtbl {
        unsafe extern "system" fn OnExceptionUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComExceptionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnExceptionUser(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&code), ::core::mem::transmute_copy(&address), ::core::mem::transmute(&pszstacktrace)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnExceptionUser: OnExceptionUser::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComExceptionEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComIdentityEvents_Impl: Sized {
    fn OnIISRequestInfo(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: &::windows::core::PCWSTR, pszserverip: &::windows::core::PCWSTR, pszurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComIdentityEvents {}
impl IComIdentityEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComIdentityEvents_Impl, const OFFSET: isize>() -> IComIdentityEvents_Vtbl {
        unsafe extern "system" fn OnIISRequestInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComIdentityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: ::windows::core::PCWSTR, pszserverip: ::windows::core::PCWSTR, pszurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIISRequestInfo(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute(&pszclientip), ::core::mem::transmute(&pszserverip), ::core::mem::transmute(&pszurl)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnIISRequestInfo: OnIISRequestInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComIdentityEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComInstance2Events_Impl: Sized {
    fn OnObjectCreate2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnObjectDestroy2(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComInstance2Events {}
impl IComInstance2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: isize>() -> IComInstance2Events_Vtbl {
        unsafe extern "system" fn OnObjectCreate2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectCreate2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjectDestroy2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectDestroy2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnObjectCreate2: OnObjectCreate2::<Identity, Impl, OFFSET>,
            OnObjectDestroy2: OnObjectDestroy2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComInstance2Events as ::windows::core::Interface>::IID
    }
}
pub trait IComInstanceEvents_Impl: Sized {
    fn OnObjectCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64) -> ::windows::core::Result<()>;
    fn OnObjectDestroy(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComInstanceEvents {}
impl IComInstanceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: isize>() -> IComInstanceEvents_Vtbl {
        unsafe extern "system" fn OnObjectCreate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, clsid: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectCreate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDestroy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectDestroy(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnObjectCreate: OnObjectCreate::<Identity, Impl, OFFSET>,
            OnObjectDestroy: OnObjectDestroy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComInstanceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComLTxEvents_Impl: Sized {
    fn OnLtxTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows::core::GUID, tsid: &::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::Result<()>;
    fn OnLtxTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows::core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnLtxTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnLtxTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnLtxTransactionPromote(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows::core::GUID, txnid: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IComLTxEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComLTxEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>() -> IComLTxEvents_Vtbl {
        unsafe extern "system" fn OnLtxTransactionStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, tsid: ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx), ::core::mem::transmute(&tsid), ::core::mem::transmute_copy(&froot), ::core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPrepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionPrepare(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx), ::core::mem::transmute_copy(&fvote)).into()
        }
        unsafe extern "system" fn OnLtxTransactionAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPromote<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows::core::GUID, txnid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionPromote(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx), ::core::mem::transmute(&txnid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnLtxTransactionStart: OnLtxTransactionStart::<Identity, Impl, OFFSET>,
            OnLtxTransactionPrepare: OnLtxTransactionPrepare::<Identity, Impl, OFFSET>,
            OnLtxTransactionAbort: OnLtxTransactionAbort::<Identity, Impl, OFFSET>,
            OnLtxTransactionCommit: OnLtxTransactionCommit::<Identity, Impl, OFFSET>,
            OnLtxTransactionPromote: OnLtxTransactionPromote::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComLTxEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComMethod2Events_Impl: Sized {
    fn OnMethodCall2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::Result<()>;
    fn OnMethodReturn2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnMethodException2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComMethod2Events {}
impl IComMethod2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>() -> IComMethod2Events_Vtbl {
        unsafe extern "system" fn OnMethodCall2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodCall2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodReturn2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, dwthread: u32, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodException2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnMethodCall2: OnMethodCall2::<Identity, Impl, OFFSET>,
            OnMethodReturn2: OnMethodReturn2::<Identity, Impl, OFFSET>,
            OnMethodException2: OnMethodException2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComMethod2Events as ::windows::core::Interface>::IID
    }
}
pub trait IComMethodEvents_Impl: Sized {
    fn OnMethodCall(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::Result<()>;
    fn OnMethodReturn(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnMethodException(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComMethodEvents {}
impl IComMethodEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>() -> IComMethodEvents_Vtbl {
        unsafe extern "system" fn OnMethodCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodCall(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32, hresult: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodReturn(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows::core::GUID, guidrid: *const ::windows::core::GUID, imeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodException(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnMethodCall: OnMethodCall::<Identity, Impl, OFFSET>,
            OnMethodReturn: OnMethodReturn::<Identity, Impl, OFFSET>,
            OnMethodException: OnMethodException::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComMethodEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComMtaThreadPoolKnobs_Impl: Sized {
    fn MTASetMaxThreadCount(&self, dwmaxthreads: u32) -> ::windows::core::Result<()>;
    fn MTAGetMaxThreadCount(&self) -> ::windows::core::Result<u32>;
    fn MTASetThrottleValue(&self, dwthrottle: u32) -> ::windows::core::Result<()>;
    fn MTAGetThrottleValue(&self) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IComMtaThreadPoolKnobs {}
impl IComMtaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>() -> IComMtaThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn MTASetMaxThreadCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MTASetMaxThreadCount(::core::mem::transmute_copy(&dwmaxthreads)).into()
        }
        unsafe extern "system" fn MTAGetMaxThreadCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MTAGetMaxThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxthreads, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MTASetThrottleValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthrottle: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MTASetThrottleValue(::core::mem::transmute_copy(&dwthrottle)).into()
        }
        unsafe extern "system" fn MTAGetThrottleValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthrottle: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MTAGetThrottleValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwthrottle, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            MTASetMaxThreadCount: MTASetMaxThreadCount::<Identity, Impl, OFFSET>,
            MTAGetMaxThreadCount: MTAGetMaxThreadCount::<Identity, Impl, OFFSET>,
            MTASetThrottleValue: MTASetThrottleValue::<Identity, Impl, OFFSET>,
            MTAGetThrottleValue: MTAGetThrottleValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComMtaThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
pub trait IComObjectConstruction2Events_Impl: Sized {
    fn OnObjectConstruct2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: &::windows::core::PCWSTR, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComObjectConstruction2Events {}
impl IComObjectConstruction2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstruction2Events_Impl, const OFFSET: isize>() -> IComObjectConstruction2Events_Vtbl {
        unsafe extern "system" fn OnObjectConstruct2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstruction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: ::windows::core::PCWSTR, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectConstruct2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute(&sconstructstring), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnObjectConstruct2: OnObjectConstruct2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectConstruction2Events as ::windows::core::Interface>::IID
    }
}
pub trait IComObjectConstructionEvents_Impl: Sized {
    fn OnObjectConstruct(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: &::windows::core::PCWSTR, oid: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComObjectConstructionEvents {}
impl IComObjectConstructionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstructionEvents_Impl, const OFFSET: isize>() -> IComObjectConstructionEvents_Vtbl {
        unsafe extern "system" fn OnObjectConstruct<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstructionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, sconstructstring: ::windows::core::PCWSTR, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectConstruct(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute(&sconstructstring), ::core::mem::transmute_copy(&oid)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnObjectConstruct: OnObjectConstruct::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectConstructionEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComObjectEvents_Impl: Sized {
    fn OnObjectActivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::Result<()>;
    fn OnObjectDeactivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::Result<()>;
    fn OnDisableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
    fn OnEnableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
    fn OnSetComplete(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
    fn OnSetAbort(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComObjectEvents {}
impl IComObjectEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>() -> IComObjectEvents_Vtbl {
        unsafe extern "system" fn OnObjectActivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectActivate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDeactivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectDeactivate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnDisableCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisableCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnEnableCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEnableCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetComplete(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnObjectActivate: OnObjectActivate::<Identity, Impl, OFFSET>,
            OnObjectDeactivate: OnObjectDeactivate::<Identity, Impl, OFFSET>,
            OnDisableCommit: OnDisableCommit::<Identity, Impl, OFFSET>,
            OnEnableCommit: OnEnableCommit::<Identity, Impl, OFFSET>,
            OnSetComplete: OnSetComplete::<Identity, Impl, OFFSET>,
            OnSetAbort: OnSetAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComObjectPool2Events_Impl: Sized {
    fn OnObjPoolPutObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolGetObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnObjPoolRecycleToTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolGetFromTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComObjectPool2Events {}
impl IComObjectPool2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>() -> IComObjectPool2Events_Vtbl {
        unsafe extern "system" fn OnObjPoolPutObject2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolPutObject2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&nreason), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolGetObject2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolRecycleToTx2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64, guidpartition: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolGetFromTx2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject2: OnObjPoolPutObject2::<Identity, Impl, OFFSET>,
            OnObjPoolGetObject2: OnObjPoolGetObject2::<Identity, Impl, OFFSET>,
            OnObjPoolRecycleToTx2: OnObjPoolRecycleToTx2::<Identity, Impl, OFFSET>,
            OnObjPoolGetFromTx2: OnObjPoolGetFromTx2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectPool2Events as ::windows::core::Interface>::IID
    }
}
pub trait IComObjectPoolEvents_Impl: Sized {
    fn OnObjPoolPutObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolGetObject(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolRecycleToTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolGetFromTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComObjectPoolEvents {}
impl IComObjectPoolEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>() -> IComObjectPoolEvents_Vtbl {
        unsafe extern "system" fn OnObjPoolPutObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolPutObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&nreason), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, dwavailable: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolGetObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolRecycleToTx(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, guidobject: *const ::windows::core::GUID, guidtx: *const ::windows::core::GUID, objid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolGetFromTx(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject: OnObjPoolPutObject::<Identity, Impl, OFFSET>,
            OnObjPoolGetObject: OnObjPoolGetObject::<Identity, Impl, OFFSET>,
            OnObjPoolRecycleToTx: OnObjPoolRecycleToTx::<Identity, Impl, OFFSET>,
            OnObjPoolGetFromTx: OnObjPoolGetFromTx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectPoolEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComObjectPoolEvents2_Impl: Sized {
    fn OnObjPoolCreateObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolDestroyObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::Result<()>;
    fn OnObjPoolCreateDecision(&self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::core::Result<()>;
    fn OnObjPoolTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, guidactivity: *const ::windows::core::GUID, dwtimeout: u32) -> ::windows::core::Result<()>;
    fn OnObjPoolCreatePool(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComObjectPoolEvents2 {}
impl IComObjectPoolEvents2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>() -> IComObjectPoolEvents2_Vtbl {
        unsafe extern "system" fn OnObjPoolCreateObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolCreateObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwobjscreated), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolDestroyObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwobjscreated: u32, oid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolDestroyObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwobjscreated), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolCreateDecision<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolCreateDecision(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&dwthreadswaiting), ::core::mem::transmute_copy(&dwavail), ::core::mem::transmute_copy(&dwcreated), ::core::mem::transmute_copy(&dwmin), ::core::mem::transmute_copy(&dwmax)).into()
        }
        unsafe extern "system" fn OnObjPoolTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, guidactivity: *const ::windows::core::GUID, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolTimeout(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnObjPoolCreatePool<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows::core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolCreatePool(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwmin), ::core::mem::transmute_copy(&dwmax), ::core::mem::transmute_copy(&dwtimeout)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnObjPoolCreateObject: OnObjPoolCreateObject::<Identity, Impl, OFFSET>,
            OnObjPoolDestroyObject: OnObjPoolDestroyObject::<Identity, Impl, OFFSET>,
            OnObjPoolCreateDecision: OnObjPoolCreateDecision::<Identity, Impl, OFFSET>,
            OnObjPoolTimeout: OnObjPoolTimeout::<Identity, Impl, OFFSET>,
            OnObjPoolCreatePool: OnObjPoolCreatePool::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComObjectPoolEvents2 as ::windows::core::Interface>::IID
    }
}
pub trait IComQCEvents_Impl: Sized {
    fn OnQCRecord(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: &::windows::core::PCWSTR, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, msmqhr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnQCQueueOpen(&self, pinfo: *const COMSVCSEVENTINFO, szqueue: &::windows::core::PCWSTR, queueid: u64, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnQCReceive(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnQCReceiveFail(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn OnQCMoveToReTryQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, retryindex: u32) -> ::windows::core::Result<()>;
    fn OnQCMoveToDeadQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnQCPlayback(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComQCEvents {}
impl IComQCEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>() -> IComQCEvents_Vtbl {
        unsafe extern "system" fn OnQCRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: ::windows::core::PCWSTR, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCRecord(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute(&szqueue), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCQueueOpen<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: ::windows::core::PCWSTR, queueid: u64, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCQueueOpen(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&szqueue), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceive<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCReceive(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceiveFail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCReceiveFail(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCMoveToReTryQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, retryindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCMoveToReTryQueue(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&retryindex)).into()
        }
        unsafe extern "system" fn OnQCMoveToDeadQueue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCMoveToDeadQueue(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid)).into()
        }
        unsafe extern "system" fn OnQCPlayback<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows::core::GUID, guidworkflowid: *const ::windows::core::GUID, hr: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCPlayback(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnQCRecord: OnQCRecord::<Identity, Impl, OFFSET>,
            OnQCQueueOpen: OnQCQueueOpen::<Identity, Impl, OFFSET>,
            OnQCReceive: OnQCReceive::<Identity, Impl, OFFSET>,
            OnQCReceiveFail: OnQCReceiveFail::<Identity, Impl, OFFSET>,
            OnQCMoveToReTryQueue: OnQCMoveToReTryQueue::<Identity, Impl, OFFSET>,
            OnQCMoveToDeadQueue: OnQCMoveToDeadQueue::<Identity, Impl, OFFSET>,
            OnQCPlayback: OnQCPlayback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComQCEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComResourceEvents_Impl: Sized {
    fn OnResourceCreate(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnResourceAllocate(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows::core::Result<()>;
    fn OnResourceRecycle(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows::core::PCWSTR, resid: u64) -> ::windows::core::Result<()>;
    fn OnResourceDestroy(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::core::HRESULT, psztype: &::windows::core::PCWSTR, resid: u64) -> ::windows::core::Result<()>;
    fn OnResourceTrack(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IComResourceEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComResourceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>() -> IComResourceEvents_Vtbl {
        unsafe extern "system" fn OnResourceCreate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceCreate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted)).into()
        }
        unsafe extern "system" fn OnResourceAllocate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceAllocate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted), ::core::mem::transmute_copy(&numrated), ::core::mem::transmute_copy(&rating)).into()
        }
        unsafe extern "system" fn OnResourceRecycle<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows::core::PCWSTR, resid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceRecycle(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceDestroy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows::core::HRESULT, psztype: ::windows::core::PCWSTR, resid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceDestroy(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&hr), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceTrack<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows::core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceTrack(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnResourceCreate: OnResourceCreate::<Identity, Impl, OFFSET>,
            OnResourceAllocate: OnResourceAllocate::<Identity, Impl, OFFSET>,
            OnResourceRecycle: OnResourceRecycle::<Identity, Impl, OFFSET>,
            OnResourceDestroy: OnResourceDestroy::<Identity, Impl, OFFSET>,
            OnResourceTrack: OnResourceTrack::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComResourceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComSecurityEvents_Impl: Sized {
    fn OnAuthenticate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnAuthenticateFail(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IComSecurityEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComSecurityEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: isize>() -> IComSecurityEvents_Vtbl {
        unsafe extern "system" fn OnAuthenticate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAuthenticate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidiid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&cbbyteorig), ::core::mem::transmute_copy(&psidoriginaluser), ::core::mem::transmute_copy(&cbbytecur), ::core::mem::transmute_copy(&psidcurrentuser), ::core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        unsafe extern "system" fn OnAuthenticateFail<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, objectid: u64, guidiid: *const ::windows::core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAuthenticateFail(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidiid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&cbbyteorig), ::core::mem::transmute_copy(&psidoriginaluser), ::core::mem::transmute_copy(&cbbytecur), ::core::mem::transmute_copy(&psidcurrentuser), ::core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnAuthenticate: OnAuthenticate::<Identity, Impl, OFFSET>,
            OnAuthenticateFail: OnAuthenticateFail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComSecurityEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComStaThreadPoolKnobs_Impl: Sized {
    fn SetMinThreadCount(&self, minthreads: u32) -> ::windows::core::Result<()>;
    fn GetMinThreadCount(&self) -> ::windows::core::Result<u32>;
    fn SetMaxThreadCount(&self, maxthreads: u32) -> ::windows::core::Result<()>;
    fn GetMaxThreadCount(&self) -> ::windows::core::Result<u32>;
    fn SetActivityPerThread(&self, activitiesperthread: u32) -> ::windows::core::Result<()>;
    fn GetActivityPerThread(&self) -> ::windows::core::Result<u32>;
    fn SetActivityRatio(&self, activityratio: f64) -> ::windows::core::Result<()>;
    fn GetActivityRatio(&self) -> ::windows::core::Result<f64>;
    fn GetThreadCount(&self) -> ::windows::core::Result<u32>;
    fn GetQueueDepth(&self) -> ::windows::core::Result<u32>;
    fn SetQueueDepth(&self, dwqdepth: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComStaThreadPoolKnobs {}
impl IComStaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>() -> IComStaThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn SetMinThreadCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinThreadCount(::core::mem::transmute_copy(&minthreads)).into()
        }
        unsafe extern "system" fn GetMinThreadCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minthreads, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxThreadCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxThreadCount(::core::mem::transmute_copy(&maxthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreadCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxthreads, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityPerThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActivityPerThread(::core::mem::transmute_copy(&activitiesperthread)).into()
        }
        unsafe extern "system" fn GetActivityPerThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivityPerThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activitiesperthread, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityRatio<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActivityRatio(::core::mem::transmute_copy(&activityratio)).into()
        }
        unsafe extern "system" fn GetActivityRatio<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivityRatio() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activityratio, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreads: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwthreads, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueueDepth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwqdepth: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQueueDepth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwqdepth, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueueDepth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqdepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueueDepth(::core::mem::transmute_copy(&dwqdepth)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetMinThreadCount: SetMinThreadCount::<Identity, Impl, OFFSET>,
            GetMinThreadCount: GetMinThreadCount::<Identity, Impl, OFFSET>,
            SetMaxThreadCount: SetMaxThreadCount::<Identity, Impl, OFFSET>,
            GetMaxThreadCount: GetMaxThreadCount::<Identity, Impl, OFFSET>,
            SetActivityPerThread: SetActivityPerThread::<Identity, Impl, OFFSET>,
            GetActivityPerThread: GetActivityPerThread::<Identity, Impl, OFFSET>,
            SetActivityRatio: SetActivityRatio::<Identity, Impl, OFFSET>,
            GetActivityRatio: GetActivityRatio::<Identity, Impl, OFFSET>,
            GetThreadCount: GetThreadCount::<Identity, Impl, OFFSET>,
            GetQueueDepth: GetQueueDepth::<Identity, Impl, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComStaThreadPoolKnobs2_Impl: Sized + IComStaThreadPoolKnobs_Impl {
    fn GetMaxCPULoad(&self) -> ::windows::core::Result<u32>;
    fn SetMaxCPULoad(&self, pdwload: i32) -> ::windows::core::Result<()>;
    fn GetCPUMetricEnabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetCPUMetricEnabled(&self, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetCreateThreadsAggressively(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetCreateThreadsAggressively(&self, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetMaxCSR(&self) -> ::windows::core::Result<u32>;
    fn SetMaxCSR(&self, dwcsr: i32) -> ::windows::core::Result<()>;
    fn GetWaitTimeForThreadCleanup(&self) -> ::windows::core::Result<u32>;
    fn SetWaitTimeForThreadCleanup(&self, dwthreadcleanupwaittime: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IComStaThreadPoolKnobs2 {}
#[cfg(feature = "Win32_Foundation")]
impl IComStaThreadPoolKnobs2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>() -> IComStaThreadPoolKnobs2_Vtbl {
        unsafe extern "system" fn GetMaxCPULoad<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxCPULoad() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwload, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCPULoad<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxCPULoad(::core::mem::transmute_copy(&pdwload)).into()
        }
        unsafe extern "system" fn GetCPUMetricEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCPUMetricEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmetricenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCPUMetricEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCPUMetricEnabled(::core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetCreateThreadsAggressively<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCreateThreadsAggressively() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmetricenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateThreadsAggressively<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCreateThreadsAggressively(::core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetMaxCSR<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcsr: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxCSR() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcsr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCSR<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcsr: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxCSR(::core::mem::transmute_copy(&dwcsr)).into()
        }
        unsafe extern "system" fn GetWaitTimeForThreadCleanup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWaitTimeForThreadCleanup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwthreadcleanupwaittime, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaitTimeForThreadCleanup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadcleanupwaittime: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWaitTimeForThreadCleanup(::core::mem::transmute_copy(&dwthreadcleanupwaittime)).into()
        }
        Self {
            base__: IComStaThreadPoolKnobs_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetMaxCPULoad: GetMaxCPULoad::<Identity, Impl, OFFSET>,
            SetMaxCPULoad: SetMaxCPULoad::<Identity, Impl, OFFSET>,
            GetCPUMetricEnabled: GetCPUMetricEnabled::<Identity, Impl, OFFSET>,
            SetCPUMetricEnabled: SetCPUMetricEnabled::<Identity, Impl, OFFSET>,
            GetCreateThreadsAggressively: GetCreateThreadsAggressively::<Identity, Impl, OFFSET>,
            SetCreateThreadsAggressively: SetCreateThreadsAggressively::<Identity, Impl, OFFSET>,
            GetMaxCSR: GetMaxCSR::<Identity, Impl, OFFSET>,
            SetMaxCSR: SetMaxCSR::<Identity, Impl, OFFSET>,
            GetWaitTimeForThreadCleanup: GetWaitTimeForThreadCleanup::<Identity, Impl, OFFSET>,
            SetWaitTimeForThreadCleanup: SetWaitTimeForThreadCleanup::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs2 as ::windows::core::Interface>::IID || iid == &<IComStaThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
pub trait IComThreadEvents_Impl: Sized {
    fn OnThreadStart(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::Result<()>;
    fn OnThreadTerminate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::Result<()>;
    fn OnThreadBindToApartment(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::core::Result<()>;
    fn OnThreadUnBind(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::core::Result<()>;
    fn OnThreadWorkEnque(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::Result<()>;
    fn OnThreadWorkPrivate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::core::Result<()>;
    fn OnThreadWorkPublic(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::Result<()>;
    fn OnThreadWorkRedirect(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::core::Result<()>;
    fn OnThreadWorkReject(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::Result<()>;
    fn OnThreadAssignApartment(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, aptid: u64) -> ::windows::core::Result<()>;
    fn OnThreadUnassignApartment(&self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComThreadEvents {}
impl IComThreadEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>() -> IComThreadEvents_Vtbl {
        unsafe extern "system" fn OnThreadStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadTerminate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadBindToApartment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadBindToApartment(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&aptid), ::core::mem::transmute_copy(&dwactcnt), ::core::mem::transmute_copy(&dwlowcnt)).into()
        }
        unsafe extern "system" fn OnThreadUnBind<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadUnBind(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&aptid), ::core::mem::transmute_copy(&dwactcnt)).into()
        }
        unsafe extern "system" fn OnThreadWorkEnque<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkEnque(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkPrivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkPrivate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid)).into()
        }
        unsafe extern "system" fn OnThreadWorkPublic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkPublic(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkRedirect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkRedirect(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen), ::core::mem::transmute_copy(&threadnum)).into()
        }
        unsafe extern "system" fn OnThreadWorkReject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkReject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadAssignApartment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows::core::GUID, aptid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadAssignApartment(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&aptid)).into()
        }
        unsafe extern "system" fn OnThreadUnassignApartment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadUnassignApartment(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&aptid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnThreadStart: OnThreadStart::<Identity, Impl, OFFSET>,
            OnThreadTerminate: OnThreadTerminate::<Identity, Impl, OFFSET>,
            OnThreadBindToApartment: OnThreadBindToApartment::<Identity, Impl, OFFSET>,
            OnThreadUnBind: OnThreadUnBind::<Identity, Impl, OFFSET>,
            OnThreadWorkEnque: OnThreadWorkEnque::<Identity, Impl, OFFSET>,
            OnThreadWorkPrivate: OnThreadWorkPrivate::<Identity, Impl, OFFSET>,
            OnThreadWorkPublic: OnThreadWorkPublic::<Identity, Impl, OFFSET>,
            OnThreadWorkRedirect: OnThreadWorkRedirect::<Identity, Impl, OFFSET>,
            OnThreadWorkReject: OnThreadWorkReject::<Identity, Impl, OFFSET>,
            OnThreadAssignApartment: OnThreadAssignApartment::<Identity, Impl, OFFSET>,
            OnThreadUnassignApartment: OnThreadUnassignApartment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComThreadEvents as ::windows::core::Interface>::IID
    }
}
pub trait IComTrackingInfoCollection_Impl: Sized {
    fn Type(&self) -> ::windows::core::Result<TRACKING_COLL_TYPE>;
    fn Count(&self) -> ::windows::core::Result<u32>;
    fn Item(&self, ulindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComTrackingInfoCollection {}
impl IComTrackingInfoCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>() -> IComTrackingInfoCollection_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Item(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoCollection as ::windows::core::Interface>::IID
    }
}
pub trait IComTrackingInfoEvents_Impl: Sized {
    fn OnNewTrackingInfo(&self, ptoplevelcollection: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComTrackingInfoEvents {}
impl IComTrackingInfoEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoEvents_Impl, const OFFSET: isize>() -> IComTrackingInfoEvents_Vtbl {
        unsafe extern "system" fn OnNewTrackingInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoplevelcollection: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNewTrackingInfo(::core::mem::transmute(&ptoplevelcollection)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnNewTrackingInfo: OnNewTrackingInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComTrackingInfoObject_Impl: Sized {
    fn GetValue(&self, szpropertyname: &::windows::core::PCWSTR) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IComTrackingInfoObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComTrackingInfoObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoObject_Impl, const OFFSET: isize>() -> IComTrackingInfoObject_Vtbl {
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szpropertyname: ::windows::core::PCWSTR, pvarout: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::core::mem::transmute(&szpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarout, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetValue: GetValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoObject as ::windows::core::Interface>::IID
    }
}
pub trait IComTrackingInfoProperties_Impl: Sized {
    fn PropCount(&self) -> ::windows::core::Result<u32>;
    fn GetPropName(&self, ulindex: u32) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for IComTrackingInfoProperties {}
impl IComTrackingInfoProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>() -> IComTrackingInfoProperties_Vtbl {
        unsafe extern "system" fn PropCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppszpropname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropName(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpropname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PropCount: PropCount::<Identity, Impl, OFFSET>,
            GetPropName: GetPropName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTrackingInfoProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransaction2Events_Impl: Sized {
    fn OnTransactionStart2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::Result<()>;
    fn OnTransactionPrepare2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTransactionAbort2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnTransactionCommit2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IComTransaction2Events {}
#[cfg(feature = "Win32_Foundation")]
impl IComTransaction2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>() -> IComTransaction2Events_Vtbl {
        unsafe extern "system" fn OnTransactionStart2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionStart2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&froot), ::core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionPrepare2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionAbort2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionCommit2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnTransactionStart2: OnTransactionStart2::<Identity, Impl, OFFSET>,
            OnTransactionPrepare2: OnTransactionPrepare2::<Identity, Impl, OFFSET>,
            OnTransactionAbort2: OnTransactionAbort2::<Identity, Impl, OFFSET>,
            OnTransactionCommit2: OnTransactionCommit2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTransaction2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransactionEvents_Impl: Sized {
    fn OnTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OnTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn OnTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IComTransactionEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComTransactionEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>() -> IComTransactionEvents_Vtbl {
        unsafe extern "system" fn OnTransactionStart<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, tsid: *const ::windows::core::GUID, froot: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&froot)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionPrepare(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnTransactionStart: OnTransactionStart::<Identity, Impl, OFFSET>,
            OnTransactionPrepare: OnTransactionPrepare::<Identity, Impl, OFFSET>,
            OnTransactionAbort: OnTransactionAbort::<Identity, Impl, OFFSET>,
            OnTransactionCommit: OnTransactionCommit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComTransactionEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IComUserEvent_Impl: Sized {
    fn OnUserEvent(&self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IComUserEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IComUserEvent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComUserEvent_Impl, const OFFSET: isize>() -> IComUserEvent_Vtbl {
        unsafe extern "system" fn OnUserEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IComUserEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUserEvent(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pvarevent)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnUserEvent: OnUserEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComUserEvent as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IContextProperties_Impl: Sized {
    fn Count(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, name: &super::super::Foundation::BSTR, pproperty: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EnumNames(&self) -> ::windows::core::Result<IEnumNames>;
    fn SetProperty(&self, name: &super::super::Foundation::BSTR, property: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn RemoveProperty(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IContextProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IContextProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>() -> IContextProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute(&name), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, property: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute(&name), ::core::mem::transmute(&property)).into()
        }
        unsafe extern "system" fn RemoveProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveProperty(::core::mem::transmute(&name)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            EnumNames: EnumNames::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            RemoveProperty: RemoveProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IContextSecurityPerimeter_Impl: Sized {
    fn GetPerimeterFlag(&self, pflag: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetPerimeterFlag(&self, fflag: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IContextSecurityPerimeter {}
#[cfg(feature = "Win32_Foundation")]
impl IContextSecurityPerimeter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>() -> IContextSecurityPerimeter_Vtbl {
        unsafe extern "system" fn GetPerimeterFlag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflag: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPerimeterFlag(::core::mem::transmute_copy(&pflag)).into()
        }
        unsafe extern "system" fn SetPerimeterFlag<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fflag: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPerimeterFlag(::core::mem::transmute_copy(&fflag)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetPerimeterFlag: GetPerimeterFlag::<Identity, Impl, OFFSET>,
            SetPerimeterFlag: SetPerimeterFlag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextSecurityPerimeter as ::windows::core::Interface>::IID
    }
}
pub trait IContextState_Impl: Sized {
    fn SetDeactivateOnReturn(&self, bdeactivate: i16) -> ::windows::core::Result<()>;
    fn GetDeactivateOnReturn(&self, pbdeactivate: *mut i16) -> ::windows::core::Result<()>;
    fn SetMyTransactionVote(&self, txvote: TransactionVote) -> ::windows::core::Result<()>;
    fn GetMyTransactionVote(&self, ptxvote: *mut TransactionVote) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IContextState {}
impl IContextState_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>() -> IContextState_Vtbl {
        unsafe extern "system" fn SetDeactivateOnReturn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdeactivate: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeactivateOnReturn(::core::mem::transmute_copy(&bdeactivate)).into()
        }
        unsafe extern "system" fn GetDeactivateOnReturn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeactivate: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeactivateOnReturn(::core::mem::transmute_copy(&pbdeactivate)).into()
        }
        unsafe extern "system" fn SetMyTransactionVote<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, txvote: TransactionVote) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMyTransactionVote(::core::mem::transmute_copy(&txvote)).into()
        }
        unsafe extern "system" fn GetMyTransactionVote<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxvote: *mut TransactionVote) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMyTransactionVote(::core::mem::transmute_copy(&ptxvote)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetDeactivateOnReturn: SetDeactivateOnReturn::<Identity, Impl, OFFSET>,
            GetDeactivateOnReturn: GetDeactivateOnReturn::<Identity, Impl, OFFSET>,
            SetMyTransactionVote: SetMyTransactionVote::<Identity, Impl, OFFSET>,
            GetMyTransactionVote: GetMyTransactionVote::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IContextState as ::windows::core::Interface>::IID
    }
}
pub trait ICreateWithLocalTransaction_Impl: Sized {
    fn CreateInstanceWithSysTx(&self, ptransaction: &::core::option::Option<::windows::core::IUnknown>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ICreateWithLocalTransaction {}
impl ICreateWithLocalTransaction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithLocalTransaction_Impl, const OFFSET: isize>() -> ICreateWithLocalTransaction_Vtbl {
        unsafe extern "system" fn CreateInstanceWithSysTx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithLocalTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstanceWithSysTx(::core::mem::transmute(&ptransaction), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateInstanceWithSysTx: CreateInstanceWithSysTx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateWithLocalTransaction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ICreateWithTipTransactionEx_Impl: Sized {
    fn CreateInstance(&self, bstrtipurl: &super::super::Foundation::BSTR, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ICreateWithTipTransactionEx {}
#[cfg(feature = "Win32_Foundation")]
impl ICreateWithTipTransactionEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTipTransactionEx_Impl, const OFFSET: isize>() -> ICreateWithTipTransactionEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTipTransactionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtipurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::core::mem::transmute(&bstrtipurl), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateWithTipTransactionEx as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ICreateWithTransactionEx_Impl: Sized {
    fn CreateInstance(&self, ptransaction: &::core::option::Option<super::DistributedTransactionCoordinator::ITransaction>, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows::core::RuntimeName for ICreateWithTransactionEx {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ICreateWithTransactionEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTransactionEx_Impl, const OFFSET: isize>() -> ICreateWithTransactionEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTransactionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::core::mem::transmute(&ptransaction), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICreateWithTransactionEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICrmCompensator_Impl: Sized {
    fn SetLogControl(&self, plogcontrol: &::core::option::Option<ICrmLogControl>) -> ::windows::core::Result<()>;
    fn BeginPrepare(&self) -> ::windows::core::Result<()>;
    fn PrepareRecord(&self, crmlogrec: &CrmLogRecordRead) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EndPrepare(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BeginCommit(&self, frecovery: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn CommitRecord(&self, crmlogrec: &CrmLogRecordRead) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EndCommit(&self) -> ::windows::core::Result<()>;
    fn BeginAbort(&self, frecovery: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn AbortRecord(&self, crmlogrec: &CrmLogRecordRead) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn EndAbort(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for ICrmCompensator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICrmCompensator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>() -> ICrmCompensator_Vtbl {
        unsafe extern "system" fn SetLogControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogControl(::core::mem::transmute(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginPrepare().into()
        }
        unsafe extern "system" fn PrepareRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrepareRecord(::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepare<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfoktoprepare: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndPrepare() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfoktoprepare, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginCommit(::core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn CommitRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitRecord(::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCommit().into()
        }
        unsafe extern "system" fn BeginAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginAbort(::core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn AbortRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AbortRecord(::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndAbort().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetLogControl: SetLogControl::<Identity, Impl, OFFSET>,
            BeginPrepare: BeginPrepare::<Identity, Impl, OFFSET>,
            PrepareRecord: PrepareRecord::<Identity, Impl, OFFSET>,
            EndPrepare: EndPrepare::<Identity, Impl, OFFSET>,
            BeginCommit: BeginCommit::<Identity, Impl, OFFSET>,
            CommitRecord: CommitRecord::<Identity, Impl, OFFSET>,
            EndCommit: EndCommit::<Identity, Impl, OFFSET>,
            BeginAbort: BeginAbort::<Identity, Impl, OFFSET>,
            AbortRecord: AbortRecord::<Identity, Impl, OFFSET>,
            EndAbort: EndAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmCompensator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmCompensatorVariants_Impl: Sized {
    fn SetLogControlVariants(&self, plogcontrol: &::core::option::Option<ICrmLogControl>) -> ::windows::core::Result<()>;
    fn BeginPrepareVariants(&self) -> ::windows::core::Result<()>;
    fn PrepareRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn EndPrepareVariants(&self) -> ::windows::core::Result<i16>;
    fn BeginCommitVariants(&self, brecovery: i16) -> ::windows::core::Result<()>;
    fn CommitRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn EndCommitVariants(&self) -> ::windows::core::Result<()>;
    fn BeginAbortVariants(&self, brecovery: i16) -> ::windows::core::Result<()>;
    fn AbortRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<i16>;
    fn EndAbortVariants(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICrmCompensatorVariants {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmCompensatorVariants_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>() -> ICrmCompensatorVariants_Vtbl {
        unsafe extern "system" fn SetLogControlVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogControlVariants(::core::mem::transmute(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepareVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginPrepareVariants().into()
        }
        unsafe extern "system" fn PrepareRecordVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrepareRecordVariants(::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbforget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepareVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboktoprepare: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndPrepareVariants() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboktoprepare, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommitVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginCommitVariants(::core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn CommitRecordVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitRecordVariants(::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbforget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommitVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCommitVariants().into()
        }
        unsafe extern "system" fn BeginAbortVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginAbortVariants(::core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn AbortRecordVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT, pbforget: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AbortRecordVariants(::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbforget, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbortVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndAbortVariants().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetLogControlVariants: SetLogControlVariants::<Identity, Impl, OFFSET>,
            BeginPrepareVariants: BeginPrepareVariants::<Identity, Impl, OFFSET>,
            PrepareRecordVariants: PrepareRecordVariants::<Identity, Impl, OFFSET>,
            EndPrepareVariants: EndPrepareVariants::<Identity, Impl, OFFSET>,
            BeginCommitVariants: BeginCommitVariants::<Identity, Impl, OFFSET>,
            CommitRecordVariants: CommitRecordVariants::<Identity, Impl, OFFSET>,
            EndCommitVariants: EndCommitVariants::<Identity, Impl, OFFSET>,
            BeginAbortVariants: BeginAbortVariants::<Identity, Impl, OFFSET>,
            AbortRecordVariants: AbortRecordVariants::<Identity, Impl, OFFSET>,
            EndAbortVariants: EndAbortVariants::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmCompensatorVariants as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmFormatLogRecords_Impl: Sized {
    fn GetColumnCount(&self) -> ::windows::core::Result<i32>;
    fn GetColumnHeaders(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetColumn(&self, crmlogrec: &CrmLogRecordRead) -> ::windows::core::Result<super::Com::VARIANT>;
    fn GetColumnVariants(&self, logrecord: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICrmFormatLogRecords {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmFormatLogRecords_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>() -> ICrmFormatLogRecords_Vtbl {
        unsafe extern "system" fn GetColumnCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcolumncount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcolumncount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheaders: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pheaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumn(::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformattedlogrecord, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logrecord: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pformattedlogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnVariants(::core::mem::transmute(&logrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformattedlogrecord, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetColumnCount: GetColumnCount::<Identity, Impl, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetColumnVariants: GetColumnVariants::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmFormatLogRecords as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmLogControl_Impl: Sized {
    fn TransactionUOW(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RegisterCompensator(&self, lpcwstrprogidcompensator: &::windows::core::PCWSTR, lpcwstrdescription: &::windows::core::PCWSTR, lcrmregflags: i32) -> ::windows::core::Result<()>;
    fn WriteLogRecordVariants(&self, plogrecord: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ForceLog(&self) -> ::windows::core::Result<()>;
    fn ForgetLogRecord(&self) -> ::windows::core::Result<()>;
    fn ForceTransactionToAbort(&self) -> ::windows::core::Result<()>;
    fn WriteLogRecord(&self, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICrmLogControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmLogControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>() -> ICrmLogControl_Vtbl {
        unsafe extern "system" fn TransactionUOW<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionUOW() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCompensator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcwstrprogidcompensator: ::windows::core::PCWSTR, lpcwstrdescription: ::windows::core::PCWSTR, lcrmregflags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterCompensator(::core::mem::transmute(&lpcwstrprogidcompensator), ::core::mem::transmute(&lpcwstrdescription), ::core::mem::transmute_copy(&lcrmregflags)).into()
        }
        unsafe extern "system" fn WriteLogRecordVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteLogRecordVariants(::core::mem::transmute_copy(&plogrecord)).into()
        }
        unsafe extern "system" fn ForceLog<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForceLog().into()
        }
        unsafe extern "system" fn ForgetLogRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForgetLogRecord().into()
        }
        unsafe extern "system" fn ForceTransactionToAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForceTransactionToAbort().into()
        }
        unsafe extern "system" fn WriteLogRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteLogRecord(::core::mem::transmute_copy(&rgblob), ::core::mem::transmute_copy(&cblob)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TransactionUOW: TransactionUOW::<Identity, Impl, OFFSET>,
            RegisterCompensator: RegisterCompensator::<Identity, Impl, OFFSET>,
            WriteLogRecordVariants: WriteLogRecordVariants::<Identity, Impl, OFFSET>,
            ForceLog: ForceLog::<Identity, Impl, OFFSET>,
            ForgetLogRecord: ForgetLogRecord::<Identity, Impl, OFFSET>,
            ForceTransactionToAbort: ForceTransactionToAbort::<Identity, Impl, OFFSET>,
            WriteLogRecord: WriteLogRecord::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmLogControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmMonitor_Impl: Sized {
    fn GetClerks(&self) -> ::windows::core::Result<ICrmMonitorClerks>;
    fn HoldClerk(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICrmMonitor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmMonitor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: isize>() -> ICrmMonitor_Vtbl {
        unsafe extern "system" fn GetClerks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclerks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClerks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclerks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldClerk<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HoldClerk(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetClerks: GetClerks::<Identity, Impl, OFFSET>,
            HoldClerk: HoldClerk::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmMonitor as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmMonitorClerks_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn ProgIdCompensator(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Description(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn TransactionUOW(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
    fn ActivityId(&self, index: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICrmMonitorClerks {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmMonitorClerks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>() -> ICrmMonitorClerks_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgIdCompensator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProgIdCompensator(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionUOW<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionUOW(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: ::core::mem::ManuallyDrop<super::Com::VARIANT>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivityId(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ProgIdCompensator: ProgIdCompensator::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            TransactionUOW: TransactionUOW::<Identity, Impl, OFFSET>,
            ActivityId: ActivityId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmMonitorClerks as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ICrmMonitorLogRecords_Impl: Sized {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn TransactionState(&self) -> ::windows::core::Result<CrmTransactionState>;
    fn StructuredRecords(&self) -> ::windows::core::Result<i16>;
    fn GetLogRecord(&self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::core::Result<()>;
    fn GetLogRecordVariants(&self, indexnumber: &super::Com::VARIANT) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ICrmMonitorLogRecords {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ICrmMonitorLogRecords_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>() -> ICrmMonitorLogRecords_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut CrmTransactionState) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StructuredRecords<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StructuredRecords() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogRecord<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLogRecord(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcrmlogrec)).into()
        }
        unsafe extern "system" fn GetLogRecordVariants<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexnumber: ::core::mem::ManuallyDrop<super::Com::VARIANT>, plogrecord: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLogRecordVariants(::core::mem::transmute(&indexnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plogrecord, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            TransactionState: TransactionState::<Identity, Impl, OFFSET>,
            StructuredRecords: StructuredRecords::<Identity, Impl, OFFSET>,
            GetLogRecord: GetLogRecord::<Identity, Impl, OFFSET>,
            GetLogRecordVariants: GetLogRecordVariants::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ICrmMonitorLogRecords as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDispenserDriver_Impl: Sized {
    fn CreateResource(&self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::core::Result<()>;
    fn RateResource(&self, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows::core::Result<()>;
    fn EnlistResource(&self, resid: usize, transid: usize) -> ::windows::core::Result<()>;
    fn ResetResource(&self, resid: usize) -> ::windows::core::Result<()>;
    fn DestroyResource(&self, resid: usize) -> ::windows::core::Result<()>;
    fn DestroyResourceS(&self, resid: *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDispenserDriver {}
#[cfg(feature = "Win32_Foundation")]
impl IDispenserDriver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>() -> IDispenserDriver_Vtbl {
        unsafe extern "system" fn CreateResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateResource(::core::mem::transmute_copy(&restypid), ::core::mem::transmute_copy(&presid), ::core::mem::transmute_copy(&psecsfreebeforedestroy)).into()
        }
        unsafe extern "system" fn RateResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RateResource(::core::mem::transmute_copy(&restypid), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&frequirestransactionenlistment), ::core::mem::transmute_copy(&prating)).into()
        }
        unsafe extern "system" fn EnlistResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize, transid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnlistResource(::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&transid)).into()
        }
        unsafe extern "system" fn ResetResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetResource(::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyResource(::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResourceS<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyResourceS(::core::mem::transmute_copy(&resid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateResource: CreateResource::<Identity, Impl, OFFSET>,
            RateResource: RateResource::<Identity, Impl, OFFSET>,
            EnlistResource: EnlistResource::<Identity, Impl, OFFSET>,
            ResetResource: ResetResource::<Identity, Impl, OFFSET>,
            DestroyResource: DestroyResource::<Identity, Impl, OFFSET>,
            DestroyResourceS: DestroyResourceS::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispenserDriver as ::windows::core::Interface>::IID
    }
}
pub trait IDispenserManager_Impl: Sized {
    fn RegisterDispenser(&self, __midl__idispensermanager0000: &::core::option::Option<IDispenserDriver>, szdispensername: &::windows::core::PCWSTR) -> ::windows::core::Result<IHolder>;
    fn GetContext(&self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDispenserManager {}
impl IDispenserManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: isize>() -> IDispenserManager_Vtbl {
        unsafe extern "system" fn RegisterDispenser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0000: *mut ::core::ffi::c_void, szdispensername: ::windows::core::PCWSTR, __midl__idispensermanager0001: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterDispenser(::core::mem::transmute(&__midl__idispensermanager0000), ::core::mem::transmute(&szdispensername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__idispensermanager0001, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContext(::core::mem::transmute_copy(&__midl__idispensermanager0002), ::core::mem::transmute_copy(&__midl__idispensermanager0003)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            RegisterDispenser: RegisterDispenser::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDispenserManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumNames_Impl: Sized {
    fn Next(&self, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNames>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IEnumNames {}
#[cfg(feature = "Win32_Foundation")]
impl IEnumNames_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>() -> IEnumNames_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgname: *mut super::super::Foundation::BSTR, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgname), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNames as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IEventServerTrace_Impl: Sized + super::Com::IDispatch_Impl {
    fn StartTraceGuid(&self, bstrguidevent: &super::super::Foundation::BSTR, bstrguidfilter: &super::super::Foundation::BSTR, lpidfilter: i32) -> ::windows::core::Result<()>;
    fn StopTraceGuid(&self, bstrguidevent: &super::super::Foundation::BSTR, bstrguidfilter: &super::super::Foundation::BSTR, lpidfilter: i32) -> ::windows::core::Result<()>;
    fn EnumTraceGuid(&self, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventServerTrace {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IEventServerTrace_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>() -> IEventServerTrace_Vtbl {
        unsafe extern "system" fn StartTraceGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartTraceGuid(::core::mem::transmute(&bstrguidevent), ::core::mem::transmute(&bstrguidfilter), ::core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn StopTraceGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrguidfilter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lpidfilter: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopTraceGuid(::core::mem::transmute(&bstrguidevent), ::core::mem::transmute(&bstrguidfilter), ::core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn EnumTraceGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumTraceGuid(::core::mem::transmute_copy(&plcntguids), ::core::mem::transmute_copy(&pbstrguidlist)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartTraceGuid: StartTraceGuid::<Identity, Impl, OFFSET>,
            StopTraceGuid: StopTraceGuid::<Identity, Impl, OFFSET>,
            EnumTraceGuid: EnumTraceGuid::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventServerTrace as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IGetAppTrackerData_Impl: Sized {
    fn GetApplicationProcesses(&self, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::core::Result<()>;
    fn GetApplicationProcessDetails(&self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetApplicationsInProcess(&self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::core::Result<()>;
    fn GetComponentsInProcess(&self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::core::Result<()>;
    fn GetComponentDetails(&self, applicationinstanceid: *const ::windows::core::GUID, processid: u32, clsid: *const ::windows::core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::core::Result<()>;
    fn GetTrackerDataAsCollectionObject(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetSuggestedPollingInterval(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IGetAppTrackerData {}
#[cfg(feature = "Win32_Foundation")]
impl IGetAppTrackerData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>() -> IGetAppTrackerData_Vtbl {
        unsafe extern "system" fn GetApplicationProcesses<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationProcesses(::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&applicationid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numapplicationprocesses), ::core::mem::transmute_copy(&applicationprocesses)).into()
        }
        unsafe extern "system" fn GetApplicationProcessDetails<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationProcessDetails(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&recycleinfo), ::core::mem::transmute_copy(&anycomponentshangmonitored)).into()
        }
        unsafe extern "system" fn GetApplicationsInProcess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationsInProcess(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numapplicationsinprocess), ::core::mem::transmute_copy(&applications)).into()
        }
        unsafe extern "system" fn GetComponentsInProcess<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, partitionid: *const ::windows::core::GUID, applicationid: *const ::windows::core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetComponentsInProcess(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&applicationid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numcomponentsinprocess), ::core::mem::transmute_copy(&components)).into()
        }
        unsafe extern "system" fn GetComponentDetails<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows::core::GUID, processid: u32, clsid: *const ::windows::core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetComponentDetails(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&hangmonitorinfo)).into()
        }
        unsafe extern "system" fn GetTrackerDataAsCollectionObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toplevelcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTrackerDataAsCollectionObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(toplevelcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSuggestedPollingInterval<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pollingintervalinseconds: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSuggestedPollingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pollingintervalinseconds, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetApplicationProcesses: GetApplicationProcesses::<Identity, Impl, OFFSET>,
            GetApplicationProcessDetails: GetApplicationProcessDetails::<Identity, Impl, OFFSET>,
            GetApplicationsInProcess: GetApplicationsInProcess::<Identity, Impl, OFFSET>,
            GetComponentsInProcess: GetComponentsInProcess::<Identity, Impl, OFFSET>,
            GetComponentDetails: GetComponentDetails::<Identity, Impl, OFFSET>,
            GetTrackerDataAsCollectionObject: GetTrackerDataAsCollectionObject::<Identity, Impl, OFFSET>,
            GetSuggestedPollingInterval: GetSuggestedPollingInterval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetAppTrackerData as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGetContextProperties_Impl: Sized {
    fn Count(&self, plcount: *mut i32) -> ::windows::core::Result<()>;
    fn GetProperty(&self, name: &super::super::Foundation::BSTR, pproperty: *mut super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn EnumNames(&self) -> ::windows::core::Result<IEnumNames>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGetContextProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGetContextProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>() -> IGetContextProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproperty: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute(&name), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            EnumNames: EnumNames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetContextProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IGetSecurityCallContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetSecurityCallContext(&self) -> ::windows::core::Result<ISecurityCallContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IGetSecurityCallContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IGetSecurityCallContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetSecurityCallContext_Impl, const OFFSET: isize>() -> IGetSecurityCallContext_Vtbl {
        unsafe extern "system" fn GetSecurityCallContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IGetSecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityCallContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetSecurityCallContext: GetSecurityCallContext::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IGetSecurityCallContext as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IHolder_Impl: Sized {
    fn AllocResource(&self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::core::Result<()>;
    fn FreeResource(&self, __midl__iholder0002: usize) -> ::windows::core::Result<()>;
    fn TrackResource(&self, __midl__iholder0003: usize) -> ::windows::core::Result<()>;
    fn TrackResourceS(&self, __midl__iholder0004: *mut u16) -> ::windows::core::Result<()>;
    fn UntrackResource(&self, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn UntrackResourceS(&self, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Close(&self) -> ::windows::core::Result<()>;
    fn RequestDestroyResource(&self, __midl__iholder0009: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IHolder {}
#[cfg(feature = "Win32_Foundation")]
impl IHolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>() -> IHolder_Vtbl {
        unsafe extern "system" fn AllocResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllocResource(::core::mem::transmute_copy(&__midl__iholder0000), ::core::mem::transmute_copy(&__midl__iholder0001)).into()
        }
        unsafe extern "system" fn FreeResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0002: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeResource(::core::mem::transmute_copy(&__midl__iholder0002)).into()
        }
        unsafe extern "system" fn TrackResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0003: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TrackResource(::core::mem::transmute_copy(&__midl__iholder0003)).into()
        }
        unsafe extern "system" fn TrackResourceS<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0004: *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TrackResourceS(::core::mem::transmute_copy(&__midl__iholder0004)).into()
        }
        unsafe extern "system" fn UntrackResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UntrackResource(::core::mem::transmute_copy(&__midl__iholder0005), ::core::mem::transmute_copy(&__midl__iholder0006)).into()
        }
        unsafe extern "system" fn UntrackResourceS<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UntrackResourceS(::core::mem::transmute_copy(&__midl__iholder0007), ::core::mem::transmute_copy(&__midl__iholder0008)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn RequestDestroyResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0009: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestDestroyResource(::core::mem::transmute_copy(&__midl__iholder0009)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AllocResource: AllocResource::<Identity, Impl, OFFSET>,
            FreeResource: FreeResource::<Identity, Impl, OFFSET>,
            TrackResource: TrackResource::<Identity, Impl, OFFSET>,
            TrackResourceS: TrackResourceS::<Identity, Impl, OFFSET>,
            UntrackResource: UntrackResource::<Identity, Impl, OFFSET>,
            UntrackResourceS: UntrackResourceS::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
            RequestDestroyResource: RequestDestroyResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ILBEvents_Impl: Sized {
    fn TargetUp(&self, bstrservername: &super::super::Foundation::BSTR, bstrclsideng: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TargetDown(&self, bstrservername: &super::super::Foundation::BSTR, bstrclsideng: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EngineDefined(&self, bstrpropname: &super::super::Foundation::BSTR, varpropvalue: *const super::Com::VARIANT, bstrclsideng: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ILBEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ILBEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>() -> ILBEvents_Vtbl {
        unsafe extern "system" fn TargetUp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TargetUp(::core::mem::transmute(&bstrservername), ::core::mem::transmute(&bstrclsideng)).into()
        }
        unsafe extern "system" fn TargetDown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TargetDown(::core::mem::transmute(&bstrservername), ::core::mem::transmute(&bstrclsideng)).into()
        }
        unsafe extern "system" fn EngineDefined<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varpropvalue: *const super::Com::VARIANT, bstrclsideng: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EngineDefined(::core::mem::transmute(&bstrpropname), ::core::mem::transmute_copy(&varpropvalue), ::core::mem::transmute(&bstrclsideng)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            TargetUp: TargetUp::<Identity, Impl, OFFSET>,
            TargetDown: TargetDown::<Identity, Impl, OFFSET>,
            EngineDefined: EngineDefined::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILBEvents as ::windows::core::Interface>::IID
    }
}
pub trait IMTSActivity_Impl: Sized {
    fn SynchronousCall(&self, pcall: &::core::option::Option<IMTSCall>) -> ::windows::core::Result<()>;
    fn AsyncCall(&self, pcall: &::core::option::Option<IMTSCall>) -> ::windows::core::Result<()>;
    fn Reserved1(&self);
    fn BindToCurrentThread(&self) -> ::windows::core::Result<()>;
    fn UnbindFromThread(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMTSActivity {}
impl IMTSActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>() -> IMTSActivity_Vtbl {
        unsafe extern "system" fn SynchronousCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCall(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn AsyncCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncCall(::core::mem::transmute(&pcall)).into()
        }
        unsafe extern "system" fn Reserved1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved1()
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToCurrentThread().into()
        }
        unsafe extern "system" fn UnbindFromThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnbindFromThread().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, Impl, OFFSET>,
            AsyncCall: AsyncCall::<Identity, Impl, OFFSET>,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, Impl, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMTSActivity as ::windows::core::Interface>::IID
    }
}
pub trait IMTSCall_Impl: Sized {
    fn OnCall(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMTSCall {}
impl IMTSCall_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSCall_Impl, const OFFSET: isize>() -> IMTSCall_Vtbl {
        unsafe extern "system" fn OnCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCall().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMTSCall as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMTSLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetEventDispatcher(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMTSLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMTSLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSLocator_Impl, const OFFSET: isize>() -> IMTSLocator_Vtbl {
        unsafe extern "system" fn GetEventDispatcher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMTSLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventDispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetEventDispatcher: GetEventDispatcher::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMTSLocator as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedActivationEvents_Impl: Sized {
    fn CreateManagedStub(&self, pinfo: &::core::option::Option<IManagedObjectInfo>, fdist: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn DestroyManagedStub(&self, pinfo: &::core::option::Option<IManagedObjectInfo>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IManagedActivationEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IManagedActivationEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: isize>() -> IManagedActivationEvents_Vtbl {
        unsafe extern "system" fn CreateManagedStub<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, fdist: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateManagedStub(::core::mem::transmute(&pinfo), ::core::mem::transmute_copy(&fdist)).into()
        }
        unsafe extern "system" fn DestroyManagedStub<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyManagedStub(::core::mem::transmute(&pinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateManagedStub: CreateManagedStub::<Identity, Impl, OFFSET>,
            DestroyManagedStub: DestroyManagedStub::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedActivationEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedObjectInfo_Impl: Sized {
    fn GetIUnknown(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetIObjectControl(&self) -> ::windows::core::Result<IObjectControl>;
    fn SetInPool(&self, binpool: super::super::Foundation::BOOL, ppooledobj: &::core::option::Option<IManagedPooledObj>) -> ::windows::core::Result<()>;
    fn SetWrapperStrength(&self, bstrong: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IManagedObjectInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IManagedObjectInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>() -> IManagedObjectInfo_Vtbl {
        unsafe extern "system" fn GetIUnknown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIObjectControl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctrl: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIObjectControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pctrl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPool<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binpool: super::super::Foundation::BOOL, ppooledobj: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInPool(::core::mem::transmute_copy(&binpool), ::core::mem::transmute(&ppooledobj)).into()
        }
        unsafe extern "system" fn SetWrapperStrength<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrong: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWrapperStrength(::core::mem::transmute_copy(&bstrong)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetIUnknown: GetIUnknown::<Identity, Impl, OFFSET>,
            GetIObjectControl: GetIObjectControl::<Identity, Impl, OFFSET>,
            SetInPool: SetInPool::<Identity, Impl, OFFSET>,
            SetWrapperStrength: SetWrapperStrength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedObjectInfo as ::windows::core::Interface>::IID
    }
}
pub trait IManagedPoolAction_Impl: Sized {
    fn LastRelease(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IManagedPoolAction {}
impl IManagedPoolAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedPoolAction_Impl, const OFFSET: isize>() -> IManagedPoolAction_Vtbl {
        unsafe extern "system" fn LastRelease<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedPoolAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LastRelease().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), LastRelease: LastRelease::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedPoolAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedPooledObj_Impl: Sized {
    fn SetHeld(&self, m_bheld: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IManagedPooledObj {}
#[cfg(feature = "Win32_Foundation")]
impl IManagedPooledObj_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedPooledObj_Impl, const OFFSET: isize>() -> IManagedPooledObj_Vtbl {
        unsafe extern "system" fn SetHeld<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IManagedPooledObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, m_bheld: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHeld(::core::mem::transmute_copy(&m_bheld)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SetHeld: SetHeld::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IManagedPooledObj as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMessageMover_Impl: Sized + super::Com::IDispatch_Impl {
    fn SourcePath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetSourcePath(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DestPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDestPath(&self, newval: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CommitBatchSize(&self) -> ::windows::core::Result<i32>;
    fn SetCommitBatchSize(&self, newval: i32) -> ::windows::core::Result<()>;
    fn MoveMessages(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMessageMover {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMessageMover_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>() -> IMessageMover_Vtbl {
        unsafe extern "system" fn SourcePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SourcePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourcePath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourcePath(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn DestPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDestPath(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn CommitBatchSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitBatchSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCommitBatchSize(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MoveMessages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagesmoved: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveMessages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmessagesmoved, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SourcePath: SourcePath::<Identity, Impl, OFFSET>,
            SetSourcePath: SetSourcePath::<Identity, Impl, OFFSET>,
            DestPath: DestPath::<Identity, Impl, OFFSET>,
            SetDestPath: SetDestPath::<Identity, Impl, OFFSET>,
            CommitBatchSize: CommitBatchSize::<Identity, Impl, OFFSET>,
            SetCommitBatchSize: SetCommitBatchSize::<Identity, Impl, OFFSET>,
            MoveMessages: MoveMessages::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMessageMover as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMtsEventInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Names(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn DisplayName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EventID(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Value(&self, skey: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMtsEventInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMtsEventInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>() -> IMtsEventInfo_Vtbl {
        unsafe extern "system" fn Names<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Names() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sdisplayname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sdisplayname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sguideventid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sguideventid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skey: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Value(::core::mem::transmute(&skey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Names: Names::<Identity, Impl, OFFSET>,
            DisplayName: DisplayName::<Identity, Impl, OFFSET>,
            EventID: EventID::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Value: get_Value::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMtsEventInfo as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMtsEvents_Impl: Sized + super::Com::IDispatch_Impl {
    fn PackageName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PackageGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn PostEvent(&self, vevent: *const super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn FireEvents(&self) -> ::windows::core::Result<i16>;
    fn GetProcessID(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMtsEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMtsEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>() -> IMtsEvents_Vtbl {
        unsafe extern "system" fn PackageName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PackageName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PackageGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vevent: *const super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostEvent(::core::mem::transmute_copy(&vevent)).into()
        }
        unsafe extern "system" fn FireEvents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FireEvents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProcessID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PackageName: PackageName::<Identity, Impl, OFFSET>,
            PackageGuid: PackageGuid::<Identity, Impl, OFFSET>,
            PostEvent: PostEvent::<Identity, Impl, OFFSET>,
            FireEvents: FireEvents::<Identity, Impl, OFFSET>,
            GetProcessID: GetProcessID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMtsEvents as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMtsGrp_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Item(&self, lindex: i32) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Refresh(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IMtsGrp {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMtsGrp_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>() -> IMtsGrp_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkdispatcher, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Refresh().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Refresh: Refresh::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMtsGrp as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IObjPool_Impl: Sized {
    fn Reserved1(&self);
    fn Reserved2(&self);
    fn Reserved3(&self);
    fn Reserved4(&self);
    fn PutEndTx(&self, pobj: &::core::option::Option<::windows::core::IUnknown>);
    fn Reserved5(&self);
    fn Reserved6(&self);
}
impl ::windows::core::RuntimeName for IObjPool {}
impl IObjPool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>() -> IObjPool_Vtbl {
        unsafe extern "system" fn Reserved1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved1()
        }
        unsafe extern "system" fn Reserved2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved2()
        }
        unsafe extern "system" fn Reserved3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved3()
        }
        unsafe extern "system" fn Reserved4<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved4()
        }
        unsafe extern "system" fn PutEndTx<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobj: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutEndTx(::core::mem::transmute(&pobj))
        }
        unsafe extern "system" fn Reserved5<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved5()
        }
        unsafe extern "system" fn Reserved6<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved6()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            Reserved3: Reserved3::<Identity, Impl, OFFSET>,
            Reserved4: Reserved4::<Identity, Impl, OFFSET>,
            PutEndTx: PutEndTx::<Identity, Impl, OFFSET>,
            Reserved5: Reserved5::<Identity, Impl, OFFSET>,
            Reserved6: Reserved6::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjPool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstruct_Impl: Sized {
    fn Construct(&self, pctorobj: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IObjectConstruct {}
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstruct_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstruct_Impl, const OFFSET: isize>() -> IObjectConstruct_Vtbl {
        unsafe extern "system" fn Construct<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstruct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctorobj: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Construct(::core::mem::transmute(&pctorobj)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Construct: Construct::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectConstruct as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IObjectConstructString_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConstructString(&self, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IObjectConstructString {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IObjectConstructString_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstructString_Impl, const OFFSET: isize>() -> IObjectConstructString_Vtbl {
        unsafe extern "system" fn ConstructString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstructString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConstructString(::core::mem::transmute_copy(&pval)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ConstructString: ConstructString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectConstructString as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContext_Impl: Sized {
    fn CreateInstance(&self, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn SetComplete(&self) -> ::windows::core::Result<()>;
    fn SetAbort(&self) -> ::windows::core::Result<()>;
    fn EnableCommit(&self) -> ::windows::core::Result<()>;
    fn DisableCommit(&self) -> ::windows::core::Result<()>;
    fn IsInTransaction(&self) -> super::super::Foundation::BOOL;
    fn IsSecurityEnabled(&self) -> super::super::Foundation::BOOL;
    fn IsCallerInRole(&self, bstrrole: &super::super::Foundation::BSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IObjectContext {}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>() -> IObjectContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn SetComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetComplete().into()
        }
        unsafe extern "system" fn SetAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAbort().into()
        }
        unsafe extern "system" fn EnableCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableCommit().into()
        }
        unsafe extern "system" fn DisableCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableCommit().into()
        }
        unsafe extern "system" fn IsInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsInTransaction()
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSecurityEnabled()
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsCallerInRole(::core::mem::transmute(&bstrrole), ::core::mem::transmute_copy(&pfisinrole)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            SetComplete: SetComplete::<Identity, Impl, OFFSET>,
            SetAbort: SetAbort::<Identity, Impl, OFFSET>,
            EnableCommit: EnableCommit::<Identity, Impl, OFFSET>,
            DisableCommit: DisableCommit::<Identity, Impl, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContext as ::windows::core::Interface>::IID
    }
}
pub trait IObjectContextActivity_Impl: Sized {
    fn GetActivityId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IObjectContextActivity {}
impl IObjectContextActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextActivity_Impl, const OFFSET: isize>() -> IObjectContextActivity_Vtbl {
        unsafe extern "system" fn GetActivityId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetActivityId(::core::mem::transmute_copy(&pguid)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetActivityId: GetActivityId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextActivity as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfo_Impl: Sized {
    fn IsInTransaction(&self) -> super::super::Foundation::BOOL;
    fn GetTransaction(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetTransactionId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetActivityId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetContextId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IObjectContextInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>() -> IObjectContextInfo_Vtbl {
        unsafe extern "system" fn IsInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsInTransaction()
        }
        unsafe extern "system" fn GetTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrans: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrans, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTransactionId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetActivityId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetActivityId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetContextId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContextId(::core::mem::transmute_copy(&pguid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            GetTransaction: GetTransaction::<Identity, Impl, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, Impl, OFFSET>,
            GetActivityId: GetActivityId::<Identity, Impl, OFFSET>,
            GetContextId: GetContextId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfo2_Impl: Sized + IObjectContextInfo_Impl {
    fn GetPartitionId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetApplicationId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn GetApplicationInstanceId(&self, pguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IObjectContextInfo2 {}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextInfo2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>() -> IObjectContextInfo2_Vtbl {
        unsafe extern "system" fn GetPartitionId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPartitionId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationInstanceId(::core::mem::transmute_copy(&pguid)).into()
        }
        Self {
            base__: IObjectContextInfo_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPartitionId: GetPartitionId::<Identity, Impl, OFFSET>,
            GetApplicationId: GetApplicationId::<Identity, Impl, OFFSET>,
            GetApplicationInstanceId: GetApplicationInstanceId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextInfo2 as ::windows::core::Interface>::IID || iid == &<IObjectContextInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextTip_Impl: Sized {
    fn GetTipUrl(&self, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IObjectContextTip {}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextTip_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextTip_Impl, const OFFSET: isize>() -> IObjectContextTip_Vtbl {
        unsafe extern "system" fn GetTipUrl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextTip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptipurl: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTipUrl(::core::mem::transmute_copy(&ptipurl)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetTipUrl: GetTipUrl::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectContextTip as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectControl_Impl: Sized {
    fn Activate(&self) -> ::windows::core::Result<()>;
    fn Deactivate(&self);
    fn CanBePooled(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IObjectControl {}
#[cfg(feature = "Win32_Foundation")]
impl IObjectControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>() -> IObjectControl_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate().into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate()
        }
        unsafe extern "system" fn CanBePooled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanBePooled()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CanBePooled: CanBePooled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IObjectControl as ::windows::core::Interface>::IID
    }
}
pub trait IPlaybackControl_Impl: Sized {
    fn FinalClientRetry(&self) -> ::windows::core::Result<()>;
    fn FinalServerRetry(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPlaybackControl {}
impl IPlaybackControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: isize>() -> IPlaybackControl_Vtbl {
        unsafe extern "system" fn FinalClientRetry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinalClientRetry().into()
        }
        unsafe extern "system" fn FinalServerRetry<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinalServerRetry().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FinalClientRetry: FinalClientRetry::<Identity, Impl, OFFSET>,
            FinalServerRetry: FinalServerRetry::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPlaybackControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IPoolManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn ShutdownPool(&self, clsidorprogid: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IPoolManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IPoolManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPoolManager_Impl, const OFFSET: isize>() -> IPoolManager_Vtbl {
        unsafe extern "system" fn ShutdownPool<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidorprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownPool(::core::mem::transmute(&clsidorprogid)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ShutdownPool: ShutdownPool::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPoolManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IProcessInitializer_Impl: Sized {
    fn Startup(&self, punkprocesscontrol: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IProcessInitializer {}
impl IProcessInitializer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: isize>() -> IProcessInitializer_Vtbl {
        unsafe extern "system" fn Startup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkprocesscontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Startup(::core::mem::transmute(&punkprocesscontrol)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Startup: Startup::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProcessInitializer as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISecurityCallContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn IsCallerInRole(&self, bstrrole: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn IsSecurityEnabled(&self) -> ::windows::core::Result<i16>;
    fn IsUserInRole(&self, puser: *const super::Com::VARIANT, bstrrole: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISecurityCallContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISecurityCallContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>() -> ISecurityCallContext_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCallerInRole(::core::mem::transmute(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinrole, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSecurityEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserInRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *const super::Com::VARIANT, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pfinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUserInRole(::core::mem::transmute_copy(&puser), ::core::mem::transmute(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinrole, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            IsUserInRole: IsUserInRole::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityCallContext as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISecurityCallersColl_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows::core::Result<ISecurityIdentityColl>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISecurityCallersColl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISecurityCallersColl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>() -> ISecurityCallersColl_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pobj: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobj, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityCallersColl as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISecurityIdentityColl_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISecurityIdentityColl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISecurityIdentityColl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>() -> ISecurityIdentityColl_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityIdentityColl as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityProperty_Impl: Sized {
    fn GetDirectCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()>;
    fn GetOriginalCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()>;
    fn GetDirectCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()>;
    fn GetOriginalCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows::core::Result<()>;
    fn ReleaseSID(&self, psid: super::super::Foundation::PSID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISecurityProperty {}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>() -> ISecurityProperty_Vtbl {
        unsafe extern "system" fn GetDirectCreatorSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDirectCreatorSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCreatorSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOriginalCreatorSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetDirectCallerSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDirectCallerSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCallerSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOriginalCallerSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn ReleaseSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: super::super::Foundation::PSID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseSID(::core::mem::transmute_copy(&psid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDirectCreatorSID: GetDirectCreatorSID::<Identity, Impl, OFFSET>,
            GetOriginalCreatorSID: GetOriginalCreatorSID::<Identity, Impl, OFFSET>,
            GetDirectCallerSID: GetDirectCallerSID::<Identity, Impl, OFFSET>,
            GetOriginalCallerSID: GetOriginalCallerSID::<Identity, Impl, OFFSET>,
            ReleaseSID: ReleaseSID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISecurityProperty as ::windows::core::Interface>::IID
    }
}
pub trait ISelectCOMLBServer_Impl: Sized {
    fn Init(&self) -> ::windows::core::Result<()>;
    fn GetLBServer(&self, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISelectCOMLBServer {}
impl ISelectCOMLBServer_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>() -> ISelectCOMLBServer_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init().into()
        }
        unsafe extern "system" fn GetLBServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLBServer(::core::mem::transmute(&punk)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetLBServer: GetLBServer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISelectCOMLBServer as ::windows::core::Interface>::IID
    }
}
pub trait ISendMethodEvents_Impl: Sized {
    fn SendMethodCall(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32) -> ::windows::core::Result<()>;
    fn SendMethodReturn(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32, hrcall: ::windows::core::HRESULT, hrserver: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ISendMethodEvents {}
impl ISendMethodEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: isize>() -> ISendMethodEvents_Vtbl {
        unsafe extern "system" fn SendMethodCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMethodCall(::core::mem::transmute_copy(&pidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwmeth)).into()
        }
        unsafe extern "system" fn SendMethodReturn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows::core::GUID, dwmeth: u32, hrcall: ::windows::core::HRESULT, hrserver: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMethodReturn(::core::mem::transmute_copy(&pidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwmeth), ::core::mem::transmute_copy(&hrcall), ::core::mem::transmute_copy(&hrserver)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SendMethodCall: SendMethodCall::<Identity, Impl, OFFSET>,
            SendMethodReturn: SendMethodReturn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISendMethodEvents as ::windows::core::Interface>::IID
    }
}
pub trait IServiceActivity_Impl: Sized {
    fn SynchronousCall(&self, piservicecall: &::core::option::Option<IServiceCall>) -> ::windows::core::Result<()>;
    fn AsynchronousCall(&self, piservicecall: &::core::option::Option<IServiceCall>) -> ::windows::core::Result<()>;
    fn BindToCurrentThread(&self) -> ::windows::core::Result<()>;
    fn UnbindFromThread(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceActivity {}
impl IServiceActivity_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>() -> IServiceActivity_Vtbl {
        unsafe extern "system" fn SynchronousCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCall(::core::mem::transmute(&piservicecall)).into()
        }
        unsafe extern "system" fn AsynchronousCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsynchronousCall(::core::mem::transmute(&piservicecall)).into()
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToCurrentThread().into()
        }
        unsafe extern "system" fn UnbindFromThread<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnbindFromThread().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, Impl, OFFSET>,
            AsynchronousCall: AsynchronousCall::<Identity, Impl, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, Impl, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceActivity as ::windows::core::Interface>::IID
    }
}
pub trait IServiceCall_Impl: Sized {
    fn OnCall(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceCall {}
impl IServiceCall_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceCall_Impl, const OFFSET: isize>() -> IServiceCall_Vtbl {
        unsafe extern "system" fn OnCall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCall().into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceCall as ::windows::core::Interface>::IID
    }
}
pub trait IServiceComTIIntrinsicsConfig_Impl: Sized {
    fn ComTIIntrinsicsConfig(&self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceComTIIntrinsicsConfig {}
impl IServiceComTIIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: isize>() -> IServiceComTIIntrinsicsConfig_Vtbl {
        unsafe extern "system" fn ComTIIntrinsicsConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ComTIIntrinsicsConfig(::core::mem::transmute_copy(&comtiintrinsicsconfig)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ComTIIntrinsicsConfig: ComTIIntrinsicsConfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceComTIIntrinsicsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceIISIntrinsicsConfig_Impl: Sized {
    fn IISIntrinsicsConfig(&self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceIISIntrinsicsConfig {}
impl IServiceIISIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceIISIntrinsicsConfig_Impl, const OFFSET: isize>() -> IServiceIISIntrinsicsConfig_Vtbl {
        unsafe extern "system" fn IISIntrinsicsConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceIISIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IISIntrinsicsConfig(::core::mem::transmute_copy(&iisintrinsicsconfig)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), IISIntrinsicsConfig: IISIntrinsicsConfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceIISIntrinsicsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceInheritanceConfig_Impl: Sized {
    fn ContainingContextTreatment(&self, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceInheritanceConfig {}
impl IServiceInheritanceConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceInheritanceConfig_Impl, const OFFSET: isize>() -> IServiceInheritanceConfig_Vtbl {
        unsafe extern "system" fn ContainingContextTreatment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceInheritanceConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ContainingContextTreatment(::core::mem::transmute_copy(&inheritanceconfig)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ContainingContextTreatment: ContainingContextTreatment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceInheritanceConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServicePartitionConfig_Impl: Sized {
    fn PartitionConfig(&self, partitionconfig: CSC_PartitionConfig) -> ::windows::core::Result<()>;
    fn PartitionID(&self, guidpartitionid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServicePartitionConfig {}
impl IServicePartitionConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: isize>() -> IServicePartitionConfig_Vtbl {
        unsafe extern "system" fn PartitionConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PartitionConfig(::core::mem::transmute_copy(&partitionconfig)).into()
        }
        unsafe extern "system" fn PartitionID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidpartitionid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PartitionID(::core::mem::transmute_copy(&guidpartitionid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PartitionConfig: PartitionConfig::<Identity, Impl, OFFSET>,
            PartitionID: PartitionID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServicePartitionConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServicePool_Impl: Sized {
    fn Initialize(&self, ppoolconfig: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetObject(&self, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Shutdown(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServicePool {}
impl IServicePool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>() -> IServicePool_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoolconfig: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&ppoolconfig)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServicePool as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IServicePoolConfig_Impl: Sized {
    fn SetMaxPoolSize(&self, dwmaxpool: u32) -> ::windows::core::Result<()>;
    fn MaxPoolSize(&self, pdwmaxpool: *mut u32) -> ::windows::core::Result<()>;
    fn SetMinPoolSize(&self, dwminpool: u32) -> ::windows::core::Result<()>;
    fn MinPoolSize(&self, pdwminpool: *mut u32) -> ::windows::core::Result<()>;
    fn SetCreationTimeout(&self, dwcreationtimeout: u32) -> ::windows::core::Result<()>;
    fn CreationTimeout(&self, pdwcreationtimeout: *mut u32) -> ::windows::core::Result<()>;
    fn SetTransactionAffinity(&self, ftxaffinity: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn TransactionAffinity(&self, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetClassFactory(&self, pfactory: &::core::option::Option<super::Com::IClassFactory>) -> ::windows::core::Result<()>;
    fn ClassFactory(&self) -> ::windows::core::Result<super::Com::IClassFactory>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IServicePoolConfig {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IServicePoolConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>() -> IServicePoolConfig_Vtbl {
        unsafe extern "system" fn SetMaxPoolSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxpool: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxPoolSize(::core::mem::transmute_copy(&dwmaxpool)).into()
        }
        unsafe extern "system" fn MaxPoolSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxpool: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MaxPoolSize(::core::mem::transmute_copy(&pdwmaxpool)).into()
        }
        unsafe extern "system" fn SetMinPoolSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminpool: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinPoolSize(::core::mem::transmute_copy(&dwminpool)).into()
        }
        unsafe extern "system" fn MinPoolSize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminpool: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MinPoolSize(::core::mem::transmute_copy(&pdwminpool)).into()
        }
        unsafe extern "system" fn SetCreationTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreationtimeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCreationTimeout(::core::mem::transmute_copy(&dwcreationtimeout)).into()
        }
        unsafe extern "system" fn CreationTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcreationtimeout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreationTimeout(::core::mem::transmute_copy(&pdwcreationtimeout)).into()
        }
        unsafe extern "system" fn SetTransactionAffinity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftxaffinity: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransactionAffinity(::core::mem::transmute_copy(&ftxaffinity)).into()
        }
        unsafe extern "system" fn TransactionAffinity<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransactionAffinity(::core::mem::transmute_copy(&pftxaffinity)).into()
        }
        unsafe extern "system" fn SetClassFactory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClassFactory(::core::mem::transmute(&pfactory)).into()
        }
        unsafe extern "system" fn ClassFactory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassFactory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfactory, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetMaxPoolSize: SetMaxPoolSize::<Identity, Impl, OFFSET>,
            MaxPoolSize: MaxPoolSize::<Identity, Impl, OFFSET>,
            SetMinPoolSize: SetMinPoolSize::<Identity, Impl, OFFSET>,
            MinPoolSize: MinPoolSize::<Identity, Impl, OFFSET>,
            SetCreationTimeout: SetCreationTimeout::<Identity, Impl, OFFSET>,
            CreationTimeout: CreationTimeout::<Identity, Impl, OFFSET>,
            SetTransactionAffinity: SetTransactionAffinity::<Identity, Impl, OFFSET>,
            TransactionAffinity: TransactionAffinity::<Identity, Impl, OFFSET>,
            SetClassFactory: SetClassFactory::<Identity, Impl, OFFSET>,
            ClassFactory: ClassFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServicePoolConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceSxsConfig_Impl: Sized {
    fn SxsConfig(&self, scsconfig: CSC_SxsConfig) -> ::windows::core::Result<()>;
    fn SxsName(&self, szsxsname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn SxsDirectory(&self, szsxsdirectory: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceSxsConfig {}
impl IServiceSxsConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>() -> IServiceSxsConfig_Vtbl {
        unsafe extern "system" fn SxsConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scsconfig: CSC_SxsConfig) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SxsConfig(::core::mem::transmute_copy(&scsconfig)).into()
        }
        unsafe extern "system" fn SxsName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SxsName(::core::mem::transmute(&szsxsname)).into()
        }
        unsafe extern "system" fn SxsDirectory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SxsDirectory(::core::mem::transmute(&szsxsdirectory)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SxsConfig: SxsConfig::<Identity, Impl, OFFSET>,
            SxsName: SxsName::<Identity, Impl, OFFSET>,
            SxsDirectory: SxsDirectory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceSxsConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceSynchronizationConfig_Impl: Sized {
    fn ConfigureSynchronization(&self, synchconfig: CSC_SynchronizationConfig) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceSynchronizationConfig {}
impl IServiceSynchronizationConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceSynchronizationConfig_Impl, const OFFSET: isize>() -> IServiceSynchronizationConfig_Vtbl {
        unsafe extern "system" fn ConfigureSynchronization<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceSynchronizationConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureSynchronization(::core::mem::transmute_copy(&synchconfig)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), ConfigureSynchronization: ConfigureSynchronization::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceSynchronizationConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceSysTxnConfig_Impl: Sized + IServiceTransactionConfigBase_Impl + IServiceTransactionConfig_Impl {
    fn ConfigureBYOTSysTxn(&self, ptxproxy: &::core::option::Option<ITransactionProxy>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows::core::RuntimeName for IServiceSysTxnConfig {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl IServiceSysTxnConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceSysTxnConfig_Impl, const OFFSET: isize>() -> IServiceSysTxnConfig_Vtbl {
        unsafe extern "system" fn ConfigureBYOTSysTxn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceSysTxnConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxproxy: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureBYOTSysTxn(::core::mem::transmute(&ptxproxy)).into()
        }
        Self { base__: IServiceTransactionConfig_Vtbl::new::<Identity, Impl, OFFSET>(), ConfigureBYOTSysTxn: ConfigureBYOTSysTxn::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceSysTxnConfig as ::windows::core::Interface>::IID || iid == &<IServiceTransactionConfigBase as ::windows::core::Interface>::IID || iid == &<IServiceTransactionConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceThreadPoolConfig_Impl: Sized {
    fn SelectThreadPool(&self, threadpool: CSC_ThreadPool) -> ::windows::core::Result<()>;
    fn SetBindingInfo(&self, binding: CSC_Binding) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceThreadPoolConfig {}
impl IServiceThreadPoolConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>() -> IServiceThreadPoolConfig_Vtbl {
        unsafe extern "system" fn SelectThreadPool<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadpool: CSC_ThreadPool) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectThreadPool(::core::mem::transmute_copy(&threadpool)).into()
        }
        unsafe extern "system" fn SetBindingInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: CSC_Binding) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBindingInfo(::core::mem::transmute_copy(&binding)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SelectThreadPool: SelectThreadPool::<Identity, Impl, OFFSET>,
            SetBindingInfo: SetBindingInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceThreadPoolConfig as ::windows::core::Interface>::IID
    }
}
pub trait IServiceTrackerConfig_Impl: Sized {
    fn TrackerConfig(&self, trackerconfig: CSC_TrackerConfig, sztrackerappname: &::windows::core::PCWSTR, sztrackerctxname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceTrackerConfig {}
impl IServiceTrackerConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTrackerConfig_Impl, const OFFSET: isize>() -> IServiceTrackerConfig_Vtbl {
        unsafe extern "system" fn TrackerConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTrackerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: ::windows::core::PCWSTR, sztrackerctxname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TrackerConfig(::core::mem::transmute_copy(&trackerconfig), ::core::mem::transmute(&sztrackerappname), ::core::mem::transmute(&sztrackerctxname)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), TrackerConfig: TrackerConfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceTrackerConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceTransactionConfig_Impl: Sized + IServiceTransactionConfigBase_Impl {
    fn ConfigureBYOT(&self, pitxbyot: &::core::option::Option<super::DistributedTransactionCoordinator::ITransaction>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows::core::RuntimeName for IServiceTransactionConfig {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl IServiceTransactionConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfig_Impl, const OFFSET: isize>() -> IServiceTransactionConfig_Vtbl {
        unsafe extern "system" fn ConfigureBYOT<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitxbyot: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureBYOT(::core::mem::transmute(&pitxbyot)).into()
        }
        Self { base__: IServiceTransactionConfigBase_Vtbl::new::<Identity, Impl, OFFSET>(), ConfigureBYOT: ConfigureBYOT::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceTransactionConfig as ::windows::core::Interface>::IID || iid == &<IServiceTransactionConfigBase as ::windows::core::Interface>::IID
    }
}
pub trait IServiceTransactionConfigBase_Impl: Sized {
    fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows::core::Result<()>;
    fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::Result<()>;
    fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows::core::Result<()>;
    fn BringYourOwnTransaction(&self, sztipurl: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn NewTransactionDescription(&self, sztxdesc: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IServiceTransactionConfigBase {}
impl IServiceTransactionConfigBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>() -> IServiceTransactionConfigBase_Vtbl {
        unsafe extern "system" fn ConfigureTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureTransaction(::core::mem::transmute_copy(&transactionconfig)).into()
        }
        unsafe extern "system" fn IsolationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsolationLevel(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn TransactionTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultimeoutsec: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransactionTimeout(::core::mem::transmute_copy(&ultimeoutsec)).into()
        }
        unsafe extern "system" fn BringYourOwnTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztipurl: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BringYourOwnTransaction(::core::mem::transmute(&sztipurl)).into()
        }
        unsafe extern "system" fn NewTransactionDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztxdesc: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NewTransactionDescription(::core::mem::transmute(&sztxdesc)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ConfigureTransaction: ConfigureTransaction::<Identity, Impl, OFFSET>,
            IsolationLevel: IsolationLevel::<Identity, Impl, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, Impl, OFFSET>,
            BringYourOwnTransaction: BringYourOwnTransaction::<Identity, Impl, OFFSET>,
            NewTransactionDescription: NewTransactionDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IServiceTransactionConfigBase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISharedProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetValue(&self, val: &super::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISharedProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISharedProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: isize>() -> ISharedProperty_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute(&val)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedProperty as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISharedPropertyGroup_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreatePropertyByPosition(&self, index: i32, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows::core::Result<()>;
    fn get_PropertyByPosition(&self, index: i32) -> ::windows::core::Result<ISharedProperty>;
    fn CreateProperty(&self, name: &super::super::Foundation::BSTR, fexists: *mut i16, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows::core::Result<()>;
    fn get_Property(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISharedProperty>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISharedPropertyGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISharedPropertyGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>() -> ISharedPropertyGroup_Vtbl {
        unsafe extern "system" fn CreatePropertyByPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, fexists: *mut i16, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePropertyByPosition(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn get_PropertyByPosition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PropertyByPosition(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, fexists: *mut i16, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateProperty(::core::mem::transmute(&name), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn get_Property<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Property(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreatePropertyByPosition: CreatePropertyByPosition::<Identity, Impl, OFFSET>,
            get_PropertyByPosition: get_PropertyByPosition::<Identity, Impl, OFFSET>,
            CreateProperty: CreateProperty::<Identity, Impl, OFFSET>,
            get_Property: get_Property::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPropertyGroup as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISharedPropertyGroupManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreatePropertyGroup(&self, name: &super::super::Foundation::BSTR, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut ::core::option::Option<ISharedPropertyGroup>) -> ::windows::core::Result<()>;
    fn get_Group(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<ISharedPropertyGroup>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISharedPropertyGroupManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISharedPropertyGroupManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>() -> ISharedPropertyGroupManager_Vtbl {
        unsafe extern "system" fn CreatePropertyGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut i16, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePropertyGroup(::core::mem::transmute(&name), ::core::mem::transmute_copy(&dwisomode), ::core::mem::transmute_copy(&dwrelmode), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppgroup)).into()
        }
        unsafe extern "system" fn get_Group<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Group(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreatePropertyGroup: CreatePropertyGroup::<Identity, Impl, OFFSET>,
            get_Group: get_Group::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISharedPropertyGroupManager as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISystemAppEventData_Impl: Sized {
    fn Startup(&self) -> ::windows::core::Result<()>;
    fn OnDataChanged(&self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: &super::super::Foundation::BSTR, dwreason: u32, u64tracehandle: u64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for ISystemAppEventData {}
#[cfg(feature = "Win32_Foundation")]
impl ISystemAppEventData_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: isize>() -> ISystemAppEventData_Vtbl {
        unsafe extern "system" fn Startup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Startup().into()
        }
        unsafe extern "system" fn OnDataChanged<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataChanged(::core::mem::transmute_copy(&dwpid), ::core::mem::transmute_copy(&dwmask), ::core::mem::transmute_copy(&dwnumbersinks), ::core::mem::transmute(&bstrdwmethodmask), ::core::mem::transmute_copy(&dwreason), ::core::mem::transmute_copy(&u64tracehandle)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Startup: Startup::<Identity, Impl, OFFSET>,
            OnDataChanged: OnDataChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISystemAppEventData as ::windows::core::Interface>::IID
    }
}
pub trait IThreadPoolKnobs_Impl: Sized {
    fn GetMaxThreads(&self, plcmaxthreads: *mut i32) -> ::windows::core::Result<()>;
    fn GetCurrentThreads(&self, plccurrentthreads: *mut i32) -> ::windows::core::Result<()>;
    fn SetMaxThreads(&self, lcmaxthreads: i32) -> ::windows::core::Result<()>;
    fn GetDeleteDelay(&self, pmsecdeletedelay: *mut i32) -> ::windows::core::Result<()>;
    fn SetDeleteDelay(&self, msecdeletedelay: i32) -> ::windows::core::Result<()>;
    fn GetMaxQueuedRequests(&self, plcmaxqueuedrequests: *mut i32) -> ::windows::core::Result<()>;
    fn GetCurrentQueuedRequests(&self, plccurrentqueuedrequests: *mut i32) -> ::windows::core::Result<()>;
    fn SetMaxQueuedRequests(&self, lcmaxqueuedrequests: i32) -> ::windows::core::Result<()>;
    fn SetMinThreads(&self, lcminthreads: i32) -> ::windows::core::Result<()>;
    fn SetQueueDepth(&self, lcqueuedepth: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IThreadPoolKnobs {}
impl IThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>() -> IThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn GetMaxThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxthreads: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaxThreads(::core::mem::transmute_copy(&plcmaxthreads)).into()
        }
        unsafe extern "system" fn GetCurrentThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentthreads: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentThreads(::core::mem::transmute_copy(&plccurrentthreads)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxthreads: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxThreads(::core::mem::transmute_copy(&lcmaxthreads)).into()
        }
        unsafe extern "system" fn GetDeleteDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsecdeletedelay: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeleteDelay(::core::mem::transmute_copy(&pmsecdeletedelay)).into()
        }
        unsafe extern "system" fn SetDeleteDelay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msecdeletedelay: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeleteDelay(::core::mem::transmute_copy(&msecdeletedelay)).into()
        }
        unsafe extern "system" fn GetMaxQueuedRequests<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaxQueuedRequests(::core::mem::transmute_copy(&plcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn GetCurrentQueuedRequests<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentQueuedRequests(::core::mem::transmute_copy(&plccurrentqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMaxQueuedRequests<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxqueuedrequests: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxQueuedRequests(::core::mem::transmute_copy(&lcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMinThreads<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcminthreads: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinThreads(::core::mem::transmute_copy(&lcminthreads)).into()
        }
        unsafe extern "system" fn SetQueueDepth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcqueuedepth: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueueDepth(::core::mem::transmute_copy(&lcqueuedepth)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetMaxThreads: GetMaxThreads::<Identity, Impl, OFFSET>,
            GetCurrentThreads: GetCurrentThreads::<Identity, Impl, OFFSET>,
            SetMaxThreads: SetMaxThreads::<Identity, Impl, OFFSET>,
            GetDeleteDelay: GetDeleteDelay::<Identity, Impl, OFFSET>,
            SetDeleteDelay: SetDeleteDelay::<Identity, Impl, OFFSET>,
            GetMaxQueuedRequests: GetMaxQueuedRequests::<Identity, Impl, OFFSET>,
            GetCurrentQueuedRequests: GetCurrentQueuedRequests::<Identity, Impl, OFFSET>,
            SetMaxQueuedRequests: SetMaxQueuedRequests::<Identity, Impl, OFFSET>,
            SetMinThreads: SetMinThreads::<Identity, Impl, OFFSET>,
            SetQueueDepth: SetQueueDepth::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IThreadPoolKnobs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITransactionContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateInstance(&self, pszprogid: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn Commit(&self) -> ::windows::core::Result<()>;
    fn Abort(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITransactionContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITransactionContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>() -> ITransactionContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(::core::mem::transmute(&pszprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionContext as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionContextEx_Impl: Sized {
    fn CreateInstance(&self, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Commit(&self) -> ::windows::core::Result<()>;
    fn Abort(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionContextEx {}
impl ITransactionContextEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>() -> ITransactionContextEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionContextEx as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionProperty_Impl: Sized {
    fn Reserved1(&self);
    fn Reserved2(&self);
    fn Reserved3(&self);
    fn Reserved4(&self);
    fn Reserved5(&self);
    fn Reserved6(&self);
    fn Reserved7(&self);
    fn Reserved8(&self);
    fn Reserved9(&self);
    fn GetTransactionResourcePool(&self) -> ::windows::core::Result<ITransactionResourcePool>;
    fn Reserved10(&self);
    fn Reserved11(&self);
    fn Reserved12(&self);
    fn Reserved13(&self);
    fn Reserved14(&self);
    fn Reserved15(&self);
    fn Reserved16(&self);
    fn Reserved17(&self);
}
impl ::windows::core::RuntimeName for ITransactionProperty {}
impl ITransactionProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>() -> ITransactionProperty_Vtbl {
        unsafe extern "system" fn Reserved1<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved1()
        }
        unsafe extern "system" fn Reserved2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved2()
        }
        unsafe extern "system" fn Reserved3<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved3()
        }
        unsafe extern "system" fn Reserved4<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved4()
        }
        unsafe extern "system" fn Reserved5<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved5()
        }
        unsafe extern "system" fn Reserved6<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved6()
        }
        unsafe extern "system" fn Reserved7<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved7()
        }
        unsafe extern "system" fn Reserved8<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved8()
        }
        unsafe extern "system" fn Reserved9<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved9()
        }
        unsafe extern "system" fn GetTransactionResourcePool<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptxpool: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransactionResourcePool() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptxpool, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved10<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved10()
        }
        unsafe extern "system" fn Reserved11<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved11()
        }
        unsafe extern "system" fn Reserved12<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved12()
        }
        unsafe extern "system" fn Reserved13<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved13()
        }
        unsafe extern "system" fn Reserved14<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved14()
        }
        unsafe extern "system" fn Reserved15<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved15()
        }
        unsafe extern "system" fn Reserved16<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved16()
        }
        unsafe extern "system" fn Reserved17<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved17()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            Reserved3: Reserved3::<Identity, Impl, OFFSET>,
            Reserved4: Reserved4::<Identity, Impl, OFFSET>,
            Reserved5: Reserved5::<Identity, Impl, OFFSET>,
            Reserved6: Reserved6::<Identity, Impl, OFFSET>,
            Reserved7: Reserved7::<Identity, Impl, OFFSET>,
            Reserved8: Reserved8::<Identity, Impl, OFFSET>,
            Reserved9: Reserved9::<Identity, Impl, OFFSET>,
            GetTransactionResourcePool: GetTransactionResourcePool::<Identity, Impl, OFFSET>,
            Reserved10: Reserved10::<Identity, Impl, OFFSET>,
            Reserved11: Reserved11::<Identity, Impl, OFFSET>,
            Reserved12: Reserved12::<Identity, Impl, OFFSET>,
            Reserved13: Reserved13::<Identity, Impl, OFFSET>,
            Reserved14: Reserved14::<Identity, Impl, OFFSET>,
            Reserved15: Reserved15::<Identity, Impl, OFFSET>,
            Reserved16: Reserved16::<Identity, Impl, OFFSET>,
            Reserved17: Reserved17::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait ITransactionProxy_Impl: Sized {
    fn Commit(&self, guid: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Abort(&self) -> ::windows::core::Result<()>;
    fn Promote(&self) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransaction>;
    fn CreateVoter(&self, ptxasync: &::core::option::Option<super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>) -> ::windows::core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>;
    fn GetIsolationLevel(&self, __midl__itransactionproxy0000: *mut i32) -> ::windows::core::Result<()>;
    fn GetIdentifier(&self, pbstridentifier: *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn IsReusable(&self, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ::windows::core::RuntimeName for ITransactionProxy {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ITransactionProxy_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>() -> ITransactionProxy_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute(&guid)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn Promote<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Promote() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVoter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxasync: *mut ::core::ffi::c_void, ppballot: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateVoter(::core::mem::transmute(&ptxasync)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppballot, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsolationLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIsolationLevel(::core::mem::transmute_copy(&__midl__itransactionproxy0000)).into()
        }
        unsafe extern "system" fn GetIdentifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstridentifier: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIdentifier(::core::mem::transmute_copy(&pbstridentifier)).into()
        }
        unsafe extern "system" fn IsReusable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsReusable(::core::mem::transmute_copy(&pfisreusable)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            Promote: Promote::<Identity, Impl, OFFSET>,
            CreateVoter: CreateVoter::<Identity, Impl, OFFSET>,
            GetIsolationLevel: GetIsolationLevel::<Identity, Impl, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET>,
            IsReusable: IsReusable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionProxy as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionResourcePool_Impl: Sized {
    fn PutResource(&self, ppool: &::core::option::Option<IObjPool>, punk: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn GetResource(&self, ppool: &::core::option::Option<IObjPool>) -> ::windows::core::Result<::windows::core::IUnknown>;
}
impl ::windows::core::RuntimeName for ITransactionResourcePool {}
impl ITransactionResourcePool_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: isize>() -> ITransactionResourcePool_Vtbl {
        unsafe extern "system" fn PutResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutResource(::core::mem::transmute(&ppool), ::core::mem::transmute(&punk)).into()
        }
        unsafe extern "system" fn GetResource<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResource(::core::mem::transmute(&ppool)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            PutResource: PutResource::<Identity, Impl, OFFSET>,
            GetResource: GetResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionResourcePool as ::windows::core::Interface>::IID
    }
}
pub trait ITransactionStatus_Impl: Sized {
    fn SetTransactionStatus(&self, hrstatus: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn GetTransactionStatus(&self, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ITransactionStatus {}
impl ITransactionStatus_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: isize>() -> ITransactionStatus_Vtbl {
        unsafe extern "system" fn SetTransactionStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransactionStatus(::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn GetTransactionStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTransactionStatus(::core::mem::transmute_copy(&phrstatus)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetTransactionStatus: SetTransactionStatus::<Identity, Impl, OFFSET>,
            GetTransactionStatus: GetTransactionStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITransactionStatus as ::windows::core::Interface>::IID
    }
}
pub trait ITxProxyHolder_Impl: Sized {
    fn GetIdentifier(&self, pguidltx: *mut ::windows::core::GUID);
}
impl ::windows::core::RuntimeName for ITxProxyHolder {}
impl ITxProxyHolder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITxProxyHolder_Impl, const OFFSET: isize>() -> ITxProxyHolder_Vtbl {
        unsafe extern "system" fn GetIdentifier<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITxProxyHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidltx: *mut ::windows::core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIdentifier(::core::mem::transmute_copy(&pguidltx))
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITxProxyHolder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ObjectContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateInstance(&self, bstrprogid: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn SetComplete(&self) -> ::windows::core::Result<()>;
    fn SetAbort(&self) -> ::windows::core::Result<()>;
    fn EnableCommit(&self) -> ::windows::core::Result<()>;
    fn DisableCommit(&self) -> ::windows::core::Result<()>;
    fn IsInTransaction(&self) -> ::windows::core::Result<i16>;
    fn IsSecurityEnabled(&self) -> ::windows::core::Result<i16>;
    fn IsCallerInRole(&self, bstrrole: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn get_Item(&self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::VARIANT>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Security(&self) -> ::windows::core::Result<SecurityProperty>;
    fn ContextInfo(&self) -> ::windows::core::Result<ContextInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ObjectContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ObjectContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>() -> ObjectContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprogid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pobject: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(::core::mem::transmute(&bstrprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComplete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetComplete().into()
        }
        unsafe extern "system" fn SetAbort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAbort().into()
        }
        unsafe extern "system" fn EnableCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableCommit().into()
        }
        unsafe extern "system" fn DisableCommit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableCommit().into()
        }
        unsafe extern "system" fn IsInTransaction<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisintx, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSecurityEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCallerInRole(::core::mem::transmute(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbinrole, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pitem: *mut super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityproperty: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityproperty, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontextinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContextInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontextinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            SetComplete: SetComplete::<Identity, Impl, OFFSET>,
            SetAbort: SetAbort::<Identity, Impl, OFFSET>,
            EnableCommit: EnableCommit::<Identity, Impl, OFFSET>,
            DisableCommit: DisableCommit::<Identity, Impl, OFFSET>,
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            IsSecurityEnabled: IsSecurityEnabled::<Identity, Impl, OFFSET>,
            IsCallerInRole: IsCallerInRole::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Security: Security::<Identity, Impl, OFFSET>,
            ContextInfo: ContextInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ObjectContext as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait ObjectControl_Impl: Sized {
    fn Activate(&self) -> ::windows::core::Result<()>;
    fn Deactivate(&self) -> ::windows::core::Result<()>;
    fn CanBePooled(&self, pbpoolable: *mut i16) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for ObjectControl {}
impl ObjectControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>() -> ObjectControl_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate().into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate().into()
        }
        unsafe extern "system" fn CanBePooled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpoolable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanBePooled(::core::mem::transmute_copy(&pbpoolable)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CanBePooled: CanBePooled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ObjectControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait SecurityProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetDirectCallerName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetDirectCreatorName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetOriginalCallerName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetOriginalCreatorName(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for SecurityProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl SecurityProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>() -> SecurityProperty_Vtbl {
        unsafe extern "system" fn GetDirectCallerName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDirectCallerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirectCreatorName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDirectCreatorName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCallerName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOriginalCallerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCreatorName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOriginalCreatorName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDirectCallerName: GetDirectCallerName::<Identity, Impl, OFFSET>,
            GetDirectCreatorName: GetDirectCreatorName::<Identity, Impl, OFFSET>,
            GetOriginalCallerName: GetOriginalCallerName::<Identity, Impl, OFFSET>,
            GetOriginalCreatorName: GetOriginalCreatorName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<SecurityProperty as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
