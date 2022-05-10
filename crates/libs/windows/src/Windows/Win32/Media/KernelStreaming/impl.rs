pub trait IKsAggregateControl_Impl: Sized {
    fn KsAddAggregate(&self, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn KsRemoveAggregate(&self, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IKsAggregateControl {}
impl IKsAggregateControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: isize>() -> IKsAggregateControl_Vtbl {
        unsafe extern "system" fn KsAddAggregate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsAddAggregate(::core::mem::transmute_copy(&aggregateclass)).into()
        }
        unsafe extern "system" fn KsRemoveAggregate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsAggregateControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsRemoveAggregate(::core::mem::transmute_copy(&aggregateclass)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            KsAddAggregate: KsAddAggregate::<Identity, Impl, OFFSET>,
            KsRemoveAggregate: KsRemoveAggregate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsAggregateControl as ::windows::core::Interface>::IID
    }
}
pub trait IKsControl_Impl: Sized {
    fn KsProperty(&self, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn KsMethod(&self, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn KsEvent(&self, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IKsControl {}
impl IKsControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>() -> IKsControl_Vtbl {
        unsafe extern "system" fn KsProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsProperty(::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&propertylength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsMethod<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsMethod(::core::mem::transmute_copy(&method), ::core::mem::transmute_copy(&methodlength), ::core::mem::transmute_copy(&methoddata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn KsEvent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.KsEvent(::core::mem::transmute_copy(&event), ::core::mem::transmute_copy(&eventlength), ::core::mem::transmute_copy(&eventdata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn IsFormatSupported(&self, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetDevicePreferredFormat(&self) -> ::windows::core::Result<*mut KSDATAFORMAT>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IKsFormatSupport {}
#[cfg(feature = "Win32_Foundation")]
impl IKsFormatSupport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: isize>() -> IKsFormatSupport_Vtbl {
        unsafe extern "system" fn IsFormatSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsFormatSupported(::core::mem::transmute_copy(&pksformat), ::core::mem::transmute_copy(&cbformat), ::core::mem::transmute_copy(&pbsupported)).into()
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsFormatSupport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppksformat: *mut *mut KSDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevicePreferredFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppksformat, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsFormatSupported: IsFormatSupported::<Identity, Impl, OFFSET>,
            GetDevicePreferredFormat: GetDevicePreferredFormat::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsFormatSupport as ::windows::core::Interface>::IID
    }
}
pub trait IKsJackContainerId_Impl: Sized {
    fn GetJackContainerId(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for IKsJackContainerId {}
impl IKsJackContainerId_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackContainerId_Impl, const OFFSET: isize>() -> IKsJackContainerId_Vtbl {
        unsafe extern "system" fn GetJackContainerId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackContainerId_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjackcontainerid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackContainerId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pjackcontainerid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetJackContainerId: GetJackContainerId::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackContainerId as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IKsJackDescription_Impl: Sized {
    fn GetJackCount(&self) -> ::windows::core::Result<u32>;
    fn GetJackDescription(&self, njack: u32) -> ::windows::core::Result<KSJACK_DESCRIPTION>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IKsJackDescription {}
#[cfg(feature = "Win32_Foundation")]
impl IKsJackDescription_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: isize>() -> IKsJackDescription_Vtbl {
        unsafe extern "system" fn GetJackCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcjacks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackDescription(::core::mem::transmute_copy(&njack)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetJackCount: GetJackCount::<Identity, Impl, OFFSET>,
            GetJackDescription: GetJackDescription::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackDescription as ::windows::core::Interface>::IID
    }
}
pub trait IKsJackDescription2_Impl: Sized {
    fn GetJackCount(&self) -> ::windows::core::Result<u32>;
    fn GetJackDescription2(&self, njack: u32) -> ::windows::core::Result<KSJACK_DESCRIPTION2>;
}
impl ::windows::core::RuntimeName for IKsJackDescription2 {}
impl IKsJackDescription2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: isize>() -> IKsJackDescription2_Vtbl {
        unsafe extern "system" fn GetJackCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcjacks, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackDescription2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackDescription2(::core::mem::transmute_copy(&njack)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdescription2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn GetJackSinkInformation(&self) -> ::windows::core::Result<KSJACK_SINK_INFORMATION>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IKsJackSinkInformation {}
#[cfg(feature = "Win32_Foundation")]
impl IKsJackSinkInformation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackSinkInformation_Impl, const OFFSET: isize>() -> IKsJackSinkInformation_Vtbl {
        unsafe extern "system" fn GetJackSinkInformation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsJackSinkInformation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJackSinkInformation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pjacksinkinformation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetJackSinkInformation: GetJackSinkInformation::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsJackSinkInformation as ::windows::core::Interface>::IID
    }
}
pub trait IKsPropertySet_Impl: Sized {
    fn Set(&self, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::core::Result<()>;
    fn Get(&self, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::Result<()>;
    fn QuerySupported(&self, propset: *const ::windows::core::GUID, id: u32) -> ::windows::core::Result<u32>;
}
impl ::windows::core::RuntimeName for IKsPropertySet {}
impl IKsPropertySet_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>() -> IKsPropertySet_Vtbl {
        unsafe extern "system" fn Set<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Set(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&instancedata), ::core::mem::transmute_copy(&instancelength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength)).into()
        }
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Get(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&instancedata), ::core::mem::transmute_copy(&instancelength), ::core::mem::transmute_copy(&propertydata), ::core::mem::transmute_copy(&datalength), ::core::mem::transmute_copy(&bytesreturned)).into()
        }
        unsafe extern "system" fn QuerySupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsPropertySet_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: *const ::windows::core::GUID, id: u32, typesupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QuerySupported(::core::mem::transmute_copy(&propset), ::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(typesupport, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
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
    fn CreateNodeInstance(&self, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: &::core::option::Option<::windows::core::IUnknown>, interfaceid: *const ::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IKsTopology {}
impl IKsTopology_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsTopology_Impl, const OFFSET: isize>() -> IKsTopology_Vtbl {
        unsafe extern "system" fn CreateNodeInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IKsTopology_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: *mut ::core::ffi::c_void, interfaceid: *const ::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateNodeInstance(::core::mem::transmute_copy(&nodeid), ::core::mem::transmute_copy(&flags), ::core::mem::transmute_copy(&desiredaccess), ::core::mem::transmute(&unkouter), ::core::mem::transmute_copy(&interfaceid), ::core::mem::transmute_copy(&interface)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateNodeInstance: CreateNodeInstance::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IKsTopology as ::windows::core::Interface>::IID
    }
}
