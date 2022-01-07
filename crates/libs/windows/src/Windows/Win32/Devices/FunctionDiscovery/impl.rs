pub trait IFunctionDiscoveryImpl: Sized {
    fn GetInstanceCollection();
    fn GetInstance();
    fn CreateInstanceCollectionQuery();
    fn CreateInstanceQuery();
    fn AddInstance();
    fn RemoveInstance();
}
impl ::windows::core::RuntimeName for IFunctionDiscovery {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionDiscovery";
}
impl IFunctionDiscoveryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryImpl, const OFFSET: isize>() -> IFunctionDiscoveryVtbl {
        unsafe extern "system" fn GetInstanceCollection<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceCollection(
                &*(&pszcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fincludeallsubcategories as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppifunctioninstancecollection),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstance<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstance(&*(&pszfunctioninstanceidentity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppifunctioninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceCollectionQuery<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, fincludeallsubcategories: super::super::Foundation::BOOL, pifunctiondiscoverynotification: ::windows::core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancecollectionquery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceCollectionQuery(
                &*(&pszcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fincludeallsubcategories as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
                &*(&pifunctiondiscoverynotification as *const <IFunctionDiscoveryNotification as ::windows::core::Abi>::Abi as *const <IFunctionDiscoveryNotification as ::windows::core::DefaultType>::DefaultType),
                pfdqcquerycontext,
                ::core::mem::transmute_copy(&ppifunctioninstancecollectionquery),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstanceQuery<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszfunctioninstanceidentity: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr, pfdqcquerycontext: *mut u64, ppifunctioninstancequery: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstanceQuery(&*(&pszfunctioninstanceidentity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pifunctiondiscoverynotification as *const <IFunctionDiscoveryNotification as ::windows::core::Abi>::Abi as *const <IFunctionDiscoveryNotification as ::windows::core::DefaultType>::DefaultType), pfdqcquerycontext, ::core::mem::transmute_copy(&ppifunctioninstancequery)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddInstance<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddInstance(
                enumsystemvisibility,
                &*(&pszcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszcategoryidentity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppifunctioninstance),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Impl: IFunctionDiscoveryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumsystemvisibility: SystemVisibilityFlags, pszcategory: super::super::Foundation::PWSTR, pszsubcategory: super::super::Foundation::PWSTR, pszcategoryidentity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveInstance(
                enumsystemvisibility,
                &*(&pszcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszcategoryidentity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFunctionDiscovery>, ::windows::core::GetTrustLevel, GetInstanceCollection::<Impl, OFFSET>, GetInstance::<Impl, OFFSET>, CreateInstanceCollectionQuery::<Impl, OFFSET>, CreateInstanceQuery::<Impl, OFFSET>, AddInstance::<Impl, OFFSET>, RemoveInstance::<Impl, OFFSET>)
    }
}
pub trait IFunctionDiscoveryNotificationImpl: Sized {
    fn OnUpdate();
    fn OnError();
    fn OnEvent();
}
impl ::windows::core::RuntimeName for IFunctionDiscoveryNotification {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionDiscoveryNotification";
}
impl IFunctionDiscoveryNotificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryNotificationImpl, const OFFSET: isize>() -> IFunctionDiscoveryNotificationVtbl {
        unsafe extern "system" fn OnUpdate<Impl: IFunctionDiscoveryNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumqueryupdateaction: QueryUpdateAction, fdqcquerycontext: u64, pifunctioninstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnUpdate(enumqueryupdateaction, fdqcquerycontext, &*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnError<Impl: IFunctionDiscoveryNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hr: ::windows::core::HRESULT, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnError(hr, fdqcquerycontext, &*(&pszprovider as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEvent<Impl: IFunctionDiscoveryNotificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweventid: u32, fdqcquerycontext: u64, pszprovider: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEvent(dweventid, fdqcquerycontext, &*(&pszprovider as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFunctionDiscoveryNotification>, ::windows::core::GetTrustLevel, OnUpdate::<Impl, OFFSET>, OnError::<Impl, OFFSET>, OnEvent::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for IFunctionDiscoveryProvider {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionDiscoveryProvider";
}
impl IFunctionDiscoveryProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>() -> IFunctionDiscoveryProviderVtbl {
        unsafe extern "system" fn Initialize<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderfactory: ::windows::core::RawPtr, pifunctiondiscoverynotification: ::windows::core::RawPtr, lciduserdefault: u32, pdwstgaccesscapabilities: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pifunctiondiscoveryproviderfactory as *const <IFunctionDiscoveryProviderFactory as ::windows::core::Abi>::Abi as *const <IFunctionDiscoveryProviderFactory as ::windows::core::DefaultType>::DefaultType), &*(&pifunctiondiscoverynotification as *const <IFunctionDiscoveryNotification as ::windows::core::Abi>::Abi as *const <IFunctionDiscoveryNotification as ::windows::core::DefaultType>::DefaultType), lciduserdefault, ::core::mem::transmute_copy(&pdwstgaccesscapabilities)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctiondiscoveryproviderquery: ::windows::core::RawPtr, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(&*(&pifunctiondiscoveryproviderquery as *const <IFunctionDiscoveryProviderQuery as ::windows::core::Abi>::Abi as *const <IFunctionDiscoveryProviderQuery as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppifunctioninstancecollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndQuery<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancePropertyStoreValidateAccess<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstancePropertyStoreValidateAccess(&*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType), iproviderinstancecontext, dwstgaccess) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancePropertyStoreOpen<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwstgaccess: u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstancePropertyStoreOpen(&*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType), iproviderinstancecontext, dwstgaccess, ::core::mem::transmute_copy(&ppipropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstancePropertyStoreFlush<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstancePropertyStoreFlush(&*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType), iproviderinstancecontext) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceQueryService<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, guidservice: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppiunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceQueryService(
                &*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType),
                iproviderinstancecontext,
                &*(&guidservice as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppiunknown),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InstanceReleased<Impl: IFunctionDiscoveryProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InstanceReleased(&*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType), iproviderinstancecontext) {
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
            ::windows::core::GetRuntimeClassName::<IFunctionDiscoveryProvider>,
            ::windows::core::GetTrustLevel,
            Initialize::<Impl, OFFSET>,
            Query::<Impl, OFFSET>,
            EndQuery::<Impl, OFFSET>,
            InstancePropertyStoreValidateAccess::<Impl, OFFSET>,
            InstancePropertyStoreOpen::<Impl, OFFSET>,
            InstancePropertyStoreFlush::<Impl, OFFSET>,
            InstanceQueryService::<Impl, OFFSET>,
            InstanceReleased::<Impl, OFFSET>,
        )
    }
}
pub trait IFunctionDiscoveryProviderFactoryImpl: Sized {
    fn CreatePropertyStore();
    fn CreateInstance();
    fn CreateFunctionInstanceCollection();
}
impl ::windows::core::RuntimeName for IFunctionDiscoveryProviderFactory {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionDiscoveryProviderFactory";
}
impl IFunctionDiscoveryProviderFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderFactoryImpl, const OFFSET: isize>() -> IFunctionDiscoveryProviderFactoryVtbl {
        unsafe extern "system" fn CreatePropertyStore<Impl: IFunctionDiscoveryProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreatePropertyStore(::core::mem::transmute_copy(&ppipropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateInstance<Impl: IFunctionDiscoveryProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, iproviderinstancecontext: isize, pipropertystore: ::windows::core::RawPtr, pifunctiondiscoveryprovider: ::windows::core::RawPtr, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(
                &*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszproviderinstanceidentity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                iproviderinstancecontext,
                &*(&pipropertystore as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType),
                &*(&pifunctiondiscoveryprovider as *const <IFunctionDiscoveryProvider as ::windows::core::Abi>::Abi as *const <IFunctionDiscoveryProvider as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppifunctioninstance),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFunctionInstanceCollection<Impl: IFunctionDiscoveryProviderFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFunctionInstanceCollection(::core::mem::transmute_copy(&ppifunctioninstancecollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFunctionDiscoveryProviderFactory>, ::windows::core::GetTrustLevel, CreatePropertyStore::<Impl, OFFSET>, CreateInstance::<Impl, OFFSET>, CreateFunctionInstanceCollection::<Impl, OFFSET>)
    }
}
pub trait IFunctionDiscoveryProviderQueryImpl: Sized {
    fn IsInstanceQuery();
    fn IsSubcategoryQuery();
    fn GetQueryConstraints();
    fn GetPropertyConstraints();
}
impl ::windows::core::RuntimeName for IFunctionDiscoveryProviderQuery {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionDiscoveryProviderQuery";
}
impl IFunctionDiscoveryProviderQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryProviderQueryImpl, const OFFSET: isize>() -> IFunctionDiscoveryProviderQueryVtbl {
        unsafe extern "system" fn IsInstanceQuery<Impl: IFunctionDiscoveryProviderQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisinstancequery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInstanceQuery(::core::mem::transmute_copy(&pisinstancequery), ::core::mem::transmute_copy(&ppszconstraintvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubcategoryQuery<Impl: IFunctionDiscoveryProviderQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pissubcategoryquery: *mut super::super::Foundation::BOOL, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubcategoryQuery(::core::mem::transmute_copy(&pissubcategoryquery), ::core::mem::transmute_copy(&ppszconstraintvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetQueryConstraints<Impl: IFunctionDiscoveryProviderQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderqueryconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetQueryConstraints(::core::mem::transmute_copy(&ppiproviderqueryconstraints)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertyConstraints<Impl: IFunctionDiscoveryProviderQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiproviderpropertyconstraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertyConstraints(::core::mem::transmute_copy(&ppiproviderpropertyconstraints)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFunctionDiscoveryProviderQuery>, ::windows::core::GetTrustLevel, IsInstanceQuery::<Impl, OFFSET>, IsSubcategoryQuery::<Impl, OFFSET>, GetQueryConstraints::<Impl, OFFSET>, GetPropertyConstraints::<Impl, OFFSET>)
    }
}
pub trait IFunctionDiscoveryServiceProviderImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IFunctionDiscoveryServiceProvider {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionDiscoveryServiceProvider";
}
impl IFunctionDiscoveryServiceProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionDiscoveryServiceProviderImpl, const OFFSET: isize>() -> IFunctionDiscoveryServiceProviderVtbl {
        unsafe extern "system" fn Initialize<Impl: IFunctionDiscoveryServiceProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppv)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFunctionDiscoveryServiceProvider>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFunctionInstanceImpl: Sized + IServiceProviderImpl {
    fn GetID();
    fn GetProviderInstanceID();
    fn OpenPropertyStore();
    fn GetCategory();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFunctionInstance {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionInstance";
}
#[cfg(feature = "Win32_System_Com")]
impl IFunctionInstanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceImpl, const OFFSET: isize>() -> IFunctionInstanceVtbl {
        unsafe extern "system" fn GetID<Impl: IFunctionInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemidentity: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetID(::core::mem::transmute_copy(&ppszcomemidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProviderInstanceID<Impl: IFunctionInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemproviderinstanceidentity: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProviderInstanceID(::core::mem::transmute_copy(&ppszcomemproviderinstanceidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenPropertyStore<Impl: IFunctionInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwstgaccess: super::super::System::Com::StructuredStorage::STGM, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenPropertyStore(dwstgaccess, ::core::mem::transmute_copy(&ppipropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: IFunctionInstanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszcomemcategory: *mut *mut u16, ppszcomemsubcategory: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategory(::core::mem::transmute_copy(&ppszcomemcategory), ::core::mem::transmute_copy(&ppszcomemsubcategory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFunctionInstance>, ::windows::core::GetTrustLevel, GetID::<Impl, OFFSET>, GetProviderInstanceID::<Impl, OFFSET>, OpenPropertyStore::<Impl, OFFSET>, GetCategory::<Impl, OFFSET>)
    }
}
pub trait IFunctionInstanceCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Add();
    fn Remove();
    fn Delete();
    fn DeleteAll();
}
impl ::windows::core::RuntimeName for IFunctionInstanceCollection {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionInstanceCollection";
}
impl IFunctionInstanceCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>() -> IFunctionInstanceCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(&*(&pszinstanceidentity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppifunctioninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(dwindex, ::core::mem::transmute_copy(&ppifunctioninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(dwindex, ::core::mem::transmute_copy(&ppifunctioninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(dwindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Impl: IFunctionInstanceCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFunctionInstanceCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, Get::<Impl, OFFSET>, Item::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Delete::<Impl, OFFSET>, DeleteAll::<Impl, OFFSET>)
    }
}
pub trait IFunctionInstanceCollectionQueryImpl: Sized {
    fn AddQueryConstraint();
    fn AddPropertyConstraint();
    fn Execute();
}
impl ::windows::core::RuntimeName for IFunctionInstanceCollectionQuery {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionInstanceCollectionQuery";
}
impl IFunctionInstanceCollectionQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceCollectionQueryImpl, const OFFSET: isize>() -> IFunctionInstanceCollectionQueryVtbl {
        unsafe extern "system" fn AddQueryConstraint<Impl: IFunctionInstanceCollectionQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: super::super::Foundation::PWSTR, pszconstraintvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddQueryConstraint(&*(&pszconstraintname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszconstraintvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyConstraint<Impl: IFunctionInstanceCollectionQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pv: *const super::super::System::Com::StructuredStorage::PROPVARIANT, enumpropertyconstraint: PropertyConstraint) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyConstraint(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), &*(&pv as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType), enumpropertyconstraint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Execute<Impl: IFunctionInstanceCollectionQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstancecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Execute(::core::mem::transmute_copy(&ppifunctioninstancecollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFunctionInstanceCollectionQuery>, ::windows::core::GetTrustLevel, AddQueryConstraint::<Impl, OFFSET>, AddPropertyConstraint::<Impl, OFFSET>, Execute::<Impl, OFFSET>)
    }
}
pub trait IFunctionInstanceQueryImpl: Sized {
    fn Execute();
}
impl ::windows::core::RuntimeName for IFunctionInstanceQuery {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IFunctionInstanceQuery";
}
impl IFunctionInstanceQueryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFunctionInstanceQueryImpl, const OFFSET: isize>() -> IFunctionInstanceQueryVtbl {
        unsafe extern "system" fn Execute<Impl: IFunctionInstanceQueryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Execute(::core::mem::transmute_copy(&ppifunctioninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFunctionInstanceQuery>, ::windows::core::GetTrustLevel, Execute::<Impl, OFFSET>)
    }
}
pub trait IPNPXAssociationImpl: Sized {
    fn Associate();
    fn Unassociate();
    fn Delete();
}
impl ::windows::core::RuntimeName for IPNPXAssociation {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IPNPXAssociation";
}
impl IPNPXAssociationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXAssociationImpl, const OFFSET: isize>() -> IPNPXAssociationVtbl {
        unsafe extern "system" fn Associate<Impl: IPNPXAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Associate(&*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unassociate<Impl: IPNPXAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unassociate(&*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPNPXAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPNPXAssociation>, ::windows::core::GetTrustLevel, Associate::<Impl, OFFSET>, Unassociate::<Impl, OFFSET>, Delete::<Impl, OFFSET>)
    }
}
pub trait IPNPXDeviceAssociationImpl: Sized {
    fn Associate();
    fn Unassociate();
    fn Delete();
}
impl ::windows::core::RuntimeName for IPNPXDeviceAssociation {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IPNPXDeviceAssociation";
}
impl IPNPXDeviceAssociationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPNPXDeviceAssociationImpl, const OFFSET: isize>() -> IPNPXDeviceAssociationVtbl {
        unsafe extern "system" fn Associate<Impl: IPNPXDeviceAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Associate(&*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pifunctiondiscoverynotification as *const <IFunctionDiscoveryNotification as ::windows::core::Abi>::Abi as *const <IFunctionDiscoveryNotification as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unassociate<Impl: IPNPXDeviceAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unassociate(&*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pifunctiondiscoverynotification as *const <IFunctionDiscoveryNotification as ::windows::core::Abi>::Abi as *const <IFunctionDiscoveryNotification as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPNPXDeviceAssociationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszsubcategory: super::super::Foundation::PWSTR, pifunctiondiscoverynotification: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pifunctiondiscoverynotification as *const <IFunctionDiscoveryNotification as ::windows::core::Abi>::Abi as *const <IFunctionDiscoveryNotification as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPNPXDeviceAssociation>, ::windows::core::GetTrustLevel, Associate::<Impl, OFFSET>, Unassociate::<Impl, OFFSET>, Delete::<Impl, OFFSET>)
    }
}
pub trait IPropertyStoreCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Add();
    fn Remove();
    fn Delete();
    fn DeleteAll();
}
impl ::windows::core::RuntimeName for IPropertyStoreCollection {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IPropertyStoreCollection";
}
impl IPropertyStoreCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>() -> IPropertyStoreCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszinstanceidentity: super::super::Foundation::PWSTR, pdwindex: *mut u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(&*(&pszinstanceidentity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwindex), ::core::mem::transmute_copy(&ppipropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(dwindex, ::core::mem::transmute_copy(&ppipropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipropertystore: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&pipropertystore as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::IPropertyStore as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pipropertystore: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(dwindex, ::core::mem::transmute_copy(&pipropertystore)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(dwindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAll<Impl: IPropertyStoreCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IPropertyStoreCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, Get::<Impl, OFFSET>, Item::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, Delete::<Impl, OFFSET>, DeleteAll::<Impl, OFFSET>)
    }
}
pub trait IProviderPropertiesImpl: Sized {
    fn GetCount();
    fn GetAt();
    fn GetValue();
    fn SetValue();
}
impl ::windows::core::RuntimeName for IProviderProperties {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IProviderProperties";
}
impl IProviderPropertiesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertiesImpl, const OFFSET: isize>() -> IProviderPropertiesVtbl {
        unsafe extern "system" fn GetCount<Impl: IProviderPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(&*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType), iproviderinstancecontext, ::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAt<Impl: IProviderPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(&*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType), iproviderinstancecontext, dwindex, ::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IProviderPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType), iproviderinstancecontext, &*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IProviderPropertiesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pifunctioninstance: ::windows::core::RawPtr, iproviderinstancecontext: isize, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *const super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetValue(
                &*(&pifunctioninstance as *const <IFunctionInstance as ::windows::core::Abi>::Abi as *const <IFunctionInstance as ::windows::core::DefaultType>::DefaultType),
                iproviderinstancecontext,
                &*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType),
                &*(&ppropvar as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::PROPVARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProviderProperties>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, GetAt::<Impl, OFFSET>, GetValue::<Impl, OFFSET>, SetValue::<Impl, OFFSET>)
    }
}
pub trait IProviderPropertyConstraintCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Next();
    fn Skip();
    fn Reset();
}
impl ::windows::core::RuntimeName for IProviderPropertyConstraintCollection {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IProviderPropertyConstraintCollection";
}
impl IProviderPropertyConstraintCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>() -> IProviderPropertyConstraintCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(dwindex, ::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *mut super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, ppropvar: *mut super::super::System::Com::StructuredStorage::PROPVARIANT, pdwpropertyconstraint: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&pkey), ::core::mem::transmute_copy(&ppropvar), ::core::mem::transmute_copy(&pdwpropertyconstraint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IProviderPropertyConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProviderPropertyConstraintCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, Get::<Impl, OFFSET>, Item::<Impl, OFFSET>, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
pub trait IProviderPublishingImpl: Sized {
    fn CreateInstance();
    fn RemoveInstance();
}
impl ::windows::core::RuntimeName for IProviderPublishing {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IProviderPublishing";
}
impl IProviderPublishingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderPublishingImpl, const OFFSET: isize>() -> IProviderPublishingVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IProviderPublishingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR, ppifunctioninstance: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(enumvisibilityflags, &*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszproviderinstanceidentity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppifunctioninstance)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInstance<Impl: IProviderPublishingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, enumvisibilityflags: SystemVisibilityFlags, pszsubcategory: super::super::Foundation::PWSTR, pszproviderinstanceidentity: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveInstance(enumvisibilityflags, &*(&pszsubcategory as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszproviderinstanceidentity as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProviderPublishing>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, OFFSET>, RemoveInstance::<Impl, OFFSET>)
    }
}
pub trait IProviderQueryConstraintCollectionImpl: Sized {
    fn GetCount();
    fn Get();
    fn Item();
    fn Next();
    fn Skip();
    fn Reset();
}
impl ::windows::core::RuntimeName for IProviderQueryConstraintCollection {
    const NAME: &'static str = "Windows.Win32.Devices.FunctionDiscovery.IProviderQueryConstraintCollection";
}
impl IProviderQueryConstraintCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>() -> IProviderQueryConstraintCollectionVtbl {
        unsafe extern "system" fn GetCount<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pdwcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszconstraintname: super::super::Foundation::PWSTR, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(&*(&pszconstraintname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszconstraintvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwindex: u32, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(dwindex, ::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszconstraintname: *mut *mut u16, ppszconstraintvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(::core::mem::transmute_copy(&ppszconstraintname), ::core::mem::transmute_copy(&ppszconstraintvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IProviderQueryConstraintCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProviderQueryConstraintCollection>, ::windows::core::GetTrustLevel, GetCount::<Impl, OFFSET>, Get::<Impl, OFFSET>, Item::<Impl, OFFSET>, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>)
    }
}
