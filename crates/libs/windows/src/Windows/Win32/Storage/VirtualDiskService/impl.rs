pub trait IEnumVdsObjectImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumVdsObjectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumVdsObjectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumVdsObjectVtbl {
        unsafe extern "system" fn Next<Impl: IEnumVdsObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, ppobjectarray: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumVdsObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumVdsObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumVdsObjectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumVdsObject as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsAdminImpl: Sized {
    fn RegisterProvider();
    fn UnregisterProvider();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsAdminVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdminImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsAdminVtbl {
        unsafe extern "system" fn RegisterProvider<Impl: IVdsAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID, providerclsid: ::windows::core::GUID, pwszname: super::super::Foundation::PWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: super::super::Foundation::PWSTR, pwszversion: super::super::Foundation::PWSTR, guidversionid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnregisterProvider<Impl: IVdsAdminImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, providerid: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, RegisterProvider::<Impl, IMPL_OFFSET>, UnregisterProvider::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsAdmin as ::windows::core::Interface>::IID
    }
}
pub trait IVdsAdviseSinkImpl: Sized {
    fn OnNotify();
}
impl IVdsAdviseSinkVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAdviseSinkImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsAdviseSinkVtbl {
        unsafe extern "system" fn OnNotify<Impl: IVdsAdviseSinkImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, OnNotify::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsAdviseSink as ::windows::core::Interface>::IID
    }
}
pub trait IVdsAsyncImpl: Sized {
    fn Cancel();
    fn Wait();
    fn QueryStatus();
}
impl IVdsAsyncVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsAsyncImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsAsyncVtbl {
        unsafe extern "system" fn Cancel<Impl: IVdsAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Wait<Impl: IVdsAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryStatus<Impl: IVdsAsyncImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phrresult: *mut ::windows::core::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, Wait::<Impl, IMPL_OFFSET>, QueryStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsAsync as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsControllerImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn GetPortProperties();
    fn FlushCache();
    fn InvalidateCache();
    fn Reset();
    fn QueryAssociatedLuns();
    fn SetStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsControllerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsControllerVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPortProperties<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FlushCache<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InvalidateCache<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAssociatedLuns<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsControllerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_CONTROLLER_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>, GetSubSystem::<Impl, IMPL_OFFSET>, GetPortProperties::<Impl, IMPL_OFFSET>, FlushCache::<Impl, IMPL_OFFSET>, InvalidateCache::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, QueryAssociatedLuns::<Impl, IMPL_OFFSET>, SetStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsController as ::windows::core::Interface>::IID
    }
}
pub trait IVdsControllerControllerPortImpl: Sized {
    fn QueryControllerPorts();
}
impl IVdsControllerControllerPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerControllerPortImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsControllerControllerPortVtbl {
        unsafe extern "system" fn QueryControllerPorts<Impl: IVdsControllerControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryControllerPorts::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsControllerControllerPort as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsControllerPortImpl: Sized {
    fn GetProperties();
    fn GetController();
    fn QueryAssociatedLuns();
    fn Reset();
    fn SetStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsControllerPortVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsControllerPortImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsControllerPortVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportprop: *mut VDS_PORT_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetController<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcontroller: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAssociatedLuns<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsControllerPortImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_PORT_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>, GetController::<Impl, IMPL_OFFSET>, QueryAssociatedLuns::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, SetStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsControllerPort as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDriveImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn QueryExtents();
    fn SetFlags();
    fn ClearFlags();
    fn SetStatus();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsDriveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDriveImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsDriveVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryExtents<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFlags<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearFlags<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsDriveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_DRIVE_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>, GetSubSystem::<Impl, IMPL_OFFSET>, QueryExtents::<Impl, IMPL_OFFSET>, SetFlags::<Impl, IMPL_OFFSET>, ClearFlags::<Impl, IMPL_OFFSET>, SetStatus::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsDrive as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsDrive2Impl: Sized {
    fn GetProperties2();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsDrive2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsDrive2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsDrive2Vtbl {
        unsafe extern "system" fn GetProperties2<Impl: IVdsDrive2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperties2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsDrive2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderImpl: Sized {
    fn QuerySubSystems();
    fn Reenumerate();
    fn Refresh();
}
impl IVdsHwProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderVtbl {
        unsafe extern "system" fn QuerySubSystems<Impl: IVdsHwProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reenumerate<Impl: IVdsHwProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Refresh<Impl: IVdsHwProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QuerySubSystems::<Impl, IMPL_OFFSET>, Reenumerate::<Impl, IMPL_OFFSET>, Refresh::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderPrivateImpl: Sized {
    fn QueryIfCreatedLun();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsHwProviderPrivateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderPrivateVtbl {
        unsafe extern "system" fn QueryIfCreatedLun<Impl: IVdsHwProviderPrivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszdevicepath: super::super::Foundation::PWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryIfCreatedLun::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivate as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderPrivateMpioImpl: Sized {
    fn SetAllPathStatusesFromHbaPort();
}
impl IVdsHwProviderPrivateMpioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderPrivateMpioImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderPrivateMpioVtbl {
        unsafe extern "system" fn SetAllPathStatusesFromHbaPort<Impl: IVdsHwProviderPrivateMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetAllPathStatusesFromHbaPort::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderPrivateMpio as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsHwProviderStoragePoolsImpl: Sized {
    fn QueryStoragePools();
    fn CreateLunInStoragePool();
    fn QueryMaxLunCreateSizeInStoragePool();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsHwProviderStoragePoolsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderStoragePoolsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderStoragePoolsVtbl {
        unsafe extern "system" fn QueryStoragePools<Impl: IVdsHwProviderStoragePoolsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLunInStoragePool<Impl: IVdsHwProviderStoragePoolsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows::core::GUID, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryMaxLunCreateSizeInStoragePool<Impl: IVdsHwProviderStoragePoolsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, storagepoolid: ::windows::core::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryStoragePools::<Impl, IMPL_OFFSET>, CreateLunInStoragePool::<Impl, IMPL_OFFSET>, QueryMaxLunCreateSizeInStoragePool::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderStoragePools as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderTypeImpl: Sized {
    fn GetProviderType();
}
impl IVdsHwProviderTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderTypeVtbl {
        unsafe extern "system" fn GetProviderType<Impl: IVdsHwProviderTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProviderType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderType as ::windows::core::Interface>::IID
    }
}
pub trait IVdsHwProviderType2Impl: Sized {
    fn GetProviderType2();
}
impl IVdsHwProviderType2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsHwProviderType2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsHwProviderType2Vtbl {
        unsafe extern "system" fn GetProviderType2<Impl: IVdsHwProviderType2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProviderType2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsHwProviderType2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsIscsiPortalImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn QueryAssociatedPortalGroups();
    fn SetStatus();
    fn SetIpsecTunnelAddress();
    fn GetIpsecSecurity();
    fn SetIpsecSecurity();
}
impl IVdsIscsiPortalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsIscsiPortalVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAssociatedPortalGroups<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIpsecTunnelAddress<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIpsecSecurity<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIpsecSecurity<Impl: IVdsIscsiPortalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>, GetSubSystem::<Impl, IMPL_OFFSET>, QueryAssociatedPortalGroups::<Impl, IMPL_OFFSET>, SetStatus::<Impl, IMPL_OFFSET>, SetIpsecTunnelAddress::<Impl, IMPL_OFFSET>, GetIpsecSecurity::<Impl, IMPL_OFFSET>, SetIpsecSecurity::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsIscsiPortal as ::windows::core::Interface>::IID
    }
}
pub trait IVdsIscsiPortalGroupImpl: Sized {
    fn GetProperties();
    fn GetTarget();
    fn QueryAssociatedPortals();
    fn AddPortal();
    fn RemovePortal();
    fn Delete();
}
impl IVdsIscsiPortalGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiPortalGroupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsIscsiPortalGroupVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTarget<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptarget: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAssociatedPortals<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPortal<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemovePortal<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, portalid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IVdsIscsiPortalGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>, GetTarget::<Impl, IMPL_OFFSET>, QueryAssociatedPortals::<Impl, IMPL_OFFSET>, AddPortal::<Impl, IMPL_OFFSET>, RemovePortal::<Impl, IMPL_OFFSET>, Delete::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsIscsiPortalGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsIscsiTargetImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn QueryPortalGroups();
    fn QueryAssociatedLuns();
    fn CreatePortalGroup();
    fn Delete();
    fn SetFriendlyName();
    fn SetSharedSecret();
    fn RememberInitiatorSharedSecret();
    fn GetConnectedInitiators();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsIscsiTargetVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsIscsiTargetImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsIscsiTargetVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryPortalGroups<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAssociatedLuns<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreatePortalGroup<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFriendlyName<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSharedSecret<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RememberInitiatorSharedSecret<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszinitiatorname: super::super::Foundation::PWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetConnectedInitiators<Impl: IVdsIscsiTargetImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppwszinitiatorlist: *mut *mut super::super::Foundation::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetProperties::<Impl, IMPL_OFFSET>,
            GetSubSystem::<Impl, IMPL_OFFSET>,
            QueryPortalGroups::<Impl, IMPL_OFFSET>,
            QueryAssociatedLuns::<Impl, IMPL_OFFSET>,
            CreatePortalGroup::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            SetFriendlyName::<Impl, IMPL_OFFSET>,
            SetSharedSecret::<Impl, IMPL_OFFSET>,
            RememberInitiatorSharedSecret::<Impl, IMPL_OFFSET>,
            GetConnectedInitiators::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsIscsiTarget as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunImpl: Sized {
    fn GetProperties();
    fn GetSubSystem();
    fn GetIdentificationData();
    fn QueryActiveControllers();
    fn Extend();
    fn Shrink();
    fn QueryPlexes();
    fn AddPlex();
    fn RemovePlex();
    fn Recover();
    fn SetMask();
    fn Delete();
    fn AssociateControllers();
    fn QueryHints();
    fn ApplyHints();
    fn SetStatus();
    fn QueryMaxLunExtendSize();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plunprop: *mut VDS_LUN_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSubSystem<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsubsystem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetIdentificationData<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryActiveControllers<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Extend<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Shrink<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ullnumberofbytestoremove: u64, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryPlexes<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddPlex<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lunid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemovePlex<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plexid: ::windows::core::GUID, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recover<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMask<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszunmaskinglist: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AssociateControllers<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrolleridarray: *const ::windows::core::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::core::GUID, lnumberofinactivecontrollers: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryHints<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyHints<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_LUN_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryMaxLunExtendSize<Impl: IVdsLunImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetProperties::<Impl, IMPL_OFFSET>,
            GetSubSystem::<Impl, IMPL_OFFSET>,
            GetIdentificationData::<Impl, IMPL_OFFSET>,
            QueryActiveControllers::<Impl, IMPL_OFFSET>,
            Extend::<Impl, IMPL_OFFSET>,
            Shrink::<Impl, IMPL_OFFSET>,
            QueryPlexes::<Impl, IMPL_OFFSET>,
            AddPlex::<Impl, IMPL_OFFSET>,
            RemovePlex::<Impl, IMPL_OFFSET>,
            Recover::<Impl, IMPL_OFFSET>,
            SetMask::<Impl, IMPL_OFFSET>,
            Delete::<Impl, IMPL_OFFSET>,
            AssociateControllers::<Impl, IMPL_OFFSET>,
            QueryHints::<Impl, IMPL_OFFSET>,
            ApplyHints::<Impl, IMPL_OFFSET>,
            SetStatus::<Impl, IMPL_OFFSET>,
            QueryMaxLunExtendSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLun as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLun2Impl: Sized {
    fn QueryHints2();
    fn ApplyHints2();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLun2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLun2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLun2Vtbl {
        unsafe extern "system" fn QueryHints2<Impl: IVdsLun2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *mut VDS_HINTS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyHints2<Impl: IVdsLun2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints2: *const VDS_HINTS2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryHints2::<Impl, IMPL_OFFSET>, ApplyHints2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLun2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunControllerPortsImpl: Sized {
    fn AssociateControllerPorts();
    fn QueryActiveControllerPorts();
}
impl IVdsLunControllerPortsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunControllerPortsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunControllerPortsVtbl {
        unsafe extern "system" fn AssociateControllerPorts<Impl: IVdsLunControllerPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::core::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryActiveControllerPorts<Impl: IVdsLunControllerPortsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AssociateControllerPorts::<Impl, IMPL_OFFSET>, QueryActiveControllerPorts::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunControllerPorts as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunIscsiImpl: Sized {
    fn AssociateTargets();
    fn QueryAssociatedTargets();
}
impl IVdsLunIscsiVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunIscsiImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunIscsiVtbl {
        unsafe extern "system" fn AssociateTargets<Impl: IVdsLunIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ptargetidarray: *const ::windows::core::GUID, lnumberoftargets: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAssociatedTargets<Impl: IVdsLunIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AssociateTargets::<Impl, IMPL_OFFSET>, QueryAssociatedTargets::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunIscsi as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunMpioImpl: Sized {
    fn GetPathInfo();
    fn GetLoadBalancePolicy();
    fn SetLoadBalancePolicy();
    fn GetSupportedLbPolicies();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunMpioVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunMpioImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunMpioVtbl {
        unsafe extern "system" fn GetPathInfo<Impl: IVdsLunMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLoadBalancePolicy<Impl: IVdsLunMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLoadBalancePolicy<Impl: IVdsLunMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedLbPolicies<Impl: IVdsLunMpioImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullbflags: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetPathInfo::<Impl, IMPL_OFFSET>, GetLoadBalancePolicy::<Impl, IMPL_OFFSET>, SetLoadBalancePolicy::<Impl, IMPL_OFFSET>, GetSupportedLbPolicies::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunMpio as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunNamingImpl: Sized {
    fn SetFriendlyName();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunNamingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNamingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunNamingVtbl {
        unsafe extern "system" fn SetFriendlyName<Impl: IVdsLunNamingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetFriendlyName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunNaming as ::windows::core::Interface>::IID
    }
}
pub trait IVdsLunNumberImpl: Sized {
    fn GetLunNumber();
}
impl IVdsLunNumberVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunNumberImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunNumberVtbl {
        unsafe extern "system" fn GetLunNumber<Impl: IVdsLunNumberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pullunnumber: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetLunNumber::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunNumber as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsLunPlexImpl: Sized {
    fn GetProperties();
    fn GetLun();
    fn QueryExtents();
    fn QueryHints();
    fn ApplyHints();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsLunPlexVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsLunPlexImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsLunPlexVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLun<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pplun: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryExtents<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryHints<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *mut VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyHints<Impl: IVdsLunPlexImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phints: *const VDS_HINTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>, GetLun::<Impl, IMPL_OFFSET>, QueryExtents::<Impl, IMPL_OFFSET>, QueryHints::<Impl, IMPL_OFFSET>, ApplyHints::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsLunPlex as ::windows::core::Interface>::IID
    }
}
pub trait IVdsMaintenanceImpl: Sized {
    fn StartMaintenance();
    fn StopMaintenance();
    fn PulseMaintenance();
}
impl IVdsMaintenanceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsMaintenanceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsMaintenanceVtbl {
        unsafe extern "system" fn StartMaintenance<Impl: IVdsMaintenanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StopMaintenance<Impl: IVdsMaintenanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PulseMaintenance<Impl: IVdsMaintenanceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, StartMaintenance::<Impl, IMPL_OFFSET>, StopMaintenance::<Impl, IMPL_OFFSET>, PulseMaintenance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsMaintenance as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsProviderImpl: Sized {
    fn GetProperties();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsProviderVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsProviderPrivateImpl: Sized {
    fn GetObject();
    fn OnLoad();
    fn OnUnload();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsProviderPrivateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderPrivateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsProviderPrivateVtbl {
        unsafe extern "system" fn GetObject<Impl: IVdsProviderPrivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, objectid: ::windows::core::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnLoad<Impl: IVdsProviderPrivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszmachinename: super::super::Foundation::PWSTR, pcallbackobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OnUnload<Impl: IVdsProviderPrivateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bforceunload: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetObject::<Impl, IMPL_OFFSET>, OnLoad::<Impl, IMPL_OFFSET>, OnUnload::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsProviderPrivate as ::windows::core::Interface>::IID
    }
}
pub trait IVdsProviderSupportImpl: Sized {
    fn GetVersionSupport();
}
impl IVdsProviderSupportVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsProviderSupportImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsProviderSupportVtbl {
        unsafe extern "system" fn GetVersionSupport<Impl: IVdsProviderSupportImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulversionsupport: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetVersionSupport::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsProviderSupport as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsStoragePoolImpl: Sized {
    fn GetProvider();
    fn GetProperties();
    fn GetAttributes();
    fn QueryDriveExtents();
    fn QueryAllocatedLuns();
    fn QueryAllocatedStoragePools();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsStoragePoolVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsStoragePoolImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsStoragePoolVtbl {
        unsafe extern "system" fn GetProvider<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProperties<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAttributes<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryDriveExtents<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAllocatedLuns<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryAllocatedStoragePools<Impl: IVdsStoragePoolImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProvider::<Impl, IMPL_OFFSET>, GetProperties::<Impl, IMPL_OFFSET>, GetAttributes::<Impl, IMPL_OFFSET>, QueryDriveExtents::<Impl, IMPL_OFFSET>, QueryAllocatedLuns::<Impl, IMPL_OFFSET>, QueryAllocatedStoragePools::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsStoragePool as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystemImpl: Sized {
    fn GetProperties();
    fn GetProvider();
    fn QueryControllers();
    fn QueryLuns();
    fn QueryDrives();
    fn GetDrive();
    fn Reenumerate();
    fn SetControllerStatus();
    fn CreateLun();
    fn ReplaceDrive();
    fn SetStatus();
    fn QueryMaxLunCreateSize();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystemVtbl {
        unsafe extern "system" fn GetProperties<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetProvider<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppprovider: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryControllers<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryLuns<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryDrives<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDrive<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reenumerate<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetControllerStatus<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ponlinecontrolleridarray: *const ::windows::core::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::core::GUID, lnumberofofflinecontrollers: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLun<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints: *const VDS_HINTS, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReplaceDrive<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, drivetobereplaced: ::windows::core::GUID, replacementdrive: ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStatus<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryMaxLunCreateSize<Impl: IVdsSubSystemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetProperties::<Impl, IMPL_OFFSET>,
            GetProvider::<Impl, IMPL_OFFSET>,
            QueryControllers::<Impl, IMPL_OFFSET>,
            QueryLuns::<Impl, IMPL_OFFSET>,
            QueryDrives::<Impl, IMPL_OFFSET>,
            GetDrive::<Impl, IMPL_OFFSET>,
            Reenumerate::<Impl, IMPL_OFFSET>,
            SetControllerStatus::<Impl, IMPL_OFFSET>,
            CreateLun::<Impl, IMPL_OFFSET>,
            ReplaceDrive::<Impl, IMPL_OFFSET>,
            SetStatus::<Impl, IMPL_OFFSET>,
            QueryMaxLunCreateSize::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystem as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystem2Impl: Sized {
    fn GetProperties2();
    fn GetDrive2();
    fn CreateLun2();
    fn QueryMaxLunCreateSize2();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystem2Vtbl {
        unsafe extern "system" fn GetProperties2<Impl: IVdsSubSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDrive2<Impl: IVdsSubSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateLun2<Impl: IVdsSubSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryMaxLunCreateSize2<Impl: IVdsSubSystem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::core::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetProperties2::<Impl, IMPL_OFFSET>, GetDrive2::<Impl, IMPL_OFFSET>, CreateLun2::<Impl, IMPL_OFFSET>, QueryMaxLunCreateSize2::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystem2 as ::windows::core::Interface>::IID
    }
}
pub trait IVdsSubSystemInterconnectImpl: Sized {
    fn GetSupportedInterconnects();
}
impl IVdsSubSystemInterconnectVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemInterconnectImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystemInterconnectVtbl {
        unsafe extern "system" fn GetSupportedInterconnects<Impl: IVdsSubSystemInterconnectImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulsupportedinterconnectsflag: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSupportedInterconnects::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystemInterconnect as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystemIscsiImpl: Sized {
    fn QueryTargets();
    fn QueryPortals();
    fn CreateTarget();
    fn SetIpsecGroupPresharedKey();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystemIscsiVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemIscsiImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystemIscsiVtbl {
        unsafe extern "system" fn QueryTargets<Impl: IVdsSubSystemIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryPortals<Impl: IVdsSubSystemIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateTarget<Impl: IVdsSubSystemIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwsziscsiname: super::super::Foundation::PWSTR, pwszfriendlyname: super::super::Foundation::PWSTR, ppasync: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIpsecGroupPresharedKey<Impl: IVdsSubSystemIscsiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryTargets::<Impl, IMPL_OFFSET>, QueryPortals::<Impl, IMPL_OFFSET>, CreateTarget::<Impl, IMPL_OFFSET>, SetIpsecGroupPresharedKey::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystemIscsi as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IVdsSubSystemNamingImpl: Sized {
    fn SetFriendlyName();
}
#[cfg(feature = "Win32_Foundation")]
impl IVdsSubSystemNamingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVdsSubSystemNamingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVdsSubSystemNamingVtbl {
        unsafe extern "system" fn SetFriendlyName<Impl: IVdsSubSystemNamingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SetFriendlyName::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVdsSubSystemNaming as ::windows::core::Interface>::IID
    }
}
