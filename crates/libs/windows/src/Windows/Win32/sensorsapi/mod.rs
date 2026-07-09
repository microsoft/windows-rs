windows_core::imp::define_interface!(ILocationPermissions, ILocationPermissions_Vtbl, 0xd5fb0a7f_e74e_44f5_8e02_4806863a274f);
windows_core::imp::interface_hierarchy!(ILocationPermissions, windows_core::IUnknown);
impl ILocationPermissions {
    pub unsafe fn GetGlobalLocationPermission(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGlobalLocationPermission)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CheckLocationCapability(&self, dwclientthreadid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CheckLocationCapability)(windows_core::Interface::as_raw(self), dwclientthreadid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILocationPermissions_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetGlobalLocationPermission: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CheckLocationCapability: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait ILocationPermissions_Impl: windows_core::IUnknownImpl {
    fn GetGlobalLocationPermission(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CheckLocationCapability(&self, dwclientthreadid: u32) -> windows_core::Result<()>;
}
impl ILocationPermissions_Vtbl {
    pub const fn new<Identity: ILocationPermissions_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetGlobalLocationPermission<Identity: ILocationPermissions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfenabled: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILocationPermissions_Impl::GetGlobalLocationPermission(this) {
                    Ok(ok__) => {
                        pfenabled.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CheckLocationCapability<Identity: ILocationPermissions_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwclientthreadid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILocationPermissions_Impl::CheckLocationCapability(this, core::mem::transmute_copy(&dwclientthreadid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetGlobalLocationPermission: GetGlobalLocationPermission::<Identity, OFFSET>,
            CheckLocationCapability: CheckLocationCapability::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILocationPermissions as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ILocationPermissions {}
windows_core::imp::define_interface!(ISensor, ISensor_Vtbl, 0x5fa08f80_2657_458e_af75_46f73fa6ac5c);
windows_core::imp::interface_hierarchy!(ISensor, windows_core::IUnknown);
impl ISensor {
    pub unsafe fn GetID(&self) -> windows_core::Result<SENSOR_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCategory(&self) -> windows_core::Result<SENSOR_CATEGORY_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCategory)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetType(&self) -> windows_core::Result<SENSOR_TYPE_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetProperty(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub unsafe fn GetProperties<P0>(&self, pkeys: P0) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDeviceKeyCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pkeys.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub unsafe fn GetSupportedDataFields(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedDataFields)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub unsafe fn SetProperties<P0>(&self, pproperties: P0) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetProperties)(windows_core::Interface::as_raw(self), pproperties.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SupportsDataField(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportsDataField)(windows_core::Interface::as_raw(self), key, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetState(&self) -> windows_core::Result<SensorState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetData(&self) -> windows_core::Result<ISensorDataReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Win32_wtypes")]
    pub unsafe fn SupportsEvent(&self, eventguid: *const windows_core::GUID) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportsEvent)(windows_core::Interface::as_raw(self), eventguid, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEventInterest(&self, ppvalues: *mut *mut windows_core::GUID, pcount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEventInterest)(windows_core::Interface::as_raw(self), ppvalues as _, pcount as _) }
    }
    pub unsafe fn SetEventInterest(&self, pvalues: *const windows_core::GUID, count: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEventInterest)(windows_core::Interface::as_raw(self), pvalues, count) }
    }
    pub unsafe fn SetEventSink<P0>(&self, pevents: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISensorEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), pevents.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SENSOR_ID) -> windows_core::HRESULT,
    pub GetCategory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SENSOR_CATEGORY_ID) -> windows_core::HRESULT,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SENSOR_TYPE_ID) -> windows_core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetProperty: usize,
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_portabledevicetypes"))]
    GetProperties: usize,
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub GetSupportedDataFields: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_portabledevicetypes"))]
    GetSupportedDataFields: usize,
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub SetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_portabledevicetypes"))]
    SetProperties: usize,
    #[cfg(feature = "Win32_wtypes")]
    pub SupportsDataField: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SupportsDataField: usize,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SensorState) -> windows_core::HRESULT,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_wtypes")]
    pub SupportsEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_wtypes"))]
    SupportsEvent: usize,
    pub GetEventInterest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut windows_core::GUID, *mut u32) -> windows_core::HRESULT,
    pub SetEventInterest: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_portabledevicetypes", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISensor_Impl: windows_core::IUnknownImpl {
    fn GetID(&self) -> windows_core::Result<SENSOR_ID>;
    fn GetCategory(&self) -> windows_core::Result<SENSOR_CATEGORY_ID>;
    fn GetType(&self) -> windows_core::Result<SENSOR_TYPE_ID>;
    fn GetFriendlyName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetProperty(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn GetProperties(&self, pkeys: windows_core::Ref<super::portabledevicetypes::IPortableDeviceKeyCollection>) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn GetSupportedDataFields(&self) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceKeyCollection>;
    fn SetProperties(&self, pproperties: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
    fn SupportsDataField(&self, key: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn GetState(&self) -> windows_core::Result<SensorState>;
    fn GetData(&self) -> windows_core::Result<ISensorDataReport>;
    fn SupportsEvent(&self, eventguid: *const windows_core::GUID) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn GetEventInterest(&self, ppvalues: *mut *mut windows_core::GUID, pcount: *mut u32) -> windows_core::Result<()>;
    fn SetEventInterest(&self, pvalues: *const windows_core::GUID, count: u32) -> windows_core::Result<()>;
    fn SetEventSink(&self, pevents: windows_core::Ref<ISensorEvents>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_portabledevicetypes", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISensor_Vtbl {
    pub const fn new<Identity: ISensor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetID<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut SENSOR_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetID(this) {
                    Ok(ok__) => {
                        pid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensorcategory: *mut SENSOR_CATEGORY_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetCategory(this) {
                    Ok(ok__) => {
                        psensorcategory.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetType<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensortype: *mut SENSOR_TYPE_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetType(this) {
                    Ok(ok__) => {
                        psensortype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFriendlyName<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfriendlyname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetFriendlyName(this) {
                    Ok(ok__) => {
                        pfriendlyname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, pproperty: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetProperty(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pproperty.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperties<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeys: *mut core::ffi::c_void, ppproperties: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetProperties(this, core::mem::transmute_copy(&pkeys)) {
                    Ok(ok__) => {
                        ppproperties.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedDataFields<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdatafields: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetSupportedDataFields(this) {
                    Ok(ok__) => {
                        ppdatafields.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperties<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pproperties: *mut core::ffi::c_void, ppresults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::SetProperties(this, core::mem::transmute_copy(&pproperties)) {
                    Ok(ok__) => {
                        ppresults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportsDataField<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, key: *const super::wtypes::PROPERTYKEY, pissupported: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::SupportsDataField(this, core::mem::transmute_copy(&key)) {
                    Ok(ok__) => {
                        pissupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetState<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstate: *mut SensorState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetState(this) {
                    Ok(ok__) => {
                        pstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetData<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdatareport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::GetData(this) {
                    Ok(ok__) => {
                        ppdatareport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportsEvent<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventguid: *const windows_core::GUID, pissupported: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensor_Impl::SupportsEvent(this, core::mem::transmute_copy(&eventguid)) {
                    Ok(ok__) => {
                        pissupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEventInterest<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvalues: *mut *mut windows_core::GUID, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensor_Impl::GetEventInterest(this, core::mem::transmute_copy(&ppvalues), core::mem::transmute_copy(&pcount)).into()
            }
        }
        unsafe extern "system" fn SetEventInterest<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvalues: *const windows_core::GUID, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensor_Impl::SetEventInterest(this, core::mem::transmute_copy(&pvalues), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SetEventSink<Identity: ISensor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevents: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensor_Impl::SetEventSink(this, core::mem::transmute_copy(&pevents)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetID: GetID::<Identity, OFFSET>,
            GetCategory: GetCategory::<Identity, OFFSET>,
            GetType: GetType::<Identity, OFFSET>,
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
            GetSupportedDataFields: GetSupportedDataFields::<Identity, OFFSET>,
            SetProperties: SetProperties::<Identity, OFFSET>,
            SupportsDataField: SupportsDataField::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            GetData: GetData::<Identity, OFFSET>,
            SupportsEvent: SupportsEvent::<Identity, OFFSET>,
            GetEventInterest: GetEventInterest::<Identity, OFFSET>,
            SetEventInterest: SetEventInterest::<Identity, OFFSET>,
            SetEventSink: SetEventSink::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensor as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_portabledevicetypes", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISensor {}
windows_core::imp::define_interface!(ISensorCollection, ISensorCollection_Vtbl, 0x23571e11_e545_4dd8_a337_b89bf44b10df);
windows_core::imp::interface_hierarchy!(ISensorCollection, windows_core::IUnknown);
impl ISensorCollection {
    pub unsafe fn GetAt(&self, ulindex: u32) -> windows_core::Result<ISensor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAt)(windows_core::Interface::as_raw(self), ulindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Add<P0>(&self, psensor: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISensor>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), psensor.param().abi()) }
    }
    pub unsafe fn Remove<P0>(&self, psensor: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISensor>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), psensor.param().abi()) }
    }
    pub unsafe fn RemoveByID(&self, sensorid: REFSENSOR_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveByID)(windows_core::Interface::as_raw(self), sensorid) }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAt: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveByID: unsafe extern "system" fn(*mut core::ffi::c_void, REFSENSOR_ID) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISensorCollection_Impl: windows_core::IUnknownImpl {
    fn GetAt(&self, ulindex: u32) -> windows_core::Result<ISensor>;
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn Add(&self, psensor: windows_core::Ref<ISensor>) -> windows_core::Result<()>;
    fn Remove(&self, psensor: windows_core::Ref<ISensor>) -> windows_core::Result<()>;
    fn RemoveByID(&self, sensorid: REFSENSOR_ID) -> windows_core::Result<()>;
    fn Clear(&self) -> windows_core::Result<()>;
}
impl ISensorCollection_Vtbl {
    pub const fn new<Identity: ISensorCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAt<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulindex: u32, ppsensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorCollection_Impl::GetAt(this, core::mem::transmute_copy(&ulindex)) {
                    Ok(ok__) => {
                        ppsensor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCount<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorCollection_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Add<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorCollection_Impl::Add(this, core::mem::transmute_copy(&psensor)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorCollection_Impl::Remove(this, core::mem::transmute_copy(&psensor)).into()
            }
        }
        unsafe extern "system" fn RemoveByID<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sensorid: REFSENSOR_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorCollection_Impl::RemoveByID(this, core::mem::transmute_copy(&sensorid)).into()
            }
        }
        unsafe extern "system" fn Clear<Identity: ISensorCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorCollection_Impl::Clear(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAt: GetAt::<Identity, OFFSET>,
            GetCount: GetCount::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            RemoveByID: RemoveByID::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISensorCollection {}
windows_core::imp::define_interface!(ISensorDataReport, ISensorDataReport_Vtbl, 0x0ab9df9b_c4b5_4796_8898_0470706a2e1d);
windows_core::imp::interface_hierarchy!(ISensorDataReport, windows_core::IUnknown);
impl ISensorDataReport {
    #[cfg(feature = "Win32_minwinbase")]
    pub unsafe fn GetTimestamp(&self) -> windows_core::Result<super::minwinbase::SYSTEMTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimestamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub unsafe fn GetSensorValue(&self, pkey: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::propidlbase::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorValue)(windows_core::Interface::as_raw(self), pkey, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub unsafe fn GetSensorValues<P0>(&self, pkeys: P0) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>
    where
        P0: windows_core::Param<super::portabledevicetypes::IPortableDeviceKeyCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorValues)(windows_core::Interface::as_raw(self), pkeys.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorDataReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_minwinbase")]
    pub GetTimestamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::minwinbase::SYSTEMTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwinbase"))]
    GetTimestamp: usize,
    #[cfg(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
    pub GetSensorValue: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::wtypes::PROPERTYKEY, *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase")))]
    GetSensorValue: usize,
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub GetSensorValues: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_portabledevicetypes"))]
    GetSensorValues: usize,
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_portabledevicetypes", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
pub trait ISensorDataReport_Impl: windows_core::IUnknownImpl {
    fn GetTimestamp(&self) -> windows_core::Result<super::minwinbase::SYSTEMTIME>;
    fn GetSensorValue(&self, pkey: *const super::wtypes::PROPERTYKEY) -> windows_core::Result<super::propidlbase::PROPVARIANT>;
    fn GetSensorValues(&self, pkeys: windows_core::Ref<super::portabledevicetypes::IPortableDeviceKeyCollection>) -> windows_core::Result<super::portabledevicetypes::IPortableDeviceValues>;
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_portabledevicetypes", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl ISensorDataReport_Vtbl {
    pub const fn new<Identity: ISensorDataReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTimestamp<Identity: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimestamp: *mut super::minwinbase::SYSTEMTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorDataReport_Impl::GetTimestamp(this) {
                    Ok(ok__) => {
                        ptimestamp.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorValue<Identity: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkey: *const super::wtypes::PROPERTYKEY, pvalue: *mut super::propidlbase::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorDataReport_Impl::GetSensorValue(this, core::mem::transmute_copy(&pkey)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorValues<Identity: ISensorDataReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pkeys: *mut core::ffi::c_void, ppvalues: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorDataReport_Impl::GetSensorValues(this, core::mem::transmute_copy(&pkeys)) {
                    Ok(ok__) => {
                        ppvalues.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTimestamp: GetTimestamp::<Identity, OFFSET>,
            GetSensorValue: GetSensorValue::<Identity, OFFSET>,
            GetSensorValues: GetSensorValues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorDataReport as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_minwindef", feature = "Win32_oaidl", feature = "Win32_objidl", feature = "Win32_objidlbase", feature = "Win32_portabledevicetypes", feature = "Win32_propidlbase", feature = "Win32_wtypes", feature = "Win32_wtypesbase"))]
impl windows_core::RuntimeName for ISensorDataReport {}
windows_core::imp::define_interface!(ISensorEvents, ISensorEvents_Vtbl, 0x5d8dcc91_4641_47e7_b7c3_b74f48a6c391);
windows_core::imp::interface_hierarchy!(ISensorEvents, windows_core::IUnknown);
impl ISensorEvents {
    pub unsafe fn OnStateChanged<P0>(&self, psensor: P0, state: SensorState) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISensor>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnStateChanged)(windows_core::Interface::as_raw(self), psensor.param().abi(), state) }
    }
    pub unsafe fn OnDataUpdated<P0, P1>(&self, psensor: P0, pnewdata: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISensor>,
        P1: windows_core::Param<ISensorDataReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDataUpdated)(windows_core::Interface::as_raw(self), psensor.param().abi(), pnewdata.param().abi()) }
    }
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub unsafe fn OnEvent<P0, P2>(&self, psensor: P0, eventid: *const windows_core::GUID, peventdata: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISensor>,
        P2: windows_core::Param<super::portabledevicetypes::IPortableDeviceValues>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnEvent)(windows_core::Interface::as_raw(self), psensor.param().abi(), eventid, peventdata.param().abi()) }
    }
    pub unsafe fn OnLeave(&self, id: REFSENSOR_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnLeave)(windows_core::Interface::as_raw(self), id) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SensorState) -> windows_core::HRESULT,
    pub OnDataUpdated: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_portabledevicetypes")]
    pub OnEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_portabledevicetypes"))]
    OnEvent: usize,
    pub OnLeave: unsafe extern "system" fn(*mut core::ffi::c_void, REFSENSOR_ID) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_portabledevicetypes")]
pub trait ISensorEvents_Impl: windows_core::IUnknownImpl {
    fn OnStateChanged(&self, psensor: windows_core::Ref<ISensor>, state: SensorState) -> windows_core::Result<()>;
    fn OnDataUpdated(&self, psensor: windows_core::Ref<ISensor>, pnewdata: windows_core::Ref<ISensorDataReport>) -> windows_core::Result<()>;
    fn OnEvent(&self, psensor: windows_core::Ref<ISensor>, eventid: *const windows_core::GUID, peventdata: windows_core::Ref<super::portabledevicetypes::IPortableDeviceValues>) -> windows_core::Result<()>;
    fn OnLeave(&self, id: REFSENSOR_ID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_portabledevicetypes")]
impl ISensorEvents_Vtbl {
    pub const fn new<Identity: ISensorEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnStateChanged<Identity: ISensorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void, state: SensorState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorEvents_Impl::OnStateChanged(this, core::mem::transmute_copy(&psensor), core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn OnDataUpdated<Identity: ISensorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void, pnewdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorEvents_Impl::OnDataUpdated(this, core::mem::transmute_copy(&psensor), core::mem::transmute_copy(&pnewdata)).into()
            }
        }
        unsafe extern "system" fn OnEvent<Identity: ISensorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void, eventid: *const windows_core::GUID, peventdata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorEvents_Impl::OnEvent(this, core::mem::transmute_copy(&psensor), core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&peventdata)).into()
            }
        }
        unsafe extern "system" fn OnLeave<Identity: ISensorEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: REFSENSOR_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorEvents_Impl::OnLeave(this, core::mem::transmute_copy(&id)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnStateChanged: OnStateChanged::<Identity, OFFSET>,
            OnDataUpdated: OnDataUpdated::<Identity, OFFSET>,
            OnEvent: OnEvent::<Identity, OFFSET>,
            OnLeave: OnLeave::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_portabledevicetypes")]
impl windows_core::RuntimeName for ISensorEvents {}
windows_core::imp::define_interface!(ISensorManager, ISensorManager_Vtbl, 0xbd77db67_45a8_42dc_8d00_6dcf15f8377a);
windows_core::imp::interface_hierarchy!(ISensorManager, windows_core::IUnknown);
impl ISensorManager {
    pub unsafe fn GetSensorsByCategory(&self, sensorcategory: REFSENSOR_CATEGORY_ID) -> windows_core::Result<ISensorCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorsByCategory)(windows_core::Interface::as_raw(self), sensorcategory, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSensorsByType(&self, sensortype: REFSENSOR_TYPE_ID) -> windows_core::Result<ISensorCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorsByType)(windows_core::Interface::as_raw(self), sensortype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSensorByID(&self, sensorid: REFSENSOR_ID) -> windows_core::Result<ISensor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorByID)(windows_core::Interface::as_raw(self), sensorid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetEventSink<P0>(&self, pevents: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISensorManagerEvents>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEventSink)(windows_core::Interface::as_raw(self), pevents.param().abi()) }
    }
    #[cfg(feature = "Win32_windef")]
    pub unsafe fn RequestPermissions<P1>(&self, hparent: super::windef::HWND, psensors: P1, fmodal: bool) -> windows_core::HRESULT
    where
        P1: windows_core::Param<ISensorCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestPermissions)(windows_core::Interface::as_raw(self), hparent, psensors.param().abi(), fmodal.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSensorsByCategory: unsafe extern "system" fn(*mut core::ffi::c_void, REFSENSOR_CATEGORY_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSensorsByType: unsafe extern "system" fn(*mut core::ffi::c_void, REFSENSOR_TYPE_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSensorByID: unsafe extern "system" fn(*mut core::ffi::c_void, REFSENSOR_ID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetEventSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_windef")]
    pub RequestPermissions: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_windef"))]
    RequestPermissions: usize,
}
#[cfg(feature = "Win32_windef")]
pub trait ISensorManager_Impl: windows_core::IUnknownImpl {
    fn GetSensorsByCategory(&self, sensorcategory: REFSENSOR_CATEGORY_ID) -> windows_core::Result<ISensorCollection>;
    fn GetSensorsByType(&self, sensortype: REFSENSOR_TYPE_ID) -> windows_core::Result<ISensorCollection>;
    fn GetSensorByID(&self, sensorid: REFSENSOR_ID) -> windows_core::Result<ISensor>;
    fn SetEventSink(&self, pevents: windows_core::Ref<ISensorManagerEvents>) -> windows_core::Result<()>;
    fn RequestPermissions(&self, hparent: super::windef::HWND, psensors: windows_core::Ref<ISensorCollection>, fmodal: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_windef")]
impl ISensorManager_Vtbl {
    pub const fn new<Identity: ISensorManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSensorsByCategory<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sensorcategory: REFSENSOR_CATEGORY_ID, ppsensorsfound: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorManager_Impl::GetSensorsByCategory(this, core::mem::transmute_copy(&sensorcategory)) {
                    Ok(ok__) => {
                        ppsensorsfound.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorsByType<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sensortype: REFSENSOR_TYPE_ID, ppsensorsfound: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorManager_Impl::GetSensorsByType(this, core::mem::transmute_copy(&sensortype)) {
                    Ok(ok__) => {
                        ppsensorsfound.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorByID<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sensorid: REFSENSOR_ID, ppsensor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISensorManager_Impl::GetSensorByID(this, core::mem::transmute_copy(&sensorid)) {
                    Ok(ok__) => {
                        ppsensor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEventSink<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevents: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorManager_Impl::SetEventSink(this, core::mem::transmute_copy(&pevents)).into()
            }
        }
        unsafe extern "system" fn RequestPermissions<Identity: ISensorManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hparent: super::windef::HWND, psensors: *mut core::ffi::c_void, fmodal: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorManager_Impl::RequestPermissions(this, core::mem::transmute_copy(&hparent), core::mem::transmute_copy(&psensors), core::mem::transmute_copy(&fmodal)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSensorsByCategory: GetSensorsByCategory::<Identity, OFFSET>,
            GetSensorsByType: GetSensorsByType::<Identity, OFFSET>,
            GetSensorByID: GetSensorByID::<Identity, OFFSET>,
            SetEventSink: SetEventSink::<Identity, OFFSET>,
            RequestPermissions: RequestPermissions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_windef")]
impl windows_core::RuntimeName for ISensorManager {}
windows_core::imp::define_interface!(ISensorManagerEvents, ISensorManagerEvents_Vtbl, 0x9b3b0b86_266a_4aad_b21f_fde5501001b7);
windows_core::imp::interface_hierarchy!(ISensorManagerEvents, windows_core::IUnknown);
impl ISensorManagerEvents {
    pub unsafe fn OnSensorEnter<P0>(&self, psensor: P0, state: SensorState) -> windows_core::HRESULT
    where
        P0: windows_core::Param<ISensor>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSensorEnter)(windows_core::Interface::as_raw(self), psensor.param().abi(), state) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISensorManagerEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnSensorEnter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, SensorState) -> windows_core::HRESULT,
}
pub trait ISensorManagerEvents_Impl: windows_core::IUnknownImpl {
    fn OnSensorEnter(&self, psensor: windows_core::Ref<ISensor>, state: SensorState) -> windows_core::Result<()>;
}
impl ISensorManagerEvents_Vtbl {
    pub const fn new<Identity: ISensorManagerEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSensorEnter<Identity: ISensorManagerEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psensor: *mut core::ffi::c_void, state: SensorState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISensorManagerEvents_Impl::OnSensorEnter(this, core::mem::transmute_copy(&psensor), core::mem::transmute_copy(&state)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSensorEnter: OnSensorEnter::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISensorManagerEvents as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISensorManagerEvents {}
pub type LOCATION_DESIRED_ACCURACY = i32;
pub const LOCATION_DESIRED_ACCURACY_DEFAULT: LOCATION_DESIRED_ACCURACY = 0;
pub const LOCATION_DESIRED_ACCURACY_HIGH: LOCATION_DESIRED_ACCURACY = 1;
pub type LOCATION_POSITION_SOURCE = i32;
pub const LOCATION_POSITION_SOURCE_CELLULAR: LOCATION_POSITION_SOURCE = 0;
pub const LOCATION_POSITION_SOURCE_IPADDRESS: LOCATION_POSITION_SOURCE = 3;
pub const LOCATION_POSITION_SOURCE_SATELLITE: LOCATION_POSITION_SOURCE = 1;
pub const LOCATION_POSITION_SOURCE_UNKNOWN: LOCATION_POSITION_SOURCE = 4;
pub const LOCATION_POSITION_SOURCE_WIFI: LOCATION_POSITION_SOURCE = 2;
pub const MAGNETOMETER_ACCURACY_APPROXIMATE: MagnetometerAccuracy = 2;
pub const MAGNETOMETER_ACCURACY_HIGH: MagnetometerAccuracy = 3;
pub const MAGNETOMETER_ACCURACY_UNKNOWN: MagnetometerAccuracy = 0;
pub const MAGNETOMETER_ACCURACY_UNRELIABLE: MagnetometerAccuracy = 1;
pub type MagnetometerAccuracy = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct REFSENSOR_CATEGORY_ID(pub *const windows_core::GUID);
impl REFSENSOR_CATEGORY_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for REFSENSOR_CATEGORY_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct REFSENSOR_ID(pub *const windows_core::GUID);
impl REFSENSOR_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for REFSENSOR_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct REFSENSOR_TYPE_ID(pub *const windows_core::GUID);
impl REFSENSOR_TYPE_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for REFSENSOR_TYPE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SENSOR_CATEGORY_ID = windows_core::GUID;
pub const SENSOR_CONNECTION_TYPE_PC_ATTACHED: SensorConnectionType = 1;
pub const SENSOR_CONNECTION_TYPE_PC_EXTERNAL: SensorConnectionType = 2;
pub const SENSOR_CONNECTION_TYPE_PC_INTEGRATED: SensorConnectionType = 0;
pub type SENSOR_ID = windows_core::GUID;
pub const SENSOR_STATE_ACCESS_DENIED: SensorState = 4;
pub const SENSOR_STATE_ERROR: SensorState = 5;
pub const SENSOR_STATE_INITIALIZING: SensorState = 3;
pub const SENSOR_STATE_MAX: SensorState = 5;
pub const SENSOR_STATE_MIN: SensorState = 0;
pub const SENSOR_STATE_NOT_AVAILABLE: SensorState = 1;
pub const SENSOR_STATE_NO_DATA: SensorState = 2;
pub const SENSOR_STATE_READY: SensorState = 0;
pub type SENSOR_TYPE_ID = windows_core::GUID;
pub const SIMPLE_DEVICE_ORIENTATION_NOT_ROTATED: SimpleDeviceOrientation = 0;
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_180: SimpleDeviceOrientation = 2;
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_270: SimpleDeviceOrientation = 3;
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_90: SimpleDeviceOrientation = 1;
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_DOWN: SimpleDeviceOrientation = 5;
pub const SIMPLE_DEVICE_ORIENTATION_ROTATED_FACE_UP: SimpleDeviceOrientation = 4;
pub const Sensor: windows_core::GUID = windows_core::GUID::from_u128(0xe97ced00_523a_4133_bf6f_d3a2dae7f6ba);
pub const SensorCollection: windows_core::GUID = windows_core::GUID::from_u128(0x79c43adb_a429_469f_aa39_2f2b74b75937);
pub type SensorConnectionType = i32;
pub const SensorDataReport: windows_core::GUID = windows_core::GUID::from_u128(0x4ea9d6ef_694b_4218_8816_ccda8da74bba);
pub const SensorManager: windows_core::GUID = windows_core::GUID::from_u128(0x77a1c827_fcd2_4689_8915_9d613cc5fa3e);
pub type SensorState = i32;
pub type SimpleDeviceOrientation = i32;
