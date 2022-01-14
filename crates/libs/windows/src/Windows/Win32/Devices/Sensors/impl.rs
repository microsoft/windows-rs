#[cfg(feature = "Win32_Foundation")]
pub trait ILocationPermissions_Impl: Sized {
    fn GetGlobalLocationPermission(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn CheckLocationCapability(&mut self, dwclientthreadid: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ILocationPermissions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ILocationPermissions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ILocationPermissions_Vtbl {
        unsafe extern "system" fn GetGlobalLocationPermission<Impl: ILocationPermissions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGlobalLocationPermission() {
                ::core::result::Result::Ok(ok__) => {
                    *pfenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckLocationCapability<Impl: ILocationPermissions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientthreadid: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CheckLocationCapability(::core::mem::transmute_copy(&dwclientthreadid)).into()
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
pub trait ISensor_Impl: Sized {
    fn GetID(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetCategory(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetType(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetFriendlyName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetProperty(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetProperties(&mut self, pkeys: ::core::option::Option<super::PortableDevices::IPortableDeviceKeyCollection>) -> ::windows::core::Result<super::PortableDevices::IPortableDeviceValues>;
    fn GetSupportedDataFields(&mut self) -> ::windows::core::Result<super::PortableDevices::IPortableDeviceKeyCollection>;
    fn SetProperties(&mut self, pproperties: ::core::option::Option<super::PortableDevices::IPortableDeviceValues>) -> ::windows::core::Result<super::PortableDevices::IPortableDeviceValues>;
    fn SupportsDataField(&mut self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<i16>;
    fn GetState(&mut self) -> ::windows::core::Result<SensorState>;
    fn GetData(&mut self) -> ::windows::core::Result<ISensorDataReport>;
    fn SupportsEvent(&mut self, eventguid: *const ::windows::core::GUID) -> ::windows::core::Result<i16>;
    fn GetEventInterest(&mut self, ppvalues: *mut *mut ::windows::core::GUID, pcount: *mut u32) -> ::windows::core::Result<()>;
    fn SetEventInterest(&mut self, pvalues: *const ::windows::core::GUID, count: u32) -> ::windows::core::Result<()>;
    fn SetEventSink(&mut self, pevents: ::core::option::Option<ISensorEvents>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISensor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensor_Vtbl {
        unsafe extern "system" fn GetID<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetID() {
                ::core::result::Result::Ok(ok__) => {
                    *pid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensorcategory: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    *psensorcategory = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensortype: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetType() {
                ::core::result::Result::Ok(ok__) => {
                    *psensortype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfriendlyname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    *pfriendlyname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pproperty: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproperty = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeys: ::windows::core::RawPtr, ppproperties: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperties(::core::mem::transmute(&pkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppproperties = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedDataFields<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdatafields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedDataFields() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdatafields = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperties<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: ::windows::core::RawPtr, ppresults: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperties(::core::mem::transmute(&pproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsDataField<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pissupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsDataField(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *pissupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut SensorState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdatareport: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetData() {
                ::core::result::Result::Ok(ok__) => {
                    *ppdatareport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsEvent<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventguid: *const ::windows::core::GUID, pissupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsEvent(::core::mem::transmute_copy(&eventguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pissupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventInterest<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::windows::core::GUID, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetEventInterest(::core::mem::transmute_copy(&ppvalues), ::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn SetEventInterest<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: *const ::windows::core::GUID, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventInterest(::core::mem::transmute_copy(&pvalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetEventSink<Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventSink(::core::mem::transmute(&pevents)).into()
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
pub trait ISensorCollection_Impl: Sized {
    fn GetAt(&mut self, ulindex: u32) -> ::windows::core::Result<ISensor>;
    fn GetCount(&mut self) -> ::windows::core::Result<u32>;
    fn Add(&mut self, psensor: ::core::option::Option<ISensor>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, psensor: ::core::option::Option<ISensor>) -> ::windows::core::Result<()>;
    fn RemoveByID(&mut self, sensorid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn Clear(&mut self) -> ::windows::core::Result<()>;
}
impl ISensorCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorCollection_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorCollection_Vtbl {
        unsafe extern "system" fn GetAt<Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppsensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsensor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&psensor)).into()
        }
        unsafe extern "system" fn Remove<Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute(&psensor)).into()
        }
        unsafe extern "system" fn RemoveByID<Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveByID(::core::mem::transmute_copy(&sensorid)).into()
        }
        unsafe extern "system" fn Clear<Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Clear().into()
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
pub trait ISensorDataReport_Impl: Sized {
    fn GetTimestamp(&mut self) -> ::windows::core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetSensorValue(&mut self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetSensorValues(&mut self, pkeys: ::core::option::Option<super::PortableDevices::IPortableDeviceKeyCollection>) -> ::windows::core::Result<super::PortableDevices::IPortableDeviceValues>;
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISensorDataReport_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorDataReport_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorDataReport_Vtbl {
        unsafe extern "system" fn GetTimestamp<Impl: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut super::super::Foundation::SYSTEMTIME) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    *ptimestamp = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorValue<Impl: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorValue(::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorValues<Impl: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeys: ::windows::core::RawPtr, ppvalues: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorValues(::core::mem::transmute(&pkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppvalues = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
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
pub trait ISensorEvents_Impl: Sized {
    fn OnStateChanged(&mut self, psensor: ::core::option::Option<ISensor>, state: SensorState) -> ::windows::core::Result<()>;
    fn OnDataUpdated(&mut self, psensor: ::core::option::Option<ISensor>, pnewdata: ::core::option::Option<ISensorDataReport>) -> ::windows::core::Result<()>;
    fn OnEvent(&mut self, psensor: ::core::option::Option<ISensor>, eventid: *const ::windows::core::GUID, peventdata: ::core::option::Option<super::PortableDevices::IPortableDeviceValues>) -> ::windows::core::Result<()>;
    fn OnLeave(&mut self, id: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl ISensorEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorEvents_Vtbl {
        unsafe extern "system" fn OnStateChanged<Impl: ISensorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, state: SensorState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnStateChanged(::core::mem::transmute(&psensor), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn OnDataUpdated<Impl: ISensorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, pnewdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDataUpdated(::core::mem::transmute(&psensor), ::core::mem::transmute(&pnewdata)).into()
        }
        unsafe extern "system" fn OnEvent<Impl: ISensorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, eventid: *const ::windows::core::GUID, peventdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnEvent(::core::mem::transmute(&psensor), ::core::mem::transmute_copy(&eventid), ::core::mem::transmute(&peventdata)).into()
        }
        unsafe extern "system" fn OnLeave<Impl: ISensorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLeave(::core::mem::transmute_copy(&id)).into()
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
pub trait ISensorManager_Impl: Sized {
    fn GetSensorsByCategory(&mut self, sensorcategory: *const ::windows::core::GUID) -> ::windows::core::Result<ISensorCollection>;
    fn GetSensorsByType(&mut self, sensortype: *const ::windows::core::GUID) -> ::windows::core::Result<ISensorCollection>;
    fn GetSensorByID(&mut self, sensorid: *const ::windows::core::GUID) -> ::windows::core::Result<ISensor>;
    fn SetEventSink(&mut self, pevents: ::core::option::Option<ISensorManagerEvents>) -> ::windows::core::Result<()>;
    fn RequestPermissions(&mut self, hparent: super::super::Foundation::HWND, psensors: ::core::option::Option<ISensorCollection>, fmodal: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ISensorManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorManager_Vtbl {
        unsafe extern "system" fn GetSensorsByCategory<Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorcategory: *const ::windows::core::GUID, ppsensorsfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorsByCategory(::core::mem::transmute_copy(&sensorcategory)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsensorsfound = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorsByType<Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensortype: *const ::windows::core::GUID, ppsensorsfound: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorsByType(::core::mem::transmute_copy(&sensortype)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsensorsfound = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorByID<Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: *const ::windows::core::GUID, ppsensor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSensorByID(::core::mem::transmute_copy(&sensorid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppsensor = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventSink<Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEventSink(::core::mem::transmute(&pevents)).into()
        }
        unsafe extern "system" fn RequestPermissions<Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, psensors: ::windows::core::RawPtr, fmodal: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RequestPermissions(::core::mem::transmute_copy(&hparent), ::core::mem::transmute(&psensors), ::core::mem::transmute_copy(&fmodal)).into()
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
pub trait ISensorManagerEvents_Impl: Sized {
    fn OnSensorEnter(&mut self, psensor: ::core::option::Option<ISensor>, state: SensorState) -> ::windows::core::Result<()>;
}
impl ISensorManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: ISensorManagerEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> ISensorManagerEvents_Vtbl {
        unsafe extern "system" fn OnSensorEnter<Impl: ISensorManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: ::windows::core::RawPtr, state: SensorState) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSensorEnter(::core::mem::transmute(&psensor), ::core::mem::transmute_copy(&state)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), OnSensorEnter: OnSensorEnter::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISensorManagerEvents as ::windows::core::Interface>::IID
    }
}
