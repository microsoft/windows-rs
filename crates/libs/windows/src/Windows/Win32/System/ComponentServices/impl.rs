#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ContextInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn IsInTransaction(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetTransaction(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetTransactionId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetActivityId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetContextId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ContextInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ContextInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>() -> ContextInfo_Vtbl {
        unsafe extern "system" fn IsInTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisintx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptx: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtxid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransactionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtxid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActivityId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstractivityid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivityId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstractivityid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContextId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrctxid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetContextId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrctxid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ContextInfo as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ContextInfo2_Impl: Sized + ContextInfo_Impl {
    fn GetPartitionId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetApplicationId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetApplicationInstanceId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ContextInfo2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ContextInfo2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>() -> ContextInfo2_Vtbl {
        unsafe extern "system" fn GetPartitionId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20000: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartitionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__contextinfo20000, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20001: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplicationId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__contextinfo20001, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__contextinfo20002: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplicationInstanceId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__contextinfo20002, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ContextInfo2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ContextInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAppDomainHelper_Impl: Sized + super::Com::IDispatch_Impl {
    fn Initialize(&self, punkad: ::core::option::Option<&::windows_core::IUnknown>, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn DoCallback(&self, punkad: ::core::option::Option<&::windows_core::IUnknown>, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAppDomainHelper {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAppDomainHelper_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: isize>() -> IAppDomainHelper_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0000: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&punkad), ::core::mem::transmute_copy(&__midl__iappdomainhelper0000), ::core::mem::transmute_copy(&ppool)).into()
        }
        unsafe extern "system" fn DoCallback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAppDomainHelper_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkad: *mut ::core::ffi::c_void, __midl__iappdomainhelper0001: isize, ppool: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DoCallback(::windows_core::from_raw_borrowed(&punkad), ::core::mem::transmute_copy(&__midl__iappdomainhelper0001), ::core::mem::transmute_copy(&ppool)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            DoCallback: DoCallback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAppDomainHelper as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IAssemblyLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetModules(&self, applicationdir: &::windows_core::BSTR, applicationname: &::windows_core::BSTR, assemblyname: &::windows_core::BSTR) -> ::windows_core::Result<*mut super::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IAssemblyLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IAssemblyLocator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyLocator_Impl, const OFFSET: isize>() -> IAssemblyLocator_Vtbl {
        unsafe extern "system" fn GetModules<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAssemblyLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationdir: ::std::mem::MaybeUninit<::windows_core::BSTR>, applicationname: ::std::mem::MaybeUninit<::windows_core::BSTR>, assemblyname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pmodules: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetModules(::core::mem::transmute(&applicationdir), ::core::mem::transmute(&applicationname), ::core::mem::transmute(&assemblyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pmodules, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetModules: GetModules::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAssemblyLocator as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IAsyncErrorNotify_Impl: Sized {
    fn OnError(&self, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IAsyncErrorNotify {}
impl IAsyncErrorNotify_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncErrorNotify_Impl, const OFFSET: isize>() -> IAsyncErrorNotify_Vtbl {
        unsafe extern "system" fn OnError<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IAsyncErrorNotify_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnError(::core::mem::transmute_copy(&hr)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnError: OnError::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IAsyncErrorNotify as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICOMAdminCatalog_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetCollection(&self, bstrcollname: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Connect(&self, bstrcatalogservername: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn MajorVersion(&self) -> ::windows_core::Result<i32>;
    fn MinorVersion(&self) -> ::windows_core::Result<i32>;
    fn GetCollectionByQuery(&self, bstrcollname: &::windows_core::BSTR, ppsavarquery: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<super::Com::IDispatch>;
    fn ImportComponent(&self, bstrapplidorname: &::windows_core::BSTR, bstrclsidorprogid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn InstallComponent(&self, bstrapplidorname: &::windows_core::BSTR, bstrdll: &::windows_core::BSTR, bstrtlb: &::windows_core::BSTR, bstrpsdll: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ShutdownApplication(&self, bstrapplidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ExportApplication(&self, bstrapplidorname: &::windows_core::BSTR, bstrapplicationfile: &::windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> ::windows_core::Result<()>;
    fn InstallApplication(&self, bstrapplicationfile: &::windows_core::BSTR, bstrdestinationdirectory: &::windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, bstrrsn: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn StopRouter(&self) -> ::windows_core::Result<()>;
    fn RefreshRouter(&self) -> ::windows_core::Result<()>;
    fn StartRouter(&self) -> ::windows_core::Result<()>;
    fn Reserved1(&self) -> ::windows_core::Result<()>;
    fn Reserved2(&self) -> ::windows_core::Result<()>;
    fn InstallMultipleComponents(&self, bstrapplidorname: &::windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn GetMultipleComponentsInfo(&self, bstrapplidorname: &::windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn RefreshComponents(&self) -> ::windows_core::Result<()>;
    fn BackupREGDB(&self, bstrbackupfilepath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RestoreREGDB(&self, bstrbackupfilepath: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn QueryApplicationFile(&self, bstrapplicationfile: &::windows_core::BSTR, pbstrapplicationname: *mut ::windows_core::BSTR, pbstrapplicationdescription: *mut ::windows_core::BSTR, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn StartApplication(&self, bstrapplidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ServiceCheck(&self, lservice: i32) -> ::windows_core::Result<i32>;
    fn InstallMultipleEventClasses(&self, bstrapplidorname: &::windows_core::BSTR, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn InstallEventClass(&self, bstrapplidorname: &::windows_core::BSTR, bstrdll: &::windows_core::BSTR, bstrtlb: &::windows_core::BSTR, bstrpsdll: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetEventClassesForIID(&self, bstriid: &::windows_core::BSTR, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICOMAdminCatalog {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICOMAdminCatalog_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>() -> ICOMAdminCatalog_Vtbl {
        unsafe extern "system" fn GetCollection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollection(::core::mem::transmute(&bstrcollname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Connect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcatalogservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Connect(::core::mem::transmute(&bstrcatalogservername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MajorVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MinorVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversion: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollectionByQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarquery: *const *const super::Com::SAFEARRAY, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollectionByQuery(::core::mem::transmute(&bstrcollname), ::core::mem::transmute_copy(&ppsavarquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsidorprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportComponent(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrclsidorprogid)).into()
        }
        unsafe extern "system" fn InstallComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdll: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtlb: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpsdll: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallComponent(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrdll), ::core::mem::transmute(&bstrtlb), ::core::mem::transmute(&bstrpsdll)).into()
        }
        unsafe extern "system" fn ShutdownApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownApplication(::core::mem::transmute(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ExportApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrapplicationfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportApplication(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrapplicationfile), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdestinationdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrsn: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallApplication(::core::mem::transmute(&bstrapplicationfile), ::core::mem::transmute(&bstrdestinationdirectory), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute(&bstruserid), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute(&bstrrsn)).into()
        }
        unsafe extern "system" fn StopRouter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopRouter().into()
        }
        unsafe extern "system" fn RefreshRouter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshRouter().into()
        }
        unsafe extern "system" fn StartRouter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartRouter().into()
        }
        unsafe extern "system" fn Reserved1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved1().into()
        }
        unsafe extern "system" fn Reserved2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved2().into()
        }
        unsafe extern "system" fn InstallMultipleComponents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallMultipleComponents(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn GetMultipleComponentsInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarclassnames: *mut *mut super::Com::SAFEARRAY, ppsavarfileflags: *mut *mut super::Com::SAFEARRAY, ppsavarcomponentflags: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMultipleComponentsInfo(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids), ::core::mem::transmute_copy(&ppsavarclassnames), ::core::mem::transmute_copy(&ppsavarfileflags), ::core::mem::transmute_copy(&ppsavarcomponentflags)).into()
        }
        unsafe extern "system" fn RefreshComponents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RefreshComponents().into()
        }
        unsafe extern "system" fn BackupREGDB<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BackupREGDB(::core::mem::transmute(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn RestoreREGDB<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbackupfilepath: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreREGDB(::core::mem::transmute(&bstrbackupfilepath)).into()
        }
        unsafe extern "system" fn QueryApplicationFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrapplicationname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrapplicationdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbhasusers: *mut super::super::Foundation::VARIANT_BOOL, pbisproxy: *mut super::super::Foundation::VARIANT_BOOL, ppsavarfilenames: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryApplicationFile(::core::mem::transmute(&bstrapplicationfile), ::core::mem::transmute_copy(&pbstrapplicationname), ::core::mem::transmute_copy(&pbstrapplicationdescription), ::core::mem::transmute_copy(&pbhasusers), ::core::mem::transmute_copy(&pbisproxy), ::core::mem::transmute_copy(&ppsavarfilenames)).into()
        }
        unsafe extern "system" fn StartApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartApplication(::core::mem::transmute(&bstrapplidorname)).into()
        }
        unsafe extern "system" fn ServiceCheck<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lservice: i32, plstatus: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ServiceCheck(::core::mem::transmute_copy(&lservice)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstallMultipleEventClasses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarfilenames: *const *const super::Com::SAFEARRAY, ppsavarclsids: *const *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallMultipleEventClasses(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute_copy(&ppsavarfilenames), ::core::mem::transmute_copy(&ppsavarclsids)).into()
        }
        unsafe extern "system" fn InstallEventClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdll: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrtlb: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpsdll: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallEventClass(::core::mem::transmute(&bstrapplidorname), ::core::mem::transmute(&bstrdll), ::core::mem::transmute(&bstrtlb), ::core::mem::transmute(&bstrpsdll)).into()
        }
        unsafe extern "system" fn GetEventClassesForIID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstriid: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppsavarclsids: *mut *mut super::Com::SAFEARRAY, ppsavarprogids: *mut *mut super::Com::SAFEARRAY, ppsavardescriptions: *mut *mut super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICOMAdminCatalog as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICOMAdminCatalog2_Impl: Sized + ICOMAdminCatalog_Impl {
    fn GetCollectionByQuery2(&self, bstrcollectionname: &::windows_core::BSTR, pvarquerystrings: *const super::Variant::VARIANT) -> ::windows_core::Result<super::Com::IDispatch>;
    fn GetApplicationInstanceIDFromProcessID(&self, lprocessid: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ShutdownApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PauseApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ResumeApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RecycleApplicationInstances(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT, lreasoncode: i32) -> ::windows_core::Result<()>;
    fn AreApplicationInstancesPaused(&self, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn DumpApplicationInstance(&self, bstrapplicationinstanceid: &::windows_core::BSTR, bstrdirectory: &::windows_core::BSTR, lmaximages: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IsApplicationInstanceDumpSupported(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CreateServiceForApplication(&self, bstrapplicationidorname: &::windows_core::BSTR, bstrservicename: &::windows_core::BSTR, bstrstarttype: &::windows_core::BSTR, bstrerrorcontrol: &::windows_core::BSTR, bstrdependencies: &::windows_core::BSTR, bstrrunas: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DeleteServiceForApplication(&self, bstrapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetPartitionID(&self, bstrapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPartitionName(&self, bstrapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetCurrentPartition(&self, bstrpartitionidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CurrentPartitionID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CurrentPartitionName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GlobalPartitionID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FlushPartitionCache(&self) -> ::windows_core::Result<()>;
    fn CopyApplications(&self, bstrsourcepartitionidorname: &::windows_core::BSTR, pvarapplicationid: *const super::Variant::VARIANT, bstrdestinationpartitionidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CopyComponents(&self, bstrsourceapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MoveComponents(&self, bstrsourceapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AliasComponent(&self, bstrsrcapplicationidorname: &::windows_core::BSTR, bstrclsidorprogid: &::windows_core::BSTR, bstrdestapplicationidorname: &::windows_core::BSTR, bstrnewprogid: &::windows_core::BSTR, bstrnewclsid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsSafeToDelete(&self, bstrdllname: &::windows_core::BSTR) -> ::windows_core::Result<COMAdminInUse>;
    fn ImportUnconfiguredComponents(&self, bstrapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn PromoteUnconfiguredComponents(&self, bstrapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ImportComponents(&self, bstrapplicationidorname: &::windows_core::BSTR, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Is64BitCatalogServer(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExportPartition(&self, bstrpartitionidorname: &::windows_core::BSTR, bstrpartitionfilename: &::windows_core::BSTR, loptions: COMAdminApplicationExportOptions) -> ::windows_core::Result<()>;
    fn InstallPartition(&self, bstrfilename: &::windows_core::BSTR, bstrdestdirectory: &::windows_core::BSTR, loptions: COMAdminApplicationInstallOptions, bstruserid: &::windows_core::BSTR, bstrpassword: &::windows_core::BSTR, bstrrsn: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn QueryApplicationFile2(&self, bstrapplicationfile: &::windows_core::BSTR) -> ::windows_core::Result<super::Com::IDispatch>;
    fn GetComponentVersionCount(&self, bstrclsidorprogid: &::windows_core::BSTR) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICOMAdminCatalog2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICOMAdminCatalog2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>() -> ICOMAdminCatalog2_Vtbl {
        unsafe extern "system" fn GetCollectionByQuery2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollectionname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarquerystrings: *const super::Variant::VARIANT, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollectionByQuery2(::core::mem::transmute(&bstrcollectionname), ::core::mem::transmute_copy(&pvarquerystrings)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetApplicationInstanceIDFromProcessID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprocessid: i32, pbstrapplicationinstanceid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetApplicationInstanceIDFromProcessID(::core::mem::transmute_copy(&lprocessid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationinstanceid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ShutdownApplicationInstances<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn PauseApplicationInstances<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PauseApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn ResumeApplicationInstances<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResumeApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid)).into()
        }
        unsafe extern "system" fn RecycleApplicationInstances<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT, lreasoncode: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecycleApplicationInstances(::core::mem::transmute_copy(&pvarapplicationinstanceid), ::core::mem::transmute_copy(&lreasoncode)).into()
        }
        unsafe extern "system" fn AreApplicationInstancesPaused<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarapplicationinstanceid: *const super::Variant::VARIANT, pvarboolpaused: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AreApplicationInstancesPaused(::core::mem::transmute_copy(&pvarapplicationinstanceid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarboolpaused, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DumpApplicationInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationinstanceid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, lmaximages: i32, pbstrdumpfile: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DumpApplicationInstance(::core::mem::transmute(&bstrapplicationinstanceid), ::core::mem::transmute(&bstrdirectory), ::core::mem::transmute_copy(&lmaximages)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdumpfile, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsApplicationInstanceDumpSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbooldumpsupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsApplicationInstanceDumpSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbooldumpsupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateServiceForApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrservicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrstarttype: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrerrorcontrol: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdependencies: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrunas: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bdesktopok: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateServiceForApplication(::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute(&bstrservicename), ::core::mem::transmute(&bstrstarttype), ::core::mem::transmute(&bstrerrorcontrol), ::core::mem::transmute(&bstrdependencies), ::core::mem::transmute(&bstrrunas), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute_copy(&bdesktopok)).into()
        }
        unsafe extern "system" fn DeleteServiceForApplication<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteServiceForApplication(::core::mem::transmute(&bstrapplicationidorname)).into()
        }
        unsafe extern "system" fn GetPartitionID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrpartitionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartitionID(::core::mem::transmute(&bstrapplicationidorname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPartitionName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrpartitionname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPartitionName(::core::mem::transmute(&bstrapplicationidorname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCurrentPartition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCurrentPartition(::core::mem::transmute(&bstrpartitionidorname)).into()
        }
        unsafe extern "system" fn CurrentPartitionID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPartitionID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPartitionName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpartitionname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPartitionName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpartitionname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GlobalPartitionID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrglobalpartitionid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GlobalPartitionID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrglobalpartitionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushPartitionCache<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FlushPartitionCache().into()
        }
        unsafe extern "system" fn CopyApplications<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourcepartitionidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarapplicationid: *const super::Variant::VARIANT, bstrdestinationpartitionidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyApplications(::core::mem::transmute(&bstrsourcepartitionidorname), ::core::mem::transmute_copy(&pvarapplicationid), ::core::mem::transmute(&bstrdestinationpartitionidorname)).into()
        }
        unsafe extern "system" fn CopyComponents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CopyComponents(::core::mem::transmute(&bstrsourceapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn MoveComponents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsourceapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, bstrdestinationapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveComponents(::core::mem::transmute(&bstrsourceapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute(&bstrdestinationapplicationidorname)).into()
        }
        unsafe extern "system" fn AliasComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsrcapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsidorprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdestapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnewprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrnewclsid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AliasComponent(::core::mem::transmute(&bstrsrcapplicationidorname), ::core::mem::transmute(&bstrclsidorprogid), ::core::mem::transmute(&bstrdestapplicationidorname), ::core::mem::transmute(&bstrnewprogid), ::core::mem::transmute(&bstrnewclsid)).into()
        }
        unsafe extern "system" fn IsSafeToDelete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdllname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcomadmininuse: *mut COMAdminInUse) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSafeToDelete(::core::mem::transmute(&bstrdllname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcomadmininuse, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportUnconfiguredComponents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportUnconfiguredComponents(::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn PromoteUnconfiguredComponents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PromoteUnconfiguredComponents(::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn ImportComponents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarclsidorprogid: *const super::Variant::VARIANT, pvarcomponenttype: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportComponents(::core::mem::transmute(&bstrapplicationidorname), ::core::mem::transmute_copy(&pvarclsidorprogid), ::core::mem::transmute_copy(&pvarcomponenttype)).into()
        }
        unsafe extern "system" fn Is64BitCatalogServer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbis64bit: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Is64BitCatalogServer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbis64bit, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExportPartition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpartitionidorname: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpartitionfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: COMAdminApplicationExportOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ExportPartition(::core::mem::transmute(&bstrpartitionidorname), ::core::mem::transmute(&bstrpartitionfilename), ::core::mem::transmute_copy(&loptions)).into()
        }
        unsafe extern "system" fn InstallPartition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfilename: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrdestdirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, loptions: COMAdminApplicationInstallOptions, bstruserid: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrpassword: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrrsn: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InstallPartition(::core::mem::transmute(&bstrfilename), ::core::mem::transmute(&bstrdestdirectory), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute(&bstruserid), ::core::mem::transmute(&bstrpassword), ::core::mem::transmute(&bstrrsn)).into()
        }
        unsafe extern "system" fn QueryApplicationFile2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppfilesforimport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryApplicationFile2(::core::mem::transmute(&bstrapplicationfile)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppfilesforimport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetComponentVersionCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMAdminCatalog2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrclsidorprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, plversioncount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetComponentVersionCount(::core::mem::transmute(&bstrclsidorprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plversioncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICOMAdminCatalog2 as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ICOMAdminCatalog as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ICOMLBArguments_Impl: Sized {
    fn GetCLSID(&self, pclsid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn SetCLSID(&self, pclsid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetMachineName(&self, cchsvr: u32, szservername: ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetMachineName(&self, cchsvr: u32, szservername: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICOMLBArguments {}
impl ICOMLBArguments_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>() -> ICOMLBArguments_Vtbl {
        unsafe extern "system" fn GetCLSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCLSID(::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn SetCLSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclsid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCLSID(::core::mem::transmute_copy(&pclsid)).into()
        }
        unsafe extern "system" fn GetMachineName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMachineName(::core::mem::transmute_copy(&cchsvr), ::core::mem::transmute_copy(&szservername)).into()
        }
        unsafe extern "system" fn SetMachineName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICOMLBArguments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cchsvr: u32, szservername: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMachineName(::core::mem::transmute_copy(&cchsvr), ::core::mem::transmute(&szservername)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCLSID: GetCLSID::<Identity, Impl, OFFSET>,
            SetCLSID: SetCLSID::<Identity, Impl, OFFSET>,
            GetMachineName: GetMachineName::<Identity, Impl, OFFSET>,
            SetMachineName: SetMachineName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICOMLBArguments as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalogCollection_Impl: Sized + super::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn get_Item(&self, lindex: i32) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Remove(&self, lindex: i32) -> ::windows_core::Result<()>;
    fn Add(&self) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Populate(&self) -> ::windows_core::Result<()>;
    fn SaveChanges(&self) -> ::windows_core::Result<i32>;
    fn GetCollection(&self, bstrcollname: &::windows_core::BSTR, varobjectkey: &super::Variant::VARIANT) -> ::windows_core::Result<super::Com::IDispatch>;
    fn Name(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn AddEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn RemoveEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetUtilInterface(&self) -> ::windows_core::Result<super::Com::IDispatch>;
    fn DataStoreMajorVersion(&self) -> ::windows_core::Result<i32>;
    fn DataStoreMinorVersion(&self) -> ::windows_core::Result<i32>;
    fn PopulateByKey(&self, psakeys: *const super::Com::SAFEARRAY) -> ::windows_core::Result<()>;
    fn PopulateByQuery(&self, bstrquerystring: &::windows_core::BSTR, lquerytype: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICatalogCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICatalogCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>() -> ICatalogCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plobjectcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plobjectcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute_copy(&lindex)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcatalogobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Add() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Populate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Populate().into()
        }
        unsafe extern "system" fn SaveChanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcchanges: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SaveChanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcchanges, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCollection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcollname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varobjectkey: super::Variant::VARIANT, ppcatalogcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCollection(::core::mem::transmute(&bstrcollname), ::core::mem::transmute(&varobjectkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcatalogcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarnamel: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarnamel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbool, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemoveEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarbool, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUtilInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppidispatch: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUtilInterface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppidispatch, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMajorVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataStoreMajorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmajorversion, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataStoreMinorVersion<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plminorversionl: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataStoreMinorVersion() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plminorversionl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PopulateByKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psakeys: *const super::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PopulateByKey(::core::mem::transmute_copy(&psakeys)).into()
        }
        unsafe extern "system" fn PopulateByQuery<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrquerystring: ::std::mem::MaybeUninit<::windows_core::BSTR>, lquerytype: i32) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICatalogCollection as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICatalogObject_Impl: Sized + super::Com::IDispatch_Impl {
    fn get_Value(&self, bstrpropname: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn put_Value(&self, bstrpropname: &::windows_core::BSTR, val: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn Key(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Name(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn IsPropertyReadOnly(&self, bstrpropname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Valid(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsPropertyWriteOnly(&self, bstrpropname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICatalogObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICatalogObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>() -> ICatalogObject_Vtbl {
        unsafe extern "system" fn get_Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pvarretval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Value(::core::mem::transmute(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, val: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_Value(::core::mem::transmute(&bstrpropname), ::core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn Key<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Key() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarretval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyReadOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPropertyReadOnly(::core::mem::transmute(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Valid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Valid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertyWriteOnly<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICatalogObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbretval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsPropertyWriteOnly(::core::mem::transmute(&bstrpropname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbretval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICatalogObject as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ICheckSxsConfig_Impl: Sized {
    fn IsSameSxsConfig(&self, wszsxsname: &::windows_core::PCWSTR, wszsxsdirectory: &::windows_core::PCWSTR, wszsxsappname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICheckSxsConfig {}
impl ICheckSxsConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICheckSxsConfig_Impl, const OFFSET: isize>() -> ICheckSxsConfig_Vtbl {
        unsafe extern "system" fn IsSameSxsConfig<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICheckSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, wszsxsname: ::windows_core::PCWSTR, wszsxsdirectory: ::windows_core::PCWSTR, wszsxsappname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSameSxsConfig(::core::mem::transmute(&wszsxsname), ::core::mem::transmute(&wszsxsdirectory), ::core::mem::transmute(&wszsxsappname)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IsSameSxsConfig: IsSameSxsConfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICheckSxsConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComActivityEvents_Impl: Sized {
    fn OnActivityCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnActivityDestroy(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnActivityEnter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32) -> ::windows_core::Result<()>;
    fn OnActivityTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows_core::Result<()>;
    fn OnActivityReenter(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows_core::Result<()>;
    fn OnActivityLeave(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidleft: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnActivityLeaveSame(&self, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwcalldepth: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComActivityEvents {}
impl IComActivityEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>() -> IComActivityEvents_Vtbl {
        unsafe extern "system" fn OnActivityCreate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityCreate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityDestroy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityDestroy(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity)).into()
        }
        unsafe extern "system" fn OnActivityEnter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityEnter(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidentered), ::core::mem::transmute_copy(&dwthread)).into()
        }
        unsafe extern "system" fn OnActivityTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidentered: *const ::windows_core::GUID, dwthread: u32, dwtimeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityTimeout(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidentered), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnActivityReenter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwthread: u32, dwcalldepth: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityReenter(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwcalldepth)).into()
        }
        unsafe extern "system" fn OnActivityLeave<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, guidleft: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityLeave(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&guidleft)).into()
        }
        unsafe extern "system" fn OnActivityLeaveSame<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComActivityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidcurrent: *const ::windows_core::GUID, dwcalldepth: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnActivityLeaveSame(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidcurrent), ::core::mem::transmute_copy(&dwcalldepth)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnActivityCreate: OnActivityCreate::<Identity, Impl, OFFSET>,
            OnActivityDestroy: OnActivityDestroy::<Identity, Impl, OFFSET>,
            OnActivityEnter: OnActivityEnter::<Identity, Impl, OFFSET>,
            OnActivityTimeout: OnActivityTimeout::<Identity, Impl, OFFSET>,
            OnActivityReenter: OnActivityReenter::<Identity, Impl, OFFSET>,
            OnActivityLeave: OnActivityLeave::<Identity, Impl, OFFSET>,
            OnActivityLeaveSame: OnActivityLeaveSame::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComActivityEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComApp2Events_Impl: Sized {
    fn OnAppActivation2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID, guidprocess: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppForceShutdown2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppPaused2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnAppRecycle2(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID, guidprocess: &::windows_core::GUID, lreason: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IComApp2Events {}
#[cfg(feature = "Win32_Foundation")]
impl IComApp2Events_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>() -> IComApp2Events_Vtbl {
        unsafe extern "system" fn OnAppActivation2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID, guidprocess: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppActivation2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp), ::core::mem::transmute(&guidprocess)).into()
        }
        unsafe extern "system" fn OnAppShutdown2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppShutdown2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppForceShutdown2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppPaused2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID, bpaused: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppPaused2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp), ::core::mem::transmute_copy(&bpaused)).into()
        }
        unsafe extern "system" fn OnAppRecycle2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComApp2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID, guidprocess: ::windows_core::GUID, lreason: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppRecycle2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp), ::core::mem::transmute(&guidprocess), ::core::mem::transmute_copy(&lreason)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAppActivation2: OnAppActivation2::<Identity, Impl, OFFSET>,
            OnAppShutdown2: OnAppShutdown2::<Identity, Impl, OFFSET>,
            OnAppForceShutdown2: OnAppForceShutdown2::<Identity, Impl, OFFSET>,
            OnAppPaused2: OnAppPaused2::<Identity, Impl, OFFSET>,
            OnAppRecycle2: OnAppRecycle2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComApp2Events as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComAppEvents_Impl: Sized {
    fn OnAppActivation(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnAppForceShutdown(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComAppEvents {}
impl IComAppEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>() -> IComAppEvents_Vtbl {
        unsafe extern "system" fn OnAppActivation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppActivation(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppShutdown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppShutdown(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnAppForceShutdown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComAppEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAppForceShutdown(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAppActivation: OnAppActivation::<Identity, Impl, OFFSET>,
            OnAppShutdown: OnAppShutdown::<Identity, Impl, OFFSET>,
            OnAppForceShutdown: OnAppForceShutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComAppEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComCRMEvents_Impl: Sized {
    fn OnCRMRecoveryStart(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMRecoveryDone(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMCheckpoint(&self, pinfo: *const COMSVCSEVENTINFO, guidapp: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMBegin(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID, guidactivity: &::windows_core::GUID, guidtx: &::windows_core::GUID, szprogidcompensator: &::windows_core::PCWSTR, szdescription: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn OnCRMPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMIndoubt(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMDone(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMRelease(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMAnalyze(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows_core::Result<()>;
    fn OnCRMWrite(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_core::Result<()>;
    fn OnCRMForget(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMForce(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnCRMDeliver(&self, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: &::windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IComCRMEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComCRMEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>() -> IComCRMEvents_Vtbl {
        unsafe extern "system" fn OnCRMRecoveryStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMRecoveryStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMRecoveryDone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMRecoveryDone(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMCheckpoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidapp: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMCheckpoint(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidapp)).into()
        }
        unsafe extern "system" fn OnCRMBegin<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, guidactivity: ::windows_core::GUID, guidtx: ::windows_core::GUID, szprogidcompensator: ::windows_core::PCWSTR, szdescription: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMBegin(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute(&guidactivity), ::core::mem::transmute(&guidtx), ::core::mem::transmute(&szprogidcompensator), ::core::mem::transmute(&szdescription)).into()
        }
        unsafe extern "system" fn OnCRMPrepare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMPrepare(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMIndoubt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMIndoubt(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMDone(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMRelease<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMRelease(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMAnalyze<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, dwcrmrecordtype: u32, dwrecordsize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMAnalyze(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute_copy(&dwcrmrecordtype), ::core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMWrite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMWrite(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute_copy(&fvariants), ::core::mem::transmute_copy(&dwrecordsize)).into()
        }
        unsafe extern "system" fn OnCRMForget<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMForget(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMForce<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMForce(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid)).into()
        }
        unsafe extern "system" fn OnCRMDeliver<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComCRMEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidclerkclsid: ::windows_core::GUID, fvariants: super::super::Foundation::BOOL, dwrecordsize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCRMDeliver(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidclerkclsid), ::core::mem::transmute_copy(&fvariants), ::core::mem::transmute_copy(&dwrecordsize)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComCRMEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComExceptionEvents_Impl: Sized {
    fn OnExceptionUser(&self, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComExceptionEvents {}
impl IComExceptionEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComExceptionEvents_Impl, const OFFSET: isize>() -> IComExceptionEvents_Vtbl {
        unsafe extern "system" fn OnExceptionUser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComExceptionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, code: u32, address: u64, pszstacktrace: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnExceptionUser(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&code), ::core::mem::transmute_copy(&address), ::core::mem::transmute(&pszstacktrace)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnExceptionUser: OnExceptionUser::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComExceptionEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComIdentityEvents_Impl: Sized {
    fn OnIISRequestInfo(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: &::windows_core::PCWSTR, pszserverip: &::windows_core::PCWSTR, pszurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComIdentityEvents {}
impl IComIdentityEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComIdentityEvents_Impl, const OFFSET: isize>() -> IComIdentityEvents_Vtbl {
        unsafe extern "system" fn OnIISRequestInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComIdentityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, pszclientip: ::windows_core::PCWSTR, pszserverip: ::windows_core::PCWSTR, pszurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnIISRequestInfo(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute(&pszclientip), ::core::mem::transmute(&pszserverip), ::core::mem::transmute(&pszurl)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnIISRequestInfo: OnIISRequestInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComIdentityEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComInstance2Events_Impl: Sized {
    fn OnObjectCreate2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnObjectDestroy2(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComInstance2Events {}
impl IComInstance2Events_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: isize>() -> IComInstance2Events_Vtbl {
        unsafe extern "system" fn OnObjectCreate2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectCreate2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjectDestroy2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstance2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectDestroy2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectCreate2: OnObjectCreate2::<Identity, Impl, OFFSET>,
            OnObjectDestroy2: OnObjectDestroy2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComInstance2Events as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComInstanceEvents_Impl: Sized {
    fn OnObjectCreate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64) -> ::windows_core::Result<()>;
    fn OnObjectDestroy(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComInstanceEvents {}
impl IComInstanceEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: isize>() -> IComInstanceEvents_Vtbl {
        unsafe extern "system" fn OnObjectCreate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, clsid: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, ctxtid: u64, objectid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectCreate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDestroy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComInstanceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectDestroy(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectCreate: OnObjectCreate::<Identity, Impl, OFFSET>,
            OnObjectDestroy: OnObjectDestroy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComInstanceEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComLTxEvents_Impl: Sized {
    fn OnLtxTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID, tsid: &::windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_core::Result<()>;
    fn OnLtxTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnLtxTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnLtxTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnLtxTransactionPromote(&self, pinfo: *const COMSVCSEVENTINFO, guidltx: &::windows_core::GUID, txnid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IComLTxEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComLTxEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>() -> IComLTxEvents_Vtbl {
        unsafe extern "system" fn OnLtxTransactionStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID, tsid: ::windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx), ::core::mem::transmute(&tsid), ::core::mem::transmute_copy(&froot), ::core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPrepare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID, fvote: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionPrepare(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx), ::core::mem::transmute_copy(&fvote)).into()
        }
        unsafe extern "system" fn OnLtxTransactionAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx)).into()
        }
        unsafe extern "system" fn OnLtxTransactionPromote<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComLTxEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidltx: ::windows_core::GUID, txnid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLtxTransactionPromote(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&guidltx), ::core::mem::transmute(&txnid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnLtxTransactionStart: OnLtxTransactionStart::<Identity, Impl, OFFSET>,
            OnLtxTransactionPrepare: OnLtxTransactionPrepare::<Identity, Impl, OFFSET>,
            OnLtxTransactionAbort: OnLtxTransactionAbort::<Identity, Impl, OFFSET>,
            OnLtxTransactionCommit: OnLtxTransactionCommit::<Identity, Impl, OFFSET>,
            OnLtxTransactionPromote: OnLtxTransactionPromote::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComLTxEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComMethod2Events_Impl: Sized {
    fn OnMethodCall2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::Result<()>;
    fn OnMethodReturn2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnMethodException2(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComMethod2Events {}
impl IComMethod2Events_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>() -> IComMethod2Events_Vtbl {
        unsafe extern "system" fn OnMethodCall2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodCall2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodReturn2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethod2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, dwthread: u32, imeth: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodException2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMethodCall2: OnMethodCall2::<Identity, Impl, OFFSET>,
            OnMethodReturn2: OnMethodReturn2::<Identity, Impl, OFFSET>,
            OnMethodException2: OnMethodException2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComMethod2Events as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComMethodEvents_Impl: Sized {
    fn OnMethodCall(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::Result<()>;
    fn OnMethodReturn(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnMethodException(&self, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComMethodEvents {}
impl IComMethodEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>() -> IComMethodEvents_Vtbl {
        unsafe extern "system" fn OnMethodCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodCall(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth)).into()
        }
        unsafe extern "system" fn OnMethodReturn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32, hresult: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodReturn(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&hresult)).into()
        }
        unsafe extern "system" fn OnMethodException<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, oid: u64, guidcid: *const ::windows_core::GUID, guidrid: *const ::windows_core::GUID, imeth: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnMethodException(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidcid), ::core::mem::transmute_copy(&guidrid), ::core::mem::transmute_copy(&imeth)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnMethodCall: OnMethodCall::<Identity, Impl, OFFSET>,
            OnMethodReturn: OnMethodReturn::<Identity, Impl, OFFSET>,
            OnMethodException: OnMethodException::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComMethodEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComMtaThreadPoolKnobs_Impl: Sized {
    fn MTASetMaxThreadCount(&self, dwmaxthreads: u32) -> ::windows_core::Result<()>;
    fn MTAGetMaxThreadCount(&self) -> ::windows_core::Result<u32>;
    fn MTASetThrottleValue(&self, dwthrottle: u32) -> ::windows_core::Result<()>;
    fn MTAGetThrottleValue(&self) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IComMtaThreadPoolKnobs {}
impl IComMtaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>() -> IComMtaThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn MTASetMaxThreadCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxthreads: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MTASetMaxThreadCount(::core::mem::transmute_copy(&dwmaxthreads)).into()
        }
        unsafe extern "system" fn MTAGetMaxThreadCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MTAGetMaxThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwmaxthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MTASetThrottleValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthrottle: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MTASetThrottleValue(::core::mem::transmute_copy(&dwthrottle)).into()
        }
        unsafe extern "system" fn MTAGetThrottleValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComMtaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthrottle: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MTAGetThrottleValue() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwthrottle, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            MTASetMaxThreadCount: MTASetMaxThreadCount::<Identity, Impl, OFFSET>,
            MTAGetMaxThreadCount: MTAGetMaxThreadCount::<Identity, Impl, OFFSET>,
            MTASetThrottleValue: MTASetThrottleValue::<Identity, Impl, OFFSET>,
            MTAGetThrottleValue: MTAGetThrottleValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComMtaThreadPoolKnobs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComObjectConstruction2Events_Impl: Sized {
    fn OnObjectConstruct2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: &::windows_core::PCWSTR, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComObjectConstruction2Events {}
impl IComObjectConstruction2Events_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstruction2Events_Impl, const OFFSET: isize>() -> IComObjectConstruction2Events_Vtbl {
        unsafe extern "system" fn OnObjectConstruct2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstruction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: ::windows_core::PCWSTR, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectConstruct2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute(&sconstructstring), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnObjectConstruct2: OnObjectConstruct2::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComObjectConstruction2Events as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComObjectConstructionEvents_Impl: Sized {
    fn OnObjectConstruct(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: &::windows_core::PCWSTR, oid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComObjectConstructionEvents {}
impl IComObjectConstructionEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstructionEvents_Impl, const OFFSET: isize>() -> IComObjectConstructionEvents_Vtbl {
        unsafe extern "system" fn OnObjectConstruct<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectConstructionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, sconstructstring: ::windows_core::PCWSTR, oid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectConstruct(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute(&sconstructstring), ::core::mem::transmute_copy(&oid)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnObjectConstruct: OnObjectConstruct::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComObjectConstructionEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComObjectEvents_Impl: Sized {
    fn OnObjectActivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::Result<()>;
    fn OnObjectDeactivate(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::Result<()>;
    fn OnDisableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
    fn OnEnableCommit(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
    fn OnSetComplete(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
    fn OnSetAbort(&self, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComObjectEvents {}
impl IComObjectEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>() -> IComObjectEvents_Vtbl {
        unsafe extern "system" fn OnObjectActivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectActivate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnObjectDeactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64, objectid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjectDeactivate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn OnDisableCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDisableCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnEnableCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEnableCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetComplete(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        unsafe extern "system" fn OnSetAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, ctxtid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSetAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&ctxtid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjectActivate: OnObjectActivate::<Identity, Impl, OFFSET>,
            OnObjectDeactivate: OnObjectDeactivate::<Identity, Impl, OFFSET>,
            OnDisableCommit: OnDisableCommit::<Identity, Impl, OFFSET>,
            OnEnableCommit: OnEnableCommit::<Identity, Impl, OFFSET>,
            OnSetComplete: OnSetComplete::<Identity, Impl, OFFSET>,
            OnSetAbort: OnSetAbort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComObjectEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComObjectPool2Events_Impl: Sized {
    fn OnObjPoolPutObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolGetObject2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnObjPoolRecycleToTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolGetFromTx2(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComObjectPool2Events {}
impl IComObjectPool2Events_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>() -> IComObjectPool2Events_Vtbl {
        unsafe extern "system" fn OnObjPoolPutObject2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolPutObject2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&nreason), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolGetObject2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolRecycleToTx2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPool2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64, guidpartition: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolGetFromTx2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&guidpartition)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject2: OnObjPoolPutObject2::<Identity, Impl, OFFSET>,
            OnObjPoolGetObject2: OnObjPoolGetObject2::<Identity, Impl, OFFSET>,
            OnObjPoolRecycleToTx2: OnObjPoolRecycleToTx2::<Identity, Impl, OFFSET>,
            OnObjPoolGetFromTx2: OnObjPoolGetFromTx2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComObjectPool2Events as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComObjectPoolEvents_Impl: Sized {
    fn OnObjPoolPutObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolGetObject(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolRecycleToTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolGetFromTx(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComObjectPoolEvents {}
impl IComObjectPoolEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>() -> IComObjectPoolEvents_Vtbl {
        unsafe extern "system" fn OnObjPoolPutObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, nreason: i32, dwavailable: u32, oid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolPutObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&nreason), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, dwavailable: u32, oid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolGetObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwavailable), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolRecycleToTx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolRecycleToTx(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into()
        }
        unsafe extern "system" fn OnObjPoolGetFromTx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, guidobject: *const ::windows_core::GUID, guidtx: *const ::windows_core::GUID, objid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolGetFromTx(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&objid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolPutObject: OnObjPoolPutObject::<Identity, Impl, OFFSET>,
            OnObjPoolGetObject: OnObjPoolGetObject::<Identity, Impl, OFFSET>,
            OnObjPoolRecycleToTx: OnObjPoolRecycleToTx::<Identity, Impl, OFFSET>,
            OnObjPoolGetFromTx: OnObjPoolGetFromTx::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComObjectPoolEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComObjectPoolEvents2_Impl: Sized {
    fn OnObjPoolCreateObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolDestroyObject(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::Result<()>;
    fn OnObjPoolCreateDecision(&self, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows_core::Result<()>;
    fn OnObjPoolTimeout(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, guidactivity: *const ::windows_core::GUID, dwtimeout: u32) -> ::windows_core::Result<()>;
    fn OnObjPoolCreatePool(&self, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComObjectPoolEvents2 {}
impl IComObjectPoolEvents2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>() -> IComObjectPoolEvents2_Vtbl {
        unsafe extern "system" fn OnObjPoolCreateObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolCreateObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwobjscreated), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolDestroyObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwobjscreated: u32, oid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolDestroyObject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwobjscreated), ::core::mem::transmute_copy(&oid)).into()
        }
        unsafe extern "system" fn OnObjPoolCreateDecision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, dwthreadswaiting: u32, dwavail: u32, dwcreated: u32, dwmin: u32, dwmax: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolCreateDecision(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&dwthreadswaiting), ::core::mem::transmute_copy(&dwavail), ::core::mem::transmute_copy(&dwcreated), ::core::mem::transmute_copy(&dwmin), ::core::mem::transmute_copy(&dwmax)).into()
        }
        unsafe extern "system" fn OnObjPoolTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, guidactivity: *const ::windows_core::GUID, dwtimeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolTimeout(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&dwtimeout)).into()
        }
        unsafe extern "system" fn OnObjPoolCreatePool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComObjectPoolEvents2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidobject: *const ::windows_core::GUID, dwmin: u32, dwmax: u32, dwtimeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnObjPoolCreatePool(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidobject), ::core::mem::transmute_copy(&dwmin), ::core::mem::transmute_copy(&dwmax), ::core::mem::transmute_copy(&dwtimeout)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnObjPoolCreateObject: OnObjPoolCreateObject::<Identity, Impl, OFFSET>,
            OnObjPoolDestroyObject: OnObjPoolDestroyObject::<Identity, Impl, OFFSET>,
            OnObjPoolCreateDecision: OnObjPoolCreateDecision::<Identity, Impl, OFFSET>,
            OnObjPoolTimeout: OnObjPoolTimeout::<Identity, Impl, OFFSET>,
            OnObjPoolCreatePool: OnObjPoolCreatePool::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComObjectPoolEvents2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComQCEvents_Impl: Sized {
    fn OnQCRecord(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: &::windows_core::PCWSTR, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, msmqhr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnQCQueueOpen(&self, pinfo: *const COMSVCSEVENTINFO, szqueue: &::windows_core::PCWSTR, queueid: u64, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnQCReceive(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnQCReceiveFail(&self, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn OnQCMoveToReTryQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, retryindex: u32) -> ::windows_core::Result<()>;
    fn OnQCMoveToDeadQueue(&self, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnQCPlayback(&self, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComQCEvents {}
impl IComQCEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>() -> IComQCEvents_Vtbl {
        unsafe extern "system" fn OnQCRecord<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, szqueue: ::windows_core::PCWSTR, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, msmqhr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCRecord(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute(&szqueue), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCQueueOpen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, szqueue: ::windows_core::PCWSTR, queueid: u64, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCQueueOpen(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute(&szqueue), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCReceive(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&hr)).into()
        }
        unsafe extern "system" fn OnQCReceiveFail<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, queueid: u64, msmqhr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCReceiveFail(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&queueid), ::core::mem::transmute_copy(&msmqhr)).into()
        }
        unsafe extern "system" fn OnQCMoveToReTryQueue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, retryindex: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCMoveToReTryQueue(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&retryindex)).into()
        }
        unsafe extern "system" fn OnQCMoveToDeadQueue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCMoveToDeadQueue(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid)).into()
        }
        unsafe extern "system" fn OnQCPlayback<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComQCEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objid: u64, guidmsgid: *const ::windows_core::GUID, guidworkflowid: *const ::windows_core::GUID, hr: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnQCPlayback(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objid), ::core::mem::transmute_copy(&guidmsgid), ::core::mem::transmute_copy(&guidworkflowid), ::core::mem::transmute_copy(&hr)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnQCRecord: OnQCRecord::<Identity, Impl, OFFSET>,
            OnQCQueueOpen: OnQCQueueOpen::<Identity, Impl, OFFSET>,
            OnQCReceive: OnQCReceive::<Identity, Impl, OFFSET>,
            OnQCReceiveFail: OnQCReceiveFail::<Identity, Impl, OFFSET>,
            OnQCMoveToReTryQueue: OnQCMoveToReTryQueue::<Identity, Impl, OFFSET>,
            OnQCMoveToDeadQueue: OnQCMoveToDeadQueue::<Identity, Impl, OFFSET>,
            OnQCPlayback: OnQCPlayback::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComQCEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComResourceEvents_Impl: Sized {
    fn OnResourceCreate(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnResourceAllocate(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows_core::Result<()>;
    fn OnResourceRecycle(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows_core::PCWSTR, resid: u64) -> ::windows_core::Result<()>;
    fn OnResourceDestroy(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows_core::HRESULT, psztype: &::windows_core::PCWSTR, resid: u64) -> ::windows_core::Result<()>;
    fn OnResourceTrack(&self, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: &::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IComResourceEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComResourceEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>() -> IComResourceEvents_Vtbl {
        unsafe extern "system" fn OnResourceCreate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceCreate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted)).into()
        }
        unsafe extern "system" fn OnResourceAllocate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL, numrated: u32, rating: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceAllocate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted), ::core::mem::transmute_copy(&numrated), ::core::mem::transmute_copy(&rating)).into()
        }
        unsafe extern "system" fn OnResourceRecycle<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceRecycle(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceDestroy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, hr: ::windows_core::HRESULT, psztype: ::windows_core::PCWSTR, resid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceDestroy(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&hr), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn OnResourceTrack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComResourceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, objectid: u64, psztype: ::windows_core::PCWSTR, resid: u64, enlisted: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnResourceTrack(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute(&psztype), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&enlisted)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnResourceCreate: OnResourceCreate::<Identity, Impl, OFFSET>,
            OnResourceAllocate: OnResourceAllocate::<Identity, Impl, OFFSET>,
            OnResourceRecycle: OnResourceRecycle::<Identity, Impl, OFFSET>,
            OnResourceDestroy: OnResourceDestroy::<Identity, Impl, OFFSET>,
            OnResourceTrack: OnResourceTrack::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComResourceEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComSecurityEvents_Impl: Sized {
    fn OnAuthenticate(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnAuthenticateFail(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IComSecurityEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComSecurityEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: isize>() -> IComSecurityEvents_Vtbl {
        unsafe extern "system" fn OnAuthenticate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAuthenticate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidiid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&cbbyteorig), ::core::mem::transmute_copy(&psidoriginaluser), ::core::mem::transmute_copy(&cbbytecur), ::core::mem::transmute_copy(&psidcurrentuser), ::core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        unsafe extern "system" fn OnAuthenticateFail<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComSecurityEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, objectid: u64, guidiid: *const ::windows_core::GUID, imeth: u32, cbbyteorig: u32, psidoriginaluser: *const u8, cbbytecur: u32, psidcurrentuser: *const u8, bcurrentuserinpersonatinginproc: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnAuthenticateFail(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&objectid), ::core::mem::transmute_copy(&guidiid), ::core::mem::transmute_copy(&imeth), ::core::mem::transmute_copy(&cbbyteorig), ::core::mem::transmute_copy(&psidoriginaluser), ::core::mem::transmute_copy(&cbbytecur), ::core::mem::transmute_copy(&psidcurrentuser), ::core::mem::transmute_copy(&bcurrentuserinpersonatinginproc)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAuthenticate: OnAuthenticate::<Identity, Impl, OFFSET>,
            OnAuthenticateFail: OnAuthenticateFail::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComSecurityEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComStaThreadPoolKnobs_Impl: Sized {
    fn SetMinThreadCount(&self, minthreads: u32) -> ::windows_core::Result<()>;
    fn GetMinThreadCount(&self) -> ::windows_core::Result<u32>;
    fn SetMaxThreadCount(&self, maxthreads: u32) -> ::windows_core::Result<()>;
    fn GetMaxThreadCount(&self) -> ::windows_core::Result<u32>;
    fn SetActivityPerThread(&self, activitiesperthread: u32) -> ::windows_core::Result<()>;
    fn GetActivityPerThread(&self) -> ::windows_core::Result<u32>;
    fn SetActivityRatio(&self, activityratio: f64) -> ::windows_core::Result<()>;
    fn GetActivityRatio(&self) -> ::windows_core::Result<f64>;
    fn GetThreadCount(&self) -> ::windows_core::Result<u32>;
    fn GetQueueDepth(&self) -> ::windows_core::Result<u32>;
    fn SetQueueDepth(&self, dwqdepth: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComStaThreadPoolKnobs {}
impl IComStaThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>() -> IComStaThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn SetMinThreadCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinThreadCount(::core::mem::transmute_copy(&minthreads)).into()
        }
        unsafe extern "system" fn GetMinThreadCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, minthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMinThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(minthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxThreadCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxThreadCount(::core::mem::transmute_copy(&maxthreads)).into()
        }
        unsafe extern "system" fn GetMaxThreadCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, maxthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(maxthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityPerThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActivityPerThread(::core::mem::transmute_copy(&activitiesperthread)).into()
        }
        unsafe extern "system" fn GetActivityPerThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activitiesperthread: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivityPerThread() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activitiesperthread, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActivityRatio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActivityRatio(::core::mem::transmute_copy(&activityratio)).into()
        }
        unsafe extern "system" fn GetActivityRatio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, activityratio: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActivityRatio() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(activityratio, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetThreadCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreads: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetThreadCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwthreads, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueueDepth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwqdepth: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetQueueDepth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwqdepth, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetQueueDepth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwqdepth: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueueDepth(::core::mem::transmute_copy(&dwqdepth)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComStaThreadPoolKnobs2_Impl: Sized + IComStaThreadPoolKnobs_Impl {
    fn GetMaxCPULoad(&self) -> ::windows_core::Result<u32>;
    fn SetMaxCPULoad(&self, pdwload: i32) -> ::windows_core::Result<()>;
    fn GetCPUMetricEnabled(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCPUMetricEnabled(&self, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetCreateThreadsAggressively(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn SetCreateThreadsAggressively(&self, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetMaxCSR(&self) -> ::windows_core::Result<u32>;
    fn SetMaxCSR(&self, dwcsr: i32) -> ::windows_core::Result<()>;
    fn GetWaitTimeForThreadCleanup(&self) -> ::windows_core::Result<u32>;
    fn SetWaitTimeForThreadCleanup(&self, dwthreadcleanupwaittime: i32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IComStaThreadPoolKnobs2 {}
#[cfg(feature = "Win32_Foundation")]
impl IComStaThreadPoolKnobs2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>() -> IComStaThreadPoolKnobs2_Vtbl {
        unsafe extern "system" fn GetMaxCPULoad<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxCPULoad() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwload, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCPULoad<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwload: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxCPULoad(::core::mem::transmute_copy(&pdwload)).into()
        }
        unsafe extern "system" fn GetCPUMetricEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCPUMetricEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmetricenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCPUMetricEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCPUMetricEnabled(::core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetCreateThreadsAggressively<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbmetricenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCreateThreadsAggressively() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbmetricenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateThreadsAggressively<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bmetricenabled: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCreateThreadsAggressively(::core::mem::transmute_copy(&bmetricenabled)).into()
        }
        unsafe extern "system" fn GetMaxCSR<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcsr: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaxCSR() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcsr, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxCSR<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcsr: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxCSR(::core::mem::transmute_copy(&dwcsr)).into()
        }
        unsafe extern "system" fn GetWaitTimeForThreadCleanup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwthreadcleanupwaittime: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetWaitTimeForThreadCleanup() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwthreadcleanupwaittime, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWaitTimeForThreadCleanup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComStaThreadPoolKnobs2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwthreadcleanupwaittime: i32) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComStaThreadPoolKnobs2 as ::windows_core::ComInterface>::IID || iid == &<IComStaThreadPoolKnobs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComThreadEvents_Impl: Sized {
    fn OnThreadStart(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::Result<()>;
    fn OnThreadTerminate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::Result<()>;
    fn OnThreadBindToApartment(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows_core::Result<()>;
    fn OnThreadUnBind(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows_core::Result<()>;
    fn OnThreadWorkEnque(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::Result<()>;
    fn OnThreadWorkPrivate(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows_core::Result<()>;
    fn OnThreadWorkPublic(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::Result<()>;
    fn OnThreadWorkRedirect(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows_core::Result<()>;
    fn OnThreadWorkReject(&self, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::Result<()>;
    fn OnThreadAssignApartment(&self, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, aptid: u64) -> ::windows_core::Result<()>;
    fn OnThreadUnassignApartment(&self, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComThreadEvents {}
impl IComThreadEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>() -> IComThreadEvents_Vtbl {
        unsafe extern "system" fn OnThreadStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadTerminate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, dwthread: u32, dwtheadcnt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadTerminate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&dwthread), ::core::mem::transmute_copy(&dwtheadcnt)).into()
        }
        unsafe extern "system" fn OnThreadBindToApartment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32, dwlowcnt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadBindToApartment(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&aptid), ::core::mem::transmute_copy(&dwactcnt), ::core::mem::transmute_copy(&dwlowcnt)).into()
        }
        unsafe extern "system" fn OnThreadUnBind<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, aptid: u64, dwactcnt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadUnBind(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&aptid), ::core::mem::transmute_copy(&dwactcnt)).into()
        }
        unsafe extern "system" fn OnThreadWorkEnque<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkEnque(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkPrivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkPrivate(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid)).into()
        }
        unsafe extern "system" fn OnThreadWorkPublic<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkPublic(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadWorkRedirect<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32, threadnum: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkRedirect(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen), ::core::mem::transmute_copy(&threadnum)).into()
        }
        unsafe extern "system" fn OnThreadWorkReject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, threadid: u64, msgworkid: u64, queuelen: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadWorkReject(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&threadid), ::core::mem::transmute_copy(&msgworkid), ::core::mem::transmute_copy(&queuelen)).into()
        }
        unsafe extern "system" fn OnThreadAssignApartment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidactivity: *const ::windows_core::GUID, aptid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadAssignApartment(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidactivity), ::core::mem::transmute_copy(&aptid)).into()
        }
        unsafe extern "system" fn OnThreadUnassignApartment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComThreadEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, aptid: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnThreadUnassignApartment(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&aptid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComThreadEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComTrackingInfoCollection_Impl: Sized {
    fn Type(&self) -> ::windows_core::Result<TRACKING_COLL_TYPE>;
    fn Count(&self) -> ::windows_core::Result<u32>;
    fn Item(&self, ulindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComTrackingInfoCollection {}
impl IComTrackingInfoCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>() -> IComTrackingInfoCollection_Vtbl {
        unsafe extern "system" fn Type<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut TRACKING_COLL_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Item(::core::mem::transmute_copy(&ulindex), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Type: Type::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComTrackingInfoEvents_Impl: Sized {
    fn OnNewTrackingInfo(&self, ptoplevelcollection: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IComTrackingInfoEvents {}
impl IComTrackingInfoEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoEvents_Impl, const OFFSET: isize>() -> IComTrackingInfoEvents_Vtbl {
        unsafe extern "system" fn OnNewTrackingInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptoplevelcollection: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnNewTrackingInfo(::windows_core::from_raw_borrowed(&ptoplevelcollection)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnNewTrackingInfo: OnNewTrackingInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComTrackingInfoObject_Impl: Sized {
    fn GetValue(&self, szpropertyname: &::windows_core::PCWSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IComTrackingInfoObject {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IComTrackingInfoObject_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoObject_Impl, const OFFSET: isize>() -> IComTrackingInfoObject_Vtbl {
        unsafe extern "system" fn GetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szpropertyname: ::windows_core::PCWSTR, pvarout: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetValue(::core::mem::transmute(&szpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarout, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetValue: GetValue::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoObject as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IComTrackingInfoProperties_Impl: Sized {
    fn PropCount(&self) -> ::windows_core::Result<u32>;
    fn GetPropName(&self, ulindex: u32) -> ::windows_core::Result<::windows_core::PWSTR>;
}
impl ::windows_core::RuntimeName for IComTrackingInfoProperties {}
impl IComTrackingInfoProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>() -> IComTrackingInfoProperties_Vtbl {
        unsafe extern "system" fn PropCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PropCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTrackingInfoProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppszpropname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropName(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszpropname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PropCount: PropCount::<Identity, Impl, OFFSET>,
            GetPropName: GetPropName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComTrackingInfoProperties as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransaction2Events_Impl: Sized {
    fn OnTransactionStart2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_core::Result<()>;
    fn OnTransactionPrepare2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnTransactionAbort2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnTransactionCommit2(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IComTransaction2Events {}
#[cfg(feature = "Win32_Foundation")]
impl IComTransaction2Events_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>() -> IComTransaction2Events_Vtbl {
        unsafe extern "system" fn OnTransactionStart2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: super::super::Foundation::BOOL, nisolationlevel: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionStart2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&froot), ::core::mem::transmute_copy(&nisolationlevel)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionPrepare2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionAbort2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransaction2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionCommit2(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTransactionStart2: OnTransactionStart2::<Identity, Impl, OFFSET>,
            OnTransactionPrepare2: OnTransactionPrepare2::<Identity, Impl, OFFSET>,
            OnTransactionAbort2: OnTransactionAbort2::<Identity, Impl, OFFSET>,
            OnTransactionCommit2: OnTransactionCommit2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComTransaction2Events as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IComTransactionEvents_Impl: Sized {
    fn OnTransactionStart(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnTransactionPrepare(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn OnTransactionAbort(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn OnTransactionCommit(&self, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IComTransactionEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IComTransactionEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>() -> IComTransactionEvents_Vtbl {
        unsafe extern "system" fn OnTransactionStart<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, tsid: *const ::windows_core::GUID, froot: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionStart(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&tsid), ::core::mem::transmute_copy(&froot)).into()
        }
        unsafe extern "system" fn OnTransactionPrepare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID, fvoteyes: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionPrepare(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx), ::core::mem::transmute_copy(&fvoteyes)).into()
        }
        unsafe extern "system" fn OnTransactionAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionAbort(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        unsafe extern "system" fn OnTransactionCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComTransactionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, guidtx: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnTransactionCommit(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&guidtx)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnTransactionStart: OnTransactionStart::<Identity, Impl, OFFSET>,
            OnTransactionPrepare: OnTransactionPrepare::<Identity, Impl, OFFSET>,
            OnTransactionAbort: OnTransactionAbort::<Identity, Impl, OFFSET>,
            OnTransactionCommit: OnTransactionCommit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComTransactionEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IComUserEvent_Impl: Sized {
    fn OnUserEvent(&self, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IComUserEvent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IComUserEvent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComUserEvent_Impl, const OFFSET: isize>() -> IComUserEvent_Vtbl {
        unsafe extern "system" fn OnUserEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IComUserEvent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMSVCSEVENTINFO, pvarevent: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnUserEvent(::core::mem::transmute_copy(&pinfo), ::core::mem::transmute_copy(&pvarevent)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnUserEvent: OnUserEvent::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IComUserEvent as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IContextProperties_Impl: Sized {
    fn Count(&self, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn GetProperty(&self, name: &::windows_core::BSTR, pproperty: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn EnumNames(&self) -> ::windows_core::Result<IEnumNames>;
    fn SetProperty(&self, name: &::windows_core::BSTR, property: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn RemoveProperty(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IContextProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IContextProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>() -> IContextProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute(&name), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, property: super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute(&name), ::core::mem::transmute(&property)).into()
        }
        unsafe extern "system" fn RemoveProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveProperty(::core::mem::transmute(&name)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            EnumNames: EnumNames::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            RemoveProperty: RemoveProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContextProperties as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContextSecurityPerimeter_Impl: Sized {
    fn GetPerimeterFlag(&self, pflag: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetPerimeterFlag(&self, fflag: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IContextSecurityPerimeter {}
#[cfg(feature = "Win32_Foundation")]
impl IContextSecurityPerimeter_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>() -> IContextSecurityPerimeter_Vtbl {
        unsafe extern "system" fn GetPerimeterFlag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pflag: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPerimeterFlag(::core::mem::transmute_copy(&pflag)).into()
        }
        unsafe extern "system" fn SetPerimeterFlag<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextSecurityPerimeter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fflag: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPerimeterFlag(::core::mem::transmute_copy(&fflag)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPerimeterFlag: GetPerimeterFlag::<Identity, Impl, OFFSET>,
            SetPerimeterFlag: SetPerimeterFlag::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContextSecurityPerimeter as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IContextState_Impl: Sized {
    fn SetDeactivateOnReturn(&self, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn GetDeactivateOnReturn(&self, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetMyTransactionVote(&self, txvote: TransactionVote) -> ::windows_core::Result<()>;
    fn GetMyTransactionVote(&self, ptxvote: *mut TransactionVote) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IContextState {}
#[cfg(feature = "Win32_Foundation")]
impl IContextState_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>() -> IContextState_Vtbl {
        unsafe extern "system" fn SetDeactivateOnReturn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bdeactivate: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeactivateOnReturn(::core::mem::transmute_copy(&bdeactivate)).into()
        }
        unsafe extern "system" fn GetDeactivateOnReturn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbdeactivate: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeactivateOnReturn(::core::mem::transmute_copy(&pbdeactivate)).into()
        }
        unsafe extern "system" fn SetMyTransactionVote<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, txvote: TransactionVote) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMyTransactionVote(::core::mem::transmute_copy(&txvote)).into()
        }
        unsafe extern "system" fn GetMyTransactionVote<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IContextState_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxvote: *mut TransactionVote) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMyTransactionVote(::core::mem::transmute_copy(&ptxvote)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDeactivateOnReturn: SetDeactivateOnReturn::<Identity, Impl, OFFSET>,
            GetDeactivateOnReturn: GetDeactivateOnReturn::<Identity, Impl, OFFSET>,
            SetMyTransactionVote: SetMyTransactionVote::<Identity, Impl, OFFSET>,
            GetMyTransactionVote: GetMyTransactionVote::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IContextState as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ICreateWithLocalTransaction_Impl: Sized {
    fn CreateInstanceWithSysTx(&self, ptransaction: ::core::option::Option<&::windows_core::IUnknown>, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICreateWithLocalTransaction {}
impl ICreateWithLocalTransaction_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithLocalTransaction_Impl, const OFFSET: isize>() -> ICreateWithLocalTransaction_Vtbl {
        unsafe extern "system" fn CreateInstanceWithSysTx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithLocalTransaction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstanceWithSysTx(::windows_core::from_raw_borrowed(&ptransaction), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstanceWithSysTx: CreateInstanceWithSysTx::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICreateWithLocalTransaction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ICreateWithTipTransactionEx_Impl: Sized {
    fn CreateInstance(&self, bstrtipurl: &::windows_core::BSTR, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ICreateWithTipTransactionEx {}
impl ICreateWithTipTransactionEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTipTransactionEx_Impl, const OFFSET: isize>() -> ICreateWithTipTransactionEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTipTransactionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtipurl: ::std::mem::MaybeUninit<::windows_core::BSTR>, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::core::mem::transmute(&bstrtipurl), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICreateWithTipTransactionEx as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_DistributedTransactionCoordinator\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait ICreateWithTransactionEx_Impl: Sized {
    fn CreateInstance(&self, ptransaction: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransaction>, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows_core::RuntimeName for ICreateWithTransactionEx {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ICreateWithTransactionEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTransactionEx_Impl, const OFFSET: isize>() -> ICreateWithTransactionEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICreateWithTransactionEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::windows_core::from_raw_borrowed(&ptransaction), ::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateInstance: CreateInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICreateWithTransactionEx as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait ICrmCompensator_Impl: Sized {
    fn SetLogControl(&self, plogcontrol: ::core::option::Option<&ICrmLogControl>) -> ::windows_core::Result<()>;
    fn BeginPrepare(&self) -> ::windows_core::Result<()>;
    fn PrepareRecord(&self, crmlogrec: &CrmLogRecordRead) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EndPrepare(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn BeginCommit(&self, frecovery: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn CommitRecord(&self, crmlogrec: &CrmLogRecordRead) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EndCommit(&self) -> ::windows_core::Result<()>;
    fn BeginAbort(&self, frecovery: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn AbortRecord(&self, crmlogrec: &CrmLogRecordRead) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn EndAbort(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for ICrmCompensator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ICrmCompensator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>() -> ICrmCompensator_Vtbl {
        unsafe extern "system" fn SetLogControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogControl(::windows_core::from_raw_borrowed(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginPrepare().into()
        }
        unsafe extern "system" fn PrepareRecord<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrepareRecord(::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepare<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfoktoprepare: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndPrepare() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfoktoprepare, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginCommit(::core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn CommitRecord<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitRecord(::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCommit().into()
        }
        unsafe extern "system" fn BeginAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, frecovery: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginAbort(::core::mem::transmute_copy(&frecovery)).into()
        }
        unsafe extern "system" fn AbortRecord<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pfforget: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AbortRecord(::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndAbort().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICrmCompensator as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmCompensatorVariants_Impl: Sized {
    fn SetLogControlVariants(&self, plogcontrol: ::core::option::Option<&ICrmLogControl>) -> ::windows_core::Result<()>;
    fn BeginPrepareVariants(&self) -> ::windows_core::Result<()>;
    fn PrepareRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndPrepareVariants(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn BeginCommitVariants(&self, brecovery: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn CommitRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndCommitVariants(&self) -> ::windows_core::Result<()>;
    fn BeginAbortVariants(&self, brecovery: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn AbortRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn EndAbortVariants(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICrmCompensatorVariants {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmCompensatorVariants_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>() -> ICrmCompensatorVariants_Vtbl {
        unsafe extern "system" fn SetLogControlVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogcontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLogControlVariants(::windows_core::from_raw_borrowed(&plogcontrol)).into()
        }
        unsafe extern "system" fn BeginPrepareVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginPrepareVariants().into()
        }
        unsafe extern "system" fn PrepareRecordVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Variant::VARIANT, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PrepareRecordVariants(::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndPrepareVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pboktoprepare: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndPrepareVariants() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pboktoprepare, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginCommitVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginCommitVariants(::core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn CommitRecordVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Variant::VARIANT, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitRecordVariants(::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndCommitVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndCommitVariants().into()
        }
        unsafe extern "system" fn BeginAbortVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, brecovery: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BeginAbortVariants(::core::mem::transmute_copy(&brecovery)).into()
        }
        unsafe extern "system" fn AbortRecordVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Variant::VARIANT, pbforget: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AbortRecordVariants(::core::mem::transmute_copy(&plogrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbforget, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndAbortVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmCompensatorVariants_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EndAbortVariants().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICrmCompensatorVariants as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmFormatLogRecords_Impl: Sized {
    fn GetColumnCount(&self) -> ::windows_core::Result<i32>;
    fn GetColumnHeaders(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetColumn(&self, crmlogrec: &CrmLogRecordRead) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn GetColumnVariants(&self, logrecord: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICrmFormatLogRecords {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmFormatLogRecords_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>() -> ICrmFormatLogRecords_Vtbl {
        unsafe extern "system" fn GetColumnCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcolumncount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcolumncount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pheaders: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnHeaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pheaders, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crmlogrec: CrmLogRecordRead, pformattedlogrecord: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumn(::core::mem::transmute(&crmlogrec)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformattedlogrecord, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetColumnVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmFormatLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, logrecord: super::Variant::VARIANT, pformattedlogrecord: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetColumnVariants(::core::mem::transmute(&logrecord)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pformattedlogrecord, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetColumnCount: GetColumnCount::<Identity, Impl, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, Impl, OFFSET>,
            GetColumn: GetColumn::<Identity, Impl, OFFSET>,
            GetColumnVariants: GetColumnVariants::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICrmFormatLogRecords as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmLogControl_Impl: Sized {
    fn TransactionUOW(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RegisterCompensator(&self, lpcwstrprogidcompensator: &::windows_core::PCWSTR, lpcwstrdescription: &::windows_core::PCWSTR, lcrmregflags: i32) -> ::windows_core::Result<()>;
    fn WriteLogRecordVariants(&self, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ForceLog(&self) -> ::windows_core::Result<()>;
    fn ForgetLogRecord(&self) -> ::windows_core::Result<()>;
    fn ForceTransactionToAbort(&self) -> ::windows_core::Result<()>;
    fn WriteLogRecord(&self, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICrmLogControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmLogControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>() -> ICrmLogControl_Vtbl {
        unsafe extern "system" fn TransactionUOW<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionUOW() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterCompensator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpcwstrprogidcompensator: ::windows_core::PCWSTR, lpcwstrdescription: ::windows_core::PCWSTR, lcrmregflags: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RegisterCompensator(::core::mem::transmute(&lpcwstrprogidcompensator), ::core::mem::transmute(&lpcwstrdescription), ::core::mem::transmute_copy(&lcrmregflags)).into()
        }
        unsafe extern "system" fn WriteLogRecordVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plogrecord: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteLogRecordVariants(::core::mem::transmute_copy(&plogrecord)).into()
        }
        unsafe extern "system" fn ForceLog<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForceLog().into()
        }
        unsafe extern "system" fn ForgetLogRecord<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForgetLogRecord().into()
        }
        unsafe extern "system" fn ForceTransactionToAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ForceTransactionToAbort().into()
        }
        unsafe extern "system" fn WriteLogRecord<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmLogControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rgblob: *const super::Com::BLOB, cblob: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteLogRecord(::core::mem::transmute_copy(&rgblob), ::core::mem::transmute_copy(&cblob)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TransactionUOW: TransactionUOW::<Identity, Impl, OFFSET>,
            RegisterCompensator: RegisterCompensator::<Identity, Impl, OFFSET>,
            WriteLogRecordVariants: WriteLogRecordVariants::<Identity, Impl, OFFSET>,
            ForceLog: ForceLog::<Identity, Impl, OFFSET>,
            ForgetLogRecord: ForgetLogRecord::<Identity, Impl, OFFSET>,
            ForceTransactionToAbort: ForceTransactionToAbort::<Identity, Impl, OFFSET>,
            WriteLogRecord: WriteLogRecord::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICrmLogControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmMonitor_Impl: Sized {
    fn GetClerks(&self) -> ::windows_core::Result<ICrmMonitorClerks>;
    fn HoldClerk(&self, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICrmMonitor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmMonitor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: isize>() -> ICrmMonitor_Vtbl {
        unsafe extern "system" fn GetClerks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pclerks: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClerks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pclerks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HoldClerk<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.HoldClerk(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClerks: GetClerks::<Identity, Impl, OFFSET>,
            HoldClerk: HoldClerk::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICrmMonitor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmMonitorClerks_Impl: Sized + super::Com::IDispatch_Impl {
    fn Item(&self, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn ProgIdCompensator(&self, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Description(&self, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn TransactionUOW(&self, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn ActivityId(&self, index: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICrmMonitorClerks {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmMonitorClerks_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>() -> ICrmMonitorClerks_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgIdCompensator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProgIdCompensator(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionUOW<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionUOW(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivityId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorClerks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: super::Variant::VARIANT, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActivityId(::core::mem::transmute(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICrmMonitorClerks as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ICrmMonitorLogRecords_Impl: Sized {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn TransactionState(&self) -> ::windows_core::Result<CrmTransactionState>;
    fn StructuredRecords(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetLogRecord(&self, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows_core::Result<()>;
    fn GetLogRecordVariants(&self, indexnumber: &super::Variant::VARIANT) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ICrmMonitorLogRecords {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ICrmMonitorLogRecords_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>() -> ICrmMonitorLogRecords_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TransactionState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut CrmTransactionState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TransactionState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StructuredRecords<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StructuredRecords() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLogRecord<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pcrmlogrec: *mut CrmLogRecordRead) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLogRecord(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pcrmlogrec)).into()
        }
        unsafe extern "system" fn GetLogRecordVariants<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ICrmMonitorLogRecords_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, indexnumber: super::Variant::VARIANT, plogrecord: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLogRecordVariants(::core::mem::transmute(&indexnumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plogrecord, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            TransactionState: TransactionState::<Identity, Impl, OFFSET>,
            StructuredRecords: StructuredRecords::<Identity, Impl, OFFSET>,
            GetLogRecord: GetLogRecord::<Identity, Impl, OFFSET>,
            GetLogRecordVariants: GetLogRecordVariants::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ICrmMonitorLogRecords as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDispenserDriver_Impl: Sized {
    fn CreateResource(&self, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows_core::Result<()>;
    fn RateResource(&self, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows_core::Result<()>;
    fn EnlistResource(&self, resid: usize, transid: usize) -> ::windows_core::Result<()>;
    fn ResetResource(&self, resid: usize) -> ::windows_core::Result<()>;
    fn DestroyResource(&self, resid: usize) -> ::windows_core::Result<()>;
    fn DestroyResourceS(&self, resid: *mut u16) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDispenserDriver {}
#[cfg(feature = "Win32_Foundation")]
impl IDispenserDriver_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>() -> IDispenserDriver_Vtbl {
        unsafe extern "system" fn CreateResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, presid: *mut usize, psecsfreebeforedestroy: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateResource(::core::mem::transmute_copy(&restypid), ::core::mem::transmute_copy(&presid), ::core::mem::transmute_copy(&psecsfreebeforedestroy)).into()
        }
        unsafe extern "system" fn RateResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, restypid: usize, resid: usize, frequirestransactionenlistment: super::super::Foundation::BOOL, prating: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RateResource(::core::mem::transmute_copy(&restypid), ::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&frequirestransactionenlistment), ::core::mem::transmute_copy(&prating)).into()
        }
        unsafe extern "system" fn EnlistResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize, transid: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnlistResource(::core::mem::transmute_copy(&resid), ::core::mem::transmute_copy(&transid)).into()
        }
        unsafe extern "system" fn ResetResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetResource(::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyResource(::core::mem::transmute_copy(&resid)).into()
        }
        unsafe extern "system" fn DestroyResourceS<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserDriver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resid: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyResourceS(::core::mem::transmute_copy(&resid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateResource: CreateResource::<Identity, Impl, OFFSET>,
            RateResource: RateResource::<Identity, Impl, OFFSET>,
            EnlistResource: EnlistResource::<Identity, Impl, OFFSET>,
            ResetResource: ResetResource::<Identity, Impl, OFFSET>,
            DestroyResource: DestroyResource::<Identity, Impl, OFFSET>,
            DestroyResourceS: DestroyResourceS::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDispenserDriver as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IDispenserManager_Impl: Sized {
    fn RegisterDispenser(&self, __midl__idispensermanager0000: ::core::option::Option<&IDispenserDriver>, szdispensername: &::windows_core::PCWSTR) -> ::windows_core::Result<IHolder>;
    fn GetContext(&self, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDispenserManager {}
impl IDispenserManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: isize>() -> IDispenserManager_Vtbl {
        unsafe extern "system" fn RegisterDispenser<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0000: *mut ::core::ffi::c_void, szdispensername: ::windows_core::PCWSTR, __midl__idispensermanager0001: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterDispenser(::windows_core::from_raw_borrowed(&__midl__idispensermanager0000), ::core::mem::transmute(&szdispensername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(__midl__idispensermanager0001, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDispenserManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__idispensermanager0002: *mut usize, __midl__idispensermanager0003: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContext(::core::mem::transmute_copy(&__midl__idispensermanager0002), ::core::mem::transmute_copy(&__midl__idispensermanager0003)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterDispenser: RegisterDispenser::<Identity, Impl, OFFSET>,
            GetContext: GetContext::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDispenserManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IEnumNames_Impl: Sized {
    fn Next(&self, celt: u32, rgname: *mut ::windows_core::BSTR, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumNames>;
}
impl ::windows_core::RuntimeName for IEnumNames {}
impl IEnumNames_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>() -> IEnumNames_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgname), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNames_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumNames as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEventServerTrace_Impl: Sized + super::Com::IDispatch_Impl {
    fn StartTraceGuid(&self, bstrguidevent: &::windows_core::BSTR, bstrguidfilter: &::windows_core::BSTR, lpidfilter: i32) -> ::windows_core::Result<()>;
    fn StopTraceGuid(&self, bstrguidevent: &::windows_core::BSTR, bstrguidfilter: &::windows_core::BSTR, lpidfilter: i32) -> ::windows_core::Result<()>;
    fn EnumTraceGuid(&self, plcntguids: *mut i32, pbstrguidlist: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IEventServerTrace {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEventServerTrace_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>() -> IEventServerTrace_Vtbl {
        unsafe extern "system" fn StartTraceGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrguidfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpidfilter: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartTraceGuid(::core::mem::transmute(&bstrguidevent), ::core::mem::transmute(&bstrguidfilter), ::core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn StopTraceGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrguidevent: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrguidfilter: ::std::mem::MaybeUninit<::windows_core::BSTR>, lpidfilter: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopTraceGuid(::core::mem::transmute(&bstrguidevent), ::core::mem::transmute(&bstrguidfilter), ::core::mem::transmute_copy(&lpidfilter)).into()
        }
        unsafe extern "system" fn EnumTraceGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEventServerTrace_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcntguids: *mut i32, pbstrguidlist: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEventServerTrace as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IGetAppTrackerData_Impl: Sized {
    fn GetApplicationProcesses(&self, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows_core::Result<()>;
    fn GetApplicationProcessDetails(&self, applicationinstanceid: *const ::windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetApplicationsInProcess(&self, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows_core::Result<()>;
    fn GetComponentsInProcess(&self, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows_core::Result<()>;
    fn GetComponentDetails(&self, applicationinstanceid: *const ::windows_core::GUID, processid: u32, clsid: *const ::windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows_core::Result<()>;
    fn GetTrackerDataAsCollectionObject(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetSuggestedPollingInterval(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IGetAppTrackerData {}
#[cfg(feature = "Win32_Foundation")]
impl IGetAppTrackerData_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>() -> IGetAppTrackerData_Vtbl {
        unsafe extern "system" fn GetApplicationProcesses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numapplicationprocesses: *mut u32, applicationprocesses: *mut *mut ApplicationProcessSummary) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationProcesses(::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&applicationid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numapplicationprocesses), ::core::mem::transmute_copy(&applicationprocesses)).into()
        }
        unsafe extern "system" fn GetApplicationProcessDetails<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, flags: u32, summary: *mut ApplicationProcessSummary, statistics: *mut ApplicationProcessStatistics, recycleinfo: *mut ApplicationProcessRecycleInfo, anycomponentshangmonitored: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationProcessDetails(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&recycleinfo), ::core::mem::transmute_copy(&anycomponentshangmonitored)).into()
        }
        unsafe extern "system" fn GetApplicationsInProcess<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, flags: u32, numapplicationsinprocess: *mut u32, applications: *mut *mut ApplicationSummary) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationsInProcess(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numapplicationsinprocess), ::core::mem::transmute_copy(&applications)).into()
        }
        unsafe extern "system" fn GetComponentsInProcess<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, partitionid: *const ::windows_core::GUID, applicationid: *const ::windows_core::GUID, flags: u32, numcomponentsinprocess: *mut u32, components: *mut *mut ComponentSummary) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetComponentsInProcess(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&partitionid), ::core::mem::transmute_copy(&applicationid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&numcomponentsinprocess), ::core::mem::transmute_copy(&components)).into()
        }
        unsafe extern "system" fn GetComponentDetails<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, applicationinstanceid: *const ::windows_core::GUID, processid: u32, clsid: *const ::windows_core::GUID, flags: u32, summary: *mut ComponentSummary, statistics: *mut ComponentStatistics, hangmonitorinfo: *mut ComponentHangMonitorInfo) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetComponentDetails(::core::mem::transmute_copy(&applicationinstanceid), ::core::mem::transmute_copy(&processid), ::core::mem::transmute_copy(&clsid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&summary), ::core::mem::transmute_copy(&statistics), ::core::mem::transmute_copy(&hangmonitorinfo)).into()
        }
        unsafe extern "system" fn GetTrackerDataAsCollectionObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, toplevelcollection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTrackerDataAsCollectionObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(toplevelcollection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSuggestedPollingInterval<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetAppTrackerData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pollingintervalinseconds: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSuggestedPollingInterval() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pollingintervalinseconds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetApplicationProcesses: GetApplicationProcesses::<Identity, Impl, OFFSET>,
            GetApplicationProcessDetails: GetApplicationProcessDetails::<Identity, Impl, OFFSET>,
            GetApplicationsInProcess: GetApplicationsInProcess::<Identity, Impl, OFFSET>,
            GetComponentsInProcess: GetComponentsInProcess::<Identity, Impl, OFFSET>,
            GetComponentDetails: GetComponentDetails::<Identity, Impl, OFFSET>,
            GetTrackerDataAsCollectionObject: GetTrackerDataAsCollectionObject::<Identity, Impl, OFFSET>,
            GetSuggestedPollingInterval: GetSuggestedPollingInterval::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGetAppTrackerData as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGetContextProperties_Impl: Sized {
    fn Count(&self, plcount: *mut i32) -> ::windows_core::Result<()>;
    fn GetProperty(&self, name: &::windows_core::BSTR, pproperty: *mut super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn EnumNames(&self) -> ::windows_core::Result<IEnumNames>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IGetContextProperties {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGetContextProperties_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>() -> IGetContextProperties_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Count(::core::mem::transmute_copy(&plcount)).into()
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pproperty: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute(&name), ::core::mem::transmute_copy(&pproperty)).into()
        }
        unsafe extern "system" fn EnumNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetContextProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            EnumNames: EnumNames::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGetContextProperties as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IGetSecurityCallContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetSecurityCallContext(&self) -> ::windows_core::Result<ISecurityCallContext>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IGetSecurityCallContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IGetSecurityCallContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetSecurityCallContext_Impl, const OFFSET: isize>() -> IGetSecurityCallContext_Vtbl {
        unsafe extern "system" fn GetSecurityCallContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IGetSecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSecurityCallContext() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetSecurityCallContext: GetSecurityCallContext::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IGetSecurityCallContext as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IHolder_Impl: Sized {
    fn AllocResource(&self, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows_core::Result<()>;
    fn FreeResource(&self, __midl__iholder0002: usize) -> ::windows_core::Result<()>;
    fn TrackResource(&self, __midl__iholder0003: usize) -> ::windows_core::Result<()>;
    fn TrackResourceS(&self, __midl__iholder0004: *mut u16) -> ::windows_core::Result<()>;
    fn UntrackResource(&self, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn UntrackResourceS(&self, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn Close(&self) -> ::windows_core::Result<()>;
    fn RequestDestroyResource(&self, __midl__iholder0009: usize) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IHolder {}
#[cfg(feature = "Win32_Foundation")]
impl IHolder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>() -> IHolder_Vtbl {
        unsafe extern "system" fn AllocResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0000: usize, __midl__iholder0001: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AllocResource(::core::mem::transmute_copy(&__midl__iholder0000), ::core::mem::transmute_copy(&__midl__iholder0001)).into()
        }
        unsafe extern "system" fn FreeResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0002: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FreeResource(::core::mem::transmute_copy(&__midl__iholder0002)).into()
        }
        unsafe extern "system" fn TrackResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0003: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TrackResource(::core::mem::transmute_copy(&__midl__iholder0003)).into()
        }
        unsafe extern "system" fn TrackResourceS<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0004: *mut u16) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TrackResourceS(::core::mem::transmute_copy(&__midl__iholder0004)).into()
        }
        unsafe extern "system" fn UntrackResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0005: usize, __midl__iholder0006: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UntrackResource(::core::mem::transmute_copy(&__midl__iholder0005), ::core::mem::transmute_copy(&__midl__iholder0006)).into()
        }
        unsafe extern "system" fn UntrackResourceS<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0007: *mut u16, __midl__iholder0008: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UntrackResourceS(::core::mem::transmute_copy(&__midl__iholder0007), ::core::mem::transmute_copy(&__midl__iholder0008)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        unsafe extern "system" fn RequestDestroyResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__iholder0009: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestDestroyResource(::core::mem::transmute_copy(&__midl__iholder0009)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IHolder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ILBEvents_Impl: Sized {
    fn TargetUp(&self, bstrservername: &::windows_core::BSTR, bstrclsideng: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn TargetDown(&self, bstrservername: &::windows_core::BSTR, bstrclsideng: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn EngineDefined(&self, bstrpropname: &::windows_core::BSTR, varpropvalue: *const super::Variant::VARIANT, bstrclsideng: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ILBEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ILBEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>() -> ILBEvents_Vtbl {
        unsafe extern "system" fn TargetUp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsideng: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TargetUp(::core::mem::transmute(&bstrservername), ::core::mem::transmute(&bstrclsideng)).into()
        }
        unsafe extern "system" fn TargetDown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservername: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrclsideng: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TargetDown(::core::mem::transmute(&bstrservername), ::core::mem::transmute(&bstrclsideng)).into()
        }
        unsafe extern "system" fn EngineDefined<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILBEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropname: ::std::mem::MaybeUninit<::windows_core::BSTR>, varpropvalue: *const super::Variant::VARIANT, bstrclsideng: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EngineDefined(::core::mem::transmute(&bstrpropname), ::core::mem::transmute_copy(&varpropvalue), ::core::mem::transmute(&bstrclsideng)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            TargetUp: TargetUp::<Identity, Impl, OFFSET>,
            TargetDown: TargetDown::<Identity, Impl, OFFSET>,
            EngineDefined: EngineDefined::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ILBEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IMTSActivity_Impl: Sized {
    fn SynchronousCall(&self, pcall: ::core::option::Option<&IMTSCall>) -> ::windows_core::Result<()>;
    fn AsyncCall(&self, pcall: ::core::option::Option<&IMTSCall>) -> ::windows_core::Result<()>;
    fn Reserved1(&self);
    fn BindToCurrentThread(&self) -> ::windows_core::Result<()>;
    fn UnbindFromThread(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IMTSActivity {}
impl IMTSActivity_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>() -> IMTSActivity_Vtbl {
        unsafe extern "system" fn SynchronousCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCall(::windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn AsyncCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsyncCall(::windows_core::from_raw_borrowed(&pcall)).into()
        }
        unsafe extern "system" fn Reserved1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved1()
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToCurrentThread().into()
        }
        unsafe extern "system" fn UnbindFromThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnbindFromThread().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, Impl, OFFSET>,
            AsyncCall: AsyncCall::<Identity, Impl, OFFSET>,
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, Impl, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMTSActivity as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IMTSCall_Impl: Sized {
    fn OnCall(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IMTSCall {}
impl IMTSCall_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSCall_Impl, const OFFSET: isize>() -> IMTSCall_Vtbl {
        unsafe extern "system" fn OnCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCall().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMTSCall as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMTSLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetEventDispatcher(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMTSLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMTSLocator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSLocator_Impl, const OFFSET: isize>() -> IMTSLocator_Vtbl {
        unsafe extern "system" fn GetEventDispatcher<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMTSLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetEventDispatcher() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), GetEventDispatcher: GetEventDispatcher::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMTSLocator as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedActivationEvents_Impl: Sized {
    fn CreateManagedStub(&self, pinfo: ::core::option::Option<&IManagedObjectInfo>, fdist: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn DestroyManagedStub(&self, pinfo: ::core::option::Option<&IManagedObjectInfo>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IManagedActivationEvents {}
#[cfg(feature = "Win32_Foundation")]
impl IManagedActivationEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: isize>() -> IManagedActivationEvents_Vtbl {
        unsafe extern "system" fn CreateManagedStub<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void, fdist: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateManagedStub(::windows_core::from_raw_borrowed(&pinfo), ::core::mem::transmute_copy(&fdist)).into()
        }
        unsafe extern "system" fn DestroyManagedStub<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedActivationEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DestroyManagedStub(::windows_core::from_raw_borrowed(&pinfo)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateManagedStub: CreateManagedStub::<Identity, Impl, OFFSET>,
            DestroyManagedStub: DestroyManagedStub::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IManagedActivationEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedObjectInfo_Impl: Sized {
    fn GetIUnknown(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetIObjectControl(&self) -> ::windows_core::Result<IObjectControl>;
    fn SetInPool(&self, binpool: super::super::Foundation::BOOL, ppooledobj: ::core::option::Option<&IManagedPooledObj>) -> ::windows_core::Result<()>;
    fn SetWrapperStrength(&self, bstrong: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IManagedObjectInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IManagedObjectInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>() -> IManagedObjectInfo_Vtbl {
        unsafe extern "system" fn GetIUnknown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIUnknown() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIObjectControl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctrl: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetIObjectControl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pctrl, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInPool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binpool: super::super::Foundation::BOOL, ppooledobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInPool(::core::mem::transmute_copy(&binpool), ::windows_core::from_raw_borrowed(&ppooledobj)).into()
        }
        unsafe extern "system" fn SetWrapperStrength<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedObjectInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrong: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWrapperStrength(::core::mem::transmute_copy(&bstrong)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetIUnknown: GetIUnknown::<Identity, Impl, OFFSET>,
            GetIObjectControl: GetIObjectControl::<Identity, Impl, OFFSET>,
            SetInPool: SetInPool::<Identity, Impl, OFFSET>,
            SetWrapperStrength: SetWrapperStrength::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IManagedObjectInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IManagedPoolAction_Impl: Sized {
    fn LastRelease(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IManagedPoolAction {}
impl IManagedPoolAction_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedPoolAction_Impl, const OFFSET: isize>() -> IManagedPoolAction_Vtbl {
        unsafe extern "system" fn LastRelease<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedPoolAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LastRelease().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), LastRelease: LastRelease::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IManagedPoolAction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IManagedPooledObj_Impl: Sized {
    fn SetHeld(&self, m_bheld: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IManagedPooledObj {}
#[cfg(feature = "Win32_Foundation")]
impl IManagedPooledObj_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedPooledObj_Impl, const OFFSET: isize>() -> IManagedPooledObj_Vtbl {
        unsafe extern "system" fn SetHeld<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IManagedPooledObj_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, m_bheld: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetHeld(::core::mem::transmute_copy(&m_bheld)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetHeld: SetHeld::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IManagedPooledObj as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMessageMover_Impl: Sized + super::Com::IDispatch_Impl {
    fn SourcePath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetSourcePath(&self, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DestPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDestPath(&self, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn CommitBatchSize(&self) -> ::windows_core::Result<i32>;
    fn SetCommitBatchSize(&self, newval: i32) -> ::windows_core::Result<()>;
    fn MoveMessages(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMessageMover {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMessageMover_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>() -> IMessageMover_Vtbl {
        unsafe extern "system" fn SourcePath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SourcePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSourcePath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSourcePath(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn DestPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DestPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDestPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDestPath(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn CommitBatchSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommitBatchSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCommitBatchSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCommitBatchSize(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MoveMessages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMessageMover_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmessagesmoved: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MoveMessages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plmessagesmoved, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMessageMover as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMtsEventInfo_Impl: Sized + super::Com::IDispatch_Impl {
    fn Names(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn DisplayName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EventID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn get_Value(&self, skey: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMtsEventInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMtsEventInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>() -> IMtsEventInfo_Vtbl {
        unsafe extern "system" fn Names<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Names() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(punk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sdisplayname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sdisplayname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sguideventid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(sguideventid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEventInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, skey: ::std::mem::MaybeUninit<::windows_core::BSTR>, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Value(::core::mem::transmute(&skey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMtsEventInfo as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMtsEvents_Impl: Sized + super::Com::IDispatch_Impl {
    fn PackageName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PackageGuid(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn PostEvent(&self, vevent: *const super::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn FireEvents(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetProcessID(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMtsEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMtsEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>() -> IMtsEvents_Vtbl {
        unsafe extern "system" fn PackageName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PackageName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PackageGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PackageGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vevent: *const super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PostEvent(::core::mem::transmute_copy(&vevent)).into()
        }
        unsafe extern "system" fn FireEvents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FireEvents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProcessID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProcessID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(id, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMtsEvents as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMtsGrp_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Item(&self, lindex: i32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Refresh(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMtsGrp {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMtsGrp_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>() -> IMtsGrp_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, ppunkdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkdispatcher, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Refresh<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMtsGrp_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMtsGrp as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IObjPool_Impl: Sized {
    fn Reserved1(&self);
    fn Reserved2(&self);
    fn Reserved3(&self);
    fn Reserved4(&self);
    fn PutEndTx(&self, pobj: ::core::option::Option<&::windows_core::IUnknown>);
    fn Reserved5(&self);
    fn Reserved6(&self);
}
impl ::windows_core::RuntimeName for IObjPool {}
impl IObjPool_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>() -> IObjPool_Vtbl {
        unsafe extern "system" fn Reserved1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved1()
        }
        unsafe extern "system" fn Reserved2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved2()
        }
        unsafe extern "system" fn Reserved3<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved3()
        }
        unsafe extern "system" fn Reserved4<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved4()
        }
        unsafe extern "system" fn PutEndTx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pobj: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutEndTx(::windows_core::from_raw_borrowed(&pobj))
        }
        unsafe extern "system" fn Reserved5<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved5()
        }
        unsafe extern "system" fn Reserved6<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjPool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved6()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reserved1: Reserved1::<Identity, Impl, OFFSET>,
            Reserved2: Reserved2::<Identity, Impl, OFFSET>,
            Reserved3: Reserved3::<Identity, Impl, OFFSET>,
            Reserved4: Reserved4::<Identity, Impl, OFFSET>,
            PutEndTx: PutEndTx::<Identity, Impl, OFFSET>,
            Reserved5: Reserved5::<Identity, Impl, OFFSET>,
            Reserved6: Reserved6::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjPool as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectConstruct_Impl: Sized {
    fn Construct(&self, pctorobj: ::core::option::Option<&super::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IObjectConstruct {}
#[cfg(feature = "Win32_System_Com")]
impl IObjectConstruct_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstruct_Impl, const OFFSET: isize>() -> IObjectConstruct_Vtbl {
        unsafe extern "system" fn Construct<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstruct_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pctorobj: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Construct(::windows_core::from_raw_borrowed(&pctorobj)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Construct: Construct::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectConstruct as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IObjectConstructString_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConstructString(&self, pval: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IObjectConstructString {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IObjectConstructString_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstructString_Impl, const OFFSET: isize>() -> IObjectConstructString_Vtbl {
        unsafe extern "system" fn ConstructString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectConstructString_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConstructString(::core::mem::transmute_copy(&pval)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ConstructString: ConstructString::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectConstructString as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContext_Impl: Sized {
    fn CreateInstance(&self, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn SetComplete(&self) -> ::windows_core::Result<()>;
    fn SetAbort(&self) -> ::windows_core::Result<()>;
    fn EnableCommit(&self) -> ::windows_core::Result<()>;
    fn DisableCommit(&self) -> ::windows_core::Result<()>;
    fn IsInTransaction(&self) -> super::super::Foundation::BOOL;
    fn IsSecurityEnabled(&self) -> super::super::Foundation::BOOL;
    fn IsCallerInRole(&self, bstrrole: &::windows_core::BSTR, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IObjectContext {}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>() -> IObjectContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn SetComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetComplete().into()
        }
        unsafe extern "system" fn SetAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAbort().into()
        }
        unsafe extern "system" fn EnableCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableCommit().into()
        }
        unsafe extern "system" fn DisableCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableCommit().into()
        }
        unsafe extern "system" fn IsInTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsInTransaction()
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSecurityEnabled()
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfisinrole: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsCallerInRole(::core::mem::transmute(&bstrrole), ::core::mem::transmute_copy(&pfisinrole)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectContext as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IObjectContextActivity_Impl: Sized {
    fn GetActivityId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IObjectContextActivity {}
impl IObjectContextActivity_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextActivity_Impl, const OFFSET: isize>() -> IObjectContextActivity_Vtbl {
        unsafe extern "system" fn GetActivityId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetActivityId(::core::mem::transmute_copy(&pguid)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetActivityId: GetActivityId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectContextActivity as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfo_Impl: Sized {
    fn IsInTransaction(&self) -> super::super::Foundation::BOOL;
    fn GetTransaction(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetTransactionId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetActivityId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetContextId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IObjectContextInfo {}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>() -> IObjectContextInfo_Vtbl {
        unsafe extern "system" fn IsInTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsInTransaction()
        }
        unsafe extern "system" fn GetTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptrans: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptrans, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTransactionId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTransactionId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetActivityId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetActivityId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetContextId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetContextId(::core::mem::transmute_copy(&pguid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsInTransaction: IsInTransaction::<Identity, Impl, OFFSET>,
            GetTransaction: GetTransaction::<Identity, Impl, OFFSET>,
            GetTransactionId: GetTransactionId::<Identity, Impl, OFFSET>,
            GetActivityId: GetActivityId::<Identity, Impl, OFFSET>,
            GetContextId: GetContextId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectContextInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectContextInfo2_Impl: Sized + IObjectContextInfo_Impl {
    fn GetPartitionId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetApplicationId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetApplicationInstanceId(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IObjectContextInfo2 {}
#[cfg(feature = "Win32_Foundation")]
impl IObjectContextInfo2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>() -> IObjectContextInfo2_Vtbl {
        unsafe extern "system" fn GetPartitionId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPartitionId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetApplicationId(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetApplicationInstanceId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextInfo2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectContextInfo2 as ::windows_core::ComInterface>::IID || iid == &<IObjectContextInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IObjectContextTip_Impl: Sized {
    fn GetTipUrl(&self, ptipurl: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IObjectContextTip {}
impl IObjectContextTip_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextTip_Impl, const OFFSET: isize>() -> IObjectContextTip_Vtbl {
        unsafe extern "system" fn GetTipUrl<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectContextTip_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptipurl: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTipUrl(::core::mem::transmute_copy(&ptipurl)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetTipUrl: GetTipUrl::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectContextTip as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IObjectControl_Impl: Sized {
    fn Activate(&self) -> ::windows_core::Result<()>;
    fn Deactivate(&self);
    fn CanBePooled(&self) -> super::super::Foundation::BOOL;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IObjectControl {}
#[cfg(feature = "Win32_Foundation")]
impl IObjectControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>() -> IObjectControl_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate().into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate()
        }
        unsafe extern "system" fn CanBePooled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanBePooled()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CanBePooled: CanBePooled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IObjectControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IPlaybackControl_Impl: Sized {
    fn FinalClientRetry(&self) -> ::windows_core::Result<()>;
    fn FinalServerRetry(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IPlaybackControl {}
impl IPlaybackControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: isize>() -> IPlaybackControl_Vtbl {
        unsafe extern "system" fn FinalClientRetry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinalClientRetry().into()
        }
        unsafe extern "system" fn FinalServerRetry<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPlaybackControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FinalServerRetry().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FinalClientRetry: FinalClientRetry::<Identity, Impl, OFFSET>,
            FinalServerRetry: FinalServerRetry::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPlaybackControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IPoolManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn ShutdownPool(&self, clsidorprogid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IPoolManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IPoolManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPoolManager_Impl, const OFFSET: isize>() -> IPoolManager_Vtbl {
        unsafe extern "system" fn ShutdownPool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IPoolManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clsidorprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ShutdownPool(::core::mem::transmute(&clsidorprogid)).into()
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ShutdownPool: ShutdownPool::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IPoolManager as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IProcessInitializer_Impl: Sized {
    fn Startup(&self, punkprocesscontrol: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn Shutdown(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IProcessInitializer {}
impl IProcessInitializer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: isize>() -> IProcessInitializer_Vtbl {
        unsafe extern "system" fn Startup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkprocesscontrol: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Startup(::windows_core::from_raw_borrowed(&punkprocesscontrol)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProcessInitializer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Startup: Startup::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProcessInitializer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISecurityCallContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn get_Item(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn IsCallerInRole(&self, bstrrole: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsSecurityEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsUserInRole(&self, puser: *const super::Variant::VARIANT, bstrrole: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISecurityCallContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISecurityCallContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>() -> ISecurityCallContext_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCallerInRole(::core::mem::transmute(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSecurityEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfisenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsUserInRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puser: *const super::Variant::VARIANT, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, pfinrole: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUserInRole(::core::mem::transmute_copy(&puser), ::core::mem::transmute(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISecurityCallContext as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISecurityCallersColl_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn get_Item(&self, lindex: i32) -> ::windows_core::Result<ISecurityIdentityColl>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISecurityCallersColl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISecurityCallersColl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>() -> ISecurityCallersColl_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lindex: i32, pobj: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&lindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobj, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityCallersColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISecurityCallersColl as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISecurityIdentityColl_Impl: Sized + super::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn get_Item(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISecurityIdentityColl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISecurityIdentityColl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>() -> ISecurityIdentityColl_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityIdentityColl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISecurityIdentityColl as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISecurityProperty_Impl: Sized {
    fn GetDirectCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows_core::Result<()>;
    fn GetOriginalCreatorSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows_core::Result<()>;
    fn GetDirectCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows_core::Result<()>;
    fn GetOriginalCallerSID(&self, psid: *mut super::super::Foundation::PSID) -> ::windows_core::Result<()>;
    fn ReleaseSID(&self, psid: super::super::Foundation::PSID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISecurityProperty {}
#[cfg(feature = "Win32_Foundation")]
impl ISecurityProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>() -> ISecurityProperty_Vtbl {
        unsafe extern "system" fn GetDirectCreatorSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDirectCreatorSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCreatorSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOriginalCreatorSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetDirectCallerSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDirectCallerSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn GetOriginalCallerSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: *mut super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOriginalCallerSID(::core::mem::transmute_copy(&psid)).into()
        }
        unsafe extern "system" fn ReleaseSID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psid: super::super::Foundation::PSID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseSID(::core::mem::transmute_copy(&psid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDirectCreatorSID: GetDirectCreatorSID::<Identity, Impl, OFFSET>,
            GetOriginalCreatorSID: GetOriginalCreatorSID::<Identity, Impl, OFFSET>,
            GetDirectCallerSID: GetDirectCallerSID::<Identity, Impl, OFFSET>,
            GetOriginalCallerSID: GetOriginalCallerSID::<Identity, Impl, OFFSET>,
            ReleaseSID: ReleaseSID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISecurityProperty as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ISelectCOMLBServer_Impl: Sized {
    fn Init(&self) -> ::windows_core::Result<()>;
    fn GetLBServer(&self, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISelectCOMLBServer {}
impl ISelectCOMLBServer_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>() -> ISelectCOMLBServer_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init().into()
        }
        unsafe extern "system" fn GetLBServer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISelectCOMLBServer_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLBServer(::windows_core::from_raw_borrowed(&punk)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetLBServer: GetLBServer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISelectCOMLBServer as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ISendMethodEvents_Impl: Sized {
    fn SendMethodCall(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32) -> ::windows_core::Result<()>;
    fn SendMethodReturn(&self, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32, hrcall: ::windows_core::HRESULT, hrserver: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISendMethodEvents {}
impl ISendMethodEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: isize>() -> ISendMethodEvents_Vtbl {
        unsafe extern "system" fn SendMethodCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMethodCall(::core::mem::transmute_copy(&pidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwmeth)).into()
        }
        unsafe extern "system" fn SendMethodReturn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISendMethodEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pidentity: *const ::core::ffi::c_void, riid: *const ::windows_core::GUID, dwmeth: u32, hrcall: ::windows_core::HRESULT, hrserver: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendMethodReturn(::core::mem::transmute_copy(&pidentity), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&dwmeth), ::core::mem::transmute_copy(&hrcall), ::core::mem::transmute_copy(&hrserver)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendMethodCall: SendMethodCall::<Identity, Impl, OFFSET>,
            SendMethodReturn: SendMethodReturn::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISendMethodEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceActivity_Impl: Sized {
    fn SynchronousCall(&self, piservicecall: ::core::option::Option<&IServiceCall>) -> ::windows_core::Result<()>;
    fn AsynchronousCall(&self, piservicecall: ::core::option::Option<&IServiceCall>) -> ::windows_core::Result<()>;
    fn BindToCurrentThread(&self) -> ::windows_core::Result<()>;
    fn UnbindFromThread(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceActivity {}
impl IServiceActivity_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>() -> IServiceActivity_Vtbl {
        unsafe extern "system" fn SynchronousCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SynchronousCall(::windows_core::from_raw_borrowed(&piservicecall)).into()
        }
        unsafe extern "system" fn AsynchronousCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, piservicecall: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AsynchronousCall(::windows_core::from_raw_borrowed(&piservicecall)).into()
        }
        unsafe extern "system" fn BindToCurrentThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindToCurrentThread().into()
        }
        unsafe extern "system" fn UnbindFromThread<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceActivity_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnbindFromThread().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SynchronousCall: SynchronousCall::<Identity, Impl, OFFSET>,
            AsynchronousCall: AsynchronousCall::<Identity, Impl, OFFSET>,
            BindToCurrentThread: BindToCurrentThread::<Identity, Impl, OFFSET>,
            UnbindFromThread: UnbindFromThread::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceActivity as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceCall_Impl: Sized {
    fn OnCall(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceCall {}
impl IServiceCall_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceCall_Impl, const OFFSET: isize>() -> IServiceCall_Vtbl {
        unsafe extern "system" fn OnCall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceCall_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnCall().into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnCall: OnCall::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceCall as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceComTIIntrinsicsConfig_Impl: Sized {
    fn ComTIIntrinsicsConfig(&self, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceComTIIntrinsicsConfig {}
impl IServiceComTIIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: isize>() -> IServiceComTIIntrinsicsConfig_Vtbl {
        unsafe extern "system" fn ComTIIntrinsicsConfig<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceComTIIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, comtiintrinsicsconfig: CSC_COMTIIntrinsicsConfig) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ComTIIntrinsicsConfig(::core::mem::transmute_copy(&comtiintrinsicsconfig)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ComTIIntrinsicsConfig: ComTIIntrinsicsConfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceComTIIntrinsicsConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceIISIntrinsicsConfig_Impl: Sized {
    fn IISIntrinsicsConfig(&self, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceIISIntrinsicsConfig {}
impl IServiceIISIntrinsicsConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceIISIntrinsicsConfig_Impl, const OFFSET: isize>() -> IServiceIISIntrinsicsConfig_Vtbl {
        unsafe extern "system" fn IISIntrinsicsConfig<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceIISIntrinsicsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iisintrinsicsconfig: CSC_IISIntrinsicsConfig) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IISIntrinsicsConfig(::core::mem::transmute_copy(&iisintrinsicsconfig)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), IISIntrinsicsConfig: IISIntrinsicsConfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceIISIntrinsicsConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceInheritanceConfig_Impl: Sized {
    fn ContainingContextTreatment(&self, inheritanceconfig: CSC_InheritanceConfig) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceInheritanceConfig {}
impl IServiceInheritanceConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceInheritanceConfig_Impl, const OFFSET: isize>() -> IServiceInheritanceConfig_Vtbl {
        unsafe extern "system" fn ContainingContextTreatment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceInheritanceConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inheritanceconfig: CSC_InheritanceConfig) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ContainingContextTreatment(::core::mem::transmute_copy(&inheritanceconfig)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ContainingContextTreatment: ContainingContextTreatment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceInheritanceConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServicePartitionConfig_Impl: Sized {
    fn PartitionConfig(&self, partitionconfig: CSC_PartitionConfig) -> ::windows_core::Result<()>;
    fn PartitionID(&self, guidpartitionid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServicePartitionConfig {}
impl IServicePartitionConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: isize>() -> IServicePartitionConfig_Vtbl {
        unsafe extern "system" fn PartitionConfig<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, partitionconfig: CSC_PartitionConfig) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PartitionConfig(::core::mem::transmute_copy(&partitionconfig)).into()
        }
        unsafe extern "system" fn PartitionID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePartitionConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guidpartitionid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PartitionID(::core::mem::transmute_copy(&guidpartitionid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PartitionConfig: PartitionConfig::<Identity, Impl, OFFSET>,
            PartitionID: PartitionID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServicePartitionConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServicePool_Impl: Sized {
    fn Initialize(&self, ppoolconfig: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetObject(&self, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Shutdown(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServicePool {}
impl IServicePool_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>() -> IServicePool_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoolconfig: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&ppoolconfig)).into()
        }
        unsafe extern "system" fn GetObject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetObject(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Shutdown<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Shutdown().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            GetObject: GetObject::<Identity, Impl, OFFSET>,
            Shutdown: Shutdown::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServicePool as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IServicePoolConfig_Impl: Sized {
    fn SetMaxPoolSize(&self, dwmaxpool: u32) -> ::windows_core::Result<()>;
    fn MaxPoolSize(&self, pdwmaxpool: *mut u32) -> ::windows_core::Result<()>;
    fn SetMinPoolSize(&self, dwminpool: u32) -> ::windows_core::Result<()>;
    fn MinPoolSize(&self, pdwminpool: *mut u32) -> ::windows_core::Result<()>;
    fn SetCreationTimeout(&self, dwcreationtimeout: u32) -> ::windows_core::Result<()>;
    fn CreationTimeout(&self, pdwcreationtimeout: *mut u32) -> ::windows_core::Result<()>;
    fn SetTransactionAffinity(&self, ftxaffinity: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn TransactionAffinity(&self, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn SetClassFactory(&self, pfactory: ::core::option::Option<&super::Com::IClassFactory>) -> ::windows_core::Result<()>;
    fn ClassFactory(&self) -> ::windows_core::Result<super::Com::IClassFactory>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IServicePoolConfig {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IServicePoolConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>() -> IServicePoolConfig_Vtbl {
        unsafe extern "system" fn SetMaxPoolSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwmaxpool: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxPoolSize(::core::mem::transmute_copy(&dwmaxpool)).into()
        }
        unsafe extern "system" fn MaxPoolSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwmaxpool: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MaxPoolSize(::core::mem::transmute_copy(&pdwmaxpool)).into()
        }
        unsafe extern "system" fn SetMinPoolSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwminpool: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinPoolSize(::core::mem::transmute_copy(&dwminpool)).into()
        }
        unsafe extern "system" fn MinPoolSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwminpool: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MinPoolSize(::core::mem::transmute_copy(&pdwminpool)).into()
        }
        unsafe extern "system" fn SetCreationTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwcreationtimeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCreationTimeout(::core::mem::transmute_copy(&dwcreationtimeout)).into()
        }
        unsafe extern "system" fn CreationTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcreationtimeout: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreationTimeout(::core::mem::transmute_copy(&pdwcreationtimeout)).into()
        }
        unsafe extern "system" fn SetTransactionAffinity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftxaffinity: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransactionAffinity(::core::mem::transmute_copy(&ftxaffinity)).into()
        }
        unsafe extern "system" fn TransactionAffinity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pftxaffinity: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransactionAffinity(::core::mem::transmute_copy(&pftxaffinity)).into()
        }
        unsafe extern "system" fn SetClassFactory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClassFactory(::windows_core::from_raw_borrowed(&pfactory)).into()
        }
        unsafe extern "system" fn ClassFactory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServicePoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfactory: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClassFactory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfactory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServicePoolConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceSxsConfig_Impl: Sized {
    fn SxsConfig(&self, scsconfig: CSC_SxsConfig) -> ::windows_core::Result<()>;
    fn SxsName(&self, szsxsname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn SxsDirectory(&self, szsxsdirectory: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceSxsConfig {}
impl IServiceSxsConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>() -> IServiceSxsConfig_Vtbl {
        unsafe extern "system" fn SxsConfig<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, scsconfig: CSC_SxsConfig) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SxsConfig(::core::mem::transmute_copy(&scsconfig)).into()
        }
        unsafe extern "system" fn SxsName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SxsName(::core::mem::transmute(&szsxsname)).into()
        }
        unsafe extern "system" fn SxsDirectory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSxsConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szsxsdirectory: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SxsDirectory(::core::mem::transmute(&szsxsdirectory)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SxsConfig: SxsConfig::<Identity, Impl, OFFSET>,
            SxsName: SxsName::<Identity, Impl, OFFSET>,
            SxsDirectory: SxsDirectory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceSxsConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceSynchronizationConfig_Impl: Sized {
    fn ConfigureSynchronization(&self, synchconfig: CSC_SynchronizationConfig) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceSynchronizationConfig {}
impl IServiceSynchronizationConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSynchronizationConfig_Impl, const OFFSET: isize>() -> IServiceSynchronizationConfig_Vtbl {
        unsafe extern "system" fn ConfigureSynchronization<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSynchronizationConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, synchconfig: CSC_SynchronizationConfig) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureSynchronization(::core::mem::transmute_copy(&synchconfig)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConfigureSynchronization: ConfigureSynchronization::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceSynchronizationConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_DistributedTransactionCoordinator\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceSysTxnConfig_Impl: Sized + IServiceTransactionConfig_Impl {
    fn ConfigureBYOTSysTxn(&self, ptxproxy: ::core::option::Option<&ITransactionProxy>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows_core::RuntimeName for IServiceSysTxnConfig {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl IServiceSysTxnConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSysTxnConfig_Impl, const OFFSET: isize>() -> IServiceSysTxnConfig_Vtbl {
        unsafe extern "system" fn ConfigureBYOTSysTxn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceSysTxnConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxproxy: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureBYOTSysTxn(::windows_core::from_raw_borrowed(&ptxproxy)).into()
        }
        Self { base__: IServiceTransactionConfig_Vtbl::new::<Identity, Impl, OFFSET>(), ConfigureBYOTSysTxn: ConfigureBYOTSysTxn::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceSysTxnConfig as ::windows_core::ComInterface>::IID || iid == &<IServiceTransactionConfigBase as ::windows_core::ComInterface>::IID || iid == &<IServiceTransactionConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceThreadPoolConfig_Impl: Sized {
    fn SelectThreadPool(&self, threadpool: CSC_ThreadPool) -> ::windows_core::Result<()>;
    fn SetBindingInfo(&self, binding: CSC_Binding) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceThreadPoolConfig {}
impl IServiceThreadPoolConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>() -> IServiceThreadPoolConfig_Vtbl {
        unsafe extern "system" fn SelectThreadPool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, threadpool: CSC_ThreadPool) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectThreadPool(::core::mem::transmute_copy(&threadpool)).into()
        }
        unsafe extern "system" fn SetBindingInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceThreadPoolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, binding: CSC_Binding) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBindingInfo(::core::mem::transmute_copy(&binding)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SelectThreadPool: SelectThreadPool::<Identity, Impl, OFFSET>,
            SetBindingInfo: SetBindingInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceThreadPoolConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceTrackerConfig_Impl: Sized {
    fn TrackerConfig(&self, trackerconfig: CSC_TrackerConfig, sztrackerappname: &::windows_core::PCWSTR, sztrackerctxname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceTrackerConfig {}
impl IServiceTrackerConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTrackerConfig_Impl, const OFFSET: isize>() -> IServiceTrackerConfig_Vtbl {
        unsafe extern "system" fn TrackerConfig<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTrackerConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackerconfig: CSC_TrackerConfig, sztrackerappname: ::windows_core::PCWSTR, sztrackerctxname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TrackerConfig(::core::mem::transmute_copy(&trackerconfig), ::core::mem::transmute(&sztrackerappname), ::core::mem::transmute(&sztrackerctxname)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), TrackerConfig: TrackerConfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceTrackerConfig as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_System_DistributedTransactionCoordinator\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
pub trait IServiceTransactionConfig_Impl: Sized + IServiceTransactionConfigBase_Impl {
    fn ConfigureBYOT(&self, pitxbyot: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransaction>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl ::windows_core::RuntimeName for IServiceTransactionConfig {}
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
impl IServiceTransactionConfig_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfig_Impl, const OFFSET: isize>() -> IServiceTransactionConfig_Vtbl {
        unsafe extern "system" fn ConfigureBYOT<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitxbyot: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureBYOT(::windows_core::from_raw_borrowed(&pitxbyot)).into()
        }
        Self { base__: IServiceTransactionConfigBase_Vtbl::new::<Identity, Impl, OFFSET>(), ConfigureBYOT: ConfigureBYOT::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceTransactionConfig as ::windows_core::ComInterface>::IID || iid == &<IServiceTransactionConfigBase as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IServiceTransactionConfigBase_Impl: Sized {
    fn ConfigureTransaction(&self, transactionconfig: CSC_TransactionConfig) -> ::windows_core::Result<()>;
    fn IsolationLevel(&self, option: COMAdminTxIsolationLevelOptions) -> ::windows_core::Result<()>;
    fn TransactionTimeout(&self, ultimeoutsec: u32) -> ::windows_core::Result<()>;
    fn BringYourOwnTransaction(&self, sztipurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn NewTransactionDescription(&self, sztxdesc: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IServiceTransactionConfigBase {}
impl IServiceTransactionConfigBase_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>() -> IServiceTransactionConfigBase_Vtbl {
        unsafe extern "system" fn ConfigureTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, transactionconfig: CSC_TransactionConfig) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConfigureTransaction(::core::mem::transmute_copy(&transactionconfig)).into()
        }
        unsafe extern "system" fn IsolationLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, option: COMAdminTxIsolationLevelOptions) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsolationLevel(::core::mem::transmute_copy(&option)).into()
        }
        unsafe extern "system" fn TransactionTimeout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultimeoutsec: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.TransactionTimeout(::core::mem::transmute_copy(&ultimeoutsec)).into()
        }
        unsafe extern "system" fn BringYourOwnTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztipurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BringYourOwnTransaction(::core::mem::transmute(&sztipurl)).into()
        }
        unsafe extern "system" fn NewTransactionDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IServiceTransactionConfigBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sztxdesc: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NewTransactionDescription(::core::mem::transmute(&sztxdesc)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConfigureTransaction: ConfigureTransaction::<Identity, Impl, OFFSET>,
            IsolationLevel: IsolationLevel::<Identity, Impl, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, Impl, OFFSET>,
            BringYourOwnTransaction: BringYourOwnTransaction::<Identity, Impl, OFFSET>,
            NewTransactionDescription: NewTransactionDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IServiceTransactionConfigBase as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISharedProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn Value(&self) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetValue(&self, val: &super::Variant::VARIANT) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISharedProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISharedProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: isize>() -> ISharedProperty_Vtbl {
        unsafe extern "system" fn Value<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: super::Variant::VARIANT) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISharedProperty as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISharedPropertyGroup_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreatePropertyByPosition(&self, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows_core::Result<()>;
    fn get_PropertyByPosition(&self, index: i32) -> ::windows_core::Result<ISharedProperty>;
    fn CreateProperty(&self, name: &::windows_core::BSTR, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut ::core::option::Option<ISharedProperty>) -> ::windows_core::Result<()>;
    fn get_Property(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<ISharedProperty>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISharedPropertyGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISharedPropertyGroup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>() -> ISharedPropertyGroup_Vtbl {
        unsafe extern "system" fn CreatePropertyByPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePropertyByPosition(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn get_PropertyByPosition<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_PropertyByPosition(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppprop: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateProperty(::core::mem::transmute(&name), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppprop)).into()
        }
        unsafe extern "system" fn get_Property<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Property(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISharedPropertyGroup as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISharedPropertyGroupManager_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreatePropertyGroup(&self, name: &::windows_core::BSTR, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut ::core::option::Option<ISharedPropertyGroup>) -> ::windows_core::Result<()>;
    fn get_Group(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<ISharedPropertyGroup>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISharedPropertyGroupManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISharedPropertyGroupManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>() -> ISharedPropertyGroupManager_Vtbl {
        unsafe extern "system" fn CreatePropertyGroup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwisomode: *mut i32, dwrelmode: *mut i32, fexists: *mut super::super::Foundation::VARIANT_BOOL, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreatePropertyGroup(::core::mem::transmute(&name), ::core::mem::transmute_copy(&dwisomode), ::core::mem::transmute_copy(&dwrelmode), ::core::mem::transmute_copy(&fexists), ::core::mem::transmute_copy(&ppgroup)).into()
        }
        unsafe extern "system" fn get_Group<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Group(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISharedPropertyGroupManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(retval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISharedPropertyGroupManager as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ISystemAppEventData_Impl: Sized {
    fn Startup(&self) -> ::windows_core::Result<()>;
    fn OnDataChanged(&self, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: &::windows_core::BSTR, dwreason: u32, u64tracehandle: u64) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISystemAppEventData {}
impl ISystemAppEventData_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: isize>() -> ISystemAppEventData_Vtbl {
        unsafe extern "system" fn Startup<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Startup().into()
        }
        unsafe extern "system" fn OnDataChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISystemAppEventData_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwpid: u32, dwmask: u32, dwnumbersinks: u32, bstrdwmethodmask: ::std::mem::MaybeUninit<::windows_core::BSTR>, dwreason: u32, u64tracehandle: u64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataChanged(::core::mem::transmute_copy(&dwpid), ::core::mem::transmute_copy(&dwmask), ::core::mem::transmute_copy(&dwnumbersinks), ::core::mem::transmute(&bstrdwmethodmask), ::core::mem::transmute_copy(&dwreason), ::core::mem::transmute_copy(&u64tracehandle)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Startup: Startup::<Identity, Impl, OFFSET>,
            OnDataChanged: OnDataChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISystemAppEventData as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait IThreadPoolKnobs_Impl: Sized {
    fn GetMaxThreads(&self, plcmaxthreads: *mut i32) -> ::windows_core::Result<()>;
    fn GetCurrentThreads(&self, plccurrentthreads: *mut i32) -> ::windows_core::Result<()>;
    fn SetMaxThreads(&self, lcmaxthreads: i32) -> ::windows_core::Result<()>;
    fn GetDeleteDelay(&self, pmsecdeletedelay: *mut i32) -> ::windows_core::Result<()>;
    fn SetDeleteDelay(&self, msecdeletedelay: i32) -> ::windows_core::Result<()>;
    fn GetMaxQueuedRequests(&self, plcmaxqueuedrequests: *mut i32) -> ::windows_core::Result<()>;
    fn GetCurrentQueuedRequests(&self, plccurrentqueuedrequests: *mut i32) -> ::windows_core::Result<()>;
    fn SetMaxQueuedRequests(&self, lcmaxqueuedrequests: i32) -> ::windows_core::Result<()>;
    fn SetMinThreads(&self, lcminthreads: i32) -> ::windows_core::Result<()>;
    fn SetQueueDepth(&self, lcqueuedepth: i32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IThreadPoolKnobs {}
impl IThreadPoolKnobs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>() -> IThreadPoolKnobs_Vtbl {
        unsafe extern "system" fn GetMaxThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxthreads: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaxThreads(::core::mem::transmute_copy(&plcmaxthreads)).into()
        }
        unsafe extern "system" fn GetCurrentThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentthreads: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentThreads(::core::mem::transmute_copy(&plccurrentthreads)).into()
        }
        unsafe extern "system" fn SetMaxThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxthreads: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxThreads(::core::mem::transmute_copy(&lcmaxthreads)).into()
        }
        unsafe extern "system" fn GetDeleteDelay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pmsecdeletedelay: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeleteDelay(::core::mem::transmute_copy(&pmsecdeletedelay)).into()
        }
        unsafe extern "system" fn SetDeleteDelay<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, msecdeletedelay: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDeleteDelay(::core::mem::transmute_copy(&msecdeletedelay)).into()
        }
        unsafe extern "system" fn GetMaxQueuedRequests<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcmaxqueuedrequests: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetMaxQueuedRequests(::core::mem::transmute_copy(&plcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn GetCurrentQueuedRequests<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plccurrentqueuedrequests: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCurrentQueuedRequests(::core::mem::transmute_copy(&plccurrentqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMaxQueuedRequests<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcmaxqueuedrequests: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxQueuedRequests(::core::mem::transmute_copy(&lcmaxqueuedrequests)).into()
        }
        unsafe extern "system" fn SetMinThreads<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcminthreads: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMinThreads(::core::mem::transmute_copy(&lcminthreads)).into()
        }
        unsafe extern "system" fn SetQueueDepth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IThreadPoolKnobs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lcqueuedepth: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetQueueDepth(::core::mem::transmute_copy(&lcqueuedepth)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IThreadPoolKnobs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITransactionContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateInstance(&self, pszprogid: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn Commit(&self) -> ::windows_core::Result<()>;
    fn Abort(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ITransactionContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITransactionContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>() -> ITransactionContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pobject: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(::core::mem::transmute(&pszprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITransactionContext as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ITransactionContextEx_Impl: Sized {
    fn CreateInstance(&self, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Commit(&self) -> ::windows_core::Result<()>;
    fn Abort(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITransactionContextEx {}
impl ITransactionContextEx_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>() -> ITransactionContextEx_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, rclsid: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, pobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateInstance(::core::mem::transmute_copy(&rclsid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&pobject)).into()
        }
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit().into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionContextEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITransactionContextEx as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
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
    fn GetTransactionResourcePool(&self) -> ::windows_core::Result<ITransactionResourcePool>;
    fn Reserved10(&self);
    fn Reserved11(&self);
    fn Reserved12(&self);
    fn Reserved13(&self);
    fn Reserved14(&self);
    fn Reserved15(&self);
    fn Reserved16(&self);
    fn Reserved17(&self);
}
impl ::windows_core::RuntimeName for ITransactionProperty {}
impl ITransactionProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>() -> ITransactionProperty_Vtbl {
        unsafe extern "system" fn Reserved1<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved1()
        }
        unsafe extern "system" fn Reserved2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved2()
        }
        unsafe extern "system" fn Reserved3<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved3()
        }
        unsafe extern "system" fn Reserved4<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved4()
        }
        unsafe extern "system" fn Reserved5<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved5()
        }
        unsafe extern "system" fn Reserved6<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved6()
        }
        unsafe extern "system" fn Reserved7<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved7()
        }
        unsafe extern "system" fn Reserved8<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved8()
        }
        unsafe extern "system" fn Reserved9<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved9()
        }
        unsafe extern "system" fn GetTransactionResourcePool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptxpool: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTransactionResourcePool() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptxpool, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reserved10<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved10()
        }
        unsafe extern "system" fn Reserved11<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved11()
        }
        unsafe extern "system" fn Reserved12<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved12()
        }
        unsafe extern "system" fn Reserved13<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved13()
        }
        unsafe extern "system" fn Reserved14<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved14()
        }
        unsafe extern "system" fn Reserved15<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved15()
        }
        unsafe extern "system" fn Reserved16<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved16()
        }
        unsafe extern "system" fn Reserved17<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reserved17()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITransactionProperty as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_DistributedTransactionCoordinator\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
pub trait ITransactionProxy_Impl: Sized {
    fn Commit(&self, guid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Abort(&self) -> ::windows_core::Result<()>;
    fn Promote(&self) -> ::windows_core::Result<super::DistributedTransactionCoordinator::ITransaction>;
    fn CreateVoter(&self, ptxasync: ::core::option::Option<&super::DistributedTransactionCoordinator::ITransactionVoterNotifyAsync2>) -> ::windows_core::Result<super::DistributedTransactionCoordinator::ITransactionVoterBallotAsync2>;
    fn GetIsolationLevel(&self, __midl__itransactionproxy0000: *mut i32) -> ::windows_core::Result<()>;
    fn GetIdentifier(&self, pbstridentifier: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn IsReusable(&self, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ::windows_core::RuntimeName for ITransactionProxy {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_DistributedTransactionCoordinator"))]
impl ITransactionProxy_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>() -> ITransactionProxy_Vtbl {
        unsafe extern "system" fn Commit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, guid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Commit(::core::mem::transmute(&guid)).into()
        }
        unsafe extern "system" fn Abort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Abort().into()
        }
        unsafe extern "system" fn Promote<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptransaction: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Promote() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptransaction, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateVoter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptxasync: *mut ::core::ffi::c_void, ppballot: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateVoter(::windows_core::from_raw_borrowed(&ptxasync)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppballot, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIsolationLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, __midl__itransactionproxy0000: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIsolationLevel(::core::mem::transmute_copy(&__midl__itransactionproxy0000)).into()
        }
        unsafe extern "system" fn GetIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstridentifier: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIdentifier(::core::mem::transmute_copy(&pbstridentifier)).into()
        }
        unsafe extern "system" fn IsReusable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionProxy_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfisreusable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsReusable(::core::mem::transmute_copy(&pfisreusable)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Commit: Commit::<Identity, Impl, OFFSET>,
            Abort: Abort::<Identity, Impl, OFFSET>,
            Promote: Promote::<Identity, Impl, OFFSET>,
            CreateVoter: CreateVoter::<Identity, Impl, OFFSET>,
            GetIsolationLevel: GetIsolationLevel::<Identity, Impl, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET>,
            IsReusable: IsReusable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITransactionProxy as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ITransactionResourcePool_Impl: Sized {
    fn PutResource(&self, ppool: ::core::option::Option<&IObjPool>, punk: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn GetResource(&self, ppool: ::core::option::Option<&IObjPool>) -> ::windows_core::Result<::windows_core::IUnknown>;
}
impl ::windows_core::RuntimeName for ITransactionResourcePool {}
impl ITransactionResourcePool_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: isize>() -> ITransactionResourcePool_Vtbl {
        unsafe extern "system" fn PutResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: *mut ::core::ffi::c_void, punk: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutResource(::windows_core::from_raw_borrowed(&ppool), ::windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetResource<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionResourcePool_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppool: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetResource(::windows_core::from_raw_borrowed(&ppool)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            PutResource: PutResource::<Identity, Impl, OFFSET>,
            GetResource: GetResource::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITransactionResourcePool as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ITransactionStatus_Impl: Sized {
    fn SetTransactionStatus(&self, hrstatus: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn GetTransactionStatus(&self, phrstatus: *mut ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ITransactionStatus {}
impl ITransactionStatus_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: isize>() -> ITransactionStatus_Vtbl {
        unsafe extern "system" fn SetTransactionStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hrstatus: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTransactionStatus(::core::mem::transmute_copy(&hrstatus)).into()
        }
        unsafe extern "system" fn GetTransactionStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITransactionStatus_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrstatus: *mut ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTransactionStatus(::core::mem::transmute_copy(&phrstatus)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTransactionStatus: SetTransactionStatus::<Identity, Impl, OFFSET>,
            GetTransactionStatus: GetTransactionStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITransactionStatus as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"implement\"`*"]
pub trait ITxProxyHolder_Impl: Sized {
    fn GetIdentifier(&self, pguidltx: *mut ::windows_core::GUID);
}
impl ::windows_core::RuntimeName for ITxProxyHolder {}
impl ITxProxyHolder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITxProxyHolder_Impl, const OFFSET: isize>() -> ITxProxyHolder_Vtbl {
        unsafe extern "system" fn GetIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITxProxyHolder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidltx: *mut ::windows_core::GUID) {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetIdentifier(::core::mem::transmute_copy(&pguidltx))
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetIdentifier: GetIdentifier::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITxProxyHolder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ObjectContext_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateInstance(&self, bstrprogid: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn SetComplete(&self) -> ::windows_core::Result<()>;
    fn SetAbort(&self) -> ::windows_core::Result<()>;
    fn EnableCommit(&self) -> ::windows_core::Result<()>;
    fn DisableCommit(&self) -> ::windows_core::Result<()>;
    fn IsInTransaction(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsSecurityEnabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsCallerInRole(&self, bstrrole: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn get_Item(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<super::Variant::VARIANT>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn Security(&self) -> ::windows_core::Result<SecurityProperty>;
    fn ContextInfo(&self) -> ::windows_core::Result<ContextInfo>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ObjectContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ObjectContext_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>() -> ObjectContext_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprogid: ::std::mem::MaybeUninit<::windows_core::BSTR>, pobject: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateInstance(::core::mem::transmute(&bstrprogid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetComplete().into()
        }
        unsafe extern "system" fn SetAbort<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAbort().into()
        }
        unsafe extern "system" fn EnableCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableCommit().into()
        }
        unsafe extern "system" fn DisableCommit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableCommit().into()
        }
        unsafe extern "system" fn IsInTransaction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisintx: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInTransaction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisintx, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSecurityEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisenabled: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSecurityEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCallerInRole<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrole: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbinrole: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCallerInRole(::core::mem::transmute(&bstrrole)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbinrole, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, pitem: *mut super::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Security<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsecurityproperty: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Security() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsecurityproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContextInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontextinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ContextInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcontextinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ObjectContext as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ObjectControl_Impl: Sized {
    fn Activate(&self) -> ::windows_core::Result<()>;
    fn Deactivate(&self) -> ::windows_core::Result<()>;
    fn CanBePooled(&self, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ObjectControl {}
#[cfg(feature = "Win32_Foundation")]
impl ObjectControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>() -> ObjectControl_Vtbl {
        unsafe extern "system" fn Activate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate().into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate().into()
        }
        unsafe extern "system" fn CanBePooled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ObjectControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbpoolable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CanBePooled(::core::mem::transmute_copy(&pbpoolable)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
            CanBePooled: CanBePooled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ObjectControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_ComponentServices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait SecurityProperty_Impl: Sized + super::Com::IDispatch_Impl {
    fn GetDirectCallerName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetDirectCreatorName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetOriginalCallerName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetOriginalCreatorName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for SecurityProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl SecurityProperty_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>() -> SecurityProperty_Vtbl {
        unsafe extern "system" fn GetDirectCallerName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDirectCallerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDirectCreatorName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDirectCreatorName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCallerName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOriginalCallerName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOriginalCreatorName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: SecurityProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrusername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOriginalCreatorName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrusername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<SecurityProperty as ::windows_core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
