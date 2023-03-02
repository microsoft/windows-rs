#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`, `\"implement\"`*"]
pub trait IDXCoreAdapter_Impl: Sized {
    fn IsValid(&self) -> bool;
    fn IsAttributeSupported(&self, attributeguid: *const ::windows::core::GUID) -> bool;
    fn IsPropertySupported(&self, property: DXCoreAdapterProperty) -> bool;
    fn GetProperty(&self, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetPropertySize(&self, property: DXCoreAdapterProperty) -> ::windows::core::Result<usize>;
    fn IsQueryStateSupported(&self, property: DXCoreAdapterState) -> bool;
    fn QueryState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsSetStateSupported(&self, property: DXCoreAdapterState) -> bool;
    fn SetState(&self, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetFactory(&self, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDXCoreAdapter {}
impl IDXCoreAdapter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>() -> IDXCoreAdapter_Vtbl {
        unsafe extern "system" fn IsValid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsValid()
        }
        unsafe extern "system" fn IsAttributeSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, attributeguid: *const ::windows::core::GUID) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsAttributeSupported(::core::mem::transmute_copy(&attributeguid))
        }
        unsafe extern "system" fn IsPropertySupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsPropertySupported(::core::mem::transmute_copy(&property))
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: usize, propertydata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetProperty(::core::mem::transmute_copy(&property), ::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&propertydata)).into()
        }
        unsafe extern "system" fn GetPropertySize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterProperty, buffersize: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertySize(::core::mem::transmute_copy(&property)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(buffersize, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsQueryStateSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsQueryStateSupported(::core::mem::transmute_copy(&property))
        }
        unsafe extern "system" fn QueryState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, outputbuffersize: usize, outputbuffer: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryState(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&inputstatedetailssize), ::core::mem::transmute_copy(&inputstatedetails), ::core::mem::transmute_copy(&outputbuffersize), ::core::mem::transmute_copy(&outputbuffer)).into()
        }
        unsafe extern "system" fn IsSetStateSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, property: DXCoreAdapterState) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSetStateSupported(::core::mem::transmute_copy(&property))
        }
        unsafe extern "system" fn SetState<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, state: DXCoreAdapterState, inputstatedetailssize: usize, inputstatedetails: *const ::core::ffi::c_void, inputdatasize: usize, inputdata: *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetState(::core::mem::transmute_copy(&state), ::core::mem::transmute_copy(&inputstatedetailssize), ::core::mem::transmute_copy(&inputstatedetails), ::core::mem::transmute_copy(&inputdatasize), ::core::mem::transmute_copy(&inputdata)).into()
        }
        unsafe extern "system" fn GetFactory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapter_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFactory(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvfactory)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXCoreAdapter as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDXCoreAdapterFactory_Impl: Sized {
    fn CreateAdapterList(&self, numattributes: u32, filterattributes: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetAdapterByLuid(&self, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn IsNotificationTypeSupported(&self, notificationtype: DXCoreNotificationType) -> bool;
    fn RegisterEventNotification(&self, dxcoreobject: ::core::option::Option<&::windows::core::IUnknown>, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<u32>;
    fn UnregisterEventNotification(&self, eventcookie: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IDXCoreAdapterFactory {}
#[cfg(feature = "Win32_Foundation")]
impl IDXCoreAdapterFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>() -> IDXCoreAdapterFactory_Vtbl {
        unsafe extern "system" fn CreateAdapterList<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numattributes: u32, filterattributes: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvadapterlist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAdapterList(::core::mem::transmute_copy(&numattributes), ::core::mem::transmute_copy(&filterattributes), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapterlist)).into()
        }
        unsafe extern "system" fn GetAdapterByLuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, adapterluid: *const super::super::Foundation::LUID, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterByLuid(::core::mem::transmute_copy(&adapterluid), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn IsNotificationTypeSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsNotificationTypeSupported(::core::mem::transmute_copy(&notificationtype))
        }
        unsafe extern "system" fn RegisterEventNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dxcoreobject: *mut ::core::ffi::c_void, notificationtype: DXCoreNotificationType, callbackfunction: PFN_DXCORE_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void, eventcookie: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RegisterEventNotification(::windows::core::from_raw_borrowed(&dxcoreobject), ::core::mem::transmute_copy(&notificationtype), ::core::mem::transmute_copy(&callbackfunction), ::core::mem::transmute_copy(&callbackcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eventcookie, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnregisterEventNotification<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eventcookie: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnregisterEventNotification(::core::mem::transmute_copy(&eventcookie)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateAdapterList: CreateAdapterList::<Identity, Impl, OFFSET>,
            GetAdapterByLuid: GetAdapterByLuid::<Identity, Impl, OFFSET>,
            IsNotificationTypeSupported: IsNotificationTypeSupported::<Identity, Impl, OFFSET>,
            RegisterEventNotification: RegisterEventNotification::<Identity, Impl, OFFSET>,
            UnregisterEventNotification: UnregisterEventNotification::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXCoreAdapterFactory as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Graphics_DXCore\"`, `\"implement\"`*"]
pub trait IDXCoreAdapterList_Impl: Sized {
    fn GetAdapter(&self, index: u32, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn GetAdapterCount(&self) -> u32;
    fn IsStale(&self) -> bool;
    fn GetFactory(&self, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Sort(&self, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::Result<()>;
    fn IsAdapterPreferenceSupported(&self, preference: DXCoreAdapterPreference) -> bool;
}
impl ::windows::core::RuntimeName for IDXCoreAdapterList {}
impl IDXCoreAdapterList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>() -> IDXCoreAdapterList_Vtbl {
        unsafe extern "system" fn GetAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: u32, riid: *const ::windows::core::GUID, ppvadapter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapter(::core::mem::transmute_copy(&index), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvadapter)).into()
        }
        unsafe extern "system" fn GetAdapterCount<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> u32 {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterCount()
        }
        unsafe extern "system" fn IsStale<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsStale()
        }
        unsafe extern "system" fn GetFactory<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvfactory: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFactory(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvfactory)).into()
        }
        unsafe extern "system" fn Sort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, numpreferences: u32, preferences: *const DXCoreAdapterPreference) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Sort(::core::mem::transmute_copy(&numpreferences), ::core::mem::transmute_copy(&preferences)).into()
        }
        unsafe extern "system" fn IsAdapterPreferenceSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IDXCoreAdapterList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, preference: DXCoreAdapterPreference) -> bool {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsAdapterPreferenceSupported(::core::mem::transmute_copy(&preference))
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAdapter: GetAdapter::<Identity, Impl, OFFSET>,
            GetAdapterCount: GetAdapterCount::<Identity, Impl, OFFSET>,
            IsStale: IsStale::<Identity, Impl, OFFSET>,
            GetFactory: GetFactory::<Identity, Impl, OFFSET>,
            Sort: Sort::<Identity, Impl, OFFSET>,
            IsAdapterPreferenceSupported: IsAdapterPreferenceSupported::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDXCoreAdapterList as ::windows::core::ComInterface>::IID
    }
}
