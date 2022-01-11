pub trait IDXCoreAdapterImpl: Sized {
    fn IsValid();
    fn IsAttributeSupported();
    fn IsPropertySupported();
    fn GetProperty();
    fn GetPropertySize();
    fn IsQueryStateSupported();
    fn QueryState();
    fn IsSetStateSupported();
    fn SetState();
    fn GetFactory();
}
impl IDXCoreAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXCoreAdapterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXCoreAdapterVtbl {
        unsafe extern "system" fn IsValid<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAttributeSupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeguid: *const ::windows::core::GUID) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsPropertySupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPropertySize<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsQueryStateSupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryState<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSetStateSupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetState<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFactory<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            IsValid::<Impl, IMPL_OFFSET>,
            IsAttributeSupported::<Impl, IMPL_OFFSET>,
            IsPropertySupported::<Impl, IMPL_OFFSET>,
            GetProperty::<Impl, IMPL_OFFSET>,
            GetPropertySize::<Impl, IMPL_OFFSET>,
            IsQueryStateSupported::<Impl, IMPL_OFFSET>,
            QueryState::<Impl, IMPL_OFFSET>,
            IsSetStateSupported::<Impl, IMPL_OFFSET>,
            SetState::<Impl, IMPL_OFFSET>,
            GetFactory::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXCoreAdapter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDXCoreAdapterFactoryImpl: Sized {
    fn CreateAdapterList();
    fn GetAdapterByLuid();
    fn IsNotificationTypeSupported();
    fn RegisterEventNotification();
    fn UnregisterEventNotification();
}
#[cfg(feature = "Win32_Foundation")]
impl IDXCoreAdapterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXCoreAdapterFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXCoreAdapterFactoryVtbl {
        unsafe extern "system" fn CreateAdapterList<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numattributes: u32, filterattributes: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterByLuid<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsNotificationTypeSupported<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RegisterEventNotification<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoreobject: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterEventNotification<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateAdapterList::<Impl, IMPL_OFFSET>, GetAdapterByLuid::<Impl, IMPL_OFFSET>, IsNotificationTypeSupported::<Impl, IMPL_OFFSET>, RegisterEventNotification::<Impl, IMPL_OFFSET>, UnregisterEventNotification::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXCoreAdapterFactory as ::windows::core::Interface>::IID
    }
}
pub trait IDXCoreAdapterListImpl: Sized {
    fn GetAdapter();
    fn GetAdapterCount();
    fn IsStale();
    fn GetFactory();
    fn Sort();
    fn IsAdapterPreferenceSupported();
}
impl IDXCoreAdapterListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXCoreAdapterListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDXCoreAdapterListVtbl {
        unsafe extern "system" fn GetAdapter<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterCount<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsStale<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFactory<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Sort<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsAdapterPreferenceSupported<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preference: DXCoreAdapterPreference) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetAdapter::<Impl, IMPL_OFFSET>, GetAdapterCount::<Impl, IMPL_OFFSET>, IsStale::<Impl, IMPL_OFFSET>, GetFactory::<Impl, IMPL_OFFSET>, Sort::<Impl, IMPL_OFFSET>, IsAdapterPreferenceSupported::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXCoreAdapterList as ::windows::core::Interface>::IID
    }
}
