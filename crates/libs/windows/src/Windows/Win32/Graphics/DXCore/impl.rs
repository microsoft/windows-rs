pub trait IDXCoreAdapter_Impl: Sized {
    fn IsValid(&self) -> bool;
    fn IsAttributeSupported(&self, attributeguid: *const windows_core::GUID) -> bool;
    fn IsPropertySupported(&self, property: DXCoreAdapterProperty) -> bool;
    fn GetProperty(&self, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetPropertySize(&self, property: DXCoreAdapterProperty) -> windows_core::Result<usize>;
    fn IsQueryStateSupported(&self, property: DXCoreAdapterState) -> bool;
    fn QueryState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsSetStateSupported(&self, property: DXCoreAdapterState) -> bool;
    fn SetState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const core::ffi::c_void, inputdatasize: usize, inputdata: *const core::ffi::c_void) -> windows_core::Result<()>;
    fn GetFactory(&self, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXCoreAdapter {}
impl IDXCoreAdapter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>() -> IDXCoreAdapter_Vtbl {
        unsafe extern "system" fn IsValid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapter_Impl::IsValid(this)
        }
        unsafe extern "system" fn IsAttributeSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributeguid: *const windows_core::GUID) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapter_Impl::IsAttributeSupported(this, core::mem::transmute_copy(&attributeguid))
        }
        unsafe extern "system" fn IsPropertySupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterProperty) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapter_Impl::IsPropertySupported(this, core::mem::transmute_copy(&property))
        }
        unsafe extern "system" fn GetProperty<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapter_Impl::GetProperty(this, core::mem::transmute_copy(&property), core::mem::transmute_copy(&buffersize), core::mem::transmute_copy(&propertydata)).into()
        }
        unsafe extern "system" fn GetPropertySize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: *mut usize) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXCoreAdapter_Impl::GetPropertySize(this, core::mem::transmute_copy(&property)) {
                Ok(ok__) => {
                    core::ptr::write(buffersize, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsQueryStateSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapter_Impl::IsQueryStateSupported(this, core::mem::transmute_copy(&property))
        }
        unsafe extern "system" fn QueryState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapter_Impl::QueryState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&inputstatedetailssize), core::mem::transmute_copy(&inputstatedetails), core::mem::transmute_copy(&outputbuffersize), core::mem::transmute_copy(&outputbuffer)).into()
        }
        unsafe extern "system" fn IsSetStateSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapter_Impl::IsSetStateSupported(this, core::mem::transmute_copy(&property))
        }
        unsafe extern "system" fn SetState<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const core::ffi::c_void, inputdatasize: usize, inputdata: *const core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapter_Impl::SetState(this, core::mem::transmute_copy(&state), core::mem::transmute_copy(&inputstatedetailssize), core::mem::transmute_copy(&inputstatedetails), core::mem::transmute_copy(&inputdatasize), core::mem::transmute_copy(&inputdata)).into()
        }
        unsafe extern "system" fn GetFactory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapter_Impl::GetFactory(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvfactory)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsValid: IsValid::<Identity, Impl, OFFSET>,
            IsAttributeSupported: IsAttributeSupported::<Identity, Impl, OFFSET>,
            IsPropertySupported: IsPropertySupported::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetPropertySize: GetPropertySize::<Identity, Impl, OFFSET>,
            IsQueryStateSupported: IsQueryStateSupported::<Identity, Impl, OFFSET>,
            QueryState: QueryState::<Identity, Impl, OFFSET>,
            IsSetStateSupported: IsSetStateSupported::<Identity, Impl, OFFSET>,
            SetState: SetState::<Identity, Impl, OFFSET>,
            GetFactory: GetFactory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXCoreAdapter as windows_core::Interface>::IID
    }
}
pub trait IDXCoreAdapterFactory_Impl: Sized {
    fn CreateAdapterList(&self, numattributes: u32, filterattributes: *const windows_core::GUID, riid: *const windows_core::GUID, ppvadapterlist: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdapterByLuid(&self, adapterluid: *const super::super::Foundation::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn IsNotificationTypeSupported(&self, notificationtype: DXCoreNotificationType) -> bool;
    fn RegisterEventNotification(&self, dxcoreobject: Option<&windows_core::IUnknown>, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const core::ffi::c_void) -> windows_core::Result<u32>;
    fn UnregisterEventNotification(&self, eventcookie: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IDXCoreAdapterFactory {}
impl IDXCoreAdapterFactory_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>() -> IDXCoreAdapterFactory_Vtbl {
        unsafe extern "system" fn CreateAdapterList<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numattributes: u32, filterattributes: *const windows_core::GUID, riid: *const windows_core::GUID, ppvadapterlist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterFactory_Impl::CreateAdapterList(this, core::mem::transmute_copy(&numattributes), core::mem::transmute_copy(&filterattributes), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapterlist)).into()
        }
        unsafe extern "system" fn GetAdapterByLuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adapterluid: *const super::super::Foundation::LUID, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterFactory_Impl::GetAdapterByLuid(this, core::mem::transmute_copy(&adapterluid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn IsNotificationTypeSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notificationtype: DXCoreNotificationType) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterFactory_Impl::IsNotificationTypeSupported(this, core::mem::transmute_copy(&notificationtype))
        }
        unsafe extern "system" fn RegisterEventNotification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dxcoreobject: *mut core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const core::ffi::c_void, eventcookie: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDXCoreAdapterFactory_Impl::RegisterEventNotification(this, windows_core::from_raw_borrowed(&dxcoreobject), core::mem::transmute_copy(&notificationtype), core::mem::transmute_copy(&callbackfunction), core::mem::transmute_copy(&callbackcontext)) {
                Ok(ok__) => {
                    core::ptr::write(eventcookie, core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterEventNotification<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventcookie: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterFactory_Impl::UnregisterEventNotification(this, core::mem::transmute_copy(&eventcookie)).into()
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAdapterList: CreateAdapterList::<Identity, Impl, OFFSET>,
            GetAdapterByLuid: GetAdapterByLuid::<Identity, Impl, OFFSET>,
            IsNotificationTypeSupported: IsNotificationTypeSupported::<Identity, Impl, OFFSET>,
            RegisterEventNotification: RegisterEventNotification::<Identity, Impl, OFFSET>,
            UnregisterEventNotification: UnregisterEventNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXCoreAdapterFactory as windows_core::Interface>::IID
    }
}
pub trait IDXCoreAdapterList_Impl: Sized {
    fn GetAdapter(&self, index: u32, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetAdapterCount(&self) -> u32;
    fn IsStale(&self) -> bool;
    fn GetFactory(&self, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Sort(&self, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> windows_core::Result<()>;
    fn IsAdapterPreferenceSupported(&self, preference: DXCoreAdapterPreference) -> bool;
}
impl windows_core::RuntimeName for IDXCoreAdapterList {}
impl IDXCoreAdapterList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>() -> IDXCoreAdapterList_Vtbl {
        unsafe extern "system" fn GetAdapter<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, riid: *const windows_core::GUID, ppvadapter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterList_Impl::GetAdapter(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn GetAdapterCount<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterList_Impl::GetAdapterCount(this)
        }
        unsafe extern "system" fn IsStale<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterList_Impl::IsStale(this)
        }
        unsafe extern "system" fn GetFactory<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppvfactory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterList_Impl::GetFactory(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvfactory)).into()
        }
        unsafe extern "system" fn Sort<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterList_Impl::Sort(this, core::mem::transmute_copy(&numpreferences), core::mem::transmute_copy(&preferences)).into()
        }
        unsafe extern "system" fn IsAdapterPreferenceSupported<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, preference: DXCoreAdapterPreference) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDXCoreAdapterList_Impl::IsAdapterPreferenceSupported(this, core::mem::transmute_copy(&preference))
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, Impl, OFFSET>,
            GetAdapterCount: GetAdapterCount::<Identity, Impl, OFFSET>,
            IsStale: IsStale::<Identity, Impl, OFFSET>,
            GetFactory: GetFactory::<Identity, Impl, OFFSET>,
            Sort: Sort::<Identity, Impl, OFFSET>,
            IsAdapterPreferenceSupported: IsAdapterPreferenceSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDXCoreAdapterList as windows_core::Interface>::IID
    }
}
