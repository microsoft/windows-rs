pub trait IKsAggregateControlImpl: Sized {
    fn KsAddAggregate();
    fn KsRemoveAggregate();
}
impl ::windows::core::RuntimeName for IKsAggregateControl {
    const NAME: &'static str = "Windows.Win32.Media.KernelStreaming.IKsAggregateControl";
}
impl IKsAggregateControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsAggregateControlImpl, const OFFSET: isize>() -> IKsAggregateControlVtbl {
        unsafe extern "system" fn KsAddAggregate<Impl: IKsAggregateControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KsAddAggregate(&*(&aggregateclass as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsRemoveAggregate<Impl: IKsAggregateControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, aggregateclass: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KsRemoveAggregate(&*(&aggregateclass as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKsAggregateControl>, ::windows::core::GetTrustLevel, KsAddAggregate::<Impl, OFFSET>, KsRemoveAggregate::<Impl, OFFSET>)
    }
}
pub trait IKsControlImpl: Sized {
    fn KsProperty();
    fn KsMethod();
    fn KsEvent();
}
impl ::windows::core::RuntimeName for IKsControl {
    const NAME: &'static str = "Windows.Win32.Media.KernelStreaming.IKsControl";
}
impl IKsControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsControlImpl, const OFFSET: isize>() -> IKsControlVtbl {
        unsafe extern "system" fn KsProperty<Impl: IKsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: *const KSIDENTIFIER, propertylength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KsProperty(&*(&property as *const <KSIDENTIFIER as ::windows::core::Abi>::Abi as *const <KSIDENTIFIER as ::windows::core::DefaultType>::DefaultType), propertylength, &*(&propertydata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), datalength, ::core::mem::transmute_copy(&bytesreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsMethod<Impl: IKsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, method: *const KSIDENTIFIER, methodlength: u32, methoddata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KsMethod(&*(&method as *const <KSIDENTIFIER as ::windows::core::Abi>::Abi as *const <KSIDENTIFIER as ::windows::core::DefaultType>::DefaultType), methodlength, &*(&methoddata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), datalength, ::core::mem::transmute_copy(&bytesreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KsEvent<Impl: IKsControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, event: *const KSIDENTIFIER, eventlength: u32, eventdata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KsEvent(&*(&event as *const <KSIDENTIFIER as ::windows::core::Abi>::Abi as *const <KSIDENTIFIER as ::windows::core::DefaultType>::DefaultType), eventlength, &*(&eventdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), datalength, ::core::mem::transmute_copy(&bytesreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKsControl>, ::windows::core::GetTrustLevel, KsProperty::<Impl, OFFSET>, KsMethod::<Impl, OFFSET>, KsEvent::<Impl, OFFSET>)
    }
}
pub trait IKsFormatSupportImpl: Sized {
    fn IsFormatSupported();
    fn GetDevicePreferredFormat();
}
impl ::windows::core::RuntimeName for IKsFormatSupport {
    const NAME: &'static str = "Windows.Win32.Media.KernelStreaming.IKsFormatSupport";
}
impl IKsFormatSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsFormatSupportImpl, const OFFSET: isize>() -> IKsFormatSupportVtbl {
        unsafe extern "system" fn IsFormatSupported<Impl: IKsFormatSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pksformat: *mut KSDATAFORMAT, cbformat: u32, pbsupported: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFormatSupported(&*(&pksformat as *const <KSDATAFORMAT as ::windows::core::Abi>::Abi as *const <KSDATAFORMAT as ::windows::core::DefaultType>::DefaultType), cbformat, ::core::mem::transmute_copy(&pbsupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePreferredFormat<Impl: IKsFormatSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppksformat: *mut *mut KSDATAFORMAT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevicePreferredFormat(::core::mem::transmute_copy(&ppksformat)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKsFormatSupport>, ::windows::core::GetTrustLevel, IsFormatSupported::<Impl, OFFSET>, GetDevicePreferredFormat::<Impl, OFFSET>)
    }
}
pub trait IKsJackContainerIdImpl: Sized {
    fn GetJackContainerId();
}
impl ::windows::core::RuntimeName for IKsJackContainerId {
    const NAME: &'static str = "Windows.Win32.Media.KernelStreaming.IKsJackContainerId";
}
impl IKsJackContainerIdVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackContainerIdImpl, const OFFSET: isize>() -> IKsJackContainerIdVtbl {
        unsafe extern "system" fn GetJackContainerId<Impl: IKsJackContainerIdImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjackcontainerid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackContainerId(::core::mem::transmute_copy(&pjackcontainerid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKsJackContainerId>, ::windows::core::GetTrustLevel, GetJackContainerId::<Impl, OFFSET>)
    }
}
pub trait IKsJackDescriptionImpl: Sized {
    fn GetJackCount();
    fn GetJackDescription();
}
impl ::windows::core::RuntimeName for IKsJackDescription {
    const NAME: &'static str = "Windows.Win32.Media.KernelStreaming.IKsJackDescription";
}
impl IKsJackDescriptionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescriptionImpl, const OFFSET: isize>() -> IKsJackDescriptionVtbl {
        unsafe extern "system" fn GetJackCount<Impl: IKsJackDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackCount(::core::mem::transmute_copy(&pcjacks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription<Impl: IKsJackDescriptionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription: *mut KSJACK_DESCRIPTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackDescription(njack, ::core::mem::transmute_copy(&pdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKsJackDescription>, ::windows::core::GetTrustLevel, GetJackCount::<Impl, OFFSET>, GetJackDescription::<Impl, OFFSET>)
    }
}
pub trait IKsJackDescription2Impl: Sized {
    fn GetJackCount();
    fn GetJackDescription2();
}
impl ::windows::core::RuntimeName for IKsJackDescription2 {
    const NAME: &'static str = "Windows.Win32.Media.KernelStreaming.IKsJackDescription2";
}
impl IKsJackDescription2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackDescription2Impl, const OFFSET: isize>() -> IKsJackDescription2Vtbl {
        unsafe extern "system" fn GetJackCount<Impl: IKsJackDescription2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcjacks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackCount(::core::mem::transmute_copy(&pcjacks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJackDescription2<Impl: IKsJackDescription2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, njack: u32, pdescription2: *mut KSJACK_DESCRIPTION2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackDescription2(njack, ::core::mem::transmute_copy(&pdescription2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKsJackDescription2>, ::windows::core::GetTrustLevel, GetJackCount::<Impl, OFFSET>, GetJackDescription2::<Impl, OFFSET>)
    }
}
pub trait IKsJackSinkInformationImpl: Sized {
    fn GetJackSinkInformation();
}
impl ::windows::core::RuntimeName for IKsJackSinkInformation {
    const NAME: &'static str = "Windows.Win32.Media.KernelStreaming.IKsJackSinkInformation";
}
impl IKsJackSinkInformationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsJackSinkInformationImpl, const OFFSET: isize>() -> IKsJackSinkInformationVtbl {
        unsafe extern "system" fn GetJackSinkInformation<Impl: IKsJackSinkInformationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pjacksinkinformation: *mut KSJACK_SINK_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJackSinkInformation(::core::mem::transmute_copy(&pjacksinkinformation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKsJackSinkInformation>, ::windows::core::GetTrustLevel, GetJackSinkInformation::<Impl, OFFSET>)
    }
}
pub trait IKsPropertySetImpl: Sized {
    fn Set();
    fn Get();
    fn QuerySupported();
}
impl ::windows::core::RuntimeName for IKsPropertySet {
    const NAME: &'static str = "Windows.Win32.Media.KernelStreaming.IKsPropertySet";
}
impl IKsPropertySetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsPropertySetImpl, const OFFSET: isize>() -> IKsPropertySetVtbl {
        unsafe extern "system" fn Set<Impl: IKsPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: &::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *const ::core::ffi::c_void, datalength: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Set(
                &*(&propset as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                id,
                &*(&instancedata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                instancelength,
                &*(&propertydata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                datalength,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Get<Impl: IKsPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: &::windows::core::GUID, id: u32, instancedata: *const ::core::ffi::c_void, instancelength: u32, propertydata: *mut ::core::ffi::c_void, datalength: u32, bytesreturned: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(&*(&propset as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), id, &*(&instancedata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), instancelength, ::core::mem::transmute_copy(&propertydata), datalength, ::core::mem::transmute_copy(&bytesreturned)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QuerySupported<Impl: IKsPropertySetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propset: &::windows::core::GUID, id: u32, typesupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QuerySupported(&*(&propset as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), id, ::core::mem::transmute_copy(&typesupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKsPropertySet>, ::windows::core::GetTrustLevel, Set::<Impl, OFFSET>, Get::<Impl, OFFSET>, QuerySupported::<Impl, OFFSET>)
    }
}
pub trait IKsTopologyImpl: Sized {
    fn CreateNodeInstance();
}
impl ::windows::core::RuntimeName for IKsTopology {
    const NAME: &'static str = "Windows.Win32.Media.KernelStreaming.IKsTopology";
}
impl IKsTopologyVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IKsTopologyImpl, const OFFSET: isize>() -> IKsTopologyVtbl {
        unsafe extern "system" fn CreateNodeInstance<Impl: IKsTopologyImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nodeid: u32, flags: u32, desiredaccess: u32, unkouter: *mut ::core::ffi::c_void, interfaceid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateNodeInstance(nodeid, flags, desiredaccess, &*(&unkouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), &*(&interfaceid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&interface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IKsTopology>, ::windows::core::GetTrustLevel, CreateNodeInstance::<Impl, OFFSET>)
    }
}
