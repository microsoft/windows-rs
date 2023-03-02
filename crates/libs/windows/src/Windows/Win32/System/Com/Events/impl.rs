#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"implement\"`*"]
pub trait IDontSupportEventSubscription_Impl: Sized {}
impl ::windows::core::RuntimeName for IDontSupportEventSubscription {}
impl IDontSupportEventSubscription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDontSupportEventSubscription_Impl, const OFFSET: isize>() -> IDontSupportEventSubscription_Vtbl {
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDontSupportEventSubscription as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"implement\"`*"]
pub trait IEnumEventObject_Impl: Sized {
    fn Clone(&self) -> ::windows::core::Result<IEnumEventObject>;
    fn Next(&self, creqelem: u32, ppinterface: *mut ::core::option::Option<::windows::core::IUnknown>, cretelem: *mut u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Skip(&self, cskipelem: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IEnumEventObject {}
impl IEnumEventObject_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>() -> IEnumEventObject_Vtbl {
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&creqelem), ::core::mem::transmute_copy(&ppinterface), ::core::mem::transmute_copy(&cretelem)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumEventObject_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&cskipelem)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumEventObject as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventClass_Impl: Sized + super::IDispatch_Impl {
    fn EventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetEventClassID(&self, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn EventClassName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetEventClassName(&self, bstreventclassname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FiringInterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFiringInterfaceID(&self, bstrfiringinterfaceid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn CustomConfigCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCustomConfigCLSID(&self, bstrcustomconfigclsid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn TypeLib(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetTypeLib(&self, bstrtypelib: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventClass {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventClass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>() -> IEventClass_Vtbl {
        unsafe extern "system" fn EventClassID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventClassID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstreventclassid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventClassID(::core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn EventClassName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventClassName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstreventclassname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventClassName(::core::mem::transmute(&bstreventclassname)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OwnerSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrownersid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOwnerSID(::core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn FiringInterfaceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FiringInterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrfiringinterfaceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFiringInterfaceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFiringInterfaceID(::core::mem::transmute(&bstrfiringinterfaceid)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn CustomConfigCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CustomConfigCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrcustomconfigclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomConfigCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCustomConfigCLSID(::core::mem::transmute(&bstrcustomconfigclsid)).into()
        }
        unsafe extern "system" fn TypeLib<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TypeLib() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtypelib, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTypeLib<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtypelib: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTypeLib(::core::mem::transmute(&bstrtypelib)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IEventClass as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventClass2_Impl: Sized + IEventClass_Impl {
    fn PublisherID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MultiInterfacePublisherFilterCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMultiInterfacePublisherFilterCLSID(&self, bstrpubfilclsid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&self, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventClass2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventClass2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>() -> IEventClass2_Vtbl {
        unsafe extern "system" fn PublisherID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublisherid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPublisherID(::core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn MultiInterfacePublisherFilterCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MultiInterfacePublisherFilterCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpubfilclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultiInterfacePublisherFilterCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpubfilclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultiInterfacePublisherFilterCLSID(::core::mem::transmute(&bstrpubfilclsid)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowInprocActivation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallowinprocactivation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowInprocActivation(::core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FireInParallel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffireinparallel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventClass2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFireInParallel(::core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base__: IEventClass_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IEventClass2 as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IEventClass as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventControl_Impl: Sized + super::IDispatch_Impl {
    fn SetPublisherFilter(&self, methodname: &::windows::core::BSTR, ppublisherfilter: ::core::option::Option<&IPublisherFilter>) -> ::windows::core::Result<()>;
    fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetSubscriptions(&self, methodname: &::windows::core::BSTR, optionalcriteria: &::windows::core::BSTR, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&self, methodname: &::windows::core::BSTR, criteria: &::windows::core::BSTR) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>() -> IEventControl_Vtbl {
        unsafe extern "system" fn SetPublisherFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppublisherfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPublisherFilter(::core::mem::transmute(&methodname), ::windows::core::from_raw_borrowed(&ppublisherfilter)).into()
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowInprocActivation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallowinprocactivation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowInprocActivation(::core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalcriteria: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubscriptions(::core::mem::transmute(&methodname), ::core::mem::transmute(&optionalcriteria), ::core::mem::transmute_copy(&optionalerrorindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, criteria: ::std::mem::MaybeUninit<::windows::core::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetDefaultQuery(::core::mem::transmute(&methodname), ::core::mem::transmute(&criteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetPublisherFilter: SetPublisherFilter::<Identity, Impl, OFFSET>,
            AllowInprocActivation: AllowInprocActivation::<Identity, Impl, OFFSET>,
            SetAllowInprocActivation: SetAllowInprocActivation::<Identity, Impl, OFFSET>,
            GetSubscriptions: GetSubscriptions::<Identity, Impl, OFFSET>,
            SetDefaultQuery: SetDefaultQuery::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventControl as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"implement\"`*"]
pub trait IEventObjectChange_Impl: Sized {
    fn ChangedSubscription(&self, changetype: EOC_ChangeType, bstrsubscriptionid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ChangedEventClass(&self, changetype: EOC_ChangeType, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ChangedPublisher(&self, changetype: EOC_ChangeType, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IEventObjectChange {}
impl IEventObjectChange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: isize>() -> IEventObjectChange_Vtbl {
        unsafe extern "system" fn ChangedSubscription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangedSubscription(::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangedEventClass(::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn ChangedPublisher<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangedPublisher(::core::mem::transmute_copy(&changetype), ::core::mem::transmute(&bstrpublisherid)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, Impl, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, Impl, OFFSET>,
            ChangedPublisher: ChangedPublisher::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectChange as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"implement\"`*"]
pub trait IEventObjectChange2_Impl: Sized {
    fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()>;
    fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IEventObjectChange2 {}
impl IEventObjectChange2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange2_Impl, const OFFSET: isize>() -> IEventObjectChange2_Vtbl {
        unsafe extern "system" fn ChangedSubscription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangedSubscription(::core::mem::transmute_copy(&pinfo)).into()
        }
        unsafe extern "system" fn ChangedEventClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectChange2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChangedEventClass(::core::mem::transmute_copy(&pinfo)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ChangedSubscription: ChangedSubscription::<Identity, Impl, OFFSET>,
            ChangedEventClass: ChangedEventClass::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectChange2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventObjectCollection_Impl: Sized + super::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn get_Item(&self, objectid: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn NewEnum(&self) -> ::windows::core::Result<IEnumEventObject>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, item: *const super::VARIANT, objectid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Remove(&self, objectid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventObjectCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventObjectCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>() -> IEventObjectCollection_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunkenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pitem: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&objectid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *const super::VARIANT, objectid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute_copy(&item), ::core::mem::transmute(&objectid)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventObjectCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&objectid)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            NewEnum: NewEnum::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventObjectCollection as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventProperty_Impl: Sized + super::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetName(&self, propertyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::VARIANT>;
    fn SetValue(&self, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventProperty {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventProperty_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>() -> IEventProperty_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&propertyname)).into()
        }
        unsafe extern "system" fn Value<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Value() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventProperty_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetValue(::core::mem::transmute_copy(&propertyvalue)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Value: Value::<Identity, Impl, OFFSET>,
            SetValue: SetValue::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventProperty as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventPublisher_Impl: Sized + super::IDispatch_Impl {
    fn PublisherID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PublisherName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPublisherName(&self, bstrpublishername: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PublisherType(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPublisherType(&self, bstrpublishertype: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetDefaultProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn PutDefaultProperty(&self, bstrpropertyname: &::windows::core::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn RemoveDefaultProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetDefaultPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventPublisher {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventPublisher_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>() -> IEventPublisher_Vtbl {
        unsafe extern "system" fn PublisherID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublisherid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPublisherID(::core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn PublisherName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PublisherName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublishername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublishername: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPublisherName(::core::mem::transmute(&bstrpublishername)).into()
        }
        unsafe extern "system" fn PublisherType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PublisherType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublishertype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublishertype: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPublisherType(::core::mem::transmute(&bstrpublishertype)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OwnerSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrownersid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOwnerSID(::core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn GetDefaultProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultProperty(::core::mem::transmute(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutDefaultProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutDefaultProperty(::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveDefaultProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveDefaultProperty(::core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetDefaultPropertyCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventPublisher_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultPropertyCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IEventPublisher as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventSubscription_Impl: Sized + super::IDispatch_Impl {
    fn SubscriptionID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSubscriptionID(&self, bstrsubscriptionid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SubscriptionName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSubscriptionName(&self, bstrsubscriptionname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn PublisherID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetPublisherID(&self, bstrpublisherid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn EventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetEventClassID(&self, bstreventclassid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MethodName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMethodName(&self, bstrmethodname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SubscriberCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetSubscriberCLSID(&self, bstrsubscriberclsid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SubscriberInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn SetSubscriberInterface(&self, psubscriberinterface: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn PerUser(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetPerUser(&self, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn OwnerSID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetOwnerSID(&self, bstrownersid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn Enabled(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetEnabled(&self, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetDescription(&self, bstrdescription: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn MachineName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetMachineName(&self, bstrmachinename: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetPublisherProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn PutPublisherProperty(&self, bstrpropertyname: &::windows::core::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn RemovePublisherProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetPublisherPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection>;
    fn GetSubscriberProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<super::VARIANT>;
    fn PutSubscriberProperty(&self, bstrpropertyname: &::windows::core::BSTR, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()>;
    fn RemoveSubscriberProperty(&self, bstrpropertyname: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn GetSubscriberPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection>;
    fn InterfaceID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetInterfaceID(&self, bstrinterfaceid: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventSubscription {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventSubscription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>() -> IEventSubscription_Vtbl {
        unsafe extern "system" fn SubscriptionID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubscriptionID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubscriptionid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriptionid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubscriptionID(::core::mem::transmute(&bstrsubscriptionid)).into()
        }
        unsafe extern "system" fn SubscriptionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubscriptionName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubscriptionname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriptionName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriptionname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubscriptionName(::core::mem::transmute(&bstrsubscriptionname)).into()
        }
        unsafe extern "system" fn PublisherID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PublisherID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpublisherid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPublisherID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpublisherid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPublisherID(::core::mem::transmute(&bstrpublisherid)).into()
        }
        unsafe extern "system" fn EventClassID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventClassID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstreventclassid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventClassID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstreventclassid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventClassID(::core::mem::transmute(&bstreventclassid)).into()
        }
        unsafe extern "system" fn MethodName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MethodName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmethodname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethodName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmethodname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMethodName(::core::mem::transmute(&bstrmethodname)).into()
        }
        unsafe extern "system" fn SubscriberCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubscriberCLSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrsubscriberclsid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberCLSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubscriberCLSID(::core::mem::transmute(&bstrsubscriberclsid)).into()
        }
        unsafe extern "system" fn SubscriberInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SubscriberInterface() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsubscriberinterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSubscriberInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSubscriberInterface(::windows::core::from_raw_borrowed(&psubscriberinterface)).into()
        }
        unsafe extern "system" fn PerUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PerUser() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfperuser, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPerUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPerUser(::core::mem::transmute_copy(&fperuser)).into()
        }
        unsafe extern "system" fn OwnerSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrownersid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OwnerSID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrownersid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOwnerSID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrownersid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOwnerSID(::core::mem::transmute(&bstrownersid)).into()
        }
        unsafe extern "system" fn Enabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEnabled(::core::mem::transmute_copy(&fenabled)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn MachineName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MachineName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrmachinename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMachineName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrmachinename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMachineName(::core::mem::transmute(&bstrmachinename)).into()
        }
        unsafe extern "system" fn GetPublisherProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPublisherProperty(::core::mem::transmute(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutPublisherProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutPublisherProperty(::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemovePublisherProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemovePublisherProperty(::core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetPublisherPropertyCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPublisherPropertyCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSubscriberProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubscriberProperty(::core::mem::transmute(&bstrpropertyname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(propertyvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutSubscriberProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutSubscriberProperty(::core::mem::transmute(&bstrpropertyname), ::core::mem::transmute_copy(&propertyvalue)).into()
        }
        unsafe extern "system" fn RemoveSubscriberProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveSubscriberProperty(::core::mem::transmute(&bstrpropertyname)).into()
        }
        unsafe extern "system" fn GetSubscriberPropertyCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, collection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubscriberPropertyCollection() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(collection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InterfaceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InterfaceID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrinterfaceid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInterfaceID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSubscription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfaceid: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInterfaceID(::core::mem::transmute(&bstrinterfaceid)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IEventSubscription as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IEventSystem_Impl: Sized + super::IDispatch_Impl {
    fn Query(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR, errorindex: *mut i32, ppinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Store(&self, progid: &::windows::core::BSTR, pinterface: ::core::option::Option<&::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn Remove(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR) -> ::windows::core::Result<i32>;
    fn EventObjectChangeEventClassID(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn QueryS(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn RemoveS(&self, progid: &::windows::core::BSTR, querycriteria: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IEventSystem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IEventSystem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>() -> IEventSystem_Vtbl {
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows::core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows::core::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Query(::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria), ::core::mem::transmute_copy(&errorindex), ::core::mem::transmute_copy(&ppinterface)).into()
        }
        unsafe extern "system" fn Store<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows::core::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Store(::core::mem::transmute(&progid), ::windows::core::from_raw_borrowed(&pinterface)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows::core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows::core::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Remove(::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EventObjectChangeEventClassID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EventObjectChangeEventClassID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstreventclassid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryS<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows::core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows::core::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryS(::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppinterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveS<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEventSystem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, progid: ::std::mem::MaybeUninit<::windows::core::BSTR>, querycriteria: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveS(::core::mem::transmute(&progid), ::core::mem::transmute(&querycriteria)).into()
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Query: Query::<Identity, Impl, OFFSET>,
            Store: Store::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            EventObjectChangeEventClassID: EventObjectChangeEventClassID::<Identity, Impl, OFFSET>,
            QueryS: QueryS::<Identity, Impl, OFFSET>,
            RemoveS: RemoveS::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEventSystem as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
pub trait IFiringControl_Impl: Sized + super::IDispatch_Impl {
    fn FireSubscription(&self, subscription: ::core::option::Option<&IEventSubscription>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IFiringControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
impl IFiringControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFiringControl_Impl, const OFFSET: isize>() -> IFiringControl_Vtbl {
        unsafe extern "system" fn FireSubscription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IFiringControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subscription: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FireSubscription(::windows::core::from_raw_borrowed(&subscription)).into()
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), FireSubscription: FireSubscription::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFiringControl as ::windows::core::ComInterface>::IID || iid == &<super::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IMultiInterfaceEventControl_Impl: Sized {
    fn SetMultiInterfacePublisherFilter(&self, classfilter: ::core::option::Option<&IMultiInterfacePublisherFilter>) -> ::windows::core::Result<()>;
    fn GetSubscriptions(&self, eventiid: *const ::windows::core::GUID, bstrmethodname: &::windows::core::BSTR, optionalcriteria: &::windows::core::BSTR, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection>;
    fn SetDefaultQuery(&self, eventiid: *const ::windows::core::GUID, bstrmethodname: &::windows::core::BSTR, bstrcriteria: &::windows::core::BSTR) -> ::windows::core::Result<i32>;
    fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetAllowInprocActivation(&self, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL>;
    fn SetFireInParallel(&self, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IMultiInterfaceEventControl {}
#[cfg(feature = "Win32_Foundation")]
impl IMultiInterfaceEventControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>() -> IMultiInterfaceEventControl_Vtbl {
        unsafe extern "system" fn SetMultiInterfacePublisherFilter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, classfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultiInterfacePublisherFilter(::windows::core::from_raw_borrowed(&classfilter)).into()
        }
        unsafe extern "system" fn GetSubscriptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalcriteria: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSubscriptions(::core::mem::transmute_copy(&eventiid), ::core::mem::transmute(&bstrmethodname), ::core::mem::transmute(&optionalcriteria), ::core::mem::transmute_copy(&optionalerrorindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDefaultQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, bstrcriteria: ::std::mem::MaybeUninit<::windows::core::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetDefaultQuery(::core::mem::transmute_copy(&eventiid), ::core::mem::transmute(&bstrmethodname), ::core::mem::transmute(&bstrcriteria)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errorindex, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AllowInprocActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AllowInprocActivation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfallowinprocactivation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAllowInprocActivation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAllowInprocActivation(::core::mem::transmute_copy(&fallowinprocactivation)).into()
        }
        unsafe extern "system" fn FireInParallel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FireInParallel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pffireinparallel, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFireInParallel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfaceEventControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFireInParallel(::core::mem::transmute_copy(&ffireinparallel)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
        iid == &<IMultiInterfaceEventControl as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"implement\"`*"]
pub trait IMultiInterfacePublisherFilter_Impl: Sized {
    fn Initialize(&self, peic: ::core::option::Option<&IMultiInterfaceEventControl>) -> ::windows::core::Result<()>;
    fn PrepareToFire(&self, iid: *const ::windows::core::GUID, methodname: &::windows::core::BSTR, firingcontrol: ::core::option::Option<&IFiringControl>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IMultiInterfacePublisherFilter {}
impl IMultiInterfacePublisherFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>() -> IMultiInterfacePublisherFilter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, peic: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows::core::from_raw_borrowed(&peic)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IMultiInterfacePublisherFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, methodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareToFire(::core::mem::transmute_copy(&iid), ::core::mem::transmute(&methodname), ::windows::core::from_raw_borrowed(&firingcontrol)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultiInterfacePublisherFilter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"implement\"`*"]
pub trait IPublisherFilter_Impl: Sized {
    fn Initialize(&self, methodname: &::windows::core::BSTR, dispuserdefined: ::core::option::Option<&super::IDispatch>) -> ::windows::core::Result<()>;
    fn PrepareToFire(&self, methodname: &::windows::core::BSTR, firingcontrol: ::core::option::Option<&IFiringControl>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IPublisherFilter {}
impl IPublisherFilter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPublisherFilter_Impl, const OFFSET: isize>() -> IPublisherFilter_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPublisherFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, dispuserdefined: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&methodname), ::windows::core::from_raw_borrowed(&dispuserdefined)).into()
        }
        unsafe extern "system" fn PrepareToFire<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IPublisherFilter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, methodname: ::std::mem::MaybeUninit<::windows::core::BSTR>, firingcontrol: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareToFire(::core::mem::transmute(&methodname), ::windows::core::from_raw_borrowed(&firingcontrol)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            PrepareToFire: PrepareToFire::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPublisherFilter as ::windows::core::ComInterface>::IID
    }
}
