#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoImpl: Sized + IDispatchImpl {
    fn GetPropertyInfo();
    fn GetProperty();
    fn PutProperty();
    fn ResetProperty();
    fn Apply();
    fn Restore();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISdoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISdoVtbl {
        unsafe extern "system" fn GetPropertyInfo<Impl: ISdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pppropertyinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ISdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutProperty<Impl: ISdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetProperty<Impl: ISdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Apply<Impl: ISdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Restore<Impl: ISdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ISdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            GetPropertyInfo::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            PutProperty::<Impl, IMPL_OFFSET>,
            ResetProperty::<Impl, IMPL_OFFSET>,
            Apply::<Impl, IMPL_OFFSET>,
            Restore::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoCollectionImpl: Sized + IDispatchImpl {
    fn Count();
    fn Add();
    fn Remove();
    fn RemoveAll();
    fn Reload();
    fn IsNameUnique();
    fn Item();
    fn _NewEnum();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISdoCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISdoCollectionVtbl {
        unsafe extern "system" fn Count<Impl: ISdoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISdoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISdoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveAll<Impl: ISdoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reload<Impl: ISdoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsNameUnique<Impl: ISdoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: ISdoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const super::super::System::Com::VARIANT, pitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn _NewEnum<Impl: ISdoCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Add::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            RemoveAll::<Impl, IMPL_OFFSET>,
            Reload::<Impl, IMPL_OFFSET>,
            IsNameUnique::<Impl, IMPL_OFFSET>,
            Item::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoDictionaryOldImpl: Sized + IDispatchImpl {
    fn EnumAttributes();
    fn GetAttributeInfo();
    fn EnumAttributeValues();
    fn CreateAttribute();
    fn GetAttributeID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoDictionaryOldVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISdoDictionaryOldImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISdoDictionaryOldVtbl {
        unsafe extern "system" fn EnumAttributes<Impl: ISdoDictionaryOldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Com::VARIANT, pvalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeInfo<Impl: ISdoDictionaryOldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pinfoids: *const super::super::System::Com::VARIANT, pinfovalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumAttributeValues<Impl: ISdoDictionaryOldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Com::VARIANT, pvaluesdesc: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAttribute<Impl: ISdoDictionaryOldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, ppattributeobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributeID<Impl: ISdoDictionaryOldImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattributename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: *mut ATTRIBUTEID) -> ::windows::core::HRESULT {
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
            EnumAttributes::<Impl, IMPL_OFFSET>,
            GetAttributeInfo::<Impl, IMPL_OFFSET>,
            EnumAttributeValues::<Impl, IMPL_OFFSET>,
            CreateAttribute::<Impl, IMPL_OFFSET>,
            GetAttributeID::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoDictionaryOld as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoMachineImpl: Sized + IDispatchImpl {
    fn Attach();
    fn GetDictionarySDO();
    fn GetServiceSDO();
    fn GetUserSDO();
    fn GetOSType();
    fn GetDomainType();
    fn IsDirectoryAvailable();
    fn GetAttachedComputer();
    fn GetSDOSchema();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoMachineVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISdoMachineImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISdoMachineVtbl {
        unsafe extern "system" fn Attach<Impl: ISdoMachineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcomputername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDictionarySDO<Impl: ISdoMachineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdictionarysdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetServiceSDO<Impl: ISdoMachineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppservicesdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUserSDO<Impl: ISdoMachineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppusersdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOSType<Impl: ISdoMachineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eostype: *mut IASOSTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDomainType<Impl: ISdoMachineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edomaintype: *mut IASDOMAINTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDirectoryAvailable<Impl: ISdoMachineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, booldirectoryavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttachedComputer<Impl: ISdoMachineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcomputername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSDOSchema<Impl: ISdoMachineImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsdoschema: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Attach::<Impl, IMPL_OFFSET>,
            GetDictionarySDO::<Impl, IMPL_OFFSET>,
            GetServiceSDO::<Impl, IMPL_OFFSET>,
            GetUserSDO::<Impl, IMPL_OFFSET>,
            GetOSType::<Impl, IMPL_OFFSET>,
            GetDomainType::<Impl, IMPL_OFFSET>,
            IsDirectoryAvailable::<Impl, IMPL_OFFSET>,
            GetAttachedComputer::<Impl, IMPL_OFFSET>,
            GetSDOSchema::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoMachine as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoMachine2Impl: Sized + ISdoMachineImpl + IDispatchImpl {
    fn GetTemplatesSDO();
    fn EnableTemplates();
    fn SyncConfigAgainstTemplates();
    fn ImportRemoteTemplates();
    fn Reload();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoMachine2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISdoMachine2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISdoMachine2Vtbl {
        unsafe extern "system" fn GetTemplatesSDO<Impl: ISdoMachine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplatessdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableTemplates<Impl: ISdoMachine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SyncConfigAgainstTemplates<Impl: ISdoMachine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppconfigroot: *mut *mut ::core::ffi::c_void, pptemplatesroot: *mut *mut ::core::ffi::c_void, bforcedsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportRemoteTemplates<Impl: ISdoMachine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocaltemplatesroot: *mut ::core::ffi::c_void, bstrremotemachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reload<Impl: ISdoMachine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
            Attach::<Impl, IMPL_OFFSET>,
            GetDictionarySDO::<Impl, IMPL_OFFSET>,
            GetServiceSDO::<Impl, IMPL_OFFSET>,
            GetUserSDO::<Impl, IMPL_OFFSET>,
            GetOSType::<Impl, IMPL_OFFSET>,
            GetDomainType::<Impl, IMPL_OFFSET>,
            IsDirectoryAvailable::<Impl, IMPL_OFFSET>,
            GetAttachedComputer::<Impl, IMPL_OFFSET>,
            GetSDOSchema::<Impl, IMPL_OFFSET>,
            GetTemplatesSDO::<Impl, IMPL_OFFSET>,
            EnableTemplates::<Impl, IMPL_OFFSET>,
            SyncConfigAgainstTemplates::<Impl, IMPL_OFFSET>,
            ImportRemoteTemplates::<Impl, IMPL_OFFSET>,
            Reload::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoMachine2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoServiceControlImpl: Sized + IDispatchImpl {
    fn StartService();
    fn StopService();
    fn GetServiceStatus();
    fn ResetService();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoServiceControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISdoServiceControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISdoServiceControlVtbl {
        unsafe extern "system" fn StartService<Impl: ISdoServiceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopService<Impl: ISdoServiceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetServiceStatus<Impl: ISdoServiceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResetService<Impl: ISdoServiceControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, StartService::<Impl, IMPL_OFFSET>, StopService::<Impl, IMPL_OFFSET>, GetServiceStatus::<Impl, IMPL_OFFSET>, ResetService::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoServiceControl as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITemplateSdoImpl: Sized + ISdoImpl + IDispatchImpl {
    fn AddToCollection();
    fn AddToSdo();
    fn AddToSdoAsProperty();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITemplateSdoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ITemplateSdoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ITemplateSdoVtbl {
        unsafe extern "system" fn AddToCollection<Impl: ITemplateSdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcollection: ::windows::core::RawPtr, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddToSdo<Impl: ITemplateSdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psdotarget: ::windows::core::RawPtr, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddToSdoAsProperty<Impl: ITemplateSdoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psdotarget: ::windows::core::RawPtr, id: i32) -> ::windows::core::HRESULT {
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
            GetPropertyInfo::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            PutProperty::<Impl, IMPL_OFFSET>,
            ResetProperty::<Impl, IMPL_OFFSET>,
            Apply::<Impl, IMPL_OFFSET>,
            Restore::<Impl, IMPL_OFFSET>,
            _NewEnum::<Impl, IMPL_OFFSET>,
            AddToCollection::<Impl, IMPL_OFFSET>,
            AddToSdo::<Impl, IMPL_OFFSET>,
            AddToSdoAsProperty::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITemplateSdo as ::windows::core::Interface>::IID
    }
}
