#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFunctionDiscovery_Impl: Sized {
    fn GetInstanceCollection(&mut self, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL) -> ::windows::core::Result<IFunctionInstanceCollection>;
    fn GetInstance(&mut self, pszfunctioninstanceidentity: super::super::Foundation::PWSTR) -> ::windows::core::Result<IFunctionInstance>;
    fn CreateInstanceCollectionQuery(&mut self, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::core::option::Option<IFunctionInstanceCollectionQuery>) -> ::windows::core::Result<()>;
    fn CreateInstanceQuery(&mut self, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::core::option::Option<IFunctionInstanceQuery>) -> ::windows::core::Result<()>;
    fn AddInstance(&mut self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR) -> ::windows::core::Result<IFunctionInstance>;
    fn RemoveInstance(&mut self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFunctionDiscovery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscovery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscovery_Vtbl {
        unsafe extern "system" fn GetInstanceCollection<Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceCollection(::core::mem::transmute_copy(&pszcategory), ::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute_copy(&fincludeallsubcategories)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstancecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstance<Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstance(::core::mem::transmute_copy(&pszfunctioninstanceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceCollectionQuery<Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: ::windows::core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstanceCollectionQuery(::core::mem::transmute_copy(&pszcategory), ::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute_copy(&fincludeallsubcategories), ::core::mem::transmute(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&pfdqcquerycontext), ::core::mem::transmute_copy(&ppifunctioninstancecollectionquery)).into()
        }
        unsafe extern "system" fn CreateInstanceQuery<Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateInstanceQuery(::core::mem::transmute_copy(&pszfunctioninstanceidentity), ::core::mem::transmute(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&pfdqcquerycontext), ::core::mem::transmute_copy(&ppifunctioninstancequery)).into()
        }
        unsafe extern "system" fn AddInstance<Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddInstance(::core::mem::transmute_copy(&enumsystemvisibility), ::core::mem::transmute_copy(&pszcategory), ::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute_copy(&pszcategoryidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInstance(::core::mem::transmute_copy(&enumsystemvisibility), ::core::mem::transmute_copy(&pszcategory), ::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute_copy(&pszcategoryidentity)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInstanceCollection: GetInstanceCollection::<Impl, IMPL_OFFSET>,
            GetInstance: GetInstance::<Impl, IMPL_OFFSET>,
            CreateInstanceCollectionQuery: CreateInstanceCollectionQuery::<Impl, IMPL_OFFSET>,
            CreateInstanceQuery: CreateInstanceQuery::<Impl, IMPL_OFFSET>,
            AddInstance: AddInstance::<Impl, IMPL_OFFSET>,
            RemoveInstance: RemoveInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscovery as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFunctionDiscoveryNotification_Impl: Sized {
    fn OnUpdate(&mut self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: &::core::option::Option<IFunctionInstance>) -> ::windows::core::Result<()>;
    fn OnError(&mut self, hr: ::windows::core::HRESULT, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn OnEvent(&mut self, dweventid: u32, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFunctionDiscoveryNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryNotification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryNotification_Vtbl {
        unsafe extern "system" fn OnUpdate<Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnUpdate(::core::mem::transmute_copy(&enumqueryupdateaction), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute(&pifunctioninstance)).into()
        }
        unsafe extern "system" fn OnError<Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnError(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute_copy(&pszprovider)).into()
        }
        unsafe extern "system" fn OnEvent<Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweventid: u32, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEvent(::core::mem::transmute_copy(&dweventid), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute_copy(&pszprovider)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnUpdate: OnUpdate::<Impl, IMPL_OFFSET>,
            OnError: OnError::<Impl, IMPL_OFFSET>,
            OnEvent: OnEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProvider_Impl: Sized {
    fn Initialize(&mut self, pifunctiondiscoveryproviderfactory: &::core::option::Option<IFunctionDiscoveryProviderFactory>, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>, lciduserdefault: u32) -> ::windows::core::Result<u32>;
    fn Query(&mut self, pifunctiondiscoveryproviderquery: &::core::option::Option<IFunctionDiscoveryProviderQuery>) -> ::windows::core::Result<IFunctionInstanceCollection>;
    fn EndQuery(&mut self) -> ::windows::core::Result<()>;
    fn InstancePropertyStoreValidateAccess(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::Result<()>;
    fn InstancePropertyStoreOpen(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn InstancePropertyStoreFlush(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows::core::Result<()>;
    fn InstanceQueryService(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn InstanceReleased(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionDiscoveryProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryProvider_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderfactory: ::windows::core::RawPtr, pifunctiondiscoverynotification: ::windows::core::RawPtr, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(::core::mem::transmute(&pifunctiondiscoveryproviderfactory), ::core::mem::transmute(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&lciduserdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstgaccesscapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderquery: ::windows::core::RawPtr, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(::core::mem::transmute(&pifunctiondiscoveryproviderquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstancecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQuery<Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EndQuery().into()
        }
        unsafe extern "system" fn InstancePropertyStoreValidateAccess<Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstancePropertyStoreValidateAccess(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwstgaccess)).into()
        }
        unsafe extern "system" fn InstancePropertyStoreOpen<Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstancePropertyStoreOpen(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwstgaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancePropertyStoreFlush<Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstancePropertyStoreFlush(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)).into()
        }
        unsafe extern "system" fn InstanceQueryService<Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceQueryService(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&guidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceReleased<Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InstanceReleased(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Query: Query::<Impl, IMPL_OFFSET>,
            EndQuery: EndQuery::<Impl, IMPL_OFFSET>,
            InstancePropertyStoreValidateAccess: InstancePropertyStoreValidateAccess::<Impl, IMPL_OFFSET>,
            InstancePropertyStoreOpen: InstancePropertyStoreOpen::<Impl, IMPL_OFFSET>,
            InstancePropertyStoreFlush: InstancePropertyStoreFlush::<Impl, IMPL_OFFSET>,
            InstanceQueryService: InstanceQueryService::<Impl, IMPL_OFFSET>,
            InstanceReleased: InstanceReleased::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProviderFactory_Impl: Sized {
    fn CreatePropertyStore(&mut self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateInstance(&mut self, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, iproviderinstancecontext: isize, pipropertystore: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pifunctiondiscoveryprovider: &::core::option::Option<IFunctionDiscoveryProvider>) -> ::windows::core::Result<IFunctionInstance>;
    fn CreateFunctionInstanceCollection(&mut self) -> ::windows::core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionDiscoveryProviderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryProviderFactory_Vtbl {
        unsafe extern "system" fn CreatePropertyStore<Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, iproviderinstancecontext: isize, pipropertystore: ::windows::core::RawPtr, pifunctiondiscoveryprovider: ::windows::core::RawPtr, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute_copy(&pszproviderinstanceidentity), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute(&pipropertystore), ::core::mem::transmute(&pifunctiondiscoveryprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFunctionInstanceCollection<Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFunctionInstanceCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstancecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreatePropertyStore: CreatePropertyStore::<Impl, IMPL_OFFSET>,
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            CreateFunctionInstanceCollection: CreateFunctionInstanceCollection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFunctionDiscoveryProviderQuery_Impl: Sized {
    fn IsInstanceQuery(&mut self, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()>;
    fn IsSubcategoryQuery(&mut self, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()>;
    fn GetQueryConstraints(&mut self) -> ::windows::core::Result<IProviderQueryConstraintCollection>;
    fn GetPropertyConstraints(&mut self) -> ::windows::core::Result<IProviderPropertyConstraintCollection>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFunctionDiscoveryProviderQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderQuery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryProviderQuery_Vtbl {
        unsafe extern "system" fn IsInstanceQuery<Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsInstanceQuery(::core::mem::transmute_copy(&pisinstancequery), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn IsSubcategoryQuery<Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSubcategoryQuery(::core::mem::transmute_copy(&pissubcategoryquery), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn GetQueryConstraints<Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderqueryconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQueryConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiproviderqueryconstraints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyConstraints<Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderpropertyconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiproviderpropertyconstraints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsInstanceQuery: IsInstanceQuery::<Impl, IMPL_OFFSET>,
            IsSubcategoryQuery: IsSubcategoryQuery::<Impl, IMPL_OFFSET>,
            GetQueryConstraints: GetQueryConstraints::<Impl, IMPL_OFFSET>,
            GetPropertyConstraints: GetPropertyConstraints::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscoveryServiceProvider_Impl: Sized {
    fn Initialize(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionDiscoveryServiceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryServiceProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryServiceProvider_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IFunctionDiscoveryServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstance_Impl: Sized + super::super::System::Com::IServiceProvider_Impl {
    fn GetID(&mut self) -> ::windows::core::Result<*mut u16>;
    fn GetProviderInstanceID(&mut self) -> ::windows::core::Result<*mut u16>;
    fn OpenPropertyStore(&mut self, dwstgaccess: super::super::System::Com::StructuredStorage::STGM) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetCategory(&mut self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstance_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionInstance_Vtbl {
        unsafe extern "system" fn GetID<Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemidentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderInstanceID<Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderInstanceID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemproviderinstanceidentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstgaccess: super::super::System::Com::StructuredStorage::STGM, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPropertyStore(::core::mem::transmute_copy(&dwstgaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetCategory(::core::mem::transmute_copy(&ppszcomemcategory), ::core::mem::transmute_copy(&ppszcomemsubcategory)).into()
        }
        Self {
            base: super::super::System::Com::IServiceProvider_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetID: GetID::<Impl, IMPL_OFFSET>,
            GetProviderInstanceID: GetProviderInstanceID::<Impl, IMPL_OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Impl, IMPL_OFFSET>,
            GetCategory: GetCategory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstance as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFunctionInstanceCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn Get(&mut self, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::core::option::Option<IFunctionInstance>) -> ::windows::core::Result<()>;
    fn Item(&mut self, dwindex: u32) -> ::windows::core::Result<IFunctionInstance>;
    fn Add(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, dwindex: u32) -> ::windows::core::Result<IFunctionInstance>;
    fn Delete(&mut self, dwindex: u32) -> ::windows::core::Result<()>;
    fn DeleteAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFunctionInstanceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionInstanceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&pszinstanceidentity), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppifunctioninstance)).into()
        }
        unsafe extern "system" fn Item<Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pifunctioninstance)).into()
        }
        unsafe extern "system" fn Remove<Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&dwindex)).into()
        }
        unsafe extern "system" fn DeleteAll<Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAll().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            DeleteAll: DeleteAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstanceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstanceCollectionQuery_Impl: Sized {
    fn AddQueryConstraint(&mut self, pszconstraintname: super::super::Foundation::PWSTR, pszconstraintvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn AddPropertyConstraint(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows::core::Result<()>;
    fn Execute(&mut self) -> ::windows::core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionInstanceCollectionQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollectionQuery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionInstanceCollectionQuery_Vtbl {
        unsafe extern "system" fn AddQueryConstraint<Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: super::super::Foundation::PWSTR, pszconstraintvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddQueryConstraint(::core::mem::transmute_copy(&pszconstraintname), ::core::mem::transmute_copy(&pszconstraintvalue)).into()
        }
        unsafe extern "system" fn AddPropertyConstraint<Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyConstraint(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&enumpropertyconstraint)).into()
        }
        unsafe extern "system" fn Execute<Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Execute() {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstancecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddQueryConstraint: AddQueryConstraint::<Impl, IMPL_OFFSET>,
            AddPropertyConstraint: AddPropertyConstraint::<Impl, IMPL_OFFSET>,
            Execute: Execute::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstanceCollectionQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceQuery_Impl: Sized {
    fn Execute(&mut self) -> ::windows::core::Result<IFunctionInstance>;
}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstanceQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceQuery_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionInstanceQuery_Vtbl {
        unsafe extern "system" fn Execute<Impl: IFunctionInstanceQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Execute() {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Execute: Execute::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstanceQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPNPXAssociation_Impl: Sized {
    fn Associate(&mut self, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Unassociate(&mut self, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Delete(&mut self, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPNPXAssociation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXAssociation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPNPXAssociation_Vtbl {
        unsafe extern "system" fn Associate<Impl: IPNPXAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Associate(::core::mem::transmute_copy(&pszsubcategory)).into()
        }
        unsafe extern "system" fn Unassociate<Impl: IPNPXAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unassociate(::core::mem::transmute_copy(&pszsubcategory)).into()
        }
        unsafe extern "system" fn Delete<Impl: IPNPXAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&pszsubcategory)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Associate: Associate::<Impl, IMPL_OFFSET>,
            Unassociate: Unassociate::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPNPXAssociation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPNPXDeviceAssociation_Impl: Sized {
    fn Associate(&mut self, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>) -> ::windows::core::Result<()>;
    fn Unassociate(&mut self, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>) -> ::windows::core::Result<()>;
    fn Delete(&mut self, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPNPXDeviceAssociation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXDeviceAssociation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPNPXDeviceAssociation_Vtbl {
        unsafe extern "system" fn Associate<Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Associate(::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute(&pifunctiondiscoverynotification)).into()
        }
        unsafe extern "system" fn Unassociate<Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unassociate(::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute(&pifunctiondiscoverynotification)).into()
        }
        unsafe extern "system" fn Delete<Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute(&pifunctiondiscoverynotification)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Associate: Associate::<Impl, IMPL_OFFSET>,
            Unassociate: Unassociate::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPNPXDeviceAssociation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPropertyStoreCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn Get(&mut self, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Item(&mut self, dwindex: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Add(&mut self, pipropertystore: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, dwindex: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Delete(&mut self, dwindex: u32) -> ::windows::core::Result<()>;
    fn DeleteAll(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPropertyStoreCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStoreCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&pszinstanceidentity), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppipropertystore)).into()
        }
        unsafe extern "system" fn Item<Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&pipropertystore)).into()
        }
        unsafe extern "system" fn Remove<Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&dwindex)).into()
        }
        unsafe extern "system" fn DeleteAll<Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAll().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            DeleteAll: DeleteAll::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStoreCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderProperties_Impl: Sized {
    fn GetCount(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows::core::Result<u32>;
    fn GetAt(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, dwindex: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>;
    fn GetValue(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetValue(&mut self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IProviderProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderProperties_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderProperties_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderPropertyConstraintCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn Get(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::Result<()>;
    fn Item(&mut self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::Result<()>;
    fn Next(&mut self, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IProviderPropertyConstraintCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderPropertyConstraintCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Item<Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Item(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Next<Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Skip<Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip().into()
        }
        unsafe extern "system" fn Reset<Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderPropertyConstraintCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IProviderPublishing_Impl: Sized {
    fn CreateInstance(&mut self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR) -> ::windows::core::Result<IFunctionInstance>;
    fn RemoveInstance(&mut self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IProviderPublishing_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPublishing_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderPublishing_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IProviderPublishing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&enumvisibilityflags), ::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute_copy(&pszproviderinstanceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Impl: IProviderPublishing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInstance(::core::mem::transmute_copy(&enumvisibilityflags), ::core::mem::transmute_copy(&pszsubcategory), ::core::mem::transmute_copy(&pszproviderinstanceidentity)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
            RemoveInstance: RemoveInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderPublishing as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProviderQueryConstraintCollection_Impl: Sized {
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn Get(&mut self, pszconstraintname: super::super::Foundation::PWSTR) -> ::windows::core::Result<*mut u16>;
    fn Item(&mut self, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Next(&mut self, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Skip(&mut self) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProviderQueryConstraintCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderQueryConstraintCollection_Vtbl {
        unsafe extern "system" fn GetCount<Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: super::super::Foundation::PWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&pszconstraintname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszconstraintvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Item(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn Next<Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn Skip<Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip().into()
        }
        unsafe extern "system" fn Reset<Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderQueryConstraintCollection as ::windows::core::Interface>::IID
    }
}
