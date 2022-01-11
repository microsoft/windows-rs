#[cfg(feature = "Win32_Foundation")]
pub trait IFunctionDiscoveryImpl: Sized {
    fn GetInstanceCollection();
    fn GetInstance();
    fn CreateInstanceCollectionQuery();
    fn CreateInstanceQuery();
    fn AddInstance();
    fn RemoveInstance();
}
#[cfg(feature = "Win32_Foundation")]
impl IFunctionDiscoveryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryVtbl {
        unsafe extern "system" fn GetInstanceCollection<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInstance<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstanceCollectionQuery<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: ::windows::core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstanceQuery<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddInstance<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveInstance<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInstanceCollection::<Impl, IMPL_OFFSET>, GetInstance::<Impl, IMPL_OFFSET>, CreateInstanceCollectionQuery::<Impl, IMPL_OFFSET>, CreateInstanceQuery::<Impl, IMPL_OFFSET>, AddInstance::<Impl, IMPL_OFFSET>, RemoveInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscovery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFunctionDiscoveryNotificationImpl: Sized {
    fn OnUpdate();
    fn OnError();
    fn OnEvent();
}
#[cfg(feature = "Win32_Foundation")]
impl IFunctionDiscoveryNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryNotificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryNotificationVtbl {
        unsafe extern "system" fn OnUpdate<Impl: IFunctionDiscoveryNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnError<Impl: IFunctionDiscoveryNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEvent<Impl: IFunctionDiscoveryNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweventid: u32, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnUpdate::<Impl, IMPL_OFFSET>, OnError::<Impl, IMPL_OFFSET>, OnEvent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryNotification as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub trait IFunctionDiscoveryProviderImpl: Sized {
    fn Initialize();
    fn Query();
    fn EndQuery();
    fn InstancePropertyStoreValidateAccess();
    fn InstancePropertyStoreOpen();
    fn InstancePropertyStoreFlush();
    fn InstanceQueryService();
    fn InstanceReleased();
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl IFunctionDiscoveryProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryProviderVtbl {
        unsafe extern "system" fn Initialize<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderfactory: ::windows::core::RawPtr, pifunctiondiscoverynotification: ::windows::core::RawPtr, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Query<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderquery: ::windows::core::RawPtr, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndQuery<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstancePropertyStoreValidateAccess<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstancePropertyStoreOpen<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstancePropertyStoreFlush<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstanceQueryService<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InstanceReleased<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Initialize::<Impl, IMPL_OFFSET>,
            Query::<Impl, IMPL_OFFSET>,
            EndQuery::<Impl, IMPL_OFFSET>,
            InstancePropertyStoreValidateAccess::<Impl, IMPL_OFFSET>,
            InstancePropertyStoreOpen::<Impl, IMPL_OFFSET>,
            InstancePropertyStoreFlush::<Impl, IMPL_OFFSET>,
            InstanceQueryService::<Impl, IMPL_OFFSET>,
            InstanceReleased::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionDiscoveryProviderFactoryImpl: Sized {
    fn CreatePropertyStore();
    fn CreateInstance();
    fn CreateFunctionInstanceCollection();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionDiscoveryProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryProviderFactoryVtbl {
        unsafe extern "system" fn CreatePropertyStore<Impl: IFunctionDiscoveryProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateInstance<Impl: IFunctionDiscoveryProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, iproviderinstancecontext: isize, pipropertystore: ::windows::core::RawPtr, pifunctiondiscoveryprovider: ::windows::core::RawPtr, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFunctionInstanceCollection<Impl: IFunctionDiscoveryProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreatePropertyStore::<Impl, IMPL_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>, CreateFunctionInstanceCollection::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFunctionDiscoveryProviderQueryImpl: Sized {
    fn IsInstanceQuery();
    fn IsSubcategoryQuery();
    fn GetQueryConstraints();
    fn GetPropertyConstraints();
}
#[cfg(feature = "Win32_Foundation")]
impl IFunctionDiscoveryProviderQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryProviderQueryVtbl {
        unsafe extern "system" fn IsInstanceQuery<Impl: IFunctionDiscoveryProviderQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSubcategoryQuery<Impl: IFunctionDiscoveryProviderQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetQueryConstraints<Impl: IFunctionDiscoveryProviderQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderqueryconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertyConstraints<Impl: IFunctionDiscoveryProviderQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderpropertyconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsInstanceQuery::<Impl, IMPL_OFFSET>, IsSubcategoryQuery::<Impl, IMPL_OFFSET>, GetQueryConstraints::<Impl, IMPL_OFFSET>, GetPropertyConstraints::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryProviderQuery as ::windows::core::Interface>::IID
    }
}
pub trait IFunctionDiscoveryServiceProviderImpl: Sized {
    fn Initialize();
}
impl IFunctionDiscoveryServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryServiceProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionDiscoveryServiceProviderVtbl {
        unsafe extern "system" fn Initialize<Impl: IFunctionDiscoveryServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionDiscoveryServiceProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstanceImpl: Sized + IServiceProviderImpl {
    fn GetID();
    fn GetProviderInstanceID();
    fn OpenPropertyStore();
    fn GetCategory();
}
#[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionInstanceVtbl {
        unsafe extern "system" fn GetID<Impl: IFunctionInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProviderInstanceID<Impl: IFunctionInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenPropertyStore<Impl: IFunctionInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstgaccess: super::super::System::Com::StructuredStorage::STGM, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCategory<Impl: IFunctionInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryService::<Impl, IMPL_OFFSET>, GetID::<Impl, IMPL_OFFSET>, GetProviderInstanceID::<Impl, IMPL_OFFSET>, OpenPropertyStore::<Impl, IMPL_OFFSET>, GetCategory::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IFunctionInstanceCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Add();
    fn Remove();
    fn Delete();
    fn DeleteAll();
}
#[cfg(feature = "Win32_Foundation")]
impl IFunctionInstanceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionInstanceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAll<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, Get::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>, DeleteAll::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstanceCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IFunctionInstanceCollectionQueryImpl: Sized {
    fn AddQueryConstraint();
    fn AddPropertyConstraint();
    fn Execute();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IFunctionInstanceCollectionQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollectionQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionInstanceCollectionQueryVtbl {
        unsafe extern "system" fn AddQueryConstraint<Impl: IFunctionInstanceCollectionQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: super::super::Foundation::PWSTR, pszconstraintvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPropertyConstraint<Impl: IFunctionInstanceCollectionQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Execute<Impl: IFunctionInstanceCollectionQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AddQueryConstraint::<Impl, IMPL_OFFSET>, AddPropertyConstraint::<Impl, IMPL_OFFSET>, Execute::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstanceCollectionQuery as ::windows::core::Interface>::IID
    }
}
pub trait IFunctionInstanceQueryImpl: Sized {
    fn Execute();
}
impl IFunctionInstanceQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceQueryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFunctionInstanceQueryVtbl {
        unsafe extern "system" fn Execute<Impl: IFunctionInstanceQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Execute::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFunctionInstanceQuery as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPNPXAssociationImpl: Sized {
    fn Associate();
    fn Unassociate();
    fn Delete();
}
#[cfg(feature = "Win32_Foundation")]
impl IPNPXAssociationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXAssociationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPNPXAssociationVtbl {
        unsafe extern "system" fn Associate<Impl: IPNPXAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unassociate<Impl: IPNPXAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IPNPXAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Associate::<Impl, IMPL_OFFSET>, Unassociate::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPNPXAssociation as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPNPXDeviceAssociationImpl: Sized {
    fn Associate();
    fn Unassociate();
    fn Delete();
}
#[cfg(feature = "Win32_Foundation")]
impl IPNPXDeviceAssociationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXDeviceAssociationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPNPXDeviceAssociationVtbl {
        unsafe extern "system" fn Associate<Impl: IPNPXDeviceAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unassociate<Impl: IPNPXDeviceAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IPNPXDeviceAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Associate::<Impl, IMPL_OFFSET>, Unassociate::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPNPXDeviceAssociation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IPropertyStoreCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Add();
    fn Remove();
    fn Delete();
    fn DeleteAll();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IPropertyStoreCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPropertyStoreCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeleteAll<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, Get::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>, DeleteAll::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPropertyStoreCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderPropertiesImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetValue();
    fn SetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IProviderPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertiesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderPropertiesVtbl {
        unsafe extern "system" fn GetCount<Impl: IProviderPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAt<Impl: IProviderPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetValue<Impl: IProviderPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IProviderPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, GetAt::<Impl, IMPL_OFFSET>, GetValue::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderProperties as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait IProviderPropertyConstraintCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Next();
    fn Skip();
    fn Reset();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl IProviderPropertyConstraintCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderPropertyConstraintCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, Get::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderPropertyConstraintCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProviderPublishingImpl: Sized {
    fn CreateInstance();
    fn RemoveInstance();
}
#[cfg(feature = "Win32_Foundation")]
impl IProviderPublishingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPublishingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderPublishingVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IProviderPublishingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveInstance<Impl: IProviderPublishingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateInstance::<Impl, IMPL_OFFSET>, RemoveInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderPublishing as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProviderQueryConstraintCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Next();
    fn Skip();
    fn Reset();
}
#[cfg(feature = "Win32_Foundation")]
impl IProviderQueryConstraintCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProviderQueryConstraintCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Get<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: super::super::Foundation::PWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetCount::<Impl, IMPL_OFFSET>, Get::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProviderQueryConstraintCollection as ::windows::core::Interface>::IID
    }
}
