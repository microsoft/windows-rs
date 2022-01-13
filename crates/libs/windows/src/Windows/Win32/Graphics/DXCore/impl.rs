pub trait IDXCoreAdapterImpl: Sized {
    fn IsValid(&mut self) -> bool;
    fn IsAttributeSupported(&mut self, attributeguid: *const ::windows::core::GUID) -> bool;
    fn IsPropertySupported(&mut self, property: DXCoreAdapterProperty) -> bool;
    fn GetProperty(&mut self, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetPropertySize(&mut self, property: DXCoreAdapterProperty) -> ::windows::core::Result<usize>;
    fn IsQueryStateSupported(&mut self, property: DXCoreAdapterState) -> bool;
    fn QueryState(&mut self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsSetStateSupported(&mut self, property: DXCoreAdapterState) -> bool;
    fn SetState(&mut self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetFactory(&mut self, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl IDXCoreAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXCoreAdapterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXCoreAdapterVtbl {
        unsafe extern "system" fn IsValid<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsValid()
        }
        unsafe extern "system" fn IsAttributeSupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeguid: *const ::windows::core::GUID) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsAttributeSupported(::core::mem::transmute_copy(&attributeguid))
        }
        unsafe extern "system" fn IsPropertySupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsPropertySupported(::core::mem::transmute_copy(&property))
        }
        unsafe extern "system" fn GetProperty<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetProperty(::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&propertydata)).into()
        }
        unsafe extern "system" fn GetPropertySize<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertySize(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    *buffersize = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsQueryStateSupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsQueryStateSupported(::core::mem::transmute_copy(&property))
        }
        unsafe extern "system" fn QueryState<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryState(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&inputstatedetailssize), ::core::mem::transmute_copy(&inputstatedetails), ::core::mem::transmute_copy(&outputbuffersize), ::core::mem::transmute_copy(&outputbuffer)).into()
        }
        unsafe extern "system" fn IsSetStateSupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSetStateSupported(::core::mem::transmute_copy(&property))
        }
        unsafe extern "system" fn SetState<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetState(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&inputstatedetailssize), ::core::mem::transmute_copy(&inputstatedetails), ::core::mem::transmute_copy(&inputdatasize), ::core::mem::transmute_copy(&inputdata)).into()
        }
        unsafe extern "system" fn GetFactory<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFactory(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvfactory)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsValid: IsValid::<Impl, IMPL_OFFSET>,
            IsAttributeSupported: IsAttributeSupported::<Impl, IMPL_OFFSET>,
            IsPropertySupported: IsPropertySupported::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetPropertySize: GetPropertySize::<Impl, IMPL_OFFSET>,
            IsQueryStateSupported: IsQueryStateSupported::<Impl, IMPL_OFFSET>,
            QueryState: QueryState::<Impl, IMPL_OFFSET>,
            IsSetStateSupported: IsSetStateSupported::<Impl, IMPL_OFFSET>,
            SetState: SetState::<Impl, IMPL_OFFSET>,
            GetFactory: GetFactory::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXCoreAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXCoreAdapterFactoryImpl: Sized {
    fn CreateAdapterList(&mut self, numattributes: u32, filterattributes: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetAdapterByLuid(&mut self, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsNotificationTypeSupported(&mut self, notificationtype: DXCoreNotificationType) -> bool;
    fn RegisterEventNotification(&mut self, dxcoreobject: ::core::option::Option<::windows::core::IUnknown>, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u32>;
    fn UnregisterEventNotification(&mut self, eventcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDXCoreAdapterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXCoreAdapterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXCoreAdapterFactoryVtbl {
        unsafe extern "system" fn CreateAdapterList<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numattributes: u32, filterattributes: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateAdapterList(::core::mem::transmute_copy(&numattributes), ::core::mem::transmute_copy(&filterattributes), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapterlist)).into()
        }
        unsafe extern "system" fn GetAdapterByLuid<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAdapterByLuid(::core::mem::transmute_copy(&adapterluid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn IsNotificationTypeSupported<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsNotificationTypeSupported(::core::mem::transmute_copy(&notificationtype))
        }
        unsafe extern "system" fn RegisterEventNotification<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoreobject: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterEventNotification(::core::mem::transmute(&dxcoreobject), ::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute_copy(&callbackfunction), ::core::mem::transmute_copy(&callbackcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *eventcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterEventNotification<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnregisterEventNotification(::core::mem::transmute_copy(&eventcookie)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            CreateAdapterList: CreateAdapterList::<Impl, IMPL_OFFSET>,
            GetAdapterByLuid: GetAdapterByLuid::<Impl, IMPL_OFFSET>,
            IsNotificationTypeSupported: IsNotificationTypeSupported::<Impl, IMPL_OFFSET>,
            RegisterEventNotification: RegisterEventNotification::<Impl, IMPL_OFFSET>,
            UnregisterEventNotification: UnregisterEventNotification::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXCoreAdapterFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDXCoreAdapterListImpl: Sized {
    fn GetAdapter(&mut self, index: u32, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetAdapterCount(&mut self) -> u32;
    fn IsStale(&mut self) -> bool;
    fn GetFactory(&mut self, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Sort(&mut self, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::Result<()>;
    fn IsAdapterPreferenceSupported(&mut self, preference: DXCoreAdapterPreference) -> bool;
}
impl IDXCoreAdapterListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXCoreAdapterListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXCoreAdapterListVtbl {
        unsafe extern "system" fn GetAdapter<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAdapter(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn GetAdapterCount<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAdapterCount()
        }
        unsafe extern "system" fn IsStale<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsStale()
        }
        unsafe extern "system" fn GetFactory<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFactory(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvfactory)).into()
        }
        unsafe extern "system" fn Sort<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Sort(::core::mem::transmute_copy(&numpreferences), ::core::mem::transmute_copy(&preferences)).into()
        }
        unsafe extern "system" fn IsAdapterPreferenceSupported<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preference: DXCoreAdapterPreference) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsAdapterPreferenceSupported(::core::mem::transmute_copy(&preference))
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAdapter: GetAdapter::<Impl, IMPL_OFFSET>,
            GetAdapterCount: GetAdapterCount::<Impl, IMPL_OFFSET>,
            IsStale: IsStale::<Impl, IMPL_OFFSET>,
            GetFactory: GetFactory::<Impl, IMPL_OFFSET>,
            Sort: Sort::<Impl, IMPL_OFFSET>,
            IsAdapterPreferenceSupported: IsAdapterPreferenceSupported::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXCoreAdapterList as ::windows::core::Interface>::IID
    }
}
