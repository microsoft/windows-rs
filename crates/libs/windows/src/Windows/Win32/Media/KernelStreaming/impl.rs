pub trait IKsAggregateControlImpl: Sized {
    fn KsAddAggregate(&mut self, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn KsRemoveAggregate(&mut self, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IKsAggregateControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsAggregateControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKsAggregateControlVtbl {
        unsafe extern "system" fn KsAddAggregate<Impl: IKsAggregateControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KsAddAggregate(::core::mem::transmute_copy(&aggregateclass)).into()
        }
        unsafe extern "system" fn KsRemoveAggregate<Impl: IKsAggregateControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KsRemoveAggregate(::core::mem::transmute_copy(&aggregateclass)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            KsAddAggregate: KsAddAggregate::<Impl, IMPL_OFFSET>,
            KsRemoveAggregate: KsRemoveAggregate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsAggregateControl as ::windows::core::Interface>::IID
    }
}
pub trait IKsControlImpl: Sized {
    fn KsProperty(&mut self, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn KsMethod(&mut self, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn KsEvent(&mut self, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
}
impl IKsControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKsControlVtbl {
        unsafe extern "system" fn KsProperty<Impl: IKsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KsProperty(::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&propertylength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsMethod<Impl: IKsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KsMethod(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&methodlength), ::core::mem::transmute_copy(&methoddata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsEvent<Impl: IKsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).KsEvent(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&eventlength), ::core::mem::transmute_copy(&eventdata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            KsProperty: KsProperty::<Impl, IMPL_OFFSET>,
            KsMethod: KsMethod::<Impl, IMPL_OFFSET>,
            KsEvent: KsEvent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKsFormatSupportImpl: Sized {
    fn IsFormatSupported(&mut self, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDevicePreferredFormat(&mut self) -> ::windows::core::Result<*mut KSDATAFORMAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKsFormatSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsFormatSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKsFormatSupportVtbl {
        unsafe extern "system" fn IsFormatSupported<Impl: IKsFormatSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsFormatSupported(::core::mem::transmute_copy(&pksformat), ::core::mem::transmute_copy(&cbformat), ::core::mem::transmute_copy(&pbsupported)).into()
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Impl: IKsFormatSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppksformat: *mut *mut KSDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevicePreferredFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppksformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsFormatSupported: IsFormatSupported::<Impl, IMPL_OFFSET>,
            GetDevicePreferredFormat: GetDevicePreferredFormat::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsFormatSupport as ::windows::core::Interface>::IID
    }
}
pub trait IKsJackContainerIdImpl: Sized {
    fn GetJackContainerId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl IKsJackContainerIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackContainerIdImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKsJackContainerIdVtbl {
        unsafe extern "system" fn GetJackContainerId<Impl: IKsJackContainerIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjackcontainerid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackContainerId() {
                ::core::result::Result::Ok(ok__) => {
                    *pjackcontainerid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetJackContainerId: GetJackContainerId::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackContainerId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKsJackDescriptionImpl: Sized {
    fn GetJackCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetJackDescription(&mut self, njack: u32) -> ::windows::core::Result<KSJACK_DESCRIPTION>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKsJackDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescriptionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKsJackDescriptionVtbl {
        unsafe extern "system" fn GetJackCount<Impl: IKsJackDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcjacks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription<Impl: IKsJackDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackDescription(::core::mem::transmute_copy(&njack)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetJackCount: GetJackCount::<Impl, IMPL_OFFSET>,
            GetJackDescription: GetJackDescription::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackDescription as ::windows::core::Interface>::IID
    }
}
pub trait IKsJackDescription2Impl: Sized {
    fn GetJackCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetJackDescription2(&mut self, njack: u32) -> ::windows::core::Result<KSJACK_DESCRIPTION2>;
}
impl IKsJackDescription2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescription2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKsJackDescription2Vtbl {
        unsafe extern "system" fn GetJackCount<Impl: IKsJackDescription2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcjacks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription2<Impl: IKsJackDescription2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackDescription2(::core::mem::transmute_copy(&njack)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdescription2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetJackCount: GetJackCount::<Impl, IMPL_OFFSET>,
            GetJackDescription2: GetJackDescription2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackDescription2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKsJackSinkInformationImpl: Sized {
    fn GetJackSinkInformation(&mut self) -> ::windows::core::Result<KSJACK_SINK_INFORMATION>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKsJackSinkInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackSinkInformationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKsJackSinkInformationVtbl {
        unsafe extern "system" fn GetJackSinkInformation<Impl: IKsJackSinkInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackSinkInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *pjacksinkinformation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetJackSinkInformation: GetJackSinkInformation::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackSinkInformation as ::windows::core::Interface>::IID
    }
}
pub trait IKsPropertySetImpl: Sized {
    fn Set(&mut self, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::core::Result<()>;
    fn Get(&mut self, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn QuerySupported(&mut self, propset: *const ::windows::core::GUID, id: u32) -> ::windows::core::Result<u32>;
}
impl IKsPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsPropertySetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKsPropertySetVtbl {
        unsafe extern "system" fn Set<Impl: IKsPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Set(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&instancedata), ::core::mem::transmute_copy(&instancelength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength)).into()
        }
        unsafe extern "system" fn Get<Impl: IKsPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&instancedata), ::core::mem::transmute_copy(&instancelength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn QuerySupported<Impl: IKsPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, typesupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySupported(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *typesupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Set: Set::<Impl, IMPL_OFFSET>,
            Get: Get::<Impl, IMPL_OFFSET>,
            QuerySupported: QuerySupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsPropertySet as ::windows::core::Interface>::IID
    }
}
pub trait IKsTopologyImpl: Sized {
    fn CreateNodeInstance(&mut self, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: ::core::option::Option<::windows::core::IUnknown>, interfaceid: *const ::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IKsTopologyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsTopologyImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IKsTopologyVtbl {
        unsafe extern "system" fn CreateNodeInstance<Impl: IKsTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateNodeInstance(::core::mem::transmute_copy(&nodeid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&desiredaccess), ::core::mem::transmute(&unkouter), ::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute_copy(&interface)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateNodeInstance: CreateNodeInstance::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsTopology as ::windows::core::Interface>::IID
    }
}
