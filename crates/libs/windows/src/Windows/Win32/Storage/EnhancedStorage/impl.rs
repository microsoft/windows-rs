#[doc = "*Required features: `\"Win32_Storage_EnhancedStorage\"`, `\"implement\"`*"]
pub trait IEnhancedStorageACT_Impl: Sized {
    fn Authorize(&self, hwndparent: u32, dwflags: u32) -> ::windows_core::Result<()>;
    fn Unauthorize(&self) -> ::windows_core::Result<()>;
    fn GetAuthorizationState(&self) -> ::windows_core::Result<ACT_AUTHORIZATION_STATE>;
    fn GetMatchingVolume(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetUniqueIdentity(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetSilos(&self, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnhancedStorageACT {}
impl IEnhancedStorageACT_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>() -> IEnhancedStorageACT_Vtbl {
        unsafe extern "system" fn Authorize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: u32, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Authorize(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn Unauthorize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Unauthorize().into()
        }
        unsafe extern "system" fn GetAuthorizationState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAuthorizationState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstate, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingVolume<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszvolume: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingVolume() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszvolume, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueIdentity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszidentity: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUniqueIdentity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszidentity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSilos<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesilos: *mut *mut ::core::option::Option<IEnhancedStorageSilo>, pcenhancedstoragesilos: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSilos(::core::mem::transmute_copy(&pppienhancedstoragesilos), ::core::mem::transmute_copy(&pcenhancedstoragesilos)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Authorize: Authorize::<Identity, Impl, OFFSET>,
            Unauthorize: Unauthorize::<Identity, Impl, OFFSET>,
            GetAuthorizationState: GetAuthorizationState::<Identity, Impl, OFFSET>,
            GetMatchingVolume: GetMatchingVolume::<Identity, Impl, OFFSET>,
            GetUniqueIdentity: GetUniqueIdentity::<Identity, Impl, OFFSET>,
            GetSilos: GetSilos::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageACT as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_EnhancedStorage\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT2_Impl: Sized + IEnhancedStorageACT_Impl {
    fn GetDeviceName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn IsDeviceRemovable(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IEnhancedStorageACT2 {}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACT2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT2_Impl, const OFFSET: isize>() -> IEnhancedStorageACT2_Vtbl {
        unsafe extern "system" fn GetDeviceName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszdevicename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszdevicename, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeviceRemovable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDeviceRemovable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisdeviceremovable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IEnhancedStorageACT_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetDeviceName: GetDeviceName::<Identity, Impl, OFFSET>,
            IsDeviceRemovable: IsDeviceRemovable::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageACT2 as ::windows_core::ComInterface>::IID || iid == &<IEnhancedStorageACT as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_EnhancedStorage\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEnhancedStorageACT3_Impl: Sized + IEnhancedStorageACT2_Impl {
    fn UnauthorizeEx(&self, dwflags: u32) -> ::windows_core::Result<()>;
    fn IsQueueFrozen(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
    fn GetShellExtSupport(&self) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IEnhancedStorageACT3 {}
#[cfg(feature = "Win32_Foundation")]
impl IEnhancedStorageACT3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>() -> IEnhancedStorageACT3_Vtbl {
        unsafe extern "system" fn UnauthorizeEx<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnauthorizeEx(::core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn IsQueueFrozen<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisqueuefrozen: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsQueueFrozen() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pisqueuefrozen, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShellExtSupport<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageACT3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshellextsupport: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetShellExtSupport() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pshellextsupport, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IEnhancedStorageACT2_Vtbl::new::<Identity, Impl, OFFSET>(),
            UnauthorizeEx: UnauthorizeEx::<Identity, Impl, OFFSET>,
            IsQueueFrozen: IsQueueFrozen::<Identity, Impl, OFFSET>,
            GetShellExtSupport: GetShellExtSupport::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageACT3 as ::windows_core::ComInterface>::IID || iid == &<IEnhancedStorageACT as ::windows_core::ComInterface>::IID || iid == &<IEnhancedStorageACT2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_EnhancedStorage\"`, `\"Win32_Devices_PortableDevices\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Devices_PortableDevices")]
pub trait IEnhancedStorageSilo_Impl: Sized {
    fn GetInfo(&self) -> ::windows_core::Result<SILO_INFO>;
    fn GetActions(&self, pppienhancedstoragesiloactions: *mut *mut ::core::option::Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> ::windows_core::Result<()>;
    fn SendCommand(&self, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows_core::Result<()>;
    fn GetPortableDevice(&self) -> ::windows_core::Result<super::super::Devices::PortableDevices::IPortableDevice>;
    fn GetDevicePath(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl ::windows_core::RuntimeName for IEnhancedStorageSilo {}
#[cfg(feature = "Win32_Devices_PortableDevices")]
impl IEnhancedStorageSilo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>() -> IEnhancedStorageSilo_Vtbl {
        unsafe extern "system" fn GetInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psiloinfo: *mut SILO_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psiloinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesiloactions: *mut *mut ::core::option::Option<IEnhancedStorageSiloAction>, pcenhancedstoragesiloactions: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetActions(::core::mem::transmute_copy(&pppienhancedstoragesiloactions), ::core::mem::transmute_copy(&pcenhancedstoragesiloactions)).into()
        }
        unsafe extern "system" fn SendCommand<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendCommand(::core::mem::transmute_copy(&command), ::core::mem::transmute_copy(&pbcommandbuffer), ::core::mem::transmute_copy(&cbcommandbuffer), ::core::mem::transmute_copy(&pbresponsebuffer), ::core::mem::transmute_copy(&pcbresponsebuffer)).into()
        }
        unsafe extern "system" fn GetPortableDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiportabledevice: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPortableDevice() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppiportabledevice, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSilo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszsilodevicepath: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDevicePath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszsilodevicepath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInfo: GetInfo::<Identity, Impl, OFFSET>,
            GetActions: GetActions::<Identity, Impl, OFFSET>,
            SendCommand: SendCommand::<Identity, Impl, OFFSET>,
            GetPortableDevice: GetPortableDevice::<Identity, Impl, OFFSET>,
            GetDevicePath: GetDevicePath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageSilo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_EnhancedStorage\"`, `\"implement\"`*"]
pub trait IEnhancedStorageSiloAction_Impl: Sized {
    fn GetName(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn GetDescription(&self) -> ::windows_core::Result<::windows_core::PWSTR>;
    fn Invoke(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnhancedStorageSiloAction {}
impl IEnhancedStorageSiloAction_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>() -> IEnhancedStorageSiloAction_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactionname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszactionname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactiondescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppwszactiondescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnhancedStorageSiloAction_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Invoke().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            Invoke: Invoke::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnhancedStorageSiloAction as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_EnhancedStorage\"`, `\"implement\"`*"]
pub trait IEnumEnhancedStorageACT_Impl: Sized {
    fn GetACTs(&self, pppienhancedstorageacts: *mut *mut ::core::option::Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> ::windows_core::Result<()>;
    fn GetMatchingACT(&self, szvolume: &::windows_core::PCWSTR) -> ::windows_core::Result<IEnhancedStorageACT>;
}
impl ::windows_core::RuntimeName for IEnumEnhancedStorageACT {}
impl IEnumEnhancedStorageACT_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: isize>() -> IEnumEnhancedStorageACT_Vtbl {
        unsafe extern "system" fn GetACTs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstorageacts: *mut *mut ::core::option::Option<IEnhancedStorageACT>, pcenhancedstorageacts: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetACTs(::core::mem::transmute_copy(&pppienhancedstorageacts), ::core::mem::transmute_copy(&pcenhancedstorageacts)).into()
        }
        unsafe extern "system" fn GetMatchingACT<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumEnhancedStorageACT_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvolume: ::windows_core::PCWSTR, ppienhancedstorageact: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMatchingACT(::core::mem::transmute(&szvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienhancedstorageact, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetACTs: GetACTs::<Identity, Impl, OFFSET>,
            GetMatchingACT: GetMatchingACT::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumEnhancedStorageACT as ::windows_core::ComInterface>::IID
    }
}
