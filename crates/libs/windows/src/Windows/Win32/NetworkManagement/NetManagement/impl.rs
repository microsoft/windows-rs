pub trait IEnumNetCfgBindingInterfaceImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingInterface>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetCfgBindingInterface>;
}
impl IEnumNetCfgBindingInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgBindingInterfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetCfgBindingInterfaceVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumNetCfgBindingPathImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingPath>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetCfgBindingPath>;
}
impl IEnumNetCfgBindingPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgBindingPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetCfgBindingPathVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IEnumNetCfgComponentImpl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgComponent>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumNetCfgComponent>;
}
impl IEnumNetCfgComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetCfgComponentVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait INetCfgImpl: Sized {
    fn Initialize(&mut self, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Uninitialize(&mut self) -> ::windows::core::Result<()>;
    fn Apply(&mut self) -> ::windows::core::Result<()>;
    fn Cancel(&mut self) -> ::windows::core::Result<()>;
    fn EnumComponents(&mut self, pguidclass: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumNetCfgComponent>;
    fn FindComponent(&mut self, pszwinfid: super::super::Foundation::PWSTR) -> ::windows::core::Result<INetCfgComponent>;
    fn QueryNetCfgClass(&mut self, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgVtbl {
        unsafe extern "system" fn Initialize<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn Uninitialize<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize().into()
        }
        unsafe extern "system" fn Apply<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Apply().into()
        }
        unsafe extern "system" fn Cancel<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Cancel().into()
        }
        unsafe extern "system" fn EnumComponents<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumComponents(::core::mem::transmute_copy(&pguidclass)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComponent<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindComponent(::core::mem::transmute_copy(&pszwinfid)) {
                ::core::result::Result::Ok(ok__) => {
                    *pcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryNetCfgClass<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait INetCfgBindingInterfaceImpl: Sized {
    fn GetName(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetUpperComponent(&mut self) -> ::windows::core::Result<INetCfgComponent>;
    fn GetLowerComponent(&mut self) -> ::windows::core::Result<INetCfgComponent>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgBindingInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgBindingInterfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgBindingInterfaceVtbl {
        unsafe extern "system" fn GetName<Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwinterfacename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwinterfacename = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperComponent<Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpperComponent() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnccitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLowerComponent<Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait INetCfgBindingPathImpl: Sized {
    fn IsSamePathAs(&mut self, ppath: ::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn IsSubPathOf(&mut self, ppath: ::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn IsEnabled(&mut self) -> ::windows::core::Result<()>;
    fn Enable(&mut self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPathToken(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn GetOwner(&mut self) -> ::windows::core::Result<INetCfgComponent>;
    fn GetDepth(&mut self) -> ::windows::core::Result<u32>;
    fn EnumBindingInterfaces(&mut self) -> ::windows::core::Result<IEnumNetCfgBindingInterface>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgBindingPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgBindingPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgBindingPathVtbl {
        unsafe extern "system" fn IsSamePathAs<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSamePathAs(::core::mem::transmute(&ppath)).into()
        }
        unsafe extern "system" fn IsSubPathOf<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsSubPathOf(::core::mem::transmute(&ppath)).into()
        }
        unsafe extern "system" fn IsEnabled<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsEnabled().into()
        }
        unsafe extern "system" fn Enable<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Enable(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetPathToken<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwpathtoken: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPathToken() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwpathtoken = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwner<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *ppcomponent = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepth<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinterfaces: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDepth() {
                ::core::result::Result::Ok(ok__) => {
                    *pcinterfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumBindingInterfaces<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenuminterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait INetCfgClassImpl: Sized {
    fn FindComponent(&mut self, pszwinfid: super::super::Foundation::PWSTR) -> ::windows::core::Result<INetCfgComponent>;
    fn EnumComponents(&mut self) -> ::windows::core::Result<IEnumNetCfgComponent>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgClassVtbl {
        unsafe extern "system" fn FindComponent<Impl: INetCfgClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindComponent(::core::mem::transmute_copy(&pszwinfid)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnccitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumComponents<Impl: INetCfgClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait INetCfgClassSetupImpl: Sized {
    fn SelectAndInstall(&mut self, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN) -> ::windows::core::Result<INetCfgComponent>;
    fn Install(&mut self, pszwinfid: super::super::Foundation::PWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR) -> ::windows::core::Result<INetCfgComponent>;
    fn DeInstall(&mut self, pcomponent: ::core::option::Option<INetCfgComponent>, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgClassSetupVtbl {
        unsafe extern "system" fn SelectAndInstall<Impl: INetCfgClassSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectAndInstall(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pobotoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnccitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Impl: INetCfgClassSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Install(::core::mem::transmute_copy(&pszwinfid), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno), ::core::mem::transmute_copy(&pszwanswerfile), ::core::mem::transmute_copy(&pszwanswersections)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppnccitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeInstall<Impl: INetCfgClassSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomponent: ::windows::core::RawPtr, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
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
pub trait INetCfgClassSetup2Impl: Sized + INetCfgClassSetupImpl {
    fn UpdateNonEnumeratedComponent(&mut self, picomp: ::core::option::Option<INetCfgComponent>, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassSetup2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgClassSetup2Vtbl {
        unsafe extern "system" fn UpdateNonEnumeratedComponent<Impl: INetCfgClassSetup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateNonEnumeratedComponent(::core::mem::transmute(&picomp), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno)).into()
        }
        Self {
            base: INetCfgClassSetupVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UpdateNonEnumeratedComponent: UpdateNonEnumeratedComponent::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClassSetup2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait INetCfgComponentImpl: Sized {
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
    fn RaisePropertyUi(&mut self, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl INetCfgComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentVtbl {
        unsafe extern "system" fn GetDisplayName<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwdisplayname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdisplayname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisplayName(::core::mem::transmute_copy(&pszwdisplayname)).into()
        }
        unsafe extern "system" fn GetHelpText<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwhelptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    *pszwhelptext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacteristics<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristics() {
                ::core::result::Result::Ok(ok__) => {
                    *pdwcharacteristics = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceGuid<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnpDevNodeId<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdevnodeid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPnpDevNodeId() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwdevnodeid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassGuid<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pguid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindName<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwbindname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindName() {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwbindname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *pulstatus = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenParamKey<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenParamKey() {
                ::core::result::Result::Ok(ok__) => {
                    *phkey = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RaisePropertyUi<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait INetCfgComponentBindingsImpl: Sized {
    fn BindTo(&mut self, pnccitem: ::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn UnbindFrom(&mut self, pnccitem: ::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn SupportsBindingInterface(&mut self, dwflags: u32, pszwinterfacename: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn IsBoundTo(&mut self, pnccitem: ::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn IsBindableTo(&mut self, pnccitem: ::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn EnumBindingPaths(&mut self, dwflags: u32) -> ::windows::core::Result<IEnumNetCfgBindingPath>;
    fn MoveBefore(&mut self, pncbitemsrc: ::core::option::Option<INetCfgBindingPath>, pncbitemdest: ::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn MoveAfter(&mut self, pncbitemsrc: ::core::option::Option<INetCfgBindingPath>, pncbitemdest: ::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentBindingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentBindingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentBindingsVtbl {
        unsafe extern "system" fn BindTo<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).BindTo(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn UnbindFrom<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnbindFrom(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn SupportsBindingInterface<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszwinterfacename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SupportsBindingInterface(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pszwinterfacename)).into()
        }
        unsafe extern "system" fn IsBoundTo<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsBoundTo(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn IsBindableTo<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).IsBindableTo(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn EnumBindingPaths<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumBindingPaths(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppienum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveBefore<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MoveBefore(::core::mem::transmute(&pncbitemsrc), ::core::mem::transmute(&pncbitemdest)).into()
        }
        unsafe extern "system" fn MoveAfter<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait INetCfgComponentControlImpl: Sized {
    fn Initialize(&mut self, picomp: ::core::option::Option<INetCfgComponent>, pinetcfg: ::core::option::Option<INetCfg>, finstalling: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ApplyRegistryChanges(&mut self) -> ::windows::core::Result<()>;
    fn ApplyPnpChanges(&mut self, picallback: ::core::option::Option<INetCfgPnpReconfigCallback>) -> ::windows::core::Result<()>;
    fn CancelChanges(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentControlVtbl {
        unsafe extern "system" fn Initialize<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, pinetcfg: ::windows::core::RawPtr, finstalling: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&picomp), ::core::mem::transmute(&pinetcfg), ::core::mem::transmute_copy(&finstalling)).into()
        }
        unsafe extern "system" fn ApplyRegistryChanges<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyRegistryChanges().into()
        }
        unsafe extern "system" fn ApplyPnpChanges<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyPnpChanges(::core::mem::transmute(&picallback)).into()
        }
        unsafe extern "system" fn CancelChanges<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait INetCfgComponentNotifyBindingImpl: Sized {
    fn QueryBindingPath(&mut self, dwchangeflag: u32, pipath: ::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn NotifyBindingPath(&mut self, dwchangeflag: u32, pipath: ::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
}
impl INetCfgComponentNotifyBindingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentNotifyBindingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentNotifyBindingVtbl {
        unsafe extern "system" fn QueryBindingPath<Impl: INetCfgComponentNotifyBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        unsafe extern "system" fn NotifyBindingPath<Impl: INetCfgComponentNotifyBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait INetCfgComponentNotifyGlobalImpl: Sized {
    fn GetSupportedNotifications(&mut self) -> ::windows::core::Result<u32>;
    fn SysQueryBindingPath(&mut self, dwchangeflag: u32, pipath: ::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn SysNotifyBindingPath(&mut self, dwchangeflag: u32, pipath: ::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn SysNotifyComponent(&mut self, dwchangeflag: u32, picomp: ::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
}
impl INetCfgComponentNotifyGlobalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentNotifyGlobalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentNotifyGlobalVtbl {
        unsafe extern "system" fn GetSupportedNotifications<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnotifications: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    *dwnotifications = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysQueryBindingPath<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SysQueryBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyBindingPath<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SysNotifyBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyComponent<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, picomp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait INetCfgComponentPropertyUiImpl: Sized {
    fn QueryPropertyUi(&mut self, punkreserved: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetContext(&mut self, punkreserved: ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn MergePropPages(&mut self, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn ValidateProperties(&mut self, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn ApplyProperties(&mut self) -> ::windows::core::Result<()>;
    fn CancelProperties(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentPropertyUiVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentPropertyUiImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentPropertyUiVtbl {
        unsafe extern "system" fn QueryPropertyUi<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryPropertyUi(::core::mem::transmute(&punkreserved)).into()
        }
        unsafe extern "system" fn SetContext<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetContext(::core::mem::transmute(&punkreserved)).into()
        }
        unsafe extern "system" fn MergePropPages<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).MergePropPages(::core::mem::transmute_copy(&pdwdefpages), ::core::mem::transmute_copy(&pahpspprivate), ::core::mem::transmute_copy(&pcpages), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pszstartpage)).into()
        }
        unsafe extern "system" fn ValidateProperties<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ValidateProperties(::core::mem::transmute_copy(&hwndsheet)).into()
        }
        unsafe extern "system" fn ApplyProperties<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ApplyProperties().into()
        }
        unsafe extern "system" fn CancelProperties<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait INetCfgComponentSetupImpl: Sized {
    fn Install(&mut self, dwsetupflags: u32) -> ::windows::core::Result<()>;
    fn Upgrade(&mut self, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::Result<()>;
    fn ReadAnswerFile(&mut self, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Removing(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentSetupVtbl {
        unsafe extern "system" fn Install<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Install(::core::mem::transmute_copy(&dwsetupflags)).into()
        }
        unsafe extern "system" fn Upgrade<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Upgrade(::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefombuildno)).into()
        }
        unsafe extern "system" fn ReadAnswerFile<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadAnswerFile(::core::mem::transmute_copy(&pszwanswerfile), ::core::mem::transmute_copy(&pszwanswersections)).into()
        }
        unsafe extern "system" fn Removing<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
pub trait INetCfgComponentSysPrepImpl: Sized {
    fn SaveAdapterParameters(&mut self, pncsp: ::core::option::Option<INetCfgSysPrep>, pszwanswersections: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RestoreAdapterParameters(&mut self, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersection: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentSysPrepVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentSysPrepImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentSysPrepVtbl {
        unsafe extern "system" fn SaveAdapterParameters<Impl: INetCfgComponentSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncsp: ::windows::core::RawPtr, pszwanswersections: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SaveAdapterParameters(::core::mem::transmute(&pncsp), ::core::mem::transmute_copy(&pszwanswersections), ::core::mem::transmute_copy(&padapterinstanceguid)).into()
        }
        unsafe extern "system" fn RestoreAdapterParameters<Impl: INetCfgComponentSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersection: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait INetCfgComponentUpperEdgeImpl: Sized {
    fn GetInterfaceIdsForAdapter(&mut self, padapter: ::core::option::Option<INetCfgComponent>, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AddInterfacesToAdapter(&mut self, padapter: ::core::option::Option<INetCfgComponent>, dwnuminterfaces: u32) -> ::windows::core::Result<()>;
    fn RemoveInterfacesFromAdapter(&mut self, padapter: ::core::option::Option<INetCfgComponent>, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl INetCfgComponentUpperEdgeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentUpperEdgeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentUpperEdgeVtbl {
        unsafe extern "system" fn GetInterfaceIdsForAdapter<Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInterfaceIdsForAdapter(::core::mem::transmute(&padapter), ::core::mem::transmute_copy(&pdwnuminterfaces), ::core::mem::transmute_copy(&ppguidinterfaceids)).into()
        }
        unsafe extern "system" fn AddInterfacesToAdapter<Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInterfacesToAdapter(::core::mem::transmute(&padapter), ::core::mem::transmute_copy(&dwnuminterfaces)).into()
        }
        unsafe extern "system" fn RemoveInterfacesFromAdapter<Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait INetCfgLockImpl: Sized {
    fn AcquireWriteLock(&mut self, cmstimeout: u32, pszwclientdescription: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
    fn ReleaseWriteLock(&mut self) -> ::windows::core::Result<()>;
    fn IsWriteLocked(&mut self) -> ::windows::core::Result<super::super::Foundation::PWSTR>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgLockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgLockVtbl {
        unsafe extern "system" fn AcquireWriteLock<Impl: INetCfgLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmstimeout: u32, pszwclientdescription: super::super::Foundation::PWSTR, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireWriteLock(::core::mem::transmute_copy(&cmstimeout), ::core::mem::transmute_copy(&pszwclientdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppszwclientdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseWriteLock<Impl: INetCfgLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseWriteLock().into()
        }
        unsafe extern "system" fn IsWriteLocked<Impl: INetCfgLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
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
pub trait INetCfgPnpReconfigCallbackImpl: Sized {
    fn SendPnpReconfig(&mut self, layer: NCPNP_RECONFIG_LAYER, pszwupper: super::super::Foundation::PWSTR, pszwlower: super::super::Foundation::PWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgPnpReconfigCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgPnpReconfigCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgPnpReconfigCallbackVtbl {
        unsafe extern "system" fn SendPnpReconfig<Impl: INetCfgPnpReconfigCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: super::super::Foundation::PWSTR, pszwlower: super::super::Foundation::PWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::HRESULT {
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
pub trait INetCfgSysPrepImpl: Sized {
    fn HrSetupSetFirstDword(&mut self, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::Result<()>;
    fn HrSetupSetFirstString(&mut self, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn HrSetupSetFirstStringAsBool(&mut self, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HrSetupSetFirstMultiSzField(&mut self, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pmszvalue: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgSysPrepVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgSysPrepImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgSysPrepVtbl {
        unsafe extern "system" fn HrSetupSetFirstDword<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HrSetupSetFirstDword(::core::mem::transmute_copy(&pwszsection), ::core::mem::transmute_copy(&pwszkey), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstString<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HrSetupSetFirstString(::core::mem::transmute_copy(&pwszsection), ::core::mem::transmute_copy(&pwszkey), ::core::mem::transmute_copy(&pwszvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstStringAsBool<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).HrSetupSetFirstStringAsBool(::core::mem::transmute_copy(&pwszsection), ::core::mem::transmute_copy(&pwszkey), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstMultiSzField<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pmszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
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
pub trait INetLanConnectionUiInfoImpl: Sized {
    fn GetDeviceGuid(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl INetLanConnectionUiInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetLanConnectionUiInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetLanConnectionUiInfoVtbl {
        unsafe extern "system" fn GetDeviceGuid<Impl: INetLanConnectionUiInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
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
pub trait INetRasConnectionIpUiInfoImpl: Sized {
    fn GetUiInfo(&mut self) -> ::windows::core::Result<RASCON_IPUI>;
}
#[cfg(feature = "Win32_Foundation")]
impl INetRasConnectionIpUiInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetRasConnectionIpUiInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetRasConnectionIpUiInfoVtbl {
        unsafe extern "system" fn GetUiInfo<Impl: INetRasConnectionIpUiInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> ::windows::core::HRESULT {
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
pub trait IProvisioningDomainImpl: Sized {
    fn Add(&mut self, pszwpathtofolder: super::super::Foundation::PWSTR) -> ::windows::core::Result<()>;
    fn Query(&mut self, pszwdomain: super::super::Foundation::PWSTR, pszwlanguage: super::super::Foundation::PWSTR, pszwxpathquery: super::super::Foundation::PWSTR) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IProvisioningDomainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningDomainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningDomainVtbl {
        unsafe extern "system" fn Add<Impl: IProvisioningDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwpathtofolder: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute_copy(&pszwpathtofolder)).into()
        }
        unsafe extern "system" fn Query<Impl: IProvisioningDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdomain: super::super::Foundation::PWSTR, pszwlanguage: super::super::Foundation::PWSTR, pszwxpathquery: super::super::Foundation::PWSTR, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
pub trait IProvisioningProfileWirelessImpl: Sized {
    fn CreateProfile(&mut self, bstrxmlwirelessconfigprofile: super::super::Foundation::BSTR, bstrxmlconnectionconfigprofile: super::super::Foundation::BSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IProvisioningProfileWirelessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningProfileWirelessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningProfileWirelessVtbl {
        unsafe extern "system" fn CreateProfile<Impl: IProvisioningProfileWirelessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmlwirelessconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrxmlconnectionconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, padapterinstanceguid: *const ::windows::core::GUID, pulstatus: *mut u32) -> ::windows::core::HRESULT {
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
