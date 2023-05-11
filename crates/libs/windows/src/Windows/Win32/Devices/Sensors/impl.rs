#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ILocationPermissions_Impl: Sized {
    fn GetGlobalLocationPermission(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn CheckLocationCapability(&self, dwclientthreadid: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ILocationPermissions {}
#[cfg(feature = "Win32_Foundation")]
impl ILocationPermissions_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationPermissions_Impl, const OFFSET: isize>() -> ILocationPermissions_Vtbl {
        unsafe extern "system" fn GetGlobalLocationPermission<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationPermissions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGlobalLocationPermission() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfenabled, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckLocationCapability<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ILocationPermissions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwclientthreadid: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CheckLocationCapability(::core::mem::transmute_copy(&dwclientthreadid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGlobalLocationPermission: GetGlobalLocationPermission::<Identity, Impl, OFFSET>,
            CheckLocationCapability: CheckLocationCapability::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ILocationPermissions as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Devices_PortableDevices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISensor_Impl: Sized {
    fn GetID(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetCategory(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetType(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetFriendlyName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetProperty(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetProperties(&self, pkeys: ::core::option::Option<&super::PortableDevices::IPortableDeviceKeyCollection>) -> ::windows_core::Result<super::PortableDevices::IPortableDeviceValues>;
    fn GetSupportedDataFields(&self) -> ::windows_core::Result<super::PortableDevices::IPortableDeviceKeyCollection>;
    fn SetProperties(&self, pproperties: ::core::option::Option<&super::PortableDevices::IPortableDeviceValues>) -> ::windows_core::Result<super::PortableDevices::IPortableDeviceValues>;
    fn SupportsDataField(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetState(&self) -> ::windows_core::Result<SensorState>;
    fn GetData(&self) -> ::windows_core::Result<ISensorDataReport>;
    fn SupportsEvent(&self, eventguid: *const ::windows_core::GUID) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetEventInterest(&self, ppvalues: *mut *mut ::windows_core::GUID, pcount: *mut u32) -> ::windows_core::Result<()>;
    fn SetEventInterest(&self, pvalues: *const ::windows_core::GUID, count: u32) -> ::windows_core::Result<()>;
    fn SetEventSink(&self, pevents: ::core::option::Option<&ISensorEvents>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for ISensor {}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISensor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>() -> ISensor_Vtbl {
        unsafe extern "system" fn GetID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensorcategory: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psensorcategory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensortype: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psensortype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfriendlyname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetFriendlyName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfriendlyname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pproperty: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproperty, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeys: *mut ::core::ffi::c_void, ppproperties: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperties(::windows_core::from_raw_borrowed(&pkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppproperties, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedDataFields<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdatafields: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSupportedDataFields() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatafields, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproperties: *mut ::core::ffi::c_void, ppresults: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SetProperties(::windows_core::from_raw_borrowed(&pproperties)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppresults, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsDataField<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pissupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportsDataField(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pissupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut SensorState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdatareport: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdatareport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventguid: *const ::windows_core::GUID, pissupported: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportsEvent(::core::mem::transmute_copy(&eventguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pissupported, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetEventInterest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::windows_core::GUID, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetEventInterest(::core::mem::transmute_copy(&ppvalues), ::core::mem::transmute_copy(&pcount)).into()
        }
        unsafe extern "system" fn SetEventInterest<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvalues: *const ::windows_core::GUID, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventInterest(::core::mem::transmute_copy(&pvalues), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SetEventSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventSink(::windows_core::from_raw_borrowed(&pevents)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            GetType: GetType::<Identity, Impl, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetProperties: GetProperties::<Identity, Impl, OFFSET>,
            GetSupportedDataFields: GetSupportedDataFields::<Identity, Impl, OFFSET>,
            SetProperties: SetProperties::<Identity, Impl, OFFSET>,
            SupportsDataField: SupportsDataField::<Identity, Impl, OFFSET>,
            GetState: GetState::<Identity, Impl, OFFSET>,
            GetData: GetData::<Identity, Impl, OFFSET>,
            SupportsEvent: SupportsEvent::<Identity, Impl, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, Impl, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, Impl, OFFSET>,
            SetEventSink: SetEventSink::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISensor as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"implement\"`*"]
pub trait ISensorCollection_Impl: Sized {
    fn GetAt(&self, ulindex: u32) -> ::windows_core::Result<ISensor>;
    fn GetCount(&self) -> ::windows_core::Result<u32>;
    fn Add(&self, psensor: ::core::option::Option<&ISensor>) -> ::windows_core::Result<()>;
    fn Remove(&self, psensor: ::core::option::Option<&ISensor>) -> ::windows_core::Result<()>;
    fn RemoveByID(&self, sensorid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn Clear(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISensorCollection {}
impl ISensorCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: isize>() -> ISensorCollection_Vtbl {
        unsafe extern "system" fn GetAt<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulindex: u32, ppsensor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAt(::core::mem::transmute_copy(&ulindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsensor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::windows_core::from_raw_borrowed(&psensor)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::windows_core::from_raw_borrowed(&psensor)).into()
        }
        unsafe extern "system" fn RemoveByID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveByID(::core::mem::transmute_copy(&sensorid)).into()
        }
        unsafe extern "system" fn Clear<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clear().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAt: GetAt::<Identity, Impl, OFFSET>,
            GetCount: GetCount::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveByID: RemoveByID::<Identity, Impl, OFFSET>,
            Clear: Clear::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISensorCollection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Devices_PortableDevices\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_System_Variant\"`, `\"Win32_UI_Shell_PropertiesSystem\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
pub trait ISensorDataReport_Impl: Sized {
    fn GetTimestamp(&self) -> ::windows_core::Result<super::super::Foundation::SYSTEMTIME>;
    fn GetSensorValue(&self, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::PROPVARIANT>;
    fn GetSensorValues(&self, pkeys: ::core::option::Option<&super::PortableDevices::IPortableDeviceKeyCollection>) -> ::windows_core::Result<super::PortableDevices::IPortableDeviceValues>;
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ::windows_core::RuntimeName for ISensorDataReport {}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant", feature = "Win32_UI_Shell_PropertiesSystem"))]
impl ISensorDataReport_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorDataReport_Impl, const OFFSET: isize>() -> ISensorDataReport_Vtbl {
        unsafe extern "system" fn GetTimestamp<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptimestamp: *mut super::super::Foundation::SYSTEMTIME) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTimestamp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ptimestamp, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorValue<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkey: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, pvalue: *mut super::super::System::Com::StructuredStorage::PROPVARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSensorValue(::core::mem::transmute_copy(&pkey)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorValues<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pkeys: *mut ::core::ffi::c_void, ppvalues: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSensorValues(::windows_core::from_raw_borrowed(&pkeys)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppvalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTimestamp: GetTimestamp::<Identity, Impl, OFFSET>,
            GetSensorValue: GetSensorValue::<Identity, Impl, OFFSET>,
            GetSensorValues: GetSensorValues::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISensorDataReport as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Devices_PortableDevices\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub trait ISensorEvents_Impl: Sized {
    fn OnStateChanged(&self, psensor: ::core::option::Option<&ISensor>, state: SensorState) -> ::windows_core::Result<()>;
    fn OnDataUpdated(&self, psensor: ::core::option::Option<&ISensor>, pnewdata: ::core::option::Option<&ISensorDataReport>) -> ::windows_core::Result<()>;
    fn OnEvent(&self, psensor: ::core::option::Option<&ISensor>, eventid: *const ::windows_core::GUID, peventdata: ::core::option::Option<&super::PortableDevices::IPortableDeviceValues>) -> ::windows_core::Result<()>;
    fn OnLeave(&self, id: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl ::windows_core::RuntimeName for ISensorEvents {}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl ISensorEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: isize>() -> ISensorEvents_Vtbl {
        unsafe extern "system" fn OnStateChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void, state: SensorState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnStateChanged(::windows_core::from_raw_borrowed(&psensor), ::core::mem::transmute_copy(&state)).into()
        }
        unsafe extern "system" fn OnDataUpdated<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void, pnewdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnDataUpdated(::windows_core::from_raw_borrowed(&psensor), ::windows_core::from_raw_borrowed(&pnewdata)).into()
        }
        unsafe extern "system" fn OnEvent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void, eventid: *const ::windows_core::GUID, peventdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnEvent(::windows_core::from_raw_borrowed(&psensor), ::core::mem::transmute_copy(&eventid), ::windows_core::from_raw_borrowed(&peventdata)).into()
        }
        unsafe extern "system" fn OnLeave<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnLeave(::core::mem::transmute_copy(&id)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, Impl, OFFSET>,
            OnDataUpdated: OnDataUpdated::<Identity, Impl, OFFSET>,
            OnEvent: OnEvent::<Identity, Impl, OFFSET>,
            OnLeave: OnLeave::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISensorEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait ISensorManager_Impl: Sized {
    fn GetSensorsByCategory(&self, sensorcategory: *const ::windows_core::GUID) -> ::windows_core::Result<ISensorCollection>;
    fn GetSensorsByType(&self, sensortype: *const ::windows_core::GUID) -> ::windows_core::Result<ISensorCollection>;
    fn GetSensorByID(&self, sensorid: *const ::windows_core::GUID) -> ::windows_core::Result<ISensor>;
    fn SetEventSink(&self, pevents: ::core::option::Option<&ISensorManagerEvents>) -> ::windows_core::Result<()>;
    fn RequestPermissions(&self, hparent: super::super::Foundation::HWND, psensors: ::core::option::Option<&ISensorCollection>, fmodal: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for ISensorManager {}
#[cfg(feature = "Win32_Foundation")]
impl ISensorManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: isize>() -> ISensorManager_Vtbl {
        unsafe extern "system" fn GetSensorsByCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorcategory: *const ::windows_core::GUID, ppsensorsfound: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSensorsByCategory(::core::mem::transmute_copy(&sensorcategory)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsensorsfound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorsByType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensortype: *const ::windows_core::GUID, ppsensorsfound: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSensorsByType(::core::mem::transmute_copy(&sensortype)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsensorsfound, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSensorByID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sensorid: *const ::windows_core::GUID, ppsensor: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSensorByID(::core::mem::transmute_copy(&sensorid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsensor, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEventSink<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEventSink(::windows_core::from_raw_borrowed(&pevents)).into()
        }
        unsafe extern "system" fn RequestPermissions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparent: super::super::Foundation::HWND, psensors: *mut ::core::ffi::c_void, fmodal: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RequestPermissions(::core::mem::transmute_copy(&hparent), ::windows_core::from_raw_borrowed(&psensors), ::core::mem::transmute_copy(&fmodal)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSensorsByCategory: GetSensorsByCategory::<Identity, Impl, OFFSET>,
            GetSensorsByType: GetSensorsByType::<Identity, Impl, OFFSET>,
            GetSensorByID: GetSensorByID::<Identity, Impl, OFFSET>,
            SetEventSink: SetEventSink::<Identity, Impl, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISensorManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Devices_Sensors\"`, `\"implement\"`*"]
pub trait ISensorManagerEvents_Impl: Sized {
    fn OnSensorEnter(&self, psensor: ::core::option::Option<&ISensor>, state: SensorState) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for ISensorManagerEvents {}
impl ISensorManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorManagerEvents_Impl, const OFFSET: isize>() -> ISensorManagerEvents_Vtbl {
        unsafe extern "system" fn OnSensorEnter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISensorManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psensor: *mut ::core::ffi::c_void, state: SensorState) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OnSensorEnter(::windows_core::from_raw_borrowed(&psensor), ::core::mem::transmute_copy(&state)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSensorEnter: OnSensorEnter::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISensorManagerEvents as ::windows_core::ComInterface>::IID
    }
}
