pub trait IKsAggregateControl_Impl: Sized {
    fn KsAddAggregate(&mut self, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn KsRemoveAggregate(&mut self, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl IKsAggregateControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsAggregateControl_Impl, const OFFSET: isize>() -> IKsAggregateControl_Vtbl {
        unsafe extern "system" fn KsAddAggregate<Identity: ::windows::core::IUnknownImpl, Impl: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KsAddAggregate(::core::mem::transmute_copy(&aggregateclass)).into()
        }
        unsafe extern "system" fn KsRemoveAggregate<Identity: ::windows::core::IUnknownImpl, Impl: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KsRemoveAggregate(::core::mem::transmute_copy(&aggregateclass)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            KsAddAggregate: KsAddAggregate::<Identity, Impl, OFFSET>,
            KsRemoveAggregate: KsRemoveAggregate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsAggregateControl as ::windows::core::Interface>::IID
    }
}
pub trait IKsControl_Impl: Sized {
    fn KsProperty(&mut self, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn KsMethod(&mut self, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn KsEvent(&mut self, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
}
impl IKsControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsControl_Impl, const OFFSET: isize>() -> IKsControl_Vtbl {
        unsafe extern "system" fn KsProperty<Identity: ::windows::core::IUnknownImpl, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KsProperty(::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&propertylength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsMethod<Identity: ::windows::core::IUnknownImpl, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KsMethod(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&methodlength), ::core::mem::transmute_copy(&methoddata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsEvent<Identity: ::windows::core::IUnknownImpl, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).KsEvent(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&eventlength), ::core::mem::transmute_copy(&eventdata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            KsProperty: KsProperty::<Identity, Impl, OFFSET>,
            KsMethod: KsMethod::<Identity, Impl, OFFSET>,
            KsEvent: KsEvent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsControl as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKsFormatSupport_Impl: Sized {
    fn IsFormatSupported(&mut self, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDevicePreferredFormat(&mut self) -> ::windows::core::Result<*mut KSDATAFORMAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKsFormatSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsFormatSupport_Impl, const OFFSET: isize>() -> IKsFormatSupport_Vtbl {
        unsafe extern "system" fn IsFormatSupported<Identity: ::windows::core::IUnknownImpl, Impl: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).IsFormatSupported(::core::mem::transmute_copy(&pksformat), ::core::mem::transmute_copy(&cbformat), ::core::mem::transmute_copy(&pbsupported)).into()
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Identity: ::windows::core::IUnknownImpl, Impl: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppksformat: *mut *mut KSDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevicePreferredFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *ppksformat = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsFormatSupported: IsFormatSupported::<Identity, Impl, OFFSET>,
            GetDevicePreferredFormat: GetDevicePreferredFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsFormatSupport as ::windows::core::Interface>::IID
    }
}
pub trait IKsJackContainerId_Impl: Sized {
    fn GetJackContainerId(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl IKsJackContainerId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackContainerId_Impl, const OFFSET: isize>() -> IKsJackContainerId_Vtbl {
        unsafe extern "system" fn GetJackContainerId<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackContainerId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjackcontainerid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetJackContainerId() {
                ::core::result::Result::Ok(ok__) => {
                    *pjackcontainerid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetJackContainerId: GetJackContainerId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackContainerId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKsJackDescription_Impl: Sized {
    fn GetJackCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetJackDescription(&mut self, njack: u32) -> ::windows::core::Result<KSJACK_DESCRIPTION>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKsJackDescription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescription_Impl, const OFFSET: isize>() -> IKsJackDescription_Vtbl {
        unsafe extern "system" fn GetJackCount<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetJackCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcjacks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetJackDescription(::core::mem::transmute_copy(&njack)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription: GetJackDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackDescription as ::windows::core::Interface>::IID
    }
}
pub trait IKsJackDescription2_Impl: Sized {
    fn GetJackCount(&mut self) -> ::windows::core::Result<u32>;
    fn GetJackDescription2(&mut self, njack: u32) -> ::windows::core::Result<KSJACK_DESCRIPTION2>;
}
impl IKsJackDescription2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescription2_Impl, const OFFSET: isize>() -> IKsJackDescription2_Vtbl {
        unsafe extern "system" fn GetJackCount<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetJackCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcjacks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription2<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetJackDescription2(::core::mem::transmute_copy(&njack)) {
                ::core::result::Result::Ok(ok__) => {
                    *pdescription2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription2: GetJackDescription2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackDescription2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKsJackSinkInformation_Impl: Sized {
    fn GetJackSinkInformation(&mut self) -> ::windows::core::Result<KSJACK_SINK_INFORMATION>;
}
#[cfg(feature = "Win32_Foundation")]
impl IKsJackSinkInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackSinkInformation_Impl, const OFFSET: isize>() -> IKsJackSinkInformation_Vtbl {
        unsafe extern "system" fn GetJackSinkInformation<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackSinkInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetJackSinkInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *pjacksinkinformation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetJackSinkInformation: GetJackSinkInformation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackSinkInformation as ::windows::core::Interface>::IID
    }
}
pub trait IKsPropertySet_Impl: Sized {
    fn Set(&mut self, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::core::Result<()>;
    fn Get(&mut self, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn QuerySupported(&mut self, propset: *const ::windows::core::GUID, id: u32) -> ::windows::core::Result<u32>;
}
impl IKsPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsPropertySet_Impl, const OFFSET: isize>() -> IKsPropertySet_Vtbl {
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Set(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&instancedata), ::core::mem::transmute_copy(&instancelength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength)).into()
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Get(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&instancedata), ::core::mem::transmute_copy(&instancelength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn QuerySupported<Identity: ::windows::core::IUnknownImpl, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, typesupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).QuerySupported(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *typesupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Set: Set::<Identity, Impl, OFFSET>,
            Get: Get::<Identity, Impl, OFFSET>,
            QuerySupported: QuerySupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsPropertySet as ::windows::core::Interface>::IID
    }
}
pub trait IKsTopology_Impl: Sized {
    fn CreateNodeInstance(&mut self, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: &::core::option::Option<::windows::core::IUnknown>, interfaceid: *const ::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IKsTopology_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsTopology_Impl, const OFFSET: isize>() -> IKsTopology_Vtbl {
        unsafe extern "system" fn CreateNodeInstance<Identity: ::windows::core::IUnknownImpl, Impl: IKsTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CreateNodeInstance(::core::mem::transmute_copy(&nodeid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&desiredaccess), ::core::mem::transmute(&unkouter), ::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute_copy(&interface)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateNodeInstance: CreateNodeInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsTopology as ::windows::core::Interface>::IID
    }
}
