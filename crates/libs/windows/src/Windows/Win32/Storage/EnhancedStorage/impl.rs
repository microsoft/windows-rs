pub trait IEnhancedStorageACTImpl: Sized {
    fn Authorize();
    fn Unauthorize();
    fn GetAuthorizationState();
    fn GetMatchingVolume();
    fn GetUniqueIdentity();
    fn GetSilos();
}
impl ::windows::core::RuntimeName for IEnhancedStorageACT {
    const NAME: &'static str = "Windows.Win32.Storage.EnhancedStorage.IEnhancedStorageACT";
}
impl IEnhancedStorageACTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACTImpl, const OFFSET: isize>() -> IEnhancedStorageACTVtbl {
        unsafe extern "system" fn Authorize<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: u32, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Authorize(hwndparent, dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Unauthorize<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Unauthorize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAuthorizationState<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstate: *mut ACT_AUTHORIZATION_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAuthorizationState(::core::mem::transmute_copy(&pstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingVolume<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszvolume: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingVolume(::core::mem::transmute_copy(&ppwszvolume)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUniqueIdentity<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszidentity: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUniqueIdentity(::core::mem::transmute_copy(&ppwszidentity)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSilos<Impl: IEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesilos: *mut *mut ::windows::core::RawPtr, pcenhancedstoragesilos: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSilos(::core::mem::transmute_copy(&pppienhancedstoragesilos), ::core::mem::transmute_copy(&pcenhancedstoragesilos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnhancedStorageACT>, ::windows::core::GetTrustLevel, Authorize::<Impl, OFFSET>, Unauthorize::<Impl, OFFSET>, GetAuthorizationState::<Impl, OFFSET>, GetMatchingVolume::<Impl, OFFSET>, GetUniqueIdentity::<Impl, OFFSET>, GetSilos::<Impl, OFFSET>)
    }
}
pub trait IEnhancedStorageACT2Impl: Sized + IEnhancedStorageACTImpl {
    fn GetDeviceName();
    fn IsDeviceRemovable();
}
impl ::windows::core::RuntimeName for IEnhancedStorageACT2 {
    const NAME: &'static str = "Windows.Win32.Storage.EnhancedStorage.IEnhancedStorageACT2";
}
impl IEnhancedStorageACT2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT2Impl, const OFFSET: isize>() -> IEnhancedStorageACT2Vtbl {
        unsafe extern "system" fn GetDeviceName<Impl: IEnhancedStorageACT2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszdevicename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceName(::core::mem::transmute_copy(&ppwszdevicename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDeviceRemovable<Impl: IEnhancedStorageACT2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisdeviceremovable: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDeviceRemovable(::core::mem::transmute_copy(&pisdeviceremovable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnhancedStorageACT2>, ::windows::core::GetTrustLevel, GetDeviceName::<Impl, OFFSET>, IsDeviceRemovable::<Impl, OFFSET>)
    }
}
pub trait IEnhancedStorageACT3Impl: Sized + IEnhancedStorageACT2Impl + IEnhancedStorageACTImpl {
    fn UnauthorizeEx();
    fn IsQueueFrozen();
    fn GetShellExtSupport();
}
impl ::windows::core::RuntimeName for IEnhancedStorageACT3 {
    const NAME: &'static str = "Windows.Win32.Storage.EnhancedStorage.IEnhancedStorageACT3";
}
impl IEnhancedStorageACT3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageACT3Impl, const OFFSET: isize>() -> IEnhancedStorageACT3Vtbl {
        unsafe extern "system" fn UnauthorizeEx<Impl: IEnhancedStorageACT3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnauthorizeEx(dwflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsQueueFrozen<Impl: IEnhancedStorageACT3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pisqueuefrozen: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsQueueFrozen(::core::mem::transmute_copy(&pisqueuefrozen)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetShellExtSupport<Impl: IEnhancedStorageACT3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pshellextsupport: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetShellExtSupport(::core::mem::transmute_copy(&pshellextsupport)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnhancedStorageACT3>, ::windows::core::GetTrustLevel, UnauthorizeEx::<Impl, OFFSET>, IsQueueFrozen::<Impl, OFFSET>, GetShellExtSupport::<Impl, OFFSET>)
    }
}
pub trait IEnhancedStorageSiloImpl: Sized {
    fn GetInfo();
    fn GetActions();
    fn SendCommand();
    fn GetPortableDevice();
    fn GetDevicePath();
}
impl ::windows::core::RuntimeName for IEnhancedStorageSilo {
    const NAME: &'static str = "Windows.Win32.Storage.EnhancedStorage.IEnhancedStorageSilo";
}
impl IEnhancedStorageSiloVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>() -> IEnhancedStorageSiloVtbl {
        unsafe extern "system" fn GetInfo<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psiloinfo: *mut SILO_INFO) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInfo(::core::mem::transmute_copy(&psiloinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActions<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstoragesiloactions: *mut *mut ::windows::core::RawPtr, pcenhancedstoragesiloactions: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActions(::core::mem::transmute_copy(&pppienhancedstoragesiloactions), ::core::mem::transmute_copy(&pcenhancedstoragesiloactions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendCommand<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, command: u8, pbcommandbuffer: *const u8, cbcommandbuffer: u32, pbresponsebuffer: *mut u8, pcbresponsebuffer: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCommand(command, pbcommandbuffer, cbcommandbuffer, ::core::mem::transmute_copy(&pbresponsebuffer), pcbresponsebuffer) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPortableDevice<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppiportabledevice: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPortableDevice(::core::mem::transmute_copy(&ppiportabledevice)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDevicePath<Impl: IEnhancedStorageSiloImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszsilodevicepath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDevicePath(::core::mem::transmute_copy(&ppwszsilodevicepath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnhancedStorageSilo>, ::windows::core::GetTrustLevel, GetInfo::<Impl, OFFSET>, GetActions::<Impl, OFFSET>, SendCommand::<Impl, OFFSET>, GetPortableDevice::<Impl, OFFSET>, GetDevicePath::<Impl, OFFSET>)
    }
}
pub trait IEnhancedStorageSiloActionImpl: Sized {
    fn GetName();
    fn GetDescription();
    fn Invoke();
}
impl ::windows::core::RuntimeName for IEnhancedStorageSiloAction {
    const NAME: &'static str = "Windows.Win32.Storage.EnhancedStorage.IEnhancedStorageSiloAction";
}
impl IEnhancedStorageSiloActionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnhancedStorageSiloActionImpl, const OFFSET: isize>() -> IEnhancedStorageSiloActionVtbl {
        unsafe extern "system" fn GetName<Impl: IEnhancedStorageSiloActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactionname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&ppwszactionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDescription<Impl: IEnhancedStorageSiloActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppwszactiondescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDescription(::core::mem::transmute_copy(&ppwszactiondescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Impl: IEnhancedStorageSiloActionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnhancedStorageSiloAction>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetDescription::<Impl, OFFSET>, Invoke::<Impl, OFFSET>)
    }
}
pub trait IEnumEnhancedStorageACTImpl: Sized {
    fn GetACTs();
    fn GetMatchingACT();
}
impl ::windows::core::RuntimeName for IEnumEnhancedStorageACT {
    const NAME: &'static str = "Windows.Win32.Storage.EnhancedStorage.IEnumEnhancedStorageACT";
}
impl IEnumEnhancedStorageACTVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumEnhancedStorageACTImpl, const OFFSET: isize>() -> IEnumEnhancedStorageACTVtbl {
        unsafe extern "system" fn GetACTs<Impl: IEnumEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppienhancedstorageacts: *mut *mut ::windows::core::RawPtr, pcenhancedstorageacts: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetACTs(::core::mem::transmute_copy(&pppienhancedstorageacts), ::core::mem::transmute_copy(&pcenhancedstorageacts)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMatchingACT<Impl: IEnumEnhancedStorageACTImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szvolume: super::super::Foundation::PWSTR, ppienhancedstorageact: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMatchingACT(&*(&szvolume as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppienhancedstorageact)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumEnhancedStorageACT>, ::windows::core::GetTrustLevel, GetACTs::<Impl, OFFSET>, GetMatchingACT::<Impl, OFFSET>)
    }
}
