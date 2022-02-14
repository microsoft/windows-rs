pub trait IDontSupportEventSubscription_Impl: Sized {}
impl IDontSupportEventSubscription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDontSupportEventSubscription_Impl, const OFFSET: isize>() -> IDontSupportEventSubscription_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDontSupportEventSubscription as ::windows::core::Interface>::IID
    }
}
pub trait IEnumEventObject_Impl: Sized {
    fn Clone(&self) -> ::windows::core::Result<IEnumEventObject>;
    fn Next(&self, creqelem: u32, ppinterface: *mut ::core::option::Option<::windows::core::IUnknown>, cretelem: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, cskipelem: u32) -> ::windows::core::Result<()>;
}
impl IEnumEventObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEventObject_Impl, const OFFSET: isize>() -> IEnumEventObject_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&creqelem), ::core::mem::transmute_copy(&ppinterface), ::core::mem::transmute_copy(&cretelem)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cskipelem)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumEventObject as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventClass_Impl: Sized + super::IDispatch_Impl {
    fn EventClassID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetEventClassID(&self, bstreventclassid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EventClassName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetEventClassName(&self, bstreventclassname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OwnerSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FiringInterfaceID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetFiringInterfaceID(&self, bstrfiringinterfaceid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn CustomConfigCLSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetCustomConfigCLSID(&self, bstrcustomconfigclsid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn TypeLib(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetTypeLib(&self, bstrtypelib: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventClass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>() -> IEventClass_Vtbl {
        unsafe extern "system" fn EventClassID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstreventclassid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventClassID(::core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn EventClassName<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventClassName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstreventclassname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassName<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventClassName(::core::mem::transmute(&bstreventclassname)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OwnerSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrownersid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOwnerSID(::core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn FiringInterfaceID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FiringInterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrfiringinterfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFiringInterfaceID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFiringInterfaceID(::core::mem::transmute(&bstrfiringinterfaceid)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn CustomConfigCLSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CustomConfigCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrcustomconfigclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomConfigCLSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCustomConfigCLSID(::core::mem::transmute(&bstrcustomconfigclsid)).into()
        }
        unsafe extern "system" fn TypeLib<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TypeLib() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtypelib = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeLib<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypelib: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTypeLib(::core::mem::transmute(&bstrtypelib)).into()
        }
        Self {
            base: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EventClassID: EventClassID::<Identity, Impl, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, Impl, OFFSET>,
            EventClassName: EventClassName::<Identity, Impl, OFFSET>,
            SetEventClassName: SetEventClassName::<Identity, Impl, OFFSET>,
            OwnerSID: OwnerSID::<Identity, Impl, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, Impl, OFFSET>,
            FiringInterfaceID: FiringInterfaceID::<Identity, Impl, OFFSET>,
            SetFiringInterfaceID: SetFiringInterfaceID::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            CustomConfigCLSID: CustomConfigCLSID::<Identity, Impl, OFFSET>,
            SetCustomConfigCLSID: SetCustomConfigCLSID::<Identity, Impl, OFFSET>,
            TypeLib: TypeLib::<Identity, Impl, OFFSET>,
            SetTypeLib: SetTypeLib::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventClass as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventClass2_Impl: Sized + super::IDispatch_Impl + IEventClass_Impl {
    fn PublisherID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MultiInterfacePublisherFilterCLSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetMultiInterfacePublisherFilterCLSID(&self, bstrpubfilclsid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&self, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventClass2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2_Impl, const OFFSET: isize>() -> IEventClass2_Vtbl {
        unsafe extern "system" fn PublisherID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublisherid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPublisherID(::core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn MultiInterfacePublisherFilterCLSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MultiInterfacePublisherFilterCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpubfilclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiInterfacePublisherFilterCLSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpubfilclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultiInterfacePublisherFilterCLSID(::core::mem::transmute(&bstrpubfilclsid)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowInprocActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *pfallowinprocactivation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowInprocActivation(::core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FireInParallel() {
                ::core::result::Result::Ok(ok__) => {
                    *pffireinparallel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: ::windows::core::IUnknownImpl, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFireInParallel(::core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base: IEventClass_Vtbl::new::<Identity, Impl, OFFSET>(),
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, Impl, OFFSET>,
            MultiInterfacePublisherFilterCLSID: MultiInterfacePublisherFilterCLSID::<Identity, Impl, OFFSET>,
            SetMultiInterfacePublisherFilterCLSID: SetMultiInterfacePublisherFilterCLSID::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            FireInParallel: FireInParallel::<Identity, Impl, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventClass2 as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID || iid == &<IEventClass as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventControl_Impl: Sized + super::IDispatch_Impl {
    fn SetPublisherFilter(&self, methodname: &super::super::super::Foundation::BSTR, ppublisherfilter: &::core::option::Option<IPublisherFilter>) -> ::windows::core::Result<()>;
    fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSubscriptions(&self, methodname: &super::super::super::Foundation::BSTR, optionalcriteria: &super::super::super::Foundation::BSTR, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&self, methodname: &super::super::super::Foundation::BSTR, criteria: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventControl_Impl, const OFFSET: isize>() -> IEventControl_Vtbl {
        unsafe extern "system" fn SetPublisherFilter<Identity: ::windows::core::IUnknownImpl, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppublisherfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPublisherFilter(::core::mem::transmute(&methodname), ::core::mem::transmute(&ppublisherfilter)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: ::windows::core::IUnknownImpl, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowInprocActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *pfallowinprocactivation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: ::windows::core::IUnknownImpl, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowInprocActivation(::core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: ::windows::core::IUnknownImpl, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubscriptions(::core::mem::transmute(&methodname), ::core::mem::transmute(&optionalcriteria), ::core::mem::transmute_copy(&optionalerrorindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: ::windows::core::IUnknownImpl, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, criteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetDefaultQuery(::core::mem::transmute(&methodname), ::core::mem::transmute(&criteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *errorindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPublisherFilter: SetPublisherFilter::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, Impl, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventControl as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEventObjectChange_Impl: Sized {
    fn ChangedSubscription(&self, changetype: EOC_ChangeType, bstrsubscriptionid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangedEventClass(&self, changetype: EOC_ChangeType, bstreventclassid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangedPublisher(&self, changetype: EOC_ChangeType, bstrpublisherid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEventObjectChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChange_Impl, const OFFSET: isize>() -> IEventObjectChange_Vtbl {
        unsafe extern "system" fn ChangedSubscription<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangedSubscription(::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangedEventClass(::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn ChangedPublisher<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangedPublisher(::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&bstrpublisherid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, Impl, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, Impl, OFFSET>,
            ChangedPublisher: ChangedPublisher::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectChange as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEventObjectChange2_Impl: Sized {
    fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()>;
    fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEventObjectChange2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChange2_Impl, const OFFSET: isize>() -> IEventObjectChange2_Vtbl {
        unsafe extern "system" fn ChangedSubscription<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangedSubscription(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectChange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ChangedEventClass(::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, Impl, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectChange2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventObjectCollection_Impl: Sized + super::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn Item(&self, objectid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn NewEnum(&self) -> ::windows::core::Result<IEnumEventObject>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, item: *const super::VARIANT, objectid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Remove(&self, objectid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventObjectCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectCollection_Impl, const OFFSET: isize>() -> IEventObjectCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunkenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pitem: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute(&objectid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const super::VARIANT, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&item), ::core::mem::transmute(&objectid)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute(&objectid)).into()
        }
        Self {
            base: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            NewEnum: NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectCollection as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventProperty_Impl: Sized + super::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetName(&self, propertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::VARIANT>;
    fn SetValue(&self, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventProperty_Impl, const OFFSET: isize>() -> IEventProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute(&propertyname)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Value() {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetValue(::core::mem::transmute_copy(&propertyvalue)).into()
        }
        Self {
            base: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventProperty as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventPublisher_Impl: Sized + super::IDispatch_Impl {
    fn PublisherID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PublisherName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherName(&self, bstrpublishername: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PublisherType(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherType(&self, bstrpublishertype: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OwnerSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDefaultProperty(&self, bstrpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn PutDefaultProperty(&self, bstrpropertyname: &super::super::super::Foundation::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn RemoveDefaultProperty(&self, bstrpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDefaultPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventPublisher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>() -> IEventPublisher_Vtbl {
        unsafe extern "system" fn PublisherID<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublisherid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPublisherID(::core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn PublisherName<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PublisherName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublishername = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherName<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublishername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPublisherName(::core::mem::transmute(&bstrpublishername)).into()
        }
        unsafe extern "system" fn PublisherType<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PublisherType() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublishertype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherType<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublishertype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPublisherType(::core::mem::transmute(&bstrpublishertype)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OwnerSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrownersid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOwnerSID(::core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn GetDefaultProperty<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultProperty(::core::mem::transmute(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutDefaultProperty<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutDefaultProperty(::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveDefaultProperty<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveDefaultProperty(::core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetDefaultPropertyCollection<Identity: ::windows::core::IUnknownImpl, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDefaultPropertyCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *collection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, Impl, OFFSET>,
            PublisherName: PublisherName::<Identity, Impl, OFFSET>,
            SetPublisherName: SetPublisherName::<Identity, Impl, OFFSET>,
            PublisherType: PublisherType::<Identity, Impl, OFFSET>,
            SetPublisherType: SetPublisherType::<Identity, Impl, OFFSET>,
            OwnerSID: OwnerSID::<Identity, Impl, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetDefaultProperty: GetDefaultProperty::<Identity, Impl, OFFSET>,
            PutDefaultProperty: PutDefaultProperty::<Identity, Impl, OFFSET>,
            RemoveDefaultProperty: RemoveDefaultProperty::<Identity, Impl, OFFSET>,
            GetDefaultPropertyCollection: GetDefaultPropertyCollection::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventPublisher as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventSubscription_Impl: Sized + super::IDispatch_Impl {
    fn SubscriptionID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSubscriptionID(&self, bstrsubscriptionid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubscriptionName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSubscriptionName(&self, bstrsubscriptionname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PublisherID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn EventClassID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetEventClassID(&self, bstreventclassid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MethodName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetMethodName(&self, bstrmethodname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubscriberCLSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetSubscriberCLSID(&self, bstrsubscriberclsid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SubscriberInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetSubscriberInterface(&self, psubscriberinterface: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn PerUser(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetPerUser(&self, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OwnerSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MachineName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetMachineName(&self, bstrmachinename: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPublisherProperty(&self, bstrpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn PutPublisherProperty(&self, bstrpropertyname: &super::super::super::Foundation::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn RemovePublisherProperty(&self, bstrpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetPublisherPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection>;
    fn GetSubscriberProperty(&self, bstrpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn PutSubscriberProperty(&self, bstrpropertyname: &super::super::super::Foundation::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn RemoveSubscriberProperty(&self, bstrpropertyname: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetSubscriberPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection>;
    fn InterfaceID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn SetInterfaceID(&self, bstrinterfaceid: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventSubscription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>() -> IEventSubscription_Vtbl {
        unsafe extern "system" fn SubscriptionID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubscriptionID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubscriptionid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubscriptionID(::core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn SubscriptionName<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubscriptionName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubscriptionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionName<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriptionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubscriptionName(::core::mem::transmute(&bstrsubscriptionname)).into()
        }
        unsafe extern "system" fn PublisherID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpublisherid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPublisherID(::core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn EventClassID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstreventclassid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEventClassID(::core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn MethodName<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MethodName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmethodname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethodName<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMethodName(::core::mem::transmute(&bstrmethodname)).into()
        }
        unsafe extern "system" fn SubscriberCLSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubscriberCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrsubscriberclsid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberCLSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubscriberCLSID(::core::mem::transmute(&bstrsubscriberclsid)).into()
        }
        unsafe extern "system" fn SubscriberInterface<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SubscriberInterface() {
                ::core::result::Result::Ok(ok__) => {
                    *ppsubscriberinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberInterface<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetSubscriberInterface(::core::mem::transmute(&psubscriberinterface)).into()
        }
        unsafe extern "system" fn PerUser<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PerUser() {
                ::core::result::Result::Ok(ok__) => {
                    *pfperuser = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerUser<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPerUser(::core::mem::transmute_copy(&fperuser)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OwnerSID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrownersid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOwnerSID(::core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn MachineName<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MachineName() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrmachinename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachineName<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachinename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMachineName(::core::mem::transmute(&bstrmachinename)).into()
        }
        unsafe extern "system" fn GetPublisherProperty<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPublisherProperty(::core::mem::transmute(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPublisherProperty<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutPublisherProperty(::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemovePublisherProperty<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemovePublisherProperty(::core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetPublisherPropertyCollection<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPublisherPropertyCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *collection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriberProperty<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubscriberProperty(::core::mem::transmute(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *propertyvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutSubscriberProperty<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PutSubscriberProperty(::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveSubscriberProperty<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveSubscriberProperty(::core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetSubscriberPropertyCollection<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubscriberPropertyCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *collection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrinterfaceid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetInterfaceID(::core::mem::transmute(&bstrinterfaceid)).into()
        }
        Self {
            base: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SubscriptionID: SubscriptionID::<Identity, Impl, OFFSET>,
            SetSubscriptionID: SetSubscriptionID::<Identity, Impl, OFFSET>,
            SubscriptionName: SubscriptionName::<Identity, Impl, OFFSET>,
            SetSubscriptionName: SetSubscriptionName::<Identity, Impl, OFFSET>,
            PublisherID: PublisherID::<Identity, Impl, OFFSET>,
            SetPublisherID: SetPublisherID::<Identity, Impl, OFFSET>,
            EventClassID: EventClassID::<Identity, Impl, OFFSET>,
            SetEventClassID: SetEventClassID::<Identity, Impl, OFFSET>,
            MethodName: MethodName::<Identity, Impl, OFFSET>,
            SetMethodName: SetMethodName::<Identity, Impl, OFFSET>,
            SubscriberCLSID: SubscriberCLSID::<Identity, Impl, OFFSET>,
            SetSubscriberCLSID: SetSubscriberCLSID::<Identity, Impl, OFFSET>,
            SubscriberInterface: SubscriberInterface::<Identity, Impl, OFFSET>,
            SetSubscriberInterface: SetSubscriberInterface::<Identity, Impl, OFFSET>,
            PerUser: PerUser::<Identity, Impl, OFFSET>,
            SetPerUser: SetPerUser::<Identity, Impl, OFFSET>,
            OwnerSID: OwnerSID::<Identity, Impl, OFFSET>,
            SetOwnerSID: SetOwnerSID::<Identity, Impl, OFFSET>,
            Enabled: Enabled::<Identity, Impl, OFFSET>,
            SetEnabled: SetEnabled::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            MachineName: MachineName::<Identity, Impl, OFFSET>,
            SetMachineName: SetMachineName::<Identity, Impl, OFFSET>,
            GetPublisherProperty: GetPublisherProperty::<Identity, Impl, OFFSET>,
            PutPublisherProperty: PutPublisherProperty::<Identity, Impl, OFFSET>,
            RemovePublisherProperty: RemovePublisherProperty::<Identity, Impl, OFFSET>,
            GetPublisherPropertyCollection: GetPublisherPropertyCollection::<Identity, Impl, OFFSET>,
            GetSubscriberProperty: GetSubscriberProperty::<Identity, Impl, OFFSET>,
            PutSubscriberProperty: PutSubscriberProperty::<Identity, Impl, OFFSET>,
            RemoveSubscriberProperty: RemoveSubscriberProperty::<Identity, Impl, OFFSET>,
            GetSubscriberPropertyCollection: GetSubscriberPropertyCollection::<Identity, Impl, OFFSET>,
            InterfaceID: InterfaceID::<Identity, Impl, OFFSET>,
            SetInterfaceID: SetInterfaceID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventSubscription as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventSystem_Impl: Sized + super::IDispatch_Impl {
    fn Query(&self, progid: &super::super::super::Foundation::BSTR, querycriteria: &super::super::super::Foundation::BSTR, errorindex: *mut i32, ppinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Store(&self, progid: &super::super::super::Foundation::BSTR, pinterface: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Remove(&self, progid: &super::super::super::Foundation::BSTR, querycriteria: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn EventObjectChangeEventClassID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR>;
    fn QueryS(&self, progid: &super::super::super::Foundation::BSTR, querycriteria: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn RemoveS(&self, progid: &super::super::super::Foundation::BSTR, querycriteria: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventSystem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEventSystem_Impl, const OFFSET: isize>() -> IEventSystem_Vtbl {
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Query(::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria), ::core::mem::transmute_copy(&errorindex), ::core::mem::transmute_copy(&ppinterface)).into()
        }
        unsafe extern "system" fn Store<Identity: ::windows::core::IUnknownImpl, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Store(::core::mem::transmute(&progid), ::core::mem::transmute(&pinterface)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Remove(::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *errorindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventObjectChangeEventClassID<Identity: ::windows::core::IUnknownImpl, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EventObjectChangeEventClassID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstreventclassid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryS<Identity: ::windows::core::IUnknownImpl, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QueryS(::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppinterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveS<Identity: ::windows::core::IUnknownImpl, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveS(::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria)).into()
        }
        Self {
            base: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Query: Query::<Identity, Impl, OFFSET>,
            Store: Store::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            EventObjectChangeEventClassID: EventObjectChangeEventClassID::<Identity, Impl, OFFSET>,
            QueryS: QueryS::<Identity, Impl, OFFSET>,
            RemoveS: RemoveS::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventSystem as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IFiringControl_Impl: Sized + super::IDispatch_Impl {
    fn FireSubscription(&self, subscription: &::core::option::Option<IEventSubscription>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IFiringControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFiringControl_Impl, const OFFSET: isize>() -> IFiringControl_Vtbl {
        unsafe extern "system" fn FireSubscription<Identity: ::windows::core::IUnknownImpl, Impl: IFiringControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscription: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).FireSubscription(::core::mem::transmute(&subscription)).into()
        }
        Self { base: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), FireSubscription: FireSubscription::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFiringControl as ::windows::core::Interface>::IID || iid == &<super::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMultiInterfaceEventControl_Impl: Sized {
    fn SetMultiInterfacePublisherFilter(&self, classfilter: &::core::option::Option<IMultiInterfacePublisherFilter>) -> ::windows::core::Result<()>;
    fn GetSubscriptions(&self, eventiid: *const ::windows::core::GUID, bstrmethodname: &super::super::super::Foundation::BSTR, optionalcriteria: &super::super::super::Foundation::BSTR, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&self, eventiid: *const ::windows::core::GUID, bstrmethodname: &super::super::super::Foundation::BSTR, bstrcriteria: &super::super::super::Foundation::BSTR) -> ::windows::core::Result<i32>;
    fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&self, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMultiInterfaceEventControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>() -> IMultiInterfaceEventControl_Vtbl {
        unsafe extern "system" fn SetMultiInterfacePublisherFilter<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMultiInterfacePublisherFilter(::core::mem::transmute(&classfilter)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetSubscriptions(::core::mem::transmute_copy(&eventiid), ::core::mem::transmute(&bstrmethodname), ::core::mem::transmute(&optionalcriteria), ::core::mem::transmute_copy(&optionalerrorindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SetDefaultQuery(::core::mem::transmute_copy(&eventiid), ::core::mem::transmute(&bstrmethodname), ::core::mem::transmute(&bstrcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    *errorindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AllowInprocActivation() {
                ::core::result::Result::Ok(ok__) => {
                    *pfallowinprocactivation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAllowInprocActivation(::core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FireInParallel() {
                ::core::result::Result::Ok(ok__) => {
                    *pffireinparallel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFireInParallel(::core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SetMultiInterfacePublisherFilter: SetMultiInterfacePublisherFilter::<Identity, Impl, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, Impl, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            FireInParallel: FireInParallel::<Identity, Impl, OFFSET>,
            SetFireInParallel: SetFireInParallel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiInterfaceEventControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IMultiInterfacePublisherFilter_Impl: Sized {
    fn Initialize(&self, peic: &::core::option::Option<IMultiInterfaceEventControl>) -> ::windows::core::Result<()>;
    fn PrepareToFire(&self, iid: *const ::windows::core::GUID, methodname: &super::super::super::Foundation::BSTR, firingcontrol: &::core::option::Option<IFiringControl>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IMultiInterfacePublisherFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>() -> IMultiInterfacePublisherFilter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peic: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&peic)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: ::windows::core::IUnknownImpl, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrepareToFire(::core::mem::transmute_copy(&iid), ::core::mem::transmute(&methodname), ::core::mem::transmute(&firingcontrol)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiInterfacePublisherFilter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IPublisherFilter_Impl: Sized {
    fn Initialize(&self, methodname: &super::super::super::Foundation::BSTR, dispuserdefined: &::core::option::Option<super::IDispatch>) -> ::windows::core::Result<()>;
    fn PrepareToFire(&self, methodname: &super::super::super::Foundation::BSTR, firingcontrol: &::core::option::Option<IFiringControl>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IPublisherFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPublisherFilter_Impl, const OFFSET: isize>() -> IPublisherFilter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IPublisherFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dispuserdefined: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&methodname), ::core::mem::transmute(&dispuserdefined)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: ::windows::core::IUnknownImpl, Impl: IPublisherFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PrepareToFire(::core::mem::transmute(&methodname), ::core::mem::transmute(&firingcontrol)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPublisherFilter as ::windows::core::Interface>::IID
    }
}
