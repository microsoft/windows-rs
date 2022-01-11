pub trait IDontSupportEventSubscriptionImpl: Sized {}
impl IDontSupportEventSubscriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDontSupportEventSubscriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDontSupportEventSubscriptionVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDontSupportEventSubscription as ::windows::core::Interface>::IID
    }
}
pub trait IEnumEventObjectImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
impl IEnumEventObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEventObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumEventObjectVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumEventObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Next<Impl: IEnumEventObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumEventObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumEventObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Clone::<Impl, IMPL_OFFSET>, Next::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumEventObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventClassImpl: Sized + IDispatchImpl {
    fn EventClassID();
    fn SetEventClassID();
    fn EventClassName();
    fn SetEventClassName();
    fn OwnerSID();
    fn SetOwnerSID();
    fn FiringInterfaceID();
    fn SetFiringInterfaceID();
    fn Description();
    fn SetDescription();
    fn CustomConfigCLSID();
    fn SetCustomConfigCLSID();
    fn TypeLib();
    fn SetTypeLib();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventClassVtbl {
        unsafe extern "system" fn EventClassID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventClassID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventClassName<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventClassName<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OwnerSID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOwnerSID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FiringInterfaceID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFiringInterfaceID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CustomConfigCLSID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCustomConfigCLSID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TypeLib<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTypeLib<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypelib: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            EventClassID::<Impl, IMPL_OFFSET>,
            SetEventClassID::<Impl, IMPL_OFFSET>,
            EventClassName::<Impl, IMPL_OFFSET>,
            SetEventClassName::<Impl, IMPL_OFFSET>,
            OwnerSID::<Impl, IMPL_OFFSET>,
            SetOwnerSID::<Impl, IMPL_OFFSET>,
            FiringInterfaceID::<Impl, IMPL_OFFSET>,
            SetFiringInterfaceID::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            CustomConfigCLSID::<Impl, IMPL_OFFSET>,
            SetCustomConfigCLSID::<Impl, IMPL_OFFSET>,
            TypeLib::<Impl, IMPL_OFFSET>,
            SetTypeLib::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventClass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventClass2Impl: Sized + IEventClassImpl + IDispatchImpl {
    fn PublisherID();
    fn SetPublisherID();
    fn MultiInterfacePublisherFilterCLSID();
    fn SetMultiInterfacePublisherFilterCLSID();
    fn AllowInprocActivation();
    fn SetAllowInprocActivation();
    fn FireInParallel();
    fn SetFireInParallel();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventClass2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventClass2Vtbl {
        unsafe extern "system" fn PublisherID<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPublisherID<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MultiInterfacePublisherFilterCLSID<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMultiInterfacePublisherFilterCLSID<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpubfilclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowInprocActivation<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowInprocActivation<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FireInParallel<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFireInParallel<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
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
            EventClassID::<Impl, IMPL_OFFSET>,
            SetEventClassID::<Impl, IMPL_OFFSET>,
            EventClassName::<Impl, IMPL_OFFSET>,
            SetEventClassName::<Impl, IMPL_OFFSET>,
            OwnerSID::<Impl, IMPL_OFFSET>,
            SetOwnerSID::<Impl, IMPL_OFFSET>,
            FiringInterfaceID::<Impl, IMPL_OFFSET>,
            SetFiringInterfaceID::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            CustomConfigCLSID::<Impl, IMPL_OFFSET>,
            SetCustomConfigCLSID::<Impl, IMPL_OFFSET>,
            TypeLib::<Impl, IMPL_OFFSET>,
            SetTypeLib::<Impl, IMPL_OFFSET>,
            PublisherID::<Impl, IMPL_OFFSET>,
            SetPublisherID::<Impl, IMPL_OFFSET>,
            MultiInterfacePublisherFilterCLSID::<Impl, IMPL_OFFSET>,
            SetMultiInterfacePublisherFilterCLSID::<Impl, IMPL_OFFSET>,
            AllowInprocActivation::<Impl, IMPL_OFFSET>,
            SetAllowInprocActivation::<Impl, IMPL_OFFSET>,
            FireInParallel::<Impl, IMPL_OFFSET>,
            SetFireInParallel::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventClass2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventControlImpl: Sized + IDispatchImpl {
    fn SetPublisherFilter();
    fn AllowInprocActivation();
    fn SetAllowInprocActivation();
    fn GetSubscriptions();
    fn SetDefaultQuery();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventControlVtbl {
        unsafe extern "system" fn SetPublisherFilter<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppublisherfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowInprocActivation<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowInprocActivation<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubscriptions<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultQuery<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, criteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
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
            SetPublisherFilter::<Impl, IMPL_OFFSET>,
            AllowInprocActivation::<Impl, IMPL_OFFSET>,
            SetAllowInprocActivation::<Impl, IMPL_OFFSET>,
            GetSubscriptions::<Impl, IMPL_OFFSET>,
            SetDefaultQuery::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEventObjectChangeImpl: Sized {
    fn ChangedSubscription();
    fn ChangedEventClass();
    fn ChangedPublisher();
}
#[cfg(feature = "Win32_Foundation")]
impl IEventObjectChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventObjectChangeVtbl {
        unsafe extern "system" fn ChangedSubscription<Impl: IEventObjectChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangedEventClass<Impl: IEventObjectChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangedPublisher<Impl: IEventObjectChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ChangedSubscription::<Impl, IMPL_OFFSET>, ChangedEventClass::<Impl, IMPL_OFFSET>, ChangedPublisher::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEventObjectChange2Impl: Sized {
    fn ChangedSubscription();
    fn ChangedEventClass();
}
#[cfg(feature = "Win32_Foundation")]
impl IEventObjectChange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChange2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventObjectChange2Vtbl {
        unsafe extern "system" fn ChangedSubscription<Impl: IEventObjectChange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangedEventClass<Impl: IEventObjectChange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ChangedSubscription::<Impl, IMPL_OFFSET>, ChangedEventClass::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectChange2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventObjectCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn NewEnum();
    fn Count();
    fn Add();
    fn Remove();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventObjectCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventObjectCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pitem: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NewEnum<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const super::VARIANT, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, _NewEnum::<Impl, IMPL_OFFSET>, Item::<Impl, IMPL_OFFSET>, NewEnum::<Impl, IMPL_OFFSET>, Count::<Impl, IMPL_OFFSET>, Add::<Impl, IMPL_OFFSET>, Remove::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventPropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Value();
    fn SetValue();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventPropertyVtbl {
        unsafe extern "system" fn Name<Impl: IEventPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetName<Impl: IEventPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Value<Impl: IEventPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetValue<Impl: IEventPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, Name::<Impl, IMPL_OFFSET>, SetName::<Impl, IMPL_OFFSET>, Value::<Impl, IMPL_OFFSET>, SetValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventPublisherImpl: Sized + IDispatchImpl {
    fn PublisherID();
    fn SetPublisherID();
    fn PublisherName();
    fn SetPublisherName();
    fn PublisherType();
    fn SetPublisherType();
    fn OwnerSID();
    fn SetOwnerSID();
    fn Description();
    fn SetDescription();
    fn GetDefaultProperty();
    fn PutDefaultProperty();
    fn RemoveDefaultProperty();
    fn GetDefaultPropertyCollection();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventPublisherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventPublisherVtbl {
        unsafe extern "system" fn PublisherID<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPublisherID<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PublisherName<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPublisherName<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublishername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PublisherType<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPublisherType<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublishertype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OwnerSID<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOwnerSID<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultProperty<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutDefaultProperty<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveDefaultProperty<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultPropertyCollection<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
            PublisherID::<Impl, IMPL_OFFSET>,
            SetPublisherID::<Impl, IMPL_OFFSET>,
            PublisherName::<Impl, IMPL_OFFSET>,
            SetPublisherName::<Impl, IMPL_OFFSET>,
            PublisherType::<Impl, IMPL_OFFSET>,
            SetPublisherType::<Impl, IMPL_OFFSET>,
            OwnerSID::<Impl, IMPL_OFFSET>,
            SetOwnerSID::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            GetDefaultProperty::<Impl, IMPL_OFFSET>,
            PutDefaultProperty::<Impl, IMPL_OFFSET>,
            RemoveDefaultProperty::<Impl, IMPL_OFFSET>,
            GetDefaultPropertyCollection::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventPublisher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventSubscriptionImpl: Sized + IDispatchImpl {
    fn SubscriptionID();
    fn SetSubscriptionID();
    fn SubscriptionName();
    fn SetSubscriptionName();
    fn PublisherID();
    fn SetPublisherID();
    fn EventClassID();
    fn SetEventClassID();
    fn MethodName();
    fn SetMethodName();
    fn SubscriberCLSID();
    fn SetSubscriberCLSID();
    fn SubscriberInterface();
    fn SetSubscriberInterface();
    fn PerUser();
    fn SetPerUser();
    fn OwnerSID();
    fn SetOwnerSID();
    fn Enabled();
    fn SetEnabled();
    fn Description();
    fn SetDescription();
    fn MachineName();
    fn SetMachineName();
    fn GetPublisherProperty();
    fn PutPublisherProperty();
    fn RemovePublisherProperty();
    fn GetPublisherPropertyCollection();
    fn GetSubscriberProperty();
    fn PutSubscriberProperty();
    fn RemoveSubscriberProperty();
    fn GetSubscriberPropertyCollection();
    fn InterfaceID();
    fn SetInterfaceID();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventSubscriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventSubscriptionVtbl {
        unsafe extern "system" fn SubscriptionID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubscriptionID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubscriptionName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubscriptionName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriptionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PublisherID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPublisherID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventClassID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventClassID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MethodName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMethodName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubscriberCLSID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubscriberCLSID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SubscriberInterface<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSubscriberInterface<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PerUser<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPerUser<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OwnerSID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetOwnerSID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enabled<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEnabled<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Description<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDescription<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MachineName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMachineName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachinename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPublisherProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutPublisherProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemovePublisherProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPublisherPropertyCollection<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubscriberProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PutSubscriberProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveSubscriberProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubscriberPropertyCollection<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InterfaceID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInterfaceID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            SubscriptionID::<Impl, IMPL_OFFSET>,
            SetSubscriptionID::<Impl, IMPL_OFFSET>,
            SubscriptionName::<Impl, IMPL_OFFSET>,
            SetSubscriptionName::<Impl, IMPL_OFFSET>,
            PublisherID::<Impl, IMPL_OFFSET>,
            SetPublisherID::<Impl, IMPL_OFFSET>,
            EventClassID::<Impl, IMPL_OFFSET>,
            SetEventClassID::<Impl, IMPL_OFFSET>,
            MethodName::<Impl, IMPL_OFFSET>,
            SetMethodName::<Impl, IMPL_OFFSET>,
            SubscriberCLSID::<Impl, IMPL_OFFSET>,
            SetSubscriberCLSID::<Impl, IMPL_OFFSET>,
            SubscriberInterface::<Impl, IMPL_OFFSET>,
            SetSubscriberInterface::<Impl, IMPL_OFFSET>,
            PerUser::<Impl, IMPL_OFFSET>,
            SetPerUser::<Impl, IMPL_OFFSET>,
            OwnerSID::<Impl, IMPL_OFFSET>,
            SetOwnerSID::<Impl, IMPL_OFFSET>,
            Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled::<Impl, IMPL_OFFSET>,
            Description::<Impl, IMPL_OFFSET>,
            SetDescription::<Impl, IMPL_OFFSET>,
            MachineName::<Impl, IMPL_OFFSET>,
            SetMachineName::<Impl, IMPL_OFFSET>,
            GetPublisherProperty::<Impl, IMPL_OFFSET>,
            PutPublisherProperty::<Impl, IMPL_OFFSET>,
            RemovePublisherProperty::<Impl, IMPL_OFFSET>,
            GetPublisherPropertyCollection::<Impl, IMPL_OFFSET>,
            GetSubscriberProperty::<Impl, IMPL_OFFSET>,
            PutSubscriberProperty::<Impl, IMPL_OFFSET>,
            RemoveSubscriberProperty::<Impl, IMPL_OFFSET>,
            GetSubscriberPropertyCollection::<Impl, IMPL_OFFSET>,
            InterfaceID::<Impl, IMPL_OFFSET>,
            SetInterfaceID::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventSubscription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventSystemImpl: Sized + IDispatchImpl {
    fn Query();
    fn Store();
    fn Remove();
    fn EventObjectChangeEventClassID();
    fn QueryS();
    fn RemoveS();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventSystemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventSystemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventSystemVtbl {
        unsafe extern "system" fn Query<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Store<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EventObjectChangeEventClassID<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryS<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveS<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
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
            Query::<Impl, IMPL_OFFSET>,
            Store::<Impl, IMPL_OFFSET>,
            Remove::<Impl, IMPL_OFFSET>,
            EventObjectChangeEventClassID::<Impl, IMPL_OFFSET>,
            QueryS::<Impl, IMPL_OFFSET>,
            RemoveS::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventSystem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IFiringControlImpl: Sized + IDispatchImpl {
    fn FireSubscription();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IFiringControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFiringControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFiringControlVtbl {
        unsafe extern "system" fn FireSubscription<Impl: IFiringControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscription: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetTypeInfoCount::<Impl, IMPL_OFFSET>, GetTypeInfo::<Impl, IMPL_OFFSET>, GetIDsOfNames::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>, FireSubscription::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFiringControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMultiInterfaceEventControlImpl: Sized {
    fn SetMultiInterfacePublisherFilter();
    fn GetSubscriptions();
    fn SetDefaultQuery();
    fn AllowInprocActivation();
    fn SetAllowInprocActivation();
    fn FireInParallel();
    fn SetFireInParallel();
}
#[cfg(feature = "Win32_Foundation")]
impl IMultiInterfaceEventControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiInterfaceEventControlVtbl {
        unsafe extern "system" fn SetMultiInterfacePublisherFilter<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubscriptions<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDefaultQuery<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AllowInprocActivation<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAllowInprocActivation<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FireInParallel<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFireInParallel<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetMultiInterfacePublisherFilter::<Impl, IMPL_OFFSET>, GetSubscriptions::<Impl, IMPL_OFFSET>, SetDefaultQuery::<Impl, IMPL_OFFSET>, AllowInprocActivation::<Impl, IMPL_OFFSET>, SetAllowInprocActivation::<Impl, IMPL_OFFSET>, FireInParallel::<Impl, IMPL_OFFSET>, SetFireInParallel::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiInterfaceEventControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMultiInterfacePublisherFilterImpl: Sized {
    fn Initialize();
    fn PrepareToFire();
}
#[cfg(feature = "Win32_Foundation")]
impl IMultiInterfacePublisherFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfacePublisherFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiInterfacePublisherFilterVtbl {
        unsafe extern "system" fn Initialize<Impl: IMultiInterfacePublisherFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrepareToFire<Impl: IMultiInterfacePublisherFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, PrepareToFire::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiInterfacePublisherFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPublisherFilterImpl: Sized {
    fn Initialize();
    fn PrepareToFire();
}
#[cfg(feature = "Win32_Foundation")]
impl IPublisherFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPublisherFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPublisherFilterVtbl {
        unsafe extern "system" fn Initialize<Impl: IPublisherFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dispuserdefined: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PrepareToFire<Impl: IPublisherFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, PrepareToFire::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPublisherFilter as ::windows::core::Interface>::IID
    }
}
