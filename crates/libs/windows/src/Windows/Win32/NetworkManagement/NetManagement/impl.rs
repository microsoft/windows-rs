pub trait IEnumNetCfgBindingInterface_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingInterface>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetCfgBindingInterface>;
}
impl ::windows::core::RuntimeName for IEnumNetCfgBindingInterface {}
impl IEnumNetCfgBindingInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>() -> IEnumNetCfgBindingInterface_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingInterface as ::windows::core::Interface>::IID
    }
}
pub trait IEnumNetCfgBindingPath_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingPath>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetCfgBindingPath>;
}
impl ::windows::core::RuntimeName for IEnumNetCfgBindingPath {}
impl IEnumNetCfgBindingPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>() -> IEnumNetCfgBindingPath_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingPath as ::windows::core::Interface>::IID
    }
}
pub trait IEnumNetCfgComponent_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgComponent>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&self) -> ::windows::core::Result<()>;
    fn Clone(&self) -> ::windows::core::Result<IEnumNetCfgComponent>;
}
impl ::windows::core::RuntimeName for IEnumNetCfgComponent {}
impl IEnumNetCfgComponent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>() -> IEnumNetCfgComponent_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetCfgComponent as ::windows::core::Interface>::IID
    }
}
pub trait INetCfg_Impl: Sized {
    fn Initialize(&self, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn Uninitialize(&self) -> ::windows::core::Result<()>;
    fn Apply(&self) -> ::windows::core::Result<()>;
    fn Cancel(&self) -> ::windows::core::Result<()>;
    fn EnumComponents(&self, pguidclass: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumNetCfgComponent>;
    fn FindComponent(&self, pszwinfid: &::windows::core::PCWSTR) -> ::windows::core::Result<INetCfgComponent>;
    fn QueryNetCfgClass(&self, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetCfg {}
impl INetCfg_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>() -> INetCfg_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Uninitialize().into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Apply().into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn EnumComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, ppenumcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumComponents(::core::mem::transmute_copy(&pguidclass)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcomponent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: ::windows::core::PCWSTR, pcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindComponent(::core::mem::transmute(&pszwinfid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcomponent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryNetCfgClass<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryNetCfgClass(::core::mem::transmute_copy(&pguidclass), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            EnumComponents: EnumComponents::<Identity, Impl, OFFSET>,
            FindComponent: FindComponent::<Identity, Impl, OFFSET>,
            QueryNetCfgClass: QueryNetCfgClass::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfg as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgBindingInterface_Impl: Sized {
    fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetUpperComponent(&self) -> ::windows::core::Result<INetCfgComponent>;
    fn GetLowerComponent(&self) -> ::windows::core::Result<INetCfgComponent>;
}
impl ::windows::core::RuntimeName for INetCfgBindingInterface {}
impl INetCfgBindingInterface_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>() -> INetCfgBindingInterface_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwinterfacename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwinterfacename, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUpperComponent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnccitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLowerComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetLowerComponent() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnccitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetUpperComponent: GetUpperComponent::<Identity, Impl, OFFSET>,
            GetLowerComponent: GetLowerComponent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgBindingInterface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgBindingPath_Impl: Sized {
    fn IsSamePathAs(&self, ppath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn IsSubPathOf(&self, ppath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn IsEnabled(&self) -> ::windows::core::Result<()>;
    fn Enable(&self, fenable: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn GetPathToken(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetOwner(&self) -> ::windows::core::Result<INetCfgComponent>;
    fn GetDepth(&self) -> ::windows::core::Result<u32>;
    fn EnumBindingInterfaces(&self) -> ::windows::core::Result<IEnumNetCfgBindingInterface>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetCfgBindingPath {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgBindingPath_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>() -> INetCfgBindingPath_Vtbl {
        unsafe extern "system" fn IsSamePathAs<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSamePathAs(::core::mem::transmute(&ppath)).into()
        }
        unsafe extern "system" fn IsSubPathOf<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSubPathOf(::core::mem::transmute(&ppath)).into()
        }
        unsafe extern "system" fn IsEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsEnabled().into()
        }
        unsafe extern "system" fn Enable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enable(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetPathToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwpathtoken: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPathToken() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwpathtoken, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwner<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppcomponent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepth<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinterfaces: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDepth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcinterfaces, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumBindingInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenuminterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumBindingInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenuminterface, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            IsSamePathAs: IsSamePathAs::<Identity, Impl, OFFSET>,
            IsSubPathOf: IsSubPathOf::<Identity, Impl, OFFSET>,
            IsEnabled: IsEnabled::<Identity, Impl, OFFSET>,
            Enable: Enable::<Identity, Impl, OFFSET>,
            GetPathToken: GetPathToken::<Identity, Impl, OFFSET>,
            GetOwner: GetOwner::<Identity, Impl, OFFSET>,
            GetDepth: GetDepth::<Identity, Impl, OFFSET>,
            EnumBindingInterfaces: EnumBindingInterfaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgBindingPath as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgClass_Impl: Sized {
    fn FindComponent(&self, pszwinfid: &::windows::core::PCWSTR) -> ::windows::core::Result<INetCfgComponent>;
    fn EnumComponents(&self) -> ::windows::core::Result<IEnumNetCfgComponent>;
}
impl ::windows::core::RuntimeName for INetCfgClass {}
impl INetCfgClass_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClass_Impl, const OFFSET: isize>() -> INetCfgClass_Vtbl {
        unsafe extern "system" fn FindComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: ::windows::core::PCWSTR, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FindComponent(::core::mem::transmute(&pszwinfid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnccitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumComponents<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumComponents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumcomponent, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            FindComponent: FindComponent::<Identity, Impl, OFFSET>,
            EnumComponents: EnumComponents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetup_Impl: Sized {
    fn SelectAndInstall(&self, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN) -> ::windows::core::Result<INetCfgComponent>;
    fn Install(&self, pszwinfid: &::windows::core::PCWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: &::windows::core::PCWSTR, pszwanswersections: &::windows::core::PCWSTR) -> ::windows::core::Result<INetCfgComponent>;
    fn DeInstall(&self, pcomponent: &::core::option::Option<INetCfgComponent>, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetCfgClassSetup {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: isize>() -> INetCfgClassSetup_Vtbl {
        unsafe extern "system" fn SelectAndInstall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SelectAndInstall(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pobotoken)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnccitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: ::windows::core::PCWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: ::windows::core::PCWSTR, pszwanswersections: ::windows::core::PCWSTR, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Install(::core::mem::transmute(&pszwinfid), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno), ::core::mem::transmute(&pszwanswerfile), ::core::mem::transmute(&pszwanswersections)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnccitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeInstall<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomponent: *mut ::core::ffi::c_void, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeInstall(::core::mem::transmute(&pcomponent), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&pmszwrefs)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SelectAndInstall: SelectAndInstall::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            DeInstall: DeInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClassSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetup2_Impl: Sized + INetCfgClassSetup_Impl {
    fn UpdateNonEnumeratedComponent(&self, picomp: &::core::option::Option<INetCfgComponent>, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetCfgClassSetup2 {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetup2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup2_Impl, const OFFSET: isize>() -> INetCfgClassSetup2_Vtbl {
        unsafe extern "system" fn UpdateNonEnumeratedComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateNonEnumeratedComponent(::core::mem::transmute(&picomp), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno)).into()
        }
        Self {
            base__: INetCfgClassSetup_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateNonEnumeratedComponent: UpdateNonEnumeratedComponent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClassSetup2 as ::windows::core::Interface>::IID || iid == &<INetCfgClassSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait INetCfgComponent_Impl: Sized {
    fn GetDisplayName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn SetDisplayName(&self, pszwdisplayname: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn GetHelpText(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetCharacteristics(&self) -> ::windows::core::Result<u32>;
    fn GetInstanceGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetPnpDevNodeId(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetClassGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn GetBindName(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn GetDeviceStatus(&self) -> ::windows::core::Result<u32>;
    fn OpenParamKey(&self) -> ::windows::core::Result<super::super::System::Registry::HKEY>;
    fn RaisePropertyUi(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows::core::RuntimeName for INetCfgComponent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl INetCfgComponent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>() -> INetCfgComponent_Vtbl {
        unsafe extern "system" fn GetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdisplayname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDisplayName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwdisplayname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdisplayname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&pszwdisplayname)).into()
        }
        unsafe extern "system" fn GetHelpText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwhelptext: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetHelpText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszwhelptext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacteristics<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCharacteristics() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcharacteristics, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetInstanceGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnpDevNodeId<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdevnodeid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPnpDevNodeId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwdevnodeid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetClassGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwbindname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBindName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwbindname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenParamKey<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenParamKey() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(phkey, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RaisePropertyUi<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RaisePropertyUi(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&punkcontext)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetDisplayName: GetDisplayName::<Identity, Impl, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, Impl, OFFSET>,
            GetHelpText: GetHelpText::<Identity, Impl, OFFSET>,
            GetId: GetId::<Identity, Impl, OFFSET>,
            GetCharacteristics: GetCharacteristics::<Identity, Impl, OFFSET>,
            GetInstanceGuid: GetInstanceGuid::<Identity, Impl, OFFSET>,
            GetPnpDevNodeId: GetPnpDevNodeId::<Identity, Impl, OFFSET>,
            GetClassGuid: GetClassGuid::<Identity, Impl, OFFSET>,
            GetBindName: GetBindName::<Identity, Impl, OFFSET>,
            GetDeviceStatus: GetDeviceStatus::<Identity, Impl, OFFSET>,
            OpenParamKey: OpenParamKey::<Identity, Impl, OFFSET>,
            RaisePropertyUi: RaisePropertyUi::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponent as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentBindings_Impl: Sized {
    fn BindTo(&self, pnccitem: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn UnbindFrom(&self, pnccitem: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn SupportsBindingInterface(&self, dwflags: u32, pszwinterfacename: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn IsBoundTo(&self, pnccitem: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn IsBindableTo(&self, pnccitem: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
    fn EnumBindingPaths(&self, dwflags: u32) -> ::windows::core::Result<IEnumNetCfgBindingPath>;
    fn MoveBefore(&self, pncbitemsrc: &::core::option::Option<INetCfgBindingPath>, pncbitemdest: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn MoveAfter(&self, pncbitemsrc: &::core::option::Option<INetCfgBindingPath>, pncbitemdest: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetCfgComponentBindings {}
impl INetCfgComponentBindings_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>() -> INetCfgComponentBindings_Vtbl {
        unsafe extern "system" fn BindTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindTo(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn UnbindFrom<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnbindFrom(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn SupportsBindingInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszwinterfacename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SupportsBindingInterface(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszwinterfacename)).into()
        }
        unsafe extern "system" fn IsBoundTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsBoundTo(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn IsBindableTo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsBindableTo(::core::mem::transmute(&pnccitem)).into()
        }
        unsafe extern "system" fn EnumBindingPaths<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumBindingPaths(::core::mem::transmute_copy(&dwflags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppienum, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveBefore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: *mut ::core::ffi::c_void, pncbitemdest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveBefore(::core::mem::transmute(&pncbitemsrc), ::core::mem::transmute(&pncbitemdest)).into()
        }
        unsafe extern "system" fn MoveAfter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: *mut ::core::ffi::c_void, pncbitemdest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveAfter(::core::mem::transmute(&pncbitemsrc), ::core::mem::transmute(&pncbitemdest)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            BindTo: BindTo::<Identity, Impl, OFFSET>,
            UnbindFrom: UnbindFrom::<Identity, Impl, OFFSET>,
            SupportsBindingInterface: SupportsBindingInterface::<Identity, Impl, OFFSET>,
            IsBoundTo: IsBoundTo::<Identity, Impl, OFFSET>,
            IsBindableTo: IsBindableTo::<Identity, Impl, OFFSET>,
            EnumBindingPaths: EnumBindingPaths::<Identity, Impl, OFFSET>,
            MoveBefore: MoveBefore::<Identity, Impl, OFFSET>,
            MoveAfter: MoveAfter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentBindings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentControl_Impl: Sized {
    fn Initialize(&self, picomp: &::core::option::Option<INetCfgComponent>, pinetcfg: &::core::option::Option<INetCfg>, finstalling: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ApplyRegistryChanges(&self) -> ::windows::core::Result<()>;
    fn ApplyPnpChanges(&self, picallback: &::core::option::Option<INetCfgPnpReconfigCallback>) -> ::windows::core::Result<()>;
    fn CancelChanges(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetCfgComponentControl {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>() -> INetCfgComponentControl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: *mut ::core::ffi::c_void, pinetcfg: *mut ::core::ffi::c_void, finstalling: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute(&picomp), ::core::mem::transmute(&pinetcfg), ::core::mem::transmute_copy(&finstalling)).into()
        }
        unsafe extern "system" fn ApplyRegistryChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyRegistryChanges().into()
        }
        unsafe extern "system" fn ApplyPnpChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyPnpChanges(::core::mem::transmute(&picallback)).into()
        }
        unsafe extern "system" fn CancelChanges<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelChanges().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            ApplyRegistryChanges: ApplyRegistryChanges::<Identity, Impl, OFFSET>,
            ApplyPnpChanges: ApplyPnpChanges::<Identity, Impl, OFFSET>,
            CancelChanges: CancelChanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentControl as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentNotifyBinding_Impl: Sized {
    fn QueryBindingPath(&self, dwchangeflag: u32, pipath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn NotifyBindingPath(&self, dwchangeflag: u32, pipath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetCfgComponentNotifyBinding {}
impl INetCfgComponentNotifyBinding_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: isize>() -> INetCfgComponentNotifyBinding_Vtbl {
        unsafe extern "system" fn QueryBindingPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        unsafe extern "system" fn NotifyBindingPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryBindingPath: QueryBindingPath::<Identity, Impl, OFFSET>,
            NotifyBindingPath: NotifyBindingPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyBinding as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentNotifyGlobal_Impl: Sized {
    fn GetSupportedNotifications(&self) -> ::windows::core::Result<u32>;
    fn SysQueryBindingPath(&self, dwchangeflag: u32, pipath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn SysNotifyBindingPath(&self, dwchangeflag: u32, pipath: &::core::option::Option<INetCfgBindingPath>) -> ::windows::core::Result<()>;
    fn SysNotifyComponent(&self, dwchangeflag: u32, picomp: &::core::option::Option<INetCfgComponent>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetCfgComponentNotifyGlobal {}
impl INetCfgComponentNotifyGlobal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>() -> INetCfgComponentNotifyGlobal_Vtbl {
        unsafe extern "system" fn GetSupportedNotifications<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnotifications: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSupportedNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwnotifications, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysQueryBindingPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SysQueryBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyBindingPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SysNotifyBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyComponent<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, picomp: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SysNotifyComponent(::core::mem::transmute_copy(&dwchangeflag), ::core::mem::transmute(&picomp)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetSupportedNotifications: GetSupportedNotifications::<Identity, Impl, OFFSET>,
            SysQueryBindingPath: SysQueryBindingPath::<Identity, Impl, OFFSET>,
            SysNotifyBindingPath: SysNotifyBindingPath::<Identity, Impl, OFFSET>,
            SysNotifyComponent: SysNotifyComponent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyGlobal as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentPropertyUi_Impl: Sized {
    fn QueryPropertyUi(&self, punkreserved: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn SetContext(&self, punkreserved: &::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>;
    fn MergePropPages(&self, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const ::windows::core::PWSTR) -> ::windows::core::Result<()>;
    fn ValidateProperties(&self, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::Result<()>;
    fn ApplyProperties(&self) -> ::windows::core::Result<()>;
    fn CancelProperties(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetCfgComponentPropertyUi {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentPropertyUi_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>() -> INetCfgComponentPropertyUi_Vtbl {
        unsafe extern "system" fn QueryPropertyUi<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryPropertyUi(::core::mem::transmute(&punkreserved)).into()
        }
        unsafe extern "system" fn SetContext<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContext(::core::mem::transmute(&punkreserved)).into()
        }
        unsafe extern "system" fn MergePropPages<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MergePropPages(::core::mem::transmute_copy(&pdwdefpages), ::core::mem::transmute_copy(&pahpspprivate), ::core::mem::transmute_copy(&pcpages), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pszstartpage)).into()
        }
        unsafe extern "system" fn ValidateProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ValidateProperties(::core::mem::transmute_copy(&hwndsheet)).into()
        }
        unsafe extern "system" fn ApplyProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyProperties().into()
        }
        unsafe extern "system" fn CancelProperties<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelProperties().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            QueryPropertyUi: QueryPropertyUi::<Identity, Impl, OFFSET>,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            MergePropPages: MergePropPages::<Identity, Impl, OFFSET>,
            ValidateProperties: ValidateProperties::<Identity, Impl, OFFSET>,
            ApplyProperties: ApplyProperties::<Identity, Impl, OFFSET>,
            CancelProperties: CancelProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentPropertyUi as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentSetup_Impl: Sized {
    fn Install(&self, dwsetupflags: u32) -> ::windows::core::Result<()>;
    fn Upgrade(&self, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::Result<()>;
    fn ReadAnswerFile(&self, pszwanswerfile: &::windows::core::PCWSTR, pszwanswersections: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Removing(&self) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetCfgComponentSetup {}
impl INetCfgComponentSetup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>() -> INetCfgComponentSetup_Vtbl {
        unsafe extern "system" fn Install<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Install(::core::mem::transmute_copy(&dwsetupflags)).into()
        }
        unsafe extern "system" fn Upgrade<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Upgrade(::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefombuildno)).into()
        }
        unsafe extern "system" fn ReadAnswerFile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: ::windows::core::PCWSTR, pszwanswersections: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadAnswerFile(::core::mem::transmute(&pszwanswerfile), ::core::mem::transmute(&pszwanswersections)).into()
        }
        unsafe extern "system" fn Removing<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Removing().into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            Install: Install::<Identity, Impl, OFFSET>,
            Upgrade: Upgrade::<Identity, Impl, OFFSET>,
            ReadAnswerFile: ReadAnswerFile::<Identity, Impl, OFFSET>,
            Removing: Removing::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentSetup as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentSysPrep_Impl: Sized {
    fn SaveAdapterParameters(&self, pncsp: &::core::option::Option<INetCfgSysPrep>, pszwanswersections: &::windows::core::PCWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RestoreAdapterParameters(&self, pszwanswerfile: &::windows::core::PCWSTR, pszwanswersection: &::windows::core::PCWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetCfgComponentSysPrep {}
impl INetCfgComponentSysPrep_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSysPrep_Impl, const OFFSET: isize>() -> INetCfgComponentSysPrep_Vtbl {
        unsafe extern "system" fn SaveAdapterParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncsp: *mut ::core::ffi::c_void, pszwanswersections: ::windows::core::PCWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveAdapterParameters(::core::mem::transmute(&pncsp), ::core::mem::transmute(&pszwanswersections), ::core::mem::transmute_copy(&padapterinstanceguid)).into()
        }
        unsafe extern "system" fn RestoreAdapterParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: ::windows::core::PCWSTR, pszwanswersection: ::windows::core::PCWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreAdapterParameters(::core::mem::transmute(&pszwanswerfile), ::core::mem::transmute(&pszwanswersection), ::core::mem::transmute_copy(&padapterinstanceguid)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            SaveAdapterParameters: SaveAdapterParameters::<Identity, Impl, OFFSET>,
            RestoreAdapterParameters: RestoreAdapterParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentSysPrep as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentUpperEdge_Impl: Sized {
    fn GetInterfaceIdsForAdapter(&self, padapter: &::core::option::Option<INetCfgComponent>, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AddInterfacesToAdapter(&self, padapter: &::core::option::Option<INetCfgComponent>, dwnuminterfaces: u32) -> ::windows::core::Result<()>;
    fn RemoveInterfacesFromAdapter(&self, padapter: &::core::option::Option<INetCfgComponent>, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetCfgComponentUpperEdge {}
impl INetCfgComponentUpperEdge_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>() -> INetCfgComponentUpperEdge_Vtbl {
        unsafe extern "system" fn GetInterfaceIdsForAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInterfaceIdsForAdapter(::core::mem::transmute(&padapter), ::core::mem::transmute_copy(&pdwnuminterfaces), ::core::mem::transmute_copy(&ppguidinterfaceids)).into()
        }
        unsafe extern "system" fn AddInterfacesToAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, dwnuminterfaces: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddInterfacesToAdapter(::core::mem::transmute(&padapter), ::core::mem::transmute_copy(&dwnuminterfaces)).into()
        }
        unsafe extern "system" fn RemoveInterfacesFromAdapter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveInterfacesFromAdapter(::core::mem::transmute(&padapter), ::core::mem::transmute_copy(&dwnuminterfaces), ::core::mem::transmute_copy(&pguidinterfaceids)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            GetInterfaceIdsForAdapter: GetInterfaceIdsForAdapter::<Identity, Impl, OFFSET>,
            AddInterfacesToAdapter: AddInterfacesToAdapter::<Identity, Impl, OFFSET>,
            RemoveInterfacesFromAdapter: RemoveInterfacesFromAdapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentUpperEdge as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgLock_Impl: Sized {
    fn AcquireWriteLock(&self, cmstimeout: u32, pszwclientdescription: &::windows::core::PCWSTR) -> ::windows::core::Result<::windows::core::PWSTR>;
    fn ReleaseWriteLock(&self) -> ::windows::core::Result<()>;
    fn IsWriteLocked(&self) -> ::windows::core::Result<::windows::core::PWSTR>;
}
impl ::windows::core::RuntimeName for INetCfgLock {}
impl INetCfgLock_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: isize>() -> INetCfgLock_Vtbl {
        unsafe extern "system" fn AcquireWriteLock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmstimeout: u32, pszwclientdescription: ::windows::core::PCWSTR, ppszwclientdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AcquireWriteLock(::core::mem::transmute_copy(&cmstimeout), ::core::mem::transmute(&pszwclientdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwclientdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseWriteLock<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseWriteLock().into()
        }
        unsafe extern "system" fn IsWriteLocked<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwclientdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsWriteLocked() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppszwclientdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            AcquireWriteLock: AcquireWriteLock::<Identity, Impl, OFFSET>,
            ReleaseWriteLock: ReleaseWriteLock::<Identity, Impl, OFFSET>,
            IsWriteLocked: IsWriteLocked::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgLock as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgPnpReconfigCallback_Impl: Sized {
    fn SendPnpReconfig(&self, layer: NCPNP_RECONFIG_LAYER, pszwupper: &::windows::core::PCWSTR, pszwlower: &::windows::core::PCWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for INetCfgPnpReconfigCallback {}
impl INetCfgPnpReconfigCallback_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgPnpReconfigCallback_Impl, const OFFSET: isize>() -> INetCfgPnpReconfigCallback_Vtbl {
        unsafe extern "system" fn SendPnpReconfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgPnpReconfigCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: ::windows::core::PCWSTR, pszwlower: ::windows::core::PCWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendPnpReconfig(::core::mem::transmute_copy(&layer), ::core::mem::transmute(&pszwupper), ::core::mem::transmute(&pszwlower), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&dwsizeofdata)).into()
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), SendPnpReconfig: SendPnpReconfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgPnpReconfigCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgSysPrep_Impl: Sized {
    fn HrSetupSetFirstDword(&self, pwszsection: &::windows::core::PCWSTR, pwszkey: &::windows::core::PCWSTR, dwvalue: u32) -> ::windows::core::Result<()>;
    fn HrSetupSetFirstString(&self, pwszsection: &::windows::core::PCWSTR, pwszkey: &::windows::core::PCWSTR, pwszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn HrSetupSetFirstStringAsBool(&self, pwszsection: &::windows::core::PCWSTR, pwszkey: &::windows::core::PCWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn HrSetupSetFirstMultiSzField(&self, pwszsection: &::windows::core::PCWSTR, pwszkey: &::windows::core::PCWSTR, pmszvalue: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetCfgSysPrep {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgSysPrep_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>() -> INetCfgSysPrep_Vtbl {
        unsafe extern "system" fn HrSetupSetFirstDword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows::core::PCWSTR, pwszkey: ::windows::core::PCWSTR, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HrSetupSetFirstDword(::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows::core::PCWSTR, pwszkey: ::windows::core::PCWSTR, pwszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HrSetupSetFirstString(::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstStringAsBool<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows::core::PCWSTR, pwszkey: ::windows::core::PCWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HrSetupSetFirstStringAsBool(::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstMultiSzField<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows::core::PCWSTR, pwszkey: ::windows::core::PCWSTR, pmszvalue: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HrSetupSetFirstMultiSzField(::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute(&pmszvalue)).into()
        }
        Self {
            base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(),
            HrSetupSetFirstDword: HrSetupSetFirstDword::<Identity, Impl, OFFSET>,
            HrSetupSetFirstString: HrSetupSetFirstString::<Identity, Impl, OFFSET>,
            HrSetupSetFirstStringAsBool: HrSetupSetFirstStringAsBool::<Identity, Impl, OFFSET>,
            HrSetupSetFirstMultiSzField: HrSetupSetFirstMultiSzField::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgSysPrep as ::windows::core::Interface>::IID
    }
}
pub trait INetLanConnectionUiInfo_Impl: Sized {
    fn GetDeviceGuid(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
impl ::windows::core::RuntimeName for INetLanConnectionUiInfo {}
impl INetLanConnectionUiInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetLanConnectionUiInfo_Impl, const OFFSET: isize>() -> INetLanConnectionUiInfo_Vtbl {
        unsafe extern "system" fn GetDeviceGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetLanConnectionUiInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetDeviceGuid: GetDeviceGuid::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetLanConnectionUiInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetRasConnectionIpUiInfo_Impl: Sized {
    fn GetUiInfo(&self) -> ::windows::core::Result<RASCON_IPUI>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for INetRasConnectionIpUiInfo {}
#[cfg(feature = "Win32_Foundation")]
impl INetRasConnectionIpUiInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetRasConnectionIpUiInfo_Impl, const OFFSET: isize>() -> INetRasConnectionIpUiInfo_Vtbl {
        unsafe extern "system" fn GetUiInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: INetRasConnectionIpUiInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUiInfo() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), GetUiInfo: GetUiInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetRasConnectionIpUiInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IProvisioningDomain_Impl: Sized {
    fn Add(&self, pszwpathtofolder: &::windows::core::PCWSTR) -> ::windows::core::Result<()>;
    fn Query(&self, pszwdomain: &::windows::core::PCWSTR, pszwlanguage: &::windows::core::PCWSTR, pszwxpathquery: &::windows::core::PCWSTR) -> ::windows::core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl ::windows::core::RuntimeName for IProvisioningDomain {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IProvisioningDomain_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningDomain_Impl, const OFFSET: isize>() -> IProvisioningDomain_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwpathtofolder: ::windows::core::PCWSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&pszwpathtofolder)).into()
        }
        unsafe extern "system" fn Query<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdomain: ::windows::core::PCWSTR, pszwlanguage: ::windows::core::PCWSTR, pszwxpathquery: ::windows::core::PCWSTR, nodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Query(::core::mem::transmute(&pszwdomain), ::core::mem::transmute(&pszwlanguage), ::core::mem::transmute(&pszwxpathquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodes, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, Impl, OFFSET>, Query: Query::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProvisioningProfileWireless_Impl: Sized {
    fn CreateProfile(&self, bstrxmlwirelessconfigprofile: &super::super::Foundation::BSTR, bstrxmlconnectionconfigprofile: &super::super::Foundation::BSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IProvisioningProfileWireless {}
#[cfg(feature = "Win32_Foundation")]
impl IProvisioningProfileWireless_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningProfileWireless_Impl, const OFFSET: isize>() -> IProvisioningProfileWireless_Vtbl {
        unsafe extern "system" fn CreateProfile<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningProfileWireless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmlwirelessconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrxmlconnectionconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, padapterinstanceguid: *const ::windows::core::GUID, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateProfile(::core::mem::transmute(&bstrxmlwirelessconfigprofile), ::core::mem::transmute(&bstrxmlconnectionconfigprofile), ::core::mem::transmute_copy(&padapterinstanceguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstatus, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>(), CreateProfile: CreateProfile::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningProfileWireless as ::windows::core::Interface>::IID
    }
}
