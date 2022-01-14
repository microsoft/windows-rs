pub trait IEnumNetCfgBindingInterface_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingInterface>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetCfgBindingInterface>;
}
impl IEnumNetCfgBindingInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgBindingInterface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetCfgBindingInterface_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingInterface as ::windows::core::Interface>::IID
    }
}
pub trait IEnumNetCfgBindingPath_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingPath>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetCfgBindingPath>;
}
impl IEnumNetCfgBindingPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgBindingPath_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetCfgBindingPath_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingPath as ::windows::core::Interface>::IID
    }
}
pub trait IEnumNetCfgComponent_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgComponent>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetCfgComponent>;
}
impl IEnumNetCfgComponent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgComponent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetCfgComponent_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetCfgComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfg_Impl: Sized {
    fn Initialize(&mut self, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Uninitialize(&mut self) -> ::windows::core::Result<()>;
    fn Apply(&mut self) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn EnumComponents(&mut self, pguidclass: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumNetCfgComponent>;
    fn FindComponent(&mut self, pszwinfid: super::super::Foundation::PWSTR) -> ::windows::core::Result<INetCfgComponent>;
    fn QueryNetCfgClass(&mut self, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfg_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfg_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfg_Vtbl {
        unsafe extern "system" fn Initialize<Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn Uninitialize<Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize().into()
        }
        unsafe extern "system" fn Apply<Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Apply().into()
        }
        unsafe extern "system" fn Cancel<Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn EnumComponents<Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumComponents(::core::mem::transmute_copy(&pguidclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComponent<Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindComponent(::core::mem::transmute_copy(&pszwinfid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryNetCfgClass<Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryNetCfgClass(::core::mem::transmute_copy(&pguidclass), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Uninitialize: Uninitialize::<Impl, IMPL_OFFSET>,
            Apply: Apply::<Impl, IMPL_OFFSET>,
            Cancel: Cancel::<Impl, IMPL_OFFSET>,
            EnumComponents: EnumComponents::<Impl, IMPL_OFFSET>,
            FindComponent: FindComponent::<Impl, IMPL_OFFSET>,
            QueryNetCfgClass: QueryNetCfgClass::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfg as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgBindingInterface_Impl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetUpperComponent(&mut self) -> ::windows::core::Result<INetCfgComponent>;
    fn GetLowerComponent(&mut self) -> ::windows::core::Result<INetCfgComponent>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgBindingInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgBindingInterface_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgBindingInterface_Vtbl {
        unsafe extern "system" fn GetName<Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwinterfacename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwinterfacename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperComponent<Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpperComponent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnccitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLowerComponent<Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLowerComponent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnccitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetName: GetName::<Impl, IMPL_OFFSET>,
            GetUpperComponent: GetUpperComponent::<Impl, IMPL_OFFSET>,
            GetLowerComponent: GetLowerComponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgBindingInterface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgBindingPath_Impl: Sized {
    fn IsSamePathAs(&mut self, ppath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn IsSubPathOf(&mut self, ppath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<()>;
    fn Enable(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPathToken(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetOwner(&mut self) -> ::windows::core::Result<INetCfgComponent>;
    fn GetDepth(&mut self) -> ::windows::core::Result<u32>;
    fn EnumBindingInterfaces(&mut self) -> ::windows::core::Result<IEnumNetCfgBindingInterface>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgBindingPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgBindingPath_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgBindingPath_Vtbl {
        unsafe extern "system" fn IsSamePathAs<Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSamePathAs(::core::mem::transmute(&ppath)).into()
        }
        unsafe extern "system" fn IsSubPathOf<Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSubPathOf(::core::mem::transmute(&ppath)).into()
        }
        unsafe extern "system" fn IsEnabled<Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEnabled().into()
        }
        unsafe extern "system" fn Enable<Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetPathToken<Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwpathtoken: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPathToken() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwpathtoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwner<Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepth<Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinterfaces: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pcinterfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumBindingInterfaces<Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenuminterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumBindingInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenuminterface = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            IsSamePathAs: IsSamePathAs::<Impl, IMPL_OFFSET>,
            IsSubPathOf: IsSubPathOf::<Impl, IMPL_OFFSET>,
            IsEnabled: IsEnabled::<Impl, IMPL_OFFSET>,
            Enable: Enable::<Impl, IMPL_OFFSET>,
            GetPathToken: GetPathToken::<Impl, IMPL_OFFSET>,
            GetOwner: GetOwner::<Impl, IMPL_OFFSET>,
            GetDepth: GetDepth::<Impl, IMPL_OFFSET>,
            EnumBindingInterfaces: EnumBindingInterfaces::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgBindingPath as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClass_Impl: Sized {
    fn FindComponent(&mut self, pszwinfid: super::super::Foundation::PWSTR) -> ::windows::core::Result<INetCfgComponent>;
    fn EnumComponents(&mut self) -> ::windows::core::Result<IEnumNetCfgComponent>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClass_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgClass_Vtbl {
        unsafe extern "system" fn FindComponent<Impl: INetCfgClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindComponent(::core::mem::transmute_copy(&pszwinfid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnccitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumComponents<Impl: INetCfgClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumComponents() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            FindComponent: FindComponent::<Impl, IMPL_OFFSET>,
            EnumComponents: EnumComponents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetup_Impl: Sized {
    fn SelectAndInstall(&mut self, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN) -> ::windows::core::Result<INetCfgComponent>;
    fn Install(&mut self, pszwinfid: super::super::Foundation::PWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR) -> ::windows::core::Result<INetCfgComponent>;
    fn DeInstall(&mut self, pcomponent: &::core::option::Option<INetCfgComponent>, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassSetup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgClassSetup_Vtbl {
        unsafe extern "system" fn SelectAndInstall<Impl: INetCfgClassSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectAndInstall(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pobotoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnccitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Impl: INetCfgClassSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Install(::core::mem::transmute_copy(&pszwinfid), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno), ::core::mem::transmute_copy(&pszwanswerfile), ::core::mem::transmute_copy(&pszwanswersections)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnccitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeInstall<Impl: INetCfgClassSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomponent: ::windows::core::RawPtr, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeInstall(::core::mem::transmute(&pcomponent), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&pmszwrefs)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SelectAndInstall: SelectAndInstall::<Impl, IMPL_OFFSET>,
            Install: Install::<Impl, IMPL_OFFSET>,
            DeInstall: DeInstall::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClassSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetup2_Impl: Sized + INetCfgClassSetup_Impl {
    fn UpdateNonEnumeratedComponent(&mut self, picomp: &::core::option::Option<INetCfgComponent>, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetup2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassSetup2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgClassSetup2_Vtbl {
        unsafe extern "system" fn UpdateNonEnumeratedComponent<Impl: INetCfgClassSetup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateNonEnumeratedComponent(::core::mem::transmute(&picomp), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno)).into()
        }
        Self {
            base: INetCfgClassSetup_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UpdateNonEnumeratedComponent: UpdateNonEnumeratedComponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClassSetup2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait INetCfgComponent_Impl: Sized {
    fn GetDisplayName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn SetDisplayName(&mut self, pszwdisplayname: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn GetHelpText(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetCharacteristics(&mut self) -> ::windows::core::Result<u32>;
    fn GetInstanceGuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetPnpDevNodeId(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetClassGuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetBindName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetDeviceStatus(&mut self) -> ::windows::core::Result<u32>;
    fn OpenParamKey(&mut self) -> ::windows::core::Result<super::super::System::Registry::HKEY>;
    fn RaisePropertyUi(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl INetCfgComponent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponent_Vtbl {
        unsafe extern "system" fn GetDisplayName<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdisplayname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&pszwdisplayname)).into()
        }
        unsafe extern "system" fn GetHelpText<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwhelptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    *pszwhelptext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacteristics<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcharacteristics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceGuid<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnpDevNodeId<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdevnodeid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPnpDevNodeId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwdevnodeid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassGuid<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindName<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwbindname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwbindname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pulstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenParamKey<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenParamKey() {
                ::core::result::Result::Ok(ok__) => {
                    *phkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RaisePropertyUi<Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RaisePropertyUi(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&punkcontext)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetDisplayName: GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName: SetDisplayName::<Impl, IMPL_OFFSET>,
            GetHelpText: GetHelpText::<Impl, IMPL_OFFSET>,
            GetId: GetId::<Impl, IMPL_OFFSET>,
            GetCharacteristics: GetCharacteristics::<Impl, IMPL_OFFSET>,
            GetInstanceGuid: GetInstanceGuid::<Impl, IMPL_OFFSET>,
            GetPnpDevNodeId: GetPnpDevNodeId::<Impl, IMPL_OFFSET>,
            GetClassGuid: GetClassGuid::<Impl, IMPL_OFFSET>,
            GetBindName: GetBindName::<Impl, IMPL_OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Impl, IMPL_OFFSET>,
            OpenParamKey: OpenParamKey::<Impl, IMPL_OFFSET>,
            RaisePropertyUi: RaisePropertyUi::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentBindings_Impl: Sized {
    fn BindTo(&mut self, pnccitem: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn UnbindFrom(&mut self, pnccitem: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn SupportsBindingInterface(&mut self, dwflags: u32, pszwinterfacename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn IsBoundTo(&mut self, pnccitem: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn IsBindableTo(&mut self, pnccitem: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn EnumBindingPaths(&mut self, dwflags: u32) -> ::windows::core::Result<IEnumNetCfgBindingPath>;
    fn MoveBefore(&mut self, pncbitemsrc: &::core::option::Option<INetCfgBindingPath>, pncbitemdest: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn MoveAfter(&mut self, pncbitemsrc: &::core::option::Option<INetCfgBindingPath>, pncbitemdest: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentBindings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentBindings_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentBindings_Vtbl {
        unsafe extern "system" fn BindTo<Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindTo(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn UnbindFrom<Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnbindFrom(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn SupportsBindingInterface<Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszwinterfacename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SupportsBindingInterface(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszwinterfacename)).into()
        }
        unsafe extern "system" fn IsBoundTo<Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsBoundTo(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn IsBindableTo<Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsBindableTo(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn EnumBindingPaths<Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumBindingPaths(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppienum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveBefore<Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveBefore(::core::mem::transmute(&pncbitemsrc), ::core::mem::transmute(&pncbitemdest)).into()
        }
        unsafe extern "system" fn MoveAfter<Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveAfter(::core::mem::transmute(&pncbitemsrc), ::core::mem::transmute(&pncbitemdest)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BindTo: BindTo::<Impl, IMPL_OFFSET>,
            UnbindFrom: UnbindFrom::<Impl, IMPL_OFFSET>,
            SupportsBindingInterface: SupportsBindingInterface::<Impl, IMPL_OFFSET>,
            IsBoundTo: IsBoundTo::<Impl, IMPL_OFFSET>,
            IsBindableTo: IsBindableTo::<Impl, IMPL_OFFSET>,
            EnumBindingPaths: EnumBindingPaths::<Impl, IMPL_OFFSET>,
            MoveBefore: MoveBefore::<Impl, IMPL_OFFSET>,
            MoveAfter: MoveAfter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentBindings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentControl_Impl: Sized {
    fn Initialize(&mut self, picomp: &::core::option::Option<INetCfgComponent>, pinetcfg: &::core::option::Option<INetCfg>, finstalling: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ApplyRegistryChanges(&mut self) -> ::windows::core::Result<()>;
    fn ApplyPnpChanges(&mut self, picallback: &::core::option::Option<INetCfgPnpReconfigCallback>) -> ::windows::core::Result<()>;
    fn CancelChanges(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentControl_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentControl_Vtbl {
        unsafe extern "system" fn Initialize<Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, pinetcfg: ::windows::core::RawPtr, finstalling: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&picomp), ::core::mem::transmute(&pinetcfg), ::core::mem::transmute_copy(&finstalling)).into()
        }
        unsafe extern "system" fn ApplyRegistryChanges<Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyRegistryChanges().into()
        }
        unsafe extern "system" fn ApplyPnpChanges<Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyPnpChanges(::core::mem::transmute(&picallback)).into()
        }
        unsafe extern "system" fn CancelChanges<Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelChanges().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            ApplyRegistryChanges: ApplyRegistryChanges::<Impl, IMPL_OFFSET>,
            ApplyPnpChanges: ApplyPnpChanges::<Impl, IMPL_OFFSET>,
            CancelChanges: CancelChanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentControl as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentNotifyBinding_Impl: Sized {
    fn QueryBindingPath(&mut self, dwchangeflag: u32, pipath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn NotifyBindingPath(&mut self, dwchangeflag: u32, pipath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
}
impl INetCfgComponentNotifyBinding_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentNotifyBinding_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentNotifyBinding_Vtbl {
        unsafe extern "system" fn QueryBindingPath<Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        unsafe extern "system" fn NotifyBindingPath<Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryBindingPath: QueryBindingPath::<Impl, IMPL_OFFSET>,
            NotifyBindingPath: NotifyBindingPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyBinding as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentNotifyGlobal_Impl: Sized {
    fn GetSupportedNotifications(&mut self) -> ::windows::core::Result<u32>;
    fn SysQueryBindingPath(&mut self, dwchangeflag: u32, pipath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn SysNotifyBindingPath(&mut self, dwchangeflag: u32, pipath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn SysNotifyComponent(&mut self, dwchangeflag: u32, picomp: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
}
impl INetCfgComponentNotifyGlobal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentNotifyGlobal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentNotifyGlobal_Vtbl {
        unsafe extern "system" fn GetSupportedNotifications<Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnotifications: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *dwnotifications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysQueryBindingPath<Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SysQueryBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyBindingPath<Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SysNotifyBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyComponent<Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, picomp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SysNotifyComponent(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&picomp)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetSupportedNotifications: GetSupportedNotifications::<Impl, IMPL_OFFSET>,
            SysQueryBindingPath: SysQueryBindingPath::<Impl, IMPL_OFFSET>,
            SysNotifyBindingPath: SysNotifyBindingPath::<Impl, IMPL_OFFSET>,
            SysNotifyComponent: SysNotifyComponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyGlobal as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentPropertyUi_Impl: Sized {
    fn QueryPropertyUi(&mut self, punkreserved: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetContext(&mut self, punkreserved: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn MergePropPages(&mut self, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ValidateProperties(&mut self, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn ApplyProperties(&mut self) -> ::windows::core::Result<()>;
    fn CancelProperties(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentPropertyUi_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentPropertyUi_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentPropertyUi_Vtbl {
        unsafe extern "system" fn QueryPropertyUi<Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryPropertyUi(::core::mem::transmute(&punkreserved)).into()
        }
        unsafe extern "system" fn SetContext<Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(::core::mem::transmute(&punkreserved)).into()
        }
        unsafe extern "system" fn MergePropPages<Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MergePropPages(::core::mem::transmute_copy(&pdwdefpages), ::core::mem::transmute_copy(&pahpspprivate), ::core::mem::transmute_copy(&pcpages), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pszstartpage)).into()
        }
        unsafe extern "system" fn ValidateProperties<Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValidateProperties(::core::mem::transmute_copy(&hwndsheet)).into()
        }
        unsafe extern "system" fn ApplyProperties<Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyProperties().into()
        }
        unsafe extern "system" fn CancelProperties<Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelProperties().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryPropertyUi: QueryPropertyUi::<Impl, IMPL_OFFSET>,
            SetContext: SetContext::<Impl, IMPL_OFFSET>,
            MergePropPages: MergePropPages::<Impl, IMPL_OFFSET>,
            ValidateProperties: ValidateProperties::<Impl, IMPL_OFFSET>,
            ApplyProperties: ApplyProperties::<Impl, IMPL_OFFSET>,
            CancelProperties: CancelProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentPropertyUi as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentSetup_Impl: Sized {
    fn Install(&mut self, dwsetupflags: u32) -> ::windows::core::Result<()>;
    fn Upgrade(&mut self, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::Result<()>;
    fn ReadAnswerFile(&mut self, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Removing(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentSetup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentSetup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentSetup_Vtbl {
        unsafe extern "system" fn Install<Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Install(::core::mem::transmute_copy(&dwsetupflags)).into()
        }
        unsafe extern "system" fn Upgrade<Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Upgrade(::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefombuildno)).into()
        }
        unsafe extern "system" fn ReadAnswerFile<Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadAnswerFile(::core::mem::transmute_copy(&pszwanswerfile), ::core::mem::transmute_copy(&pszwanswersections)).into()
        }
        unsafe extern "system" fn Removing<Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Removing().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Install: Install::<Impl, IMPL_OFFSET>,
            Upgrade: Upgrade::<Impl, IMPL_OFFSET>,
            ReadAnswerFile: ReadAnswerFile::<Impl, IMPL_OFFSET>,
            Removing: Removing::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentSysPrep_Impl: Sized {
    fn SaveAdapterParameters(&mut self, pncsp: &::core::option::Option<INetCfgSysPrep>, pszwanswersections: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RestoreAdapterParameters(&mut self, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersection: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentSysPrep_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentSysPrep_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentSysPrep_Vtbl {
        unsafe extern "system" fn SaveAdapterParameters<Impl: INetCfgComponentSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncsp: ::windows::core::RawPtr, pszwanswersections: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveAdapterParameters(::core::mem::transmute(&pncsp), ::core::mem::transmute_copy(&pszwanswersections), ::core::mem::transmute_copy(&padapterinstanceguid)).into()
        }
        unsafe extern "system" fn RestoreAdapterParameters<Impl: INetCfgComponentSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersection: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RestoreAdapterParameters(::core::mem::transmute_copy(&pszwanswerfile), ::core::mem::transmute_copy(&pszwanswersection), ::core::mem::transmute_copy(&padapterinstanceguid)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SaveAdapterParameters: SaveAdapterParameters::<Impl, IMPL_OFFSET>,
            RestoreAdapterParameters: RestoreAdapterParameters::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentSysPrep as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentUpperEdge_Impl: Sized {
    fn GetInterfaceIdsForAdapter(&mut self, padapter: &::core::option::Option<INetCfgComponent>, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AddInterfacesToAdapter(&mut self, padapter: &::core::option::Option<INetCfgComponent>, dwnuminterfaces: u32) -> ::windows::core::Result<()>;
    fn RemoveInterfacesFromAdapter(&mut self, padapter: &::core::option::Option<INetCfgComponent>, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl INetCfgComponentUpperEdge_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentUpperEdge_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentUpperEdge_Vtbl {
        unsafe extern "system" fn GetInterfaceIdsForAdapter<Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInterfaceIdsForAdapter(::core::mem::transmute(&padapter), ::core::mem::transmute_copy(&pdwnuminterfaces), ::core::mem::transmute_copy(&ppguidinterfaceids)).into()
        }
        unsafe extern "system" fn AddInterfacesToAdapter<Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInterfacesToAdapter(::core::mem::transmute(&padapter), ::core::mem::transmute_copy(&dwnuminterfaces)).into()
        }
        unsafe extern "system" fn RemoveInterfacesFromAdapter<Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveInterfacesFromAdapter(::core::mem::transmute(&padapter), ::core::mem::transmute_copy(&dwnuminterfaces), ::core::mem::transmute_copy(&pguidinterfaceids)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetInterfaceIdsForAdapter: GetInterfaceIdsForAdapter::<Impl, IMPL_OFFSET>,
            AddInterfacesToAdapter: AddInterfacesToAdapter::<Impl, IMPL_OFFSET>,
            RemoveInterfacesFromAdapter: RemoveInterfacesFromAdapter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentUpperEdge as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgLock_Impl: Sized {
    fn AcquireWriteLock(&mut self, cmstimeout: u32, pszwclientdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn ReleaseWriteLock(&mut self) -> ::windows::core::Result<()>;
    fn IsWriteLocked(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgLock_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgLock_Vtbl {
        unsafe extern "system" fn AcquireWriteLock<Impl: INetCfgLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmstimeout: u32, pszwclientdescription: super::super::Foundation::PWSTR, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireWriteLock(::core::mem::transmute_copy(&cmstimeout), ::core::mem::transmute_copy(&pszwclientdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwclientdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseWriteLock<Impl: INetCfgLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseWriteLock().into()
        }
        unsafe extern "system" fn IsWriteLocked<Impl: INetCfgLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWriteLocked() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwclientdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AcquireWriteLock: AcquireWriteLock::<Impl, IMPL_OFFSET>,
            ReleaseWriteLock: ReleaseWriteLock::<Impl, IMPL_OFFSET>,
            IsWriteLocked: IsWriteLocked::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgLock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgPnpReconfigCallback_Impl: Sized {
    fn SendPnpReconfig(&mut self, layer: NCPNP_RECONFIG_LAYER, pszwupper: super::super::Foundation::PWSTR, pszwlower: super::super::Foundation::PWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgPnpReconfigCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgPnpReconfigCallback_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgPnpReconfigCallback_Vtbl {
        unsafe extern "system" fn SendPnpReconfig<Impl: INetCfgPnpReconfigCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: super::super::Foundation::PWSTR, pszwlower: super::super::Foundation::PWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendPnpReconfig(::core::mem::transmute_copy(&layer), ::core::mem::transmute_copy(&pszwupper), ::core::mem::transmute_copy(&pszwlower), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&dwsizeofdata)).into()
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), SendPnpReconfig: SendPnpReconfig::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgPnpReconfigCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgSysPrep_Impl: Sized {
    fn HrSetupSetFirstDword(&mut self, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::Result<()>;
    fn HrSetupSetFirstString(&mut self, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn HrSetupSetFirstStringAsBool(&mut self, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HrSetupSetFirstMultiSzField(&mut self, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pmszvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgSysPrep_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgSysPrep_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgSysPrep_Vtbl {
        unsafe extern "system" fn HrSetupSetFirstDword<Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HrSetupSetFirstDword(::core::mem::transmute_copy(&pwszsection), ::core::mem::transmute_copy(&pwszkey), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstString<Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HrSetupSetFirstString(::core::mem::transmute_copy(&pwszsection), ::core::mem::transmute_copy(&pwszkey), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstStringAsBool<Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HrSetupSetFirstStringAsBool(::core::mem::transmute_copy(&pwszsection), ::core::mem::transmute_copy(&pwszkey), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstMultiSzField<Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pmszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HrSetupSetFirstMultiSzField(::core::mem::transmute_copy(&pwszsection), ::core::mem::transmute_copy(&pwszkey), ::core::mem::transmute_copy(&pmszvalue)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            HrSetupSetFirstDword: HrSetupSetFirstDword::<Impl, IMPL_OFFSET>,
            HrSetupSetFirstString: HrSetupSetFirstString::<Impl, IMPL_OFFSET>,
            HrSetupSetFirstStringAsBool: HrSetupSetFirstStringAsBool::<Impl, IMPL_OFFSET>,
            HrSetupSetFirstMultiSzField: HrSetupSetFirstMultiSzField::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgSysPrep as ::windows::core::Interface>::IID
    }
}
pub trait INetLanConnectionUiInfo_Impl: Sized {
    fn GetDeviceGuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl INetLanConnectionUiInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetLanConnectionUiInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetLanConnectionUiInfo_Vtbl {
        unsafe extern "system" fn GetDeviceGuid<Impl: INetLanConnectionUiInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetDeviceGuid: GetDeviceGuid::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetLanConnectionUiInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetRasConnectionIpUiInfo_Impl: Sized {
    fn GetUiInfo(&mut self) -> ::windows::core::Result<RASCON_IPUI>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetRasConnectionIpUiInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetRasConnectionIpUiInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetRasConnectionIpUiInfo_Vtbl {
        unsafe extern "system" fn GetUiInfo<Impl: INetRasConnectionIpUiInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUiInfo() {
                ::core::result::Result::Ok(ok__) => {
                    *pinfo = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), GetUiInfo: GetUiInfo::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetRasConnectionIpUiInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IProvisioningDomain_Impl: Sized {
    fn Add(&mut self, pszwpathtofolder: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Query(&mut self, pszwdomain: super::super::Foundation::PWSTR, pszwlanguage: super::super::Foundation::PWSTR, pszwxpathquery: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IProvisioningDomain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningDomain_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningDomain_Vtbl {
        unsafe extern "system" fn Add<Impl: IProvisioningDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwpathtofolder: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&pszwpathtofolder)).into()
        }
        unsafe extern "system" fn Query<Impl: IProvisioningDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdomain: super::super::Foundation::PWSTR, pszwlanguage: super::super::Foundation::PWSTR, pszwxpathquery: super::super::Foundation::PWSTR, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(::core::mem::transmute_copy(&pszwdomain), ::core::mem::transmute_copy(&pszwlanguage), ::core::mem::transmute_copy(&pszwxpathquery)) {
                ::core::result::Result::Ok(ok__) => {
                    *nodes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), Add: Add::<Impl, IMPL_OFFSET>, Query: Query::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProvisioningProfileWireless_Impl: Sized {
    fn CreateProfile(&mut self, bstrxmlwirelessconfigprofile: &super::super::Foundation::BSTR, bstrxmlconnectionconfigprofile: &super::super::Foundation::BSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProvisioningProfileWireless_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningProfileWireless_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningProfileWireless_Vtbl {
        unsafe extern "system" fn CreateProfile<Impl: IProvisioningProfileWireless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmlwirelessconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrxmlconnectionconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, padapterinstanceguid: *const ::windows::core::GUID, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProfile(::core::mem::transmute_copy(&bstrxmlwirelessconfigprofile), ::core::mem::transmute_copy(&bstrxmlconnectionconfigprofile), ::core::mem::transmute_copy(&padapterinstanceguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pulstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(), CreateProfile: CreateProfile::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningProfileWireless as ::windows::core::Interface>::IID
    }
}
