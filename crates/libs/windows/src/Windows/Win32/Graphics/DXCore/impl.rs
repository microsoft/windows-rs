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
impl ::windows::core::RuntimeName for IDXCoreAdapter {
    const NAME: &'static str = "Windows.Win32.Graphics.DXCore.IDXCoreAdapter";
}
impl IDXCoreAdapterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXCoreAdapterImpl, const OFFSET: isize>() -> IDXCoreAdapterVtbl {
        unsafe extern "system" fn IsValid<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsValid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAttributeSupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeguid: &::windows::core::GUID) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAttributeSupported(&*(&attributeguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsPropertySupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsPropertySupported(property) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(property, buffersize, ::core::mem::transmute_copy(&propertydata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPropertySize<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPropertySize(property, ::core::mem::transmute_copy(&buffersize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsQueryStateSupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsQueryStateSupported(property) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryState<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryState(state, inputstatedetailssize, &*(&inputstatedetails as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), outputbuffersize, ::core::mem::transmute_copy(&outputbuffer)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSetStateSupported<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSetStateSupported(property) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetState<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetState(state, inputstatedetailssize, &*(&inputstatedetails as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType), inputdatasize, &*(&inputdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFactory<Impl: IDXCoreAdapterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFactory(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvfactory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDXCoreAdapter>,
            ::windows::core::GetTrustLevel,
            IsValid::<Impl, OFFSET>,
            IsAttributeSupported::<Impl, OFFSET>,
            IsPropertySupported::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetPropertySize::<Impl, OFFSET>,
            IsQueryStateSupported::<Impl, OFFSET>,
            QueryState::<Impl, OFFSET>,
            IsSetStateSupported::<Impl, OFFSET>,
            SetState::<Impl, OFFSET>,
            GetFactory::<Impl, OFFSET>,
        )
    }
}
pub trait IDXCoreAdapterFactoryImpl: Sized {
    fn CreateAdapterList();
    fn GetAdapterByLuid();
    fn IsNotificationTypeSupported();
    fn RegisterEventNotification();
    fn UnregisterEventNotification();
}
impl ::windows::core::RuntimeName for IDXCoreAdapterFactory {
    const NAME: &'static str = "Windows.Win32.Graphics.DXCore.IDXCoreAdapterFactory";
}
impl IDXCoreAdapterFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>() -> IDXCoreAdapterFactoryVtbl {
        unsafe extern "system" fn CreateAdapterList<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numattributes: u32, filterattributes: &::windows::core::GUID, riid: &::windows::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAdapterList(numattributes, &*(&filterattributes as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvadapterlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterByLuid<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapterluid: *const super::super::Foundation::LUID, riid: &::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdapterByLuid(&*(&adapterluid as *const <super::super::Foundation::LUID as ::windows::core::Abi>::Abi as *const <super::super::Foundation::LUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvadapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNotificationTypeSupported<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNotificationTypeSupported(notificationtype) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RegisterEventNotification<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoreobject: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RegisterEventNotification(
                &*(&dxcoreobject as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                notificationtype,
                &*(&callbackfunction as *const <PFN_DXCORE_NOTIFICATION_CALLBACK as ::windows::core::Abi>::Abi as *const <PFN_DXCORE_NOTIFICATION_CALLBACK as ::windows::core::DefaultType>::DefaultType),
                &*(&callbackcontext as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&eventcookie),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterEventNotification<Impl: IDXCoreAdapterFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnregisterEventNotification(eventcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXCoreAdapterFactory>, ::windows::core::GetTrustLevel, CreateAdapterList::<Impl, OFFSET>, GetAdapterByLuid::<Impl, OFFSET>, IsNotificationTypeSupported::<Impl, OFFSET>, RegisterEventNotification::<Impl, OFFSET>, UnregisterEventNotification::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDXCoreAdapterList {
    const NAME: &'static str = "Windows.Win32.Graphics.DXCore.IDXCoreAdapterList";
}
impl IDXCoreAdapterListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDXCoreAdapterListImpl, const OFFSET: isize>() -> IDXCoreAdapterListVtbl {
        unsafe extern "system" fn GetAdapter<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, riid: &::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdapter(index, &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvadapter)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterCount<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdapterCount() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsStale<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsStale() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFactory<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: &::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFactory(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvfactory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sort<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sort(numpreferences, preferences) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsAdapterPreferenceSupported<Impl: IDXCoreAdapterListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preference: DXCoreAdapterPreference) -> bool {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAdapterPreferenceSupported(preference) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDXCoreAdapterList>, ::windows::core::GetTrustLevel, GetAdapter::<Impl, OFFSET>, GetAdapterCount::<Impl, OFFSET>, IsStale::<Impl, OFFSET>, GetFactory::<Impl, OFFSET>, Sort::<Impl, OFFSET>, IsAdapterPreferenceSupported::<Impl, OFFSET>)
    }
}
