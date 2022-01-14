#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT_Impl: Sized {
    fn Authorize(&mut self, hwndparent: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn Unauthorize(&mut self) -> ::windows::core::Result<()>;
    fn GetAuthorizationState(&mut self) -> ::windows::core::Result<ACT_AUTHORIZATION_STATE>;
    fn GetMatchingVolume(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetUniqueIdentity(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetSilos(&mut self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageACT_Vtbl {
        unsafe extern "system" fn Authorize<Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Authorize(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Unauthorize<Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Unauthorize().into()
        }
        unsafe extern "system" fn GetAuthorizationState<Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthorizationState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingVolume<Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszvolume: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueIdentity<Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszidentity: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUniqueIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszidentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSilos<Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesilos: *mut *mut ::windows::core::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSilos(::core::mem::transmute_copy(&pppienhancedstoragesilos), ::core::mem::transmute_copy(&pcenhancedstoragesilos)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Authorize: Authorize::<Impl, IMPL_OFFSET>,
            Unauthorize: Unauthorize::<Impl, IMPL_OFFSET>,
            GetAuthorizationState: GetAuthorizationState::<Impl, IMPL_OFFSET>,
            GetMatchingVolume: GetMatchingVolume::<Impl, IMPL_OFFSET>,
            GetUniqueIdentity: GetUniqueIdentity::<Impl, IMPL_OFFSET>,
            GetSilos: GetSilos::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageACT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT2_Impl: Sized + IEnhancedStorageACT_Impl {
    fn GetDeviceName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn IsDeviceRemovable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACT2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageACT2_Vtbl {
        unsafe extern "system" fn GetDeviceName<Impl: IEnhancedStorageACT2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszdevicename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeviceRemovable<Impl: IEnhancedStorageACT2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDeviceRemovable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisdeviceremovable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IEnhancedStorageACT_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetDeviceName: GetDeviceName::<Impl, IMPL_OFFSET>,
            IsDeviceRemovable: IsDeviceRemovable::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageACT2 as ::windows::core::Interface>::IID || iid == &<IEnhancedStorageACT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT3_Impl: Sized + IEnhancedStorageACT_Impl + IEnhancedStorageACT2_Impl {
    fn UnauthorizeEx(&mut self, dwflags: u32) -> ::windows::core::Result<()>;
    fn IsQueueFrozen(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetShellExtSupport(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACT3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageACT3_Vtbl {
        unsafe extern "system" fn UnauthorizeEx<Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnauthorizeEx(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn IsQueueFrozen<Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisqueuefrozen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsQueueFrozen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisqueuefrozen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShellExtSupport<Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshellextsupport: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShellExtSupport() {
                ::core::result::Result::Ok(ok__) => {
                    *pshellextsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IEnhancedStorageACT2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UnauthorizeEx: UnauthorizeEx::<Impl, IMPL_OFFSET>,
            IsQueueFrozen: IsQueueFrozen::<Impl, IMPL_OFFSET>,
            GetShellExtSupport: GetShellExtSupport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageACT3 as ::windows::core::Interface>::IID || iid == &<IEnhancedStorageACT as ::windows::core::Interface>::IID || iid == &<IEnhancedStorageACT2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation"))]
pub trait IEnhancedStorageSilo_Impl: Sized {
    fn GetInfo(&mut self) -> ::windows::core::Result<SILO_INFO>;
    fn GetActions(&mut self, pppienhancedstoragesiloactions: *mut *mut ::core::option::Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> ::windows::core::Result<()>;
    fn SendCommand(&mut self, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::core::Result<()>;
    fn GetPortableDevice(&mut self) -> ::windows::core::Result<super::super::Devices::PortableDevices::IPortableDevice>;
    fn GetDevicePath(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(all(feature = "Win32_Devices_PortableDevices", feature = "Win32_Foundation"))]
impl IEnhancedStorageSilo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSilo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageSilo_Vtbl {
        unsafe extern "system" fn GetInfo<Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psiloinfo: *mut SILO_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *psiloinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActions<Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesiloactions: *mut *mut ::windows::core::RawPtr, pcenhancedstoragesiloactions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetActions(::core::mem::transmute_copy(&pppienhancedstoragesiloactions), ::core::mem::transmute_copy(&pcenhancedstoragesiloactions)).into()
        }
        unsafe extern "system" fn SendCommand<Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendCommand(::core::mem::transmute_copy(&command), ::core::mem::transmute_copy(&pbcommandbuffer), ::core::mem::transmute_copy(&cbcommandbuffer), ::core::mem::transmute_copy(&pbresponsebuffer), ::core::mem::transmute_copy(&pcbresponsebuffer)).into()
        }
        unsafe extern "system" fn GetPortableDevice<Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiportabledevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPortableDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiportabledevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePath<Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszsilodevicepath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevicePath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszsilodevicepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInfo: GetInfo::<Impl, IMPL_OFFSET>,
            GetActions: GetActions::<Impl, IMPL_OFFSET>,
            SendCommand: SendCommand::<Impl, IMPL_OFFSET>,
            GetPortableDevice: GetPortableDevice::<Impl, IMPL_OFFSET>,
            GetDevicePath: GetDevicePath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageSilo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageSiloAction_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetDescription(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn Invoke(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageSiloAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSiloAction_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnhancedStorageSiloAction_Vtbl {
        unsafe extern "system" fn GetName<Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactionname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszactionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactiondescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszactiondescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Invoke().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetDescription: GetDescription::<Impl, IMPL_OFFSET>,
            Invoke: Invoke::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageSiloAction as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnumEnhancedStorageACT_Impl: Sized {
    fn GetACTs(&mut self, pppienhancedstorageacts: *mut *mut ::core::option::Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> ::windows::core::Result<()>;
    fn GetMatchingACT(&mut self, szvolume: super::super::Foundation::PWSTR) -> ::windows::core::Result<IEnhancedStorageACT>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnumEnhancedStorageACT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEnhancedStorageACT_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumEnhancedStorageACT_Vtbl {
        unsafe extern "system" fn GetACTs<Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstorageacts: *mut *mut ::windows::core::RawPtr, pcenhancedstorageacts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetACTs(::core::mem::transmute_copy(&pppienhancedstorageacts), ::core::mem::transmute_copy(&pcenhancedstorageacts)).into()
        }
        unsafe extern "system" fn GetMatchingACT<Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvolume: super::super::Foundation::PWSTR, ppienhancedstorageact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingACT(::core::mem::transmute_copy(&szvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppienhancedstorageact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetACTs: GetACTs::<Impl, IMPL_OFFSET>,
            GetMatchingACT: GetMatchingACT::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumEnhancedStorageACT as ::windows::core::Interface>::IID
    }
}
