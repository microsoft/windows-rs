pub trait ILocationPermissionsImpl: Sized {
    fn GetGlobalLocationPermission();
    fn CheckLocationCapability();
}
impl ::windows::core::RuntimeName for ILocationPermissions {
    const NAME: &'static str = "Windows.Win32.Devices.Sensors.ILocationPermissions";
}
impl ILocationPermissionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationPermissionsImpl, const OFFSET: isize>() -> ILocationPermissionsVtbl {
        unsafe extern "system" fn GetGlobalLocationPermission<Impl: ILocationPermissionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalLocationPermission(::core::mem::transmute_copy(&pfenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckLocationCapability<Impl: ILocationPermissionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckLocationCapability(dwclientthreadid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ILocationPermissions>, ::windows::core::GetTrustLevel, GetGlobalLocationPermission::<Impl, OFFSET>, CheckLocationCapability::<Impl, OFFSET>)
    }
}
pub trait ISensorImpl: Sized {
    fn GetID();
    fn GetCategory();
    fn GetType();
    fn GetFriendlyName();
    fn GetProperty();
    fn GetProperties();
    fn GetSupportedDataFields();
    fn SetProperties();
    fn SupportsDataField();
    fn GetState();
    fn GetData();
    fn SupportsEvent();
    fn GetEventInterest();
    fn SetEventInterest();
    fn SetEventSink();
}
impl ::windows::core::RuntimeName for ISensor {
    const NAME: &'static str = "Windows.Win32.Devices.Sensors.ISensor";
}
impl ISensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorImpl, const OFFSET: isize>() -> ISensorVtbl {
        unsafe extern "system" fn GetID<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetID(::core::mem::transmute_copy(&pid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensorcategory: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategory(::core::mem::transmute_copy(&psensorcategory)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensortype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType(::core::mem::transmute_copy(&psensortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFriendlyName(::core::mem::transmute_copy(&pfriendlyname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pproperty: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproperty)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeys: ::windows::core::RawPtr, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(&*(&pkeys as *const <super::PortableDevices::IPortableDeviceKeyCollection as ::windows::core::Abi>::Abi as *const <super::PortableDevices::IPortableDeviceKeyCollection as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedDataFields<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdatafields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedDataFields(::core::mem::transmute_copy(&ppdatafields)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperties<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperties(&*(&pproperties as *const <super::PortableDevices::IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <super::PortableDevices::IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppresults)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsDataField<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pissupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsDataField(&*(&key as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pissupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut SensorState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdatareport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData(::core::mem::transmute_copy(&ppdatareport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsEvent<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventguid: &::windows::core::GUID, pissupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsEvent(&*(&eventguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pissupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventInterest<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::windows::core::GUID, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetEventInterest(::core::mem::transmute_copy(&ppvalues), ::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventInterest<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: &::windows::core::GUID, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventInterest(&*(&pvalues as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventSink<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventSink(&*(&pevents as *const <ISensorEvents as ::windows::core::Abi>::Abi as *const <ISensorEvents as ::windows::core::DefaultType>::DefaultType)) {
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
            ::windows::core::GetRuntimeClassName::<ISensor>,
            ::windows::core::GetTrustLevel,
            GetID::<Impl, OFFSET>,
            GetCategory::<Impl, OFFSET>,
            GetType::<Impl, OFFSET>,
            GetFriendlyName::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetProperties::<Impl, OFFSET>,
            GetSupportedDataFields::<Impl, OFFSET>,
            SetProperties::<Impl, OFFSET>,
            SupportsDataField::<Impl, OFFSET>,
            GetState::<Impl, OFFSET>,
            GetData::<Impl, OFFSET>,
            SupportsEvent::<Impl, OFFSET>,
            GetEventInterest::<Impl, OFFSET>,
            SetEventInterest::<Impl, OFFSET>,
            SetEventSink::<Impl, OFFSET>,
        )
    }
}
pub trait ISensorCollectionImpl: Sized {
    fn GetAt();
    fn GetCount();
    fn Add();
    fn Remove();
    fn RemoveByID();
    fn Clear();
}
impl ::windows::core::RuntimeName for ISensorCollection {
    const NAME: &'static str = "Windows.Win32.Devices.Sensors.ISensorCollection";
}
impl ISensorCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorCollectionImpl, const OFFSET: isize>() -> ISensorCollectionVtbl {
        unsafe extern "system" fn GetAt<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppsensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(ulindex, ::core::mem::transmute_copy(&ppsensor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount(::core::mem::transmute_copy(&pcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&psensor as *const <ISensor as ::windows::core::Abi>::Abi as *const <ISensor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&psensor as *const <ISensor as ::windows::core::Abi>::Abi as *const <ISensor as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveByID<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveByID(&*(&sensorid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clear<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISensorCollection>, ::windows::core::GetTrustLevel, GetAt::<Impl, OFFSET>, GetCount::<Impl, OFFSET>, Add::<Impl, OFFSET>, Remove::<Impl, OFFSET>, RemoveByID::<Impl, OFFSET>, Clear::<Impl, OFFSET>)
    }
}
pub trait ISensorDataReportImpl: Sized {
    fn GetTimestamp();
    fn GetSensorValue();
    fn GetSensorValues();
}
impl ::windows::core::RuntimeName for ISensorDataReport {
    const NAME: &'static str = "Windows.Win32.Devices.Sensors.ISensorDataReport";
}
impl ISensorDataReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorDataReportImpl, const OFFSET: isize>() -> ISensorDataReportVtbl {
        unsafe extern "system" fn GetTimestamp<Impl: ISensorDataReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimestamp(::core::mem::transmute_copy(&ptimestamp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorValue<Impl: ISensorDataReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorValue(&*(&pkey as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::Abi>::Abi as *const <super::super::UI::Shell::PropertiesSystem::PROPERTYKEY as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorValues<Impl: ISensorDataReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeys: ::windows::core::RawPtr, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorValues(&*(&pkeys as *const <super::PortableDevices::IPortableDeviceKeyCollection as ::windows::core::Abi>::Abi as *const <super::PortableDevices::IPortableDeviceKeyCollection as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvalues)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISensorDataReport>, ::windows::core::GetTrustLevel, GetTimestamp::<Impl, OFFSET>, GetSensorValue::<Impl, OFFSET>, GetSensorValues::<Impl, OFFSET>)
    }
}
pub trait ISensorEventsImpl: Sized {
    fn OnStateChanged();
    fn OnDataUpdated();
    fn OnEvent();
    fn OnLeave();
}
impl ::windows::core::RuntimeName for ISensorEvents {
    const NAME: &'static str = "Windows.Win32.Devices.Sensors.ISensorEvents";
}
impl ISensorEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorEventsImpl, const OFFSET: isize>() -> ISensorEventsVtbl {
        unsafe extern "system" fn OnStateChanged<Impl: ISensorEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, state: SensorState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnStateChanged(&*(&psensor as *const <ISensor as ::windows::core::Abi>::Abi as *const <ISensor as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataUpdated<Impl: ISensorEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, pnewdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnDataUpdated(&*(&psensor as *const <ISensor as ::windows::core::Abi>::Abi as *const <ISensor as ::windows::core::DefaultType>::DefaultType), &*(&pnewdata as *const <ISensorDataReport as ::windows::core::Abi>::Abi as *const <ISensorDataReport as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnEvent<Impl: ISensorEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, eventid: &::windows::core::GUID, peventdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnEvent(
                &*(&psensor as *const <ISensor as ::windows::core::Abi>::Abi as *const <ISensor as ::windows::core::DefaultType>::DefaultType),
                &*(&eventid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                &*(&peventdata as *const <super::PortableDevices::IPortableDeviceValues as ::windows::core::Abi>::Abi as *const <super::PortableDevices::IPortableDeviceValues as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnLeave<Impl: ISensorEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: &::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnLeave(&*(&id as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISensorEvents>, ::windows::core::GetTrustLevel, OnStateChanged::<Impl, OFFSET>, OnDataUpdated::<Impl, OFFSET>, OnEvent::<Impl, OFFSET>, OnLeave::<Impl, OFFSET>)
    }
}
pub trait ISensorManagerImpl: Sized {
    fn GetSensorsByCategory();
    fn GetSensorsByType();
    fn GetSensorByID();
    fn SetEventSink();
    fn RequestPermissions();
}
impl ::windows::core::RuntimeName for ISensorManager {
    const NAME: &'static str = "Windows.Win32.Devices.Sensors.ISensorManager";
}
impl ISensorManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorManagerImpl, const OFFSET: isize>() -> ISensorManagerVtbl {
        unsafe extern "system" fn GetSensorsByCategory<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorcategory: &::windows::core::GUID, ppsensorsfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorsByCategory(&*(&sensorcategory as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsensorsfound)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorsByType<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensortype: &::windows::core::GUID, ppsensorsfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorsByType(&*(&sensortype as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsensorsfound)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorByID<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: &::windows::core::GUID, ppsensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorByID(&*(&sensorid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppsensor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventSink<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEventSink(&*(&pevents as *const <ISensorManagerEvents as ::windows::core::Abi>::Abi as *const <ISensorManagerEvents as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestPermissions<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, psensors: ::windows::core::RawPtr, fmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestPermissions(
                &*(&hparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                &*(&psensors as *const <ISensorCollection as ::windows::core::Abi>::Abi as *const <ISensorCollection as ::windows::core::DefaultType>::DefaultType),
                &*(&fmodal as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISensorManager>, ::windows::core::GetTrustLevel, GetSensorsByCategory::<Impl, OFFSET>, GetSensorsByType::<Impl, OFFSET>, GetSensorByID::<Impl, OFFSET>, SetEventSink::<Impl, OFFSET>, RequestPermissions::<Impl, OFFSET>)
    }
}
pub trait ISensorManagerEventsImpl: Sized {
    fn OnSensorEnter();
}
impl ::windows::core::RuntimeName for ISensorManagerEvents {
    const NAME: &'static str = "Windows.Win32.Devices.Sensors.ISensorManagerEvents";
}
impl ISensorManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorManagerEventsImpl, const OFFSET: isize>() -> ISensorManagerEventsVtbl {
        unsafe extern "system" fn OnSensorEnter<Impl: ISensorManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, state: SensorState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnSensorEnter(&*(&psensor as *const <ISensor as ::windows::core::Abi>::Abi as *const <ISensor as ::windows::core::DefaultType>::DefaultType), state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<ISensorManagerEvents>, ::windows::core::GetTrustLevel, OnSensorEnter::<Impl, OFFSET>)
    }
}
