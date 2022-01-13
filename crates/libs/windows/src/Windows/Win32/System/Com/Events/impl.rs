pub trait IDontSupportEventSubscriptionImpl: Sized {}
impl IDontSupportEventSubscriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDontSupportEventSubscriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDontSupportEventSubscriptionVtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDontSupportEventSubscription as ::windows::core::Interface>::IID
    }
}
pub trait IEnumEventObjectImpl: Sized {
    fn Clone(&mut self) -> ::windows::core::Result<IEnumEventObject>;
    fn Next(&mut self, creqelem: u32, ppinterface: *mut ::core::option::Option<::windows::core::IUnknown>, cretelem: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cskipelem: u32) -> ::windows::core::Result<()>;
}
impl IEnumEventObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEventObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumEventObjectVtbl {
        unsafe extern "system" fn Clone<Impl: IEnumEventObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Impl: IEnumEventObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&creqelem), ::core::mem::transmute_copy(&ppinterface), ::core::mem::transmute_copy(&cretelem)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumEventObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumEventObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cskipelem)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Clone: Clone::<Impl, IMPL_OFFSET>,
            Next: Next::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumEventObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventClassImpl: Sized + IDispatchImpl {
    fn EventClassID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetEventClassID(&mut self, bstreventclassid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EventClassName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetEventClassName(&mut self, bstreventclassname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OwnerSID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetOwnerSID(&mut self, bstrownersid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FiringInterfaceID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetFiringInterfaceID(&mut self, bstrfiringinterfaceid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CustomConfigCLSID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCustomConfigCLSID(&mut self, bstrcustomconfigclsid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TypeLib(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetTypeLib(&mut self, bstrtypelib: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventClassVtbl {
        unsafe extern "system" fn EventClassID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstreventclassid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventClassID(::core::mem::transmute_copy(&bstreventclassid)).into()
        }
        unsafe extern "system" fn EventClassName<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstreventclassname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassName<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventClassName(::core::mem::transmute_copy(&bstreventclassname)).into()
        }
        unsafe extern "system" fn OwnerSID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OwnerSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrownersid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOwnerSID(::core::mem::transmute_copy(&bstrownersid)).into()
        }
        unsafe extern "system" fn FiringInterfaceID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FiringInterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfiringinterfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFiringInterfaceID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFiringInterfaceID(::core::mem::transmute_copy(&bstrfiringinterfaceid)).into()
        }
        unsafe extern "system" fn Description<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn CustomConfigCLSID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CustomConfigCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcustomconfigclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomConfigCLSID<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCustomConfigCLSID(::core::mem::transmute_copy(&bstrcustomconfigclsid)).into()
        }
        unsafe extern "system" fn TypeLib<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TypeLib() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtypelib = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeLib<Impl: IEventClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypelib: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTypeLib(::core::mem::transmute_copy(&bstrtypelib)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EventClassID: EventClassID::<Impl, IMPL_OFFSET>,
            SetEventClassID: SetEventClassID::<Impl, IMPL_OFFSET>,
            EventClassName: EventClassName::<Impl, IMPL_OFFSET>,
            SetEventClassName: SetEventClassName::<Impl, IMPL_OFFSET>,
            OwnerSID: OwnerSID::<Impl, IMPL_OFFSET>,
            SetOwnerSID: SetOwnerSID::<Impl, IMPL_OFFSET>,
            FiringInterfaceID: FiringInterfaceID::<Impl, IMPL_OFFSET>,
            SetFiringInterfaceID: SetFiringInterfaceID::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            CustomConfigCLSID: CustomConfigCLSID::<Impl, IMPL_OFFSET>,
            SetCustomConfigCLSID: SetCustomConfigCLSID::<Impl, IMPL_OFFSET>,
            TypeLib: TypeLib::<Impl, IMPL_OFFSET>,
            SetTypeLib: SetTypeLib::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventClass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventClass2Impl: Sized + IDispatchImpl + IEventClassImpl {
    fn PublisherID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherID(&mut self, bstrpublisherid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MultiInterfacePublisherFilterCLSID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetMultiInterfacePublisherFilterCLSID(&mut self, bstrpubfilclsid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllowInprocActivation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&mut self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FireInParallel(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&mut self, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventClass2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventClass2Vtbl {
        unsafe extern "system" fn PublisherID<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublisherid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublisherID(::core::mem::transmute_copy(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn MultiInterfacePublisherFilterCLSID<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultiInterfacePublisherFilterCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpubfilclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiInterfacePublisherFilterCLSID<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpubfilclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMultiInterfacePublisherFilterCLSID(::core::mem::transmute_copy(&bstrpubfilclsid)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInprocActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *pfallowinprocactivation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInprocActivation(::core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FireInParallel() {
                ::core::result::Result::Ok(ok__) => {
                    *pffireinparallel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Impl: IEventClass2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFireInParallel(::core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base: IEventClassVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PublisherID: PublisherID::<Impl, IMPL_OFFSET>,
            SetPublisherID: SetPublisherID::<Impl, IMPL_OFFSET>,
            MultiInterfacePublisherFilterCLSID: MultiInterfacePublisherFilterCLSID::<Impl, IMPL_OFFSET>,
            SetMultiInterfacePublisherFilterCLSID: SetMultiInterfacePublisherFilterCLSID::<Impl, IMPL_OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Impl, IMPL_OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Impl, IMPL_OFFSET>,
            FireInParallel: FireInParallel::<Impl, IMPL_OFFSET>,
            SetFireInParallel: SetFireInParallel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventClass2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventControlImpl: Sized + IDispatchImpl {
    fn SetPublisherFilter(&mut self, methodname: super::super::super::Foundation::BSTR, ppublisherfilter: ::core::option::Option<IPublisherFilter>) -> ::windows::core::Result<()>;
    fn AllowInprocActivation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&mut self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSubscriptions(&mut self, methodname: super::super::super::Foundation::BSTR, optionalcriteria: super::super::super::Foundation::BSTR, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&mut self, methodname: super::super::super::Foundation::BSTR, criteria: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventControlVtbl {
        unsafe extern "system" fn SetPublisherFilter<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppublisherfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublisherFilter(::core::mem::transmute_copy(&methodname), ::core::mem::transmute(&ppublisherfilter)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInprocActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *pfallowinprocactivation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInprocActivation(::core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubscriptions(::core::mem::transmute_copy(&methodname), ::core::mem::transmute_copy(&optionalcriteria), ::core::mem::transmute_copy(&optionalerrorindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Impl: IEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, criteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultQuery(::core::mem::transmute_copy(&methodname), ::core::mem::transmute_copy(&criteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *errorindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetPublisherFilter: SetPublisherFilter::<Impl, IMPL_OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Impl, IMPL_OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Impl, IMPL_OFFSET>,
            GetSubscriptions: GetSubscriptions::<Impl, IMPL_OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEventObjectChangeImpl: Sized {
    fn ChangedSubscription(&mut self, changetype: EOC_ChangeType, bstrsubscriptionid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangedEventClass(&mut self, changetype: EOC_ChangeType, bstreventclassid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangedPublisher(&mut self, changetype: EOC_ChangeType, bstrpublisherid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEventObjectChangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventObjectChangeVtbl {
        unsafe extern "system" fn ChangedSubscription<Impl: IEventObjectChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangedSubscription(::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Impl: IEventObjectChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangedEventClass(::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&bstreventclassid)).into()
        }
        unsafe extern "system" fn ChangedPublisher<Impl: IEventObjectChangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangedPublisher(::core::mem::transmute_copy(&changetype), ::core::mem::transmute_copy(&bstrpublisherid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Impl, IMPL_OFFSET>,
            ChangedEventClass: ChangedEventClass::<Impl, IMPL_OFFSET>,
            ChangedPublisher: ChangedPublisher::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEventObjectChange2Impl: Sized {
    fn ChangedSubscription(&mut self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()>;
    fn ChangedEventClass(&mut self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEventObjectChange2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChange2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventObjectChange2Vtbl {
        unsafe extern "system" fn ChangedSubscription<Impl: IEventObjectChange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangedSubscription(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Impl: IEventObjectChange2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChangedEventClass(::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Impl, IMPL_OFFSET>,
            ChangedEventClass: ChangedEventClass::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectChange2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventObjectCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&mut self, objectid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn NewEnum(&mut self) -> ::windows::core::Result<IEnumEventObject>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn Add(&mut self, item: *const super::VARIANT, objectid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Remove(&mut self, objectid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventObjectCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventObjectCollectionVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pitem: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&objectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewEnum<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const super::VARIANT, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&item), ::core::mem::transmute_copy(&objectid)).into()
        }
        unsafe extern "system" fn Remove<Impl: IEventObjectCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&objectid)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            NewEnum: NewEnum::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventPropertyImpl: Sized + IDispatchImpl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetName(&mut self, propertyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Value(&mut self) -> ::windows::core::Result<super::VARIANT>;
    fn SetValue(&mut self, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventPropertyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventPropertyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventPropertyVtbl {
        unsafe extern "system" fn Name<Impl: IEventPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IEventPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&propertyname)).into()
        }
        unsafe extern "system" fn Value<Impl: IEventPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IEventPropertyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&propertyvalue)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Value: Value::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventProperty as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventPublisherImpl: Sized + IDispatchImpl {
    fn PublisherID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherID(&mut self, bstrpublisherid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PublisherName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherName(&mut self, bstrpublishername: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PublisherType(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherType(&mut self, bstrpublishertype: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OwnerSID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetOwnerSID(&mut self, bstrownersid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDefaultProperty(&mut self, bstrpropertyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn PutDefaultProperty(&mut self, bstrpropertyname: super::super::super::Foundation::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn RemoveDefaultProperty(&mut self, bstrpropertyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDefaultPropertyCollection(&mut self) -> ::windows::core::Result<IEventObjectCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventPublisherVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisherImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventPublisherVtbl {
        unsafe extern "system" fn PublisherID<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublisherid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublisherID(::core::mem::transmute_copy(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn PublisherName<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublisherName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublishername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherName<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublishername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublisherName(::core::mem::transmute_copy(&bstrpublishername)).into()
        }
        unsafe extern "system" fn PublisherType<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublisherType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublishertype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherType<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublishertype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublisherType(::core::mem::transmute_copy(&bstrpublishertype)).into()
        }
        unsafe extern "system" fn OwnerSID<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OwnerSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrownersid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOwnerSID(::core::mem::transmute_copy(&bstrownersid)).into()
        }
        unsafe extern "system" fn Description<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn GetDefaultProperty<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultProperty(::core::mem::transmute_copy(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutDefaultProperty<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutDefaultProperty(::core::mem::transmute_copy(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveDefaultProperty<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveDefaultProperty(::core::mem::transmute_copy(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetDefaultPropertyCollection<Impl: IEventPublisherImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultPropertyCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *collection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PublisherID: PublisherID::<Impl, IMPL_OFFSET>,
            SetPublisherID: SetPublisherID::<Impl, IMPL_OFFSET>,
            PublisherName: PublisherName::<Impl, IMPL_OFFSET>,
            SetPublisherName: SetPublisherName::<Impl, IMPL_OFFSET>,
            PublisherType: PublisherType::<Impl, IMPL_OFFSET>,
            SetPublisherType: SetPublisherType::<Impl, IMPL_OFFSET>,
            OwnerSID: OwnerSID::<Impl, IMPL_OFFSET>,
            SetOwnerSID: SetOwnerSID::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            GetDefaultProperty: GetDefaultProperty::<Impl, IMPL_OFFSET>,
            PutDefaultProperty: PutDefaultProperty::<Impl, IMPL_OFFSET>,
            RemoveDefaultProperty: RemoveDefaultProperty::<Impl, IMPL_OFFSET>,
            GetDefaultPropertyCollection: GetDefaultPropertyCollection::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventPublisher as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventSubscriptionImpl: Sized + IDispatchImpl {
    fn SubscriptionID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSubscriptionID(&mut self, bstrsubscriptionid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubscriptionName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSubscriptionName(&mut self, bstrsubscriptionname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PublisherID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherID(&mut self, bstrpublisherid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EventClassID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetEventClassID(&mut self, bstreventclassid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MethodName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetMethodName(&mut self, bstrmethodname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubscriberCLSID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSubscriberCLSID(&mut self, bstrsubscriberclsid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubscriberInterface(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetSubscriberInterface(&mut self, psubscriberinterface: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn PerUser(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetPerUser(&mut self, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OwnerSID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetOwnerSID(&mut self, bstrownersid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnabled(&mut self, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MachineName(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetMachineName(&mut self, bstrmachinename: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPublisherProperty(&mut self, bstrpropertyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn PutPublisherProperty(&mut self, bstrpropertyname: super::super::super::Foundation::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn RemovePublisherProperty(&mut self, bstrpropertyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPublisherPropertyCollection(&mut self) -> ::windows::core::Result<IEventObjectCollection>;
    fn GetSubscriberProperty(&mut self, bstrpropertyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn PutSubscriberProperty(&mut self, bstrpropertyname: super::super::super::Foundation::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn RemoveSubscriberProperty(&mut self, bstrpropertyname: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetSubscriberPropertyCollection(&mut self) -> ::windows::core::Result<IEventObjectCollection>;
    fn InterfaceID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetInterfaceID(&mut self, bstrinterfaceid: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventSubscriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventSubscriptionVtbl {
        unsafe extern "system" fn SubscriptionID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriptionID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubscriptionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubscriptionID(::core::mem::transmute_copy(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn SubscriptionName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriptionName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubscriptionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriptionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubscriptionName(::core::mem::transmute_copy(&bstrsubscriptionname)).into()
        }
        unsafe extern "system" fn PublisherID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublisherid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPublisherID(::core::mem::transmute_copy(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn EventClassID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstreventclassid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventClassID(::core::mem::transmute_copy(&bstreventclassid)).into()
        }
        unsafe extern "system" fn MethodName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MethodName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmethodname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethodName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMethodName(::core::mem::transmute_copy(&bstrmethodname)).into()
        }
        unsafe extern "system" fn SubscriberCLSID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriberCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubscriberclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberCLSID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubscriberCLSID(::core::mem::transmute_copy(&bstrsubscriberclsid)).into()
        }
        unsafe extern "system" fn SubscriberInterface<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SubscriberInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubscriberinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberInterface<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSubscriberInterface(::core::mem::transmute(&psubscriberinterface)).into()
        }
        unsafe extern "system" fn PerUser<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PerUser() {
                ::core::result::Result::Ok(ok__) => {
                    *pfperuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerUser<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPerUser(::core::mem::transmute_copy(&fperuser)).into()
        }
        unsafe extern "system" fn OwnerSID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OwnerSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrownersid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOwnerSID(::core::mem::transmute_copy(&bstrownersid)).into()
        }
        unsafe extern "system" fn Enabled<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn Description<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn MachineName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MachineName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachinename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachineName<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachinename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMachineName(::core::mem::transmute_copy(&bstrmachinename)).into()
        }
        unsafe extern "system" fn GetPublisherProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisherProperty(::core::mem::transmute_copy(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPublisherProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutPublisherProperty(::core::mem::transmute_copy(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemovePublisherProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemovePublisherProperty(::core::mem::transmute_copy(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetPublisherPropertyCollection<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPublisherPropertyCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *collection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriberProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubscriberProperty(::core::mem::transmute_copy(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutSubscriberProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PutSubscriberProperty(::core::mem::transmute_copy(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveSubscriberProperty<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveSubscriberProperty(::core::mem::transmute_copy(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetSubscriberPropertyCollection<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubscriberPropertyCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *collection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrinterfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceID<Impl: IEventSubscriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInterfaceID(::core::mem::transmute_copy(&bstrinterfaceid)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SubscriptionID: SubscriptionID::<Impl, IMPL_OFFSET>,
            SetSubscriptionID: SetSubscriptionID::<Impl, IMPL_OFFSET>,
            SubscriptionName: SubscriptionName::<Impl, IMPL_OFFSET>,
            SetSubscriptionName: SetSubscriptionName::<Impl, IMPL_OFFSET>,
            PublisherID: PublisherID::<Impl, IMPL_OFFSET>,
            SetPublisherID: SetPublisherID::<Impl, IMPL_OFFSET>,
            EventClassID: EventClassID::<Impl, IMPL_OFFSET>,
            SetEventClassID: SetEventClassID::<Impl, IMPL_OFFSET>,
            MethodName: MethodName::<Impl, IMPL_OFFSET>,
            SetMethodName: SetMethodName::<Impl, IMPL_OFFSET>,
            SubscriberCLSID: SubscriberCLSID::<Impl, IMPL_OFFSET>,
            SetSubscriberCLSID: SetSubscriberCLSID::<Impl, IMPL_OFFSET>,
            SubscriberInterface: SubscriberInterface::<Impl, IMPL_OFFSET>,
            SetSubscriberInterface: SetSubscriberInterface::<Impl, IMPL_OFFSET>,
            PerUser: PerUser::<Impl, IMPL_OFFSET>,
            SetPerUser: SetPerUser::<Impl, IMPL_OFFSET>,
            OwnerSID: OwnerSID::<Impl, IMPL_OFFSET>,
            SetOwnerSID: SetOwnerSID::<Impl, IMPL_OFFSET>,
            Enabled: Enabled::<Impl, IMPL_OFFSET>,
            SetEnabled: SetEnabled::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            MachineName: MachineName::<Impl, IMPL_OFFSET>,
            SetMachineName: SetMachineName::<Impl, IMPL_OFFSET>,
            GetPublisherProperty: GetPublisherProperty::<Impl, IMPL_OFFSET>,
            PutPublisherProperty: PutPublisherProperty::<Impl, IMPL_OFFSET>,
            RemovePublisherProperty: RemovePublisherProperty::<Impl, IMPL_OFFSET>,
            GetPublisherPropertyCollection: GetPublisherPropertyCollection::<Impl, IMPL_OFFSET>,
            GetSubscriberProperty: GetSubscriberProperty::<Impl, IMPL_OFFSET>,
            PutSubscriberProperty: PutSubscriberProperty::<Impl, IMPL_OFFSET>,
            RemoveSubscriberProperty: RemoveSubscriberProperty::<Impl, IMPL_OFFSET>,
            GetSubscriberPropertyCollection: GetSubscriberPropertyCollection::<Impl, IMPL_OFFSET>,
            InterfaceID: InterfaceID::<Impl, IMPL_OFFSET>,
            SetInterfaceID: SetInterfaceID::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventSubscription as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventSystemImpl: Sized + IDispatchImpl {
    fn Query(&mut self, progid: super::super::super::Foundation::BSTR, querycriteria: super::super::super::Foundation::BSTR, errorindex: *mut i32, ppinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Store(&mut self, progid: super::super::super::Foundation::BSTR, pinterface: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, progid: super::super::super::Foundation::BSTR, querycriteria: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn EventObjectChangeEventClassID(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn QueryS(&mut self, progid: super::super::super::Foundation::BSTR, querycriteria: super::super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn RemoveS(&mut self, progid: super::super::super::Foundation::BSTR, querycriteria: super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventSystemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventSystemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEventSystemVtbl {
        unsafe extern "system" fn Query<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Query(::core::mem::transmute_copy(&progid), ::core::mem::transmute_copy(&querycriteria), ::core::mem::transmute_copy(&errorindex), ::core::mem::transmute_copy(&ppinterface)).into()
        }
        unsafe extern "system" fn Store<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Store(::core::mem::transmute_copy(&progid), ::core::mem::transmute(&pinterface)).into()
        }
        unsafe extern "system" fn Remove<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(::core::mem::transmute_copy(&progid), ::core::mem::transmute_copy(&querycriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *errorindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventObjectChangeEventClassID<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EventObjectChangeEventClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstreventclassid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryS<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryS(::core::mem::transmute_copy(&progid), ::core::mem::transmute_copy(&querycriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveS<Impl: IEventSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveS(::core::mem::transmute_copy(&progid), ::core::mem::transmute_copy(&querycriteria)).into()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Query: Query::<Impl, IMPL_OFFSET>,
            Store: Store::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            EventObjectChangeEventClassID: EventObjectChangeEventClassID::<Impl, IMPL_OFFSET>,
            QueryS: QueryS::<Impl, IMPL_OFFSET>,
            RemoveS: RemoveS::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventSystem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IFiringControlImpl: Sized + IDispatchImpl {
    fn FireSubscription(&mut self, subscription: ::core::option::Option<IEventSubscription>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IFiringControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFiringControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFiringControlVtbl {
        unsafe extern "system" fn FireSubscription<Impl: IFiringControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscription: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).FireSubscription(::core::mem::transmute(&subscription)).into()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), FireSubscription: FireSubscription::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFiringControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMultiInterfaceEventControlImpl: Sized {
    fn SetMultiInterfacePublisherFilter(&mut self, classfilter: ::core::option::Option<IMultiInterfacePublisherFilter>) -> ::windows::core::Result<()>;
    fn GetSubscriptions(&mut self, eventiid: *const ::windows::core::GUID, bstrmethodname: super::super::super::Foundation::BSTR, optionalcriteria: super::super::super::Foundation::BSTR, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&mut self, eventiid: *const ::windows::core::GUID, bstrmethodname: super::super::super::Foundation::BSTR, bstrcriteria: super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn AllowInprocActivation(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&mut self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FireInParallel(&mut self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&mut self, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMultiInterfaceEventControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiInterfaceEventControlVtbl {
        unsafe extern "system" fn SetMultiInterfacePublisherFilter<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMultiInterfacePublisherFilter(::core::mem::transmute(&classfilter)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSubscriptions(::core::mem::transmute_copy(&eventiid), ::core::mem::transmute_copy(&bstrmethodname), ::core::mem::transmute_copy(&optionalcriteria), ::core::mem::transmute_copy(&optionalerrorindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDefaultQuery(::core::mem::transmute_copy(&eventiid), ::core::mem::transmute_copy(&bstrmethodname), ::core::mem::transmute_copy(&bstrcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *errorindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInprocActivation<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AllowInprocActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *pfallowinprocactivation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAllowInprocActivation(::core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FireInParallel() {
                ::core::result::Result::Ok(ok__) => {
                    *pffireinparallel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Impl: IMultiInterfaceEventControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFireInParallel(::core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetMultiInterfacePublisherFilter: SetMultiInterfacePublisherFilter::<Impl, IMPL_OFFSET>,
            GetSubscriptions: GetSubscriptions::<Impl, IMPL_OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Impl, IMPL_OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Impl, IMPL_OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Impl, IMPL_OFFSET>,
            FireInParallel: FireInParallel::<Impl, IMPL_OFFSET>,
            SetFireInParallel: SetFireInParallel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiInterfaceEventControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMultiInterfacePublisherFilterImpl: Sized {
    fn Initialize(&mut self, peic: ::core::option::Option<IMultiInterfaceEventControl>) -> ::windows::core::Result<()>;
    fn PrepareToFire(&mut self, iid: *const ::windows::core::GUID, methodname: super::super::super::Foundation::BSTR, firingcontrol: ::core::option::Option<IFiringControl>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMultiInterfacePublisherFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfacePublisherFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultiInterfacePublisherFilterVtbl {
        unsafe extern "system" fn Initialize<Impl: IMultiInterfacePublisherFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&peic)).into()
        }
        unsafe extern "system" fn PrepareToFire<Impl: IMultiInterfacePublisherFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareToFire(::core::mem::transmute_copy(&iid), ::core::mem::transmute_copy(&methodname), ::core::mem::transmute(&firingcontrol)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            PrepareToFire: PrepareToFire::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiInterfacePublisherFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPublisherFilterImpl: Sized {
    fn Initialize(&mut self, methodname: super::super::super::Foundation::BSTR, dispuserdefined: ::core::option::Option<super::IDispatch>) -> ::windows::core::Result<()>;
    fn PrepareToFire(&mut self, methodname: super::super::super::Foundation::BSTR, firingcontrol: ::core::option::Option<IFiringControl>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPublisherFilterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPublisherFilterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPublisherFilterVtbl {
        unsafe extern "system" fn Initialize<Impl: IPublisherFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dispuserdefined: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&methodname), ::core::mem::transmute(&dispuserdefined)).into()
        }
        unsafe extern "system" fn PrepareToFire<Impl: IPublisherFilterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareToFire(::core::mem::transmute_copy(&methodname), ::core::mem::transmute(&firingcontrol)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            PrepareToFire: PrepareToFire::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPublisherFilter as ::windows::core::Interface>::IID
    }
}
