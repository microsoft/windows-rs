#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IFunctionDiscovery_Impl: Sized {
    fn GetInstanceCollection(&self, pszcategory: &::windows::core::PCWSTR, pszsubcategory: &::windows::core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL) -> ::windows::core::Result<IFunctionInstanceCollection>;
    fn GetInstance(&self, pszfunctioninstanceidentity: &::windows::core::PCWSTR) -> ::windows::core::Result<IFunctionInstance>;
    fn CreateInstanceCollectionQuery(&self, pszcategory: &::windows::core::PCWSTR, pszsubcategory: &::windows::core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::core::option::Option<IFunctionInstanceCollectionQuery>) -> ::windows::core::Result<()>;
    fn CreateInstanceQuery(&self, pszfunctioninstanceidentity: &::windows::core::PCWSTR, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::core::option::Option<IFunctionInstanceQuery>) -> ::windows::core::Result<()>;
    fn AddInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: &::windows::core::PCWSTR, pszsubcategory: &::windows::core::PCWSTR, pszcategoryidentity: &::windows::core::PCWSTR) -> ::windows::core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumsystemvisibility: SystemVisibilityFlags, pszcategory: &::windows::core::PCWSTR, pszsubcategory: &::windows::core::PCWSTR, pszcategoryidentity: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IFunctionDiscovery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>() -> IFunctionDiscovery_Vtbl {
        unsafe extern "system" fn GetInstanceCollection<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: ::windows::core::PCWSTR, pszsubcategory: ::windows::core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInstanceCollection(::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute_copy(&fincludeallsubcategories)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstancecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstance<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows::core::PCWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInstance(::core::mem::transmute(&pszfunctioninstanceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceCollectionQuery<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: ::windows::core::PCWSTR, pszsubcategory: ::windows::core::PCWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: ::windows::core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateInstanceCollectionQuery(::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute_copy(&fincludeallsubcategories), ::core::mem::transmute(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&pfdqcquerycontext), ::core::mem::transmute_copy(&ppifunctioninstancecollectionquery)).into()
        }
        unsafe extern "system" fn CreateInstanceQuery<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: ::windows::core::PCWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateInstanceQuery(::core::mem::transmute(&pszfunctioninstanceidentity), ::core::mem::transmute(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&pfdqcquerycontext), ::core::mem::transmute_copy(&ppifunctioninstancequery)).into()
        }
        unsafe extern "system" fn AddInstance<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows::core::PCWSTR, pszsubcategory: ::windows::core::PCWSTR, pszcategoryidentity: ::windows::core::PCWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AddInstance(::core::mem::transmute_copy(&enumsystemvisibility), ::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszcategoryidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscovery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: ::windows::core::PCWSTR, pszsubcategory: ::windows::core::PCWSTR, pszcategoryidentity: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveInstance(::core::mem::transmute_copy(&enumsystemvisibility), ::core::mem::transmute(&pszcategory), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszcategoryidentity)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInstanceCollection: GetInstanceCollection::<Identity, Impl, OFFSET>,
            GetInstance: GetInstance::<Identity, Impl, OFFSET>,
            CreateInstanceCollectionQuery: CreateInstanceCollectionQuery::<Identity, Impl, OFFSET>,
            CreateInstanceQuery: CreateInstanceQuery::<Identity, Impl, OFFSET>,
            AddInstance: AddInstance::<Identity, Impl, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscovery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscoveryNotification_Impl: Sized {
    fn OnUpdate(&self, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: &::core::option::Option<IFunctionInstance>) -> ::windows::core::Result<()>;
    fn OnError(&self, hr: ::windows::core::HRESULT, fdqcquerycontext: u64, pszprovider: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn OnEvent(&self, dweventid: u32, fdqcquerycontext: u64, pszprovider: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionDiscoveryNotification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>() -> IFunctionDiscoveryNotification_Vtbl {
        unsafe extern "system" fn OnUpdate<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnUpdate(::core::mem::transmute_copy(&enumqueryupdateaction), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute(&pifunctioninstance)).into()
        }
        unsafe extern "system" fn OnError<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, fdqcquerycontext: u64, pszprovider: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnError(::core::mem::transmute_copy(&hr), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute(&pszprovider)).into()
        }
        unsafe extern "system" fn OnEvent<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryNotification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweventid: u32, fdqcquerycontext: u64, pszprovider: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnEvent(::core::mem::transmute_copy(&dweventid), ::core::mem::transmute_copy(&fdqcquerycontext), ::core::mem::transmute(&pszprovider)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            OnUpdate: OnUpdate::<Identity, Impl, OFFSET>,
            OnError: OnError::<Identity, Impl, OFFSET>,
            OnEvent: OnEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProvider_Impl: Sized {
    fn Initialize(&self, pifunctiondiscoveryproviderfactory: &::core::option::Option<IFunctionDiscoveryProviderFactory>, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>, lciduserdefault: u32) -> ::windows::core::Result<u32>;
    fn Query(&self, pifunctiondiscoveryproviderquery: &::core::option::Option<IFunctionDiscoveryProviderQuery>) -> ::windows::core::Result<IFunctionInstanceCollection>;
    fn EndQuery(&self) -> ::windows::core::Result<()>;
    fn InstancePropertyStoreValidateAccess(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::Result<()>;
    fn InstancePropertyStoreOpen(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn InstancePropertyStoreFlush(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows::core::Result<()>;
    fn InstanceQueryService(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn InstanceReleased(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionDiscoveryProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>() -> IFunctionDiscoveryProvider_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderfactory: ::windows::core::RawPtr, pifunctiondiscoverynotification: ::windows::core::RawPtr, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Initialize(::core::mem::transmute(&pifunctiondiscoveryproviderfactory), ::core::mem::transmute(&pifunctiondiscoverynotification), ::core::mem::transmute_copy(&lciduserdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwstgaccesscapabilities = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderquery: ::windows::core::RawPtr, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Query(::core::mem::transmute(&pifunctiondiscoveryproviderquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstancecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQuery<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).EndQuery().into()
        }
        unsafe extern "system" fn InstancePropertyStoreValidateAccess<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstancePropertyStoreValidateAccess(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwstgaccess)).into()
        }
        unsafe extern "system" fn InstancePropertyStoreOpen<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstancePropertyStoreOpen(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwstgaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancePropertyStoreFlush<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstancePropertyStoreFlush(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)).into()
        }
        unsafe extern "system" fn InstanceQueryService<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InstanceQueryService(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&guidservice), ::core::mem::transmute_copy(&riid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppiunknown = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceReleased<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).InstanceReleased(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Query: Query::<Identity, Impl, OFFSET>,
            EndQuery: EndQuery::<Identity, Impl, OFFSET>,
            InstancePropertyStoreValidateAccess: InstancePropertyStoreValidateAccess::<Identity, Impl, OFFSET>,
            InstancePropertyStoreOpen: InstancePropertyStoreOpen::<Identity, Impl, OFFSET>,
            InstancePropertyStoreFlush: InstancePropertyStoreFlush::<Identity, Impl, OFFSET>,
            InstanceQueryService: InstanceQueryService::<Identity, Impl, OFFSET>,
            InstanceReleased: InstanceReleased::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProviderFactory_Impl: Sized {
    fn CreatePropertyStore(&self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn CreateInstance(&self, pszsubcategory: &::windows::core::PCWSTR, pszproviderinstanceidentity: &::windows::core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>, pifunctiondiscoveryprovider: &::core::option::Option<IFunctionDiscoveryProvider>) -> ::windows::core::Result<IFunctionInstance>;
    fn CreateFunctionInstanceCollection(&self) -> ::windows::core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionDiscoveryProviderFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>() -> IFunctionDiscoveryProviderFactory_Vtbl {
        unsafe extern "system" fn CreatePropertyStore<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreatePropertyStore() {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR, pszproviderinstanceidentity: ::windows::core::PCWSTR, iproviderinstancecontext: isize, pipropertystore: ::windows::core::RawPtr, pifunctiondiscoveryprovider: ::windows::core::RawPtr, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszproviderinstanceidentity), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute(&pipropertystore), ::core::mem::transmute(&pifunctiondiscoveryprovider)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFunctionInstanceCollection<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateFunctionInstanceCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstancecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreatePropertyStore: CreatePropertyStore::<Identity, Impl, OFFSET>,
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            CreateFunctionInstanceCollection: CreateFunctionInstanceCollection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFunctionDiscoveryProviderQuery_Impl: Sized {
    fn IsInstanceQuery(&self, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()>;
    fn IsSubcategoryQuery(&self, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()>;
    fn GetQueryConstraints(&self) -> ::windows::core::Result<IProviderQueryConstraintCollection>;
    fn GetPropertyConstraints(&self) -> ::windows::core::Result<IProviderPropertyConstraintCollection>;
}
#[cfg(feature = "Win32_Foundation")]
impl IFunctionDiscoveryProviderQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>() -> IFunctionDiscoveryProviderQuery_Vtbl {
        unsafe extern "system" fn IsInstanceQuery<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsInstanceQuery(::core::mem::transmute_copy(&pisinstancequery), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn IsSubcategoryQuery<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsSubcategoryQuery(::core::mem::transmute_copy(&pissubcategoryquery), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn GetQueryConstraints<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderqueryconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetQueryConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiproviderqueryconstraints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyConstraints<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderpropertyconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPropertyConstraints() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiproviderpropertyconstraints = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsInstanceQuery: IsInstanceQuery::<Identity, Impl, OFFSET>,
            IsSubcategoryQuery: IsSubcategoryQuery::<Identity, Impl, OFFSET>,
            GetQueryConstraints: GetQueryConstraints::<Identity, Impl, OFFSET>,
            GetPropertyConstraints: GetPropertyConstraints::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionDiscoveryServiceProvider_Impl: Sized {
    fn Initialize(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionDiscoveryServiceProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryServiceProvider_Impl, const OFFSET: isize>() -> IFunctionDiscoveryServiceProvider_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryServiceProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppv)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstance_Impl: Sized + super::super::System::Com::IServiceProvider_Impl {
    fn GetID(&self) -> ::windows::core::Result<*mut u16>;
    fn GetProviderInstanceID(&self) -> ::windows::core::Result<*mut u16>;
    fn OpenPropertyStore(&self, dwstgaccess: super::super::System::Com::StructuredStorage::STGM) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn GetCategory(&self, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionInstance_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstance_Impl, const OFFSET: isize>() -> IFunctionInstance_Vtbl {
        unsafe extern "system" fn GetID<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemidentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderInstanceID<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProviderInstanceID() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszcomemproviderinstanceidentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstgaccess: super::super::System::Com::StructuredStorage::STGM, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenPropertyStore(::core::mem::transmute_copy(&dwstgaccess)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstance_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetCategory(::core::mem::transmute_copy(&ppszcomemcategory), ::core::mem::transmute_copy(&ppszcomemsubcategory)).into()
        }
        Self {
            base: super::super::System::Com::IServiceProvider_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetProviderInstanceID: GetProviderInstanceID::<Identity, Impl, OFFSET>,
            OpenPropertyStore: OpenPropertyStore::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstance as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn Get(&self, pszinstanceidentity: &::windows::core::PCWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::core::option::Option<IFunctionInstance>) -> ::windows::core::Result<()>;
    fn Item(&self, dwindex: u32) -> ::windows::core::Result<IFunctionInstance>;
    fn Add(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>) -> ::windows::core::Result<()>;
    fn Remove(&self, dwindex: u32) -> ::windows::core::Result<IFunctionInstance>;
    fn Delete(&self, dwindex: u32) -> ::windows::core::Result<()>;
    fn DeleteAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstanceCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>() -> IFunctionInstanceCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows::core::PCWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Get(::core::mem::transmute(&pszinstanceidentity), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppifunctioninstance)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&pifunctioninstance)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Remove(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&dwindex)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAll().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstanceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstanceCollectionQuery_Impl: Sized {
    fn AddQueryConstraint(&self, pszconstraintname: &::windows::core::PCWSTR, pszconstraintvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn AddPropertyConstraint(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows::core::Result<()>;
    fn Execute(&self) -> ::windows::core::Result<IFunctionInstanceCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionInstanceCollectionQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>() -> IFunctionInstanceCollectionQuery_Vtbl {
        unsafe extern "system" fn AddQueryConstraint<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows::core::PCWSTR, pszconstraintvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddQueryConstraint(::core::mem::transmute(&pszconstraintname), ::core::mem::transmute(&pszconstraintvalue)).into()
        }
        unsafe extern "system" fn AddPropertyConstraint<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyConstraint(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&pv), ::core::mem::transmute_copy(&enumpropertyconstraint)).into()
        }
        unsafe extern "system" fn Execute<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollectionQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Execute() {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstancecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AddQueryConstraint: AddQueryConstraint::<Identity, Impl, OFFSET>,
            AddPropertyConstraint: AddPropertyConstraint::<Identity, Impl, OFFSET>,
            Execute: Execute::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstanceCollectionQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceQuery_Impl: Sized {
    fn Execute(&self) -> ::windows::core::Result<IFunctionInstance>;
}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstanceQuery_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceQuery_Impl, const OFFSET: isize>() -> IFunctionInstanceQuery_Vtbl {
        unsafe extern "system" fn Execute<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceQuery_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Execute() {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Execute: Execute::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstanceQuery as ::windows::core::Interface>::IID
    }
}
pub trait IPNPXAssociation_Impl: Sized {
    fn Associate(&self, pszsubcategory: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Unassociate(&self, pszsubcategory: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Delete(&self, pszsubcategory: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
impl IPNPXAssociation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXAssociation_Impl, const OFFSET: isize>() -> IPNPXAssociation_Vtbl {
        unsafe extern "system" fn Associate<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Associate(::core::mem::transmute(&pszsubcategory)).into()
        }
        unsafe extern "system" fn Unassociate<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unassociate(::core::mem::transmute(&pszsubcategory)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute(&pszsubcategory)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Associate: Associate::<Identity, Impl, OFFSET>,
            Unassociate: Unassociate::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPNPXAssociation as ::windows::core::Interface>::IID
    }
}
pub trait IPNPXDeviceAssociation_Impl: Sized {
    fn Associate(&self, pszsubcategory: &::windows::core::PCWSTR, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>) -> ::windows::core::Result<()>;
    fn Unassociate(&self, pszsubcategory: &::windows::core::PCWSTR, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>) -> ::windows::core::Result<()>;
    fn Delete(&self, pszsubcategory: &::windows::core::PCWSTR, pifunctiondiscoverynotification: &::core::option::Option<IFunctionDiscoveryNotification>) -> ::windows::core::Result<()>;
}
impl IPNPXDeviceAssociation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>() -> IPNPXDeviceAssociation_Vtbl {
        unsafe extern "system" fn Associate<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Associate(::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pifunctiondiscoverynotification)).into()
        }
        unsafe extern "system" fn Unassociate<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unassociate(::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pifunctiondiscoverynotification)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXDeviceAssociation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: ::windows::core::PCWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pifunctiondiscoverynotification)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Associate: Associate::<Identity, Impl, OFFSET>,
            Unassociate: Unassociate::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPNPXDeviceAssociation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IPropertyStoreCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn Get(&self, pszinstanceidentity: &::windows::core::PCWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Item(&self, dwindex: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Add(&self, pipropertystore: &::core::option::Option<super::super::UI::Shell::PropertiesSystem::IPropertyStore>) -> ::windows::core::Result<()>;
    fn Remove(&self, dwindex: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore>;
    fn Delete(&self, dwindex: u32) -> ::windows::core::Result<()>;
    fn DeleteAll(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IPropertyStoreCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>() -> IPropertyStoreCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: ::windows::core::PCWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Get(::core::mem::transmute(&pszinstanceidentity), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppipropertystore)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute(&pipropertystore)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Remove(::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pipropertystore = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&dwindex)).into()
        }
        unsafe extern "system" fn DeleteAll<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAll().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            DeleteAll: DeleteAll::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStoreCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderProperties_Impl: Sized {
    fn GetCount(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize) -> ::windows::core::Result<u32>;
    fn GetAt(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, dwindex: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>;
    fn GetValue(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn SetValue(&self, pifunctioninstance: &::core::option::Option<IFunctionInstance>, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IProviderProperties_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderProperties_Impl, const OFFSET: isize>() -> IProviderProperties_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Identity: ::windows::core::IUnknownImpl, Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAt(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&dwindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *pkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Identity: ::windows::core::IUnknownImpl, Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetValue(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppropvar = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IProviderProperties_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute(&pifunctioninstance), ::core::mem::transmute_copy(&iproviderinstancecontext), ::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetValue: GetValue::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderPropertyConstraintCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn Get(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::Result<()>;
    fn Item(&self, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::Result<()>;
    fn Next(&self, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IProviderPropertyConstraintCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>() -> IProviderPropertyConstraintCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&key), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Item(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderPropertyConstraintCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProviderPublishing_Impl: Sized {
    fn CreateInstance(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: &::windows::core::PCWSTR, pszproviderinstanceidentity: &::windows::core::PCWSTR) -> ::windows::core::Result<IFunctionInstance>;
    fn RemoveInstance(&self, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: &::windows::core::PCWSTR, pszproviderinstanceidentity: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IProviderPublishing_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPublishing_Impl, const OFFSET: isize>() -> IProviderPublishing_Vtbl {
        unsafe extern "system" fn CreateInstance<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPublishing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows::core::PCWSTR, pszproviderinstanceidentity: ::windows::core::PCWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateInstance(::core::mem::transmute_copy(&enumvisibilityflags), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszproviderinstanceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppifunctioninstance = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPublishing_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: ::windows::core::PCWSTR, pszproviderinstanceidentity: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveInstance(::core::mem::transmute_copy(&enumvisibilityflags), ::core::mem::transmute(&pszsubcategory), ::core::mem::transmute(&pszproviderinstanceidentity)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            CreateInstance: CreateInstance::<Identity, Impl, OFFSET>,
            RemoveInstance: RemoveInstance::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderPublishing as ::windows::core::Interface>::IID
    }
}
pub trait IProviderQueryConstraintCollection_Impl: Sized {
    fn GetCount(&self) -> ::windows::core::Result<u32>;
    fn Get(&self, pszconstraintname: &::windows::core::PCWSTR) -> ::windows::core::Result<*mut u16>;
    fn Item(&self, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Next(&self, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::Result<()>;
    fn Skip(&self) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
}
impl IProviderQueryConstraintCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>() -> IProviderQueryConstraintCollection_Vtbl {
        unsafe extern "system" fn GetCount<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: ::windows::core::PCWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Get(::core::mem::transmute(&pszconstraintname)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszconstraintvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Item(::core::mem::transmute_copy(&dwindex), ::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip().into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderQueryConstraintCollection as ::windows::core::Interface>::IID
    }
}
