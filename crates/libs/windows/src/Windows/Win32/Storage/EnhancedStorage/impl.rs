pub trait IEnhancedStorageACT_Impl: Sized {
    fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows::core::Result<()>;
    fn Unauthorize(&self) -> ::windows::core::Result<()>;
    fn GetAuthorizationState(&self) -> ::windows::core::Result<ACT_AUTHORIZATION_STATE>;
    fn GetMatchingVolume(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetUniqueIdentity(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows::core::Result<()>;
}
impl IEnhancedStorageACT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>() -> IEnhancedStorageACT_Vtbl {
        unsafe extern "system" fn Authorize<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Authorize(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Unauthorize<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Unauthorize().into()
        }
        unsafe extern "system" fn GetAuthorizationState<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetAuthorizationState() {
                ::core::result::Result::Ok(ok__) => {
                    *pstate = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingVolume<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszvolume: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMatchingVolume() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszvolume = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueIdentity<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszidentity: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetUniqueIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszidentity = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSilos<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesilos: *mut *mut ::windows::core::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSilos(::core::mem::transmute_copy(&pppienhancedstoragesilos), ::core::mem::transmute_copy(&pcenhancedstoragesilos)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Authorize: Authorize::<Identity, Impl, OFFSET>,
            Unauthorize: Unauthorize::<Identity, Impl, OFFSET>,
            GetAuthorizationState: GetAuthorizationState::<Identity, Impl, OFFSET>,
            GetMatchingVolume: GetMatchingVolume::<Identity, Impl, OFFSET>,
            GetUniqueIdentity: GetUniqueIdentity::<Identity, Impl, OFFSET>,
            GetSilos: GetSilos::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageACT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT2_Impl: Sized + IEnhancedStorageACT_Impl {
    fn GetDeviceName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn IsDeviceRemovable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACT2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT2_Impl, const OFFSET: isize>() -> IEnhancedStorageACT2_Vtbl {
        unsafe extern "system" fn GetDeviceName<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszdevicename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszdevicename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeviceRemovable<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsDeviceRemovable() {
                ::core::result::Result::Ok(ok__) => {
                    *pisdeviceremovable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IEnhancedStorageACT_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDeviceName: GetDeviceName::<Identity, Impl, OFFSET>,
            IsDeviceRemovable: IsDeviceRemovable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageACT2 as ::windows::core::Interface>::IID || iid == &<IEnhancedStorageACT as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT3_Impl: Sized + IEnhancedStorageACT_Impl + IEnhancedStorageACT2_Impl {
    fn UnauthorizeEx(&self, dwflags: u32) -> ::windows::core::Result<()>;
    fn IsQueueFrozen(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetShellExtSupport(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACT3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>() -> IEnhancedStorageACT3_Vtbl {
        unsafe extern "system" fn UnauthorizeEx<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UnauthorizeEx(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn IsQueueFrozen<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisqueuefrozen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsQueueFrozen() {
                ::core::result::Result::Ok(ok__) => {
                    *pisqueuefrozen = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShellExtSupport<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshellextsupport: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetShellExtSupport() {
                ::core::result::Result::Ok(ok__) => {
                    *pshellextsupport = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IEnhancedStorageACT2_Vtbl::new::<Identity, Impl, OFFSET>(),
            UnauthorizeEx: UnauthorizeEx::<Identity, Impl, OFFSET>,
            IsQueueFrozen: IsQueueFrozen::<Identity, Impl, OFFSET>,
            GetShellExtSupport: GetShellExtSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageACT3 as ::windows::core::Interface>::IID || iid == &<IEnhancedStorageACT as ::windows::core::Interface>::IID || iid == &<IEnhancedStorageACT2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub trait IEnhancedStorageSilo_Impl: Sized {
    fn GetInfo(&self) -> ::windows::core::Result<SILO_INFO>;
    fn GetActions(&self, pppienhancedstoragesiloactions: *mut *mut ::core::option::Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> ::windows::core::Result<()>;
    fn SendCommand(&self, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::core::Result<()>;
    fn GetPortableDevice(&self) -> ::windows::core::Result<super::super::Devices::PortableDevices::IPortableDevice>;
    fn GetDevicePath(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl IEnhancedStorageSilo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>() -> IEnhancedStorageSilo_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psiloinfo: *mut SILO_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *psiloinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActions<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesiloactions: *mut *mut ::windows::core::RawPtr, pcenhancedstoragesiloactions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetActions(::core::mem::transmute_copy(&pppienhancedstoragesiloactions), ::core::mem::transmute_copy(&pcenhancedstoragesiloactions)).into()
        }
        unsafe extern "system" fn SendCommand<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SendCommand(::core::mem::transmute_copy(&command), ::core::mem::transmute_copy(&pbcommandbuffer), ::core::mem::transmute_copy(&cbcommandbuffer), ::core::mem::transmute_copy(&pbresponsebuffer), ::core::mem::transmute_copy(&pcbresponsebuffer)).into()
        }
        unsafe extern "system" fn GetPortableDevice<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiportabledevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetPortableDevice() {
                ::core::result::Result::Ok(ok__) => {
                    *ppiportabledevice = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePath<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszsilodevicepath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDevicePath() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszsilodevicepath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetActions: GetActions::<Identity, Impl, OFFSET>,
            SendCommand: SendCommand::<Identity, Impl, OFFSET>,
            GetPortableDevice: GetPortableDevice::<Identity, Impl, OFFSET>,
            GetDevicePath: GetDevicePath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageSilo as ::windows::core::Interface>::IID
    }
}
pub trait IEnhancedStorageSiloAction_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn Invoke(&self) -> ::windows::core::Result<()>;
}
impl IEnhancedStorageSiloAction_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>() -> IEnhancedStorageSiloAction_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactionname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszactionname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactiondescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    *ppwszactiondescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Invoke().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnhancedStorageSiloAction as ::windows::core::Interface>::IID
    }
}
pub trait IEnumEnhancedStorageACT_Impl: Sized {
    fn GetACTs(&self, pppienhancedstorageacts: *mut *mut ::core::option::Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> ::windows::core::Result<()>;
    fn GetMatchingACT(&self, szvolume: &::windows::core::PCWSTR) -> ::windows::core::Result<IEnhancedStorageACT>;
}
impl IEnumEnhancedStorageACT_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: isize>() -> IEnumEnhancedStorageACT_Vtbl {
        unsafe extern "system" fn GetACTs<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstorageacts: *mut *mut ::windows::core::RawPtr, pcenhancedstorageacts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetACTs(::core::mem::transmute_copy(&pppienhancedstorageacts), ::core::mem::transmute_copy(&pcenhancedstorageacts)).into()
        }
        unsafe extern "system" fn GetMatchingACT<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvolume: ::windows::core::PCWSTR, ppienhancedstorageact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetMatchingACT(::core::mem::transmute(&szvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppienhancedstorageact = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetACTs: GetACTs::<Identity, Impl, OFFSET>,
            GetMatchingACT: GetMatchingACT::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumEnhancedStorageACT as ::windows::core::Interface>::IID
    }
}
