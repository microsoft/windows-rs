#[cfg(feature = "Win32_Foundation")]
pub trait ILocationPermissionsImpl: Sized {
    fn GetGlobalLocationPermission();
    fn CheckLocationCapability();
}
#[cfg(feature = "Win32_Foundation")]
impl ILocationPermissionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationPermissionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocationPermissionsVtbl {
        unsafe extern "system" fn GetGlobalLocationPermission<Impl: ILocationPermissionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CheckLocationCapability<Impl: ILocationPermissionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetGlobalLocationPermission: GetGlobalLocationPermission::<Impl, IMPL_OFFSET>,
            CheckLocationCapability: CheckLocationCapability::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ILocationPermissions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
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
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISensorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorVtbl {
        unsafe extern "system" fn GetID<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCategory<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensorcategory: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetType<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensortype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFriendlyName<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperty<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pproperty: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperties<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeys: ::windows::core::RawPtr, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedDataFields<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdatafields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetProperties<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportsDataField<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pissupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetState<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut SensorState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetData<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdatareport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportsEvent<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventguid: *const ::windows::core::GUID, pissupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetEventInterest<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::windows::core::GUID, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventInterest<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: *const ::windows::core::GUID, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventSink<Impl: ISensorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetID: GetID::<Impl, IMPL_OFFSET>,
            GetCategory: GetCategory::<Impl, IMPL_OFFSET>,
            GetType: GetType::<Impl, IMPL_OFFSET>,
            GetFriendlyName: GetFriendlyName::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetProperties: GetProperties::<Impl, IMPL_OFFSET>,
            GetSupportedDataFields: GetSupportedDataFields::<Impl, IMPL_OFFSET>,
            SetProperties: SetProperties::<Impl, IMPL_OFFSET>,
            SupportsDataField: SupportsDataField::<Impl, IMPL_OFFSET>,
            GetState: GetState::<Impl, IMPL_OFFSET>,
            GetData: GetData::<Impl, IMPL_OFFSET>,
            SupportsEvent: SupportsEvent::<Impl, IMPL_OFFSET>,
            GetEventInterest: GetEventInterest::<Impl, IMPL_OFFSET>,
            SetEventInterest: SetEventInterest::<Impl, IMPL_OFFSET>,
            SetEventSink: SetEventSink::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensor as ::windows::core::Interface>::IID
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
impl ISensorCollectionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorCollectionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorCollectionVtbl {
        unsafe extern "system" fn GetAt<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppsensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCount<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveByID<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clear<Impl: ISensorCollectionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetAt: GetAt::<Impl, IMPL_OFFSET>,
            GetCount: GetCount::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveByID: RemoveByID::<Impl, IMPL_OFFSET>,
            Clear: Clear::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorCollection as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISensorDataReportImpl: Sized {
    fn GetTimestamp();
    fn GetSensorValue();
    fn GetSensorValues();
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISensorDataReportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorDataReportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorDataReportVtbl {
        unsafe extern "system" fn GetTimestamp<Impl: ISensorDataReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSensorValue<Impl: ISensorDataReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSensorValues<Impl: ISensorDataReportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeys: ::windows::core::RawPtr, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTimestamp: GetTimestamp::<Impl, IMPL_OFFSET>,
            GetSensorValue: GetSensorValue::<Impl, IMPL_OFFSET>,
            GetSensorValues: GetSensorValues::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorDataReport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub trait ISensorEventsImpl: Sized {
    fn OnStateChanged();
    fn OnDataUpdated();
    fn OnEvent();
    fn OnLeave();
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl ISensorEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorEventsVtbl {
        unsafe extern "system" fn OnStateChanged<Impl: ISensorEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, state: SensorState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnDataUpdated<Impl: ISensorEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, pnewdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnEvent<Impl: ISensorEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, eventid: *const ::windows::core::GUID, peventdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLeave<Impl: ISensorEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            OnStateChanged: OnStateChanged::<Impl, IMPL_OFFSET>,
            OnDataUpdated: OnDataUpdated::<Impl, IMPL_OFFSET>,
            OnEvent: OnEvent::<Impl, IMPL_OFFSET>,
            OnLeave: OnLeave::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait ISensorManagerImpl: Sized {
    fn GetSensorsByCategory();
    fn GetSensorsByType();
    fn GetSensorByID();
    fn SetEventSink();
    fn RequestPermissions();
}
#[cfg(feature = "Win32_Foundation")]
impl ISensorManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorManagerVtbl {
        unsafe extern "system" fn GetSensorsByCategory<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorcategory: *const ::windows::core::GUID, ppsensorsfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSensorsByType<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensortype: *const ::windows::core::GUID, ppsensorsfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSensorByID<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: *const ::windows::core::GUID, ppsensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEventSink<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestPermissions<Impl: ISensorManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, psensors: ::windows::core::RawPtr, fmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSensorsByCategory: GetSensorsByCategory::<Impl, IMPL_OFFSET>,
            GetSensorsByType: GetSensorsByType::<Impl, IMPL_OFFSET>,
            GetSensorByID: GetSensorByID::<Impl, IMPL_OFFSET>,
            SetEventSink: SetEventSink::<Impl, IMPL_OFFSET>,
            RequestPermissions: RequestPermissions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorManager as ::windows::core::Interface>::IID
    }
}
pub trait ISensorManagerEventsImpl: Sized {
    fn OnSensorEnter();
}
impl ISensorManagerEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorManagerEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorManagerEventsVtbl {
        unsafe extern "system" fn OnSensorEnter<Impl: ISensorManagerEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, state: SensorState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnSensorEnter: OnSensorEnter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorManagerEvents as ::windows::core::Interface>::IID
    }
}
