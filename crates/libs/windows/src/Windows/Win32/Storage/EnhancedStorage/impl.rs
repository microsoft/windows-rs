#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACTImpl: Sized {
    fn Authorize();
    fn Unauthorize();
    fn GetAuthorizationState();
    fn GetMatchingVolume();
    fn GetUniqueIdentity();
    fn GetSilos();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageACTVtbl {
        unsafe extern "system" fn Authorize<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Unauthorize<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAuthorizationState<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatchingVolume<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszvolume: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUniqueIdentity<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszidentity: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSilos<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesilos: *mut *mut ::windows::core::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Authorize::<Impl, IMPL_OFFSET>, Unauthorize::<Impl, IMPL_OFFSET>, GetAuthorizationState::<Impl, IMPL_OFFSET>, GetMatchingVolume::<Impl, IMPL_OFFSET>, GetUniqueIdentity::<Impl, IMPL_OFFSET>, GetSilos::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageACT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT2Impl: Sized + IEnhancedStorageACTImpl {
    fn GetDeviceName();
    fn IsDeviceRemovable();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACT2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageACT2Vtbl {
        unsafe extern "system" fn GetDeviceName<Impl: IEnhancedStorageACT2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszdevicename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsDeviceRemovable<Impl: IEnhancedStorageACT2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Authorize::<Impl, IMPL_OFFSET>, Unauthorize::<Impl, IMPL_OFFSET>, GetAuthorizationState::<Impl, IMPL_OFFSET>, GetMatchingVolume::<Impl, IMPL_OFFSET>, GetUniqueIdentity::<Impl, IMPL_OFFSET>, GetSilos::<Impl, IMPL_OFFSET>, GetDeviceName::<Impl, IMPL_OFFSET>, IsDeviceRemovable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageACT2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT3Impl: Sized + IEnhancedStorageACT2Impl + IEnhancedStorageACTImpl {
    fn UnauthorizeEx();
    fn IsQueueFrozen();
    fn GetShellExtSupport();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACT3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageACT3Vtbl {
        unsafe extern "system" fn UnauthorizeEx<Impl: IEnhancedStorageACT3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsQueueFrozen<Impl: IEnhancedStorageACT3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisqueuefrozen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetShellExtSupport<Impl: IEnhancedStorageACT3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshellextsupport: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            Authorize::<Impl, IMPL_OFFSET>,
            Unauthorize::<Impl, IMPL_OFFSET>,
            GetAuthorizationState::<Impl, IMPL_OFFSET>,
            GetMatchingVolume::<Impl, IMPL_OFFSET>,
            GetUniqueIdentity::<Impl, IMPL_OFFSET>,
            GetSilos::<Impl, IMPL_OFFSET>,
            GetDeviceName::<Impl, IMPL_OFFSET>,
            IsDeviceRemovable::<Impl, IMPL_OFFSET>,
            UnauthorizeEx::<Impl, IMPL_OFFSET>,
            IsQueueFrozen::<Impl, IMPL_OFFSET>,
            GetShellExtSupport::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageACT3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation"))]
pub trait IEnhancedStorageSiloImpl: Sized {
    fn GetInfo();
    fn GetActions();
    fn SendCommand();
    fn GetPortableDevice();
    fn GetDevicePath();
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation"))]
impl IEnhancedStorageSiloVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSiloImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageSiloVtbl {
        unsafe extern "system" fn GetInfo<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psiloinfo: *mut SILO_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActions<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesiloactions: *mut *mut ::windows::core::RawPtr, pcenhancedstoragesiloactions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendCommand<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPortableDevice<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiportabledevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDevicePath<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszsilodevicepath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInfo::<Impl, IMPL_OFFSET>, GetActions::<Impl, IMPL_OFFSET>, SendCommand::<Impl, IMPL_OFFSET>, GetPortableDevice::<Impl, IMPL_OFFSET>, GetDevicePath::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageSilo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageSiloActionImpl: Sized {
    fn GetName();
    fn GetDescription();
    fn Invoke();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageSiloActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSiloActionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageSiloActionVtbl {
        unsafe extern "system" fn GetName<Impl: IEnhancedStorageSiloActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactionname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDescription<Impl: IEnhancedStorageSiloActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactiondescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invoke<Impl: IEnhancedStorageSiloActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetName::<Impl, IMPL_OFFSET>, GetDescription::<Impl, IMPL_OFFSET>, Invoke::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageSiloAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumEnhancedStorageACTImpl: Sized {
    fn GetACTs();
    fn GetMatchingACT();
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumEnhancedStorageACTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEnhancedStorageACTImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumEnhancedStorageACTVtbl {
        unsafe extern "system" fn GetACTs<Impl: IEnumEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstorageacts: *mut *mut ::windows::core::RawPtr, pcenhancedstorageacts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMatchingACT<Impl: IEnumEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvolume: super::super::Foundation::PWSTR, ppienhancedstorageact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetACTs::<Impl, IMPL_OFFSET>, GetMatchingACT::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumEnhancedStorageACT as ::windows::core::Interface>::IID
    }
}
