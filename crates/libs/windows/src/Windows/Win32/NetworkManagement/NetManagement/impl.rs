#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait IEnumNetCfgBindingInterface_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingInterface>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self, ppenum: *const ::core::option::Option<IEnumNetCfgBindingInterface>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumNetCfgBindingInterface {}
impl IEnumNetCfgBindingInterface_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>() -> IEnumNetCfgBindingInterface_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clone(::core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingInterface as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait IEnumNetCfgBindingPath_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgBindingPath>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self, ppenum: *const ::core::option::Option<IEnumNetCfgBindingPath>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumNetCfgBindingPath {}
impl IEnumNetCfgBindingPath_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>() -> IEnumNetCfgBindingPath_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clone(::core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingPath as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait IEnumNetCfgComponent_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetCfgComponent>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self, ppenum: *const ::core::option::Option<IEnumNetCfgComponent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IEnumNetCfgComponent {}
impl IEnumNetCfgComponent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>() -> IEnumNetCfgComponent_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *const *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Clone(::core::mem::transmute_copy(&ppenum)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumNetCfgComponent as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfg_Impl: Sized {
    fn Initialize(&self, pvreserved: *const ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn Uninitialize(&self) -> ::windows_core::Result<()>;
    fn Apply(&self) -> ::windows_core::Result<()>;
    fn Cancel(&self) -> ::windows_core::Result<()>;
    fn EnumComponents(&self, pguidclass: *const ::windows_core::GUID, ppenumcomponent: *mut ::core::option::Option<IEnumNetCfgComponent>) -> ::windows_core::Result<()>;
    fn FindComponent(&self, pszwinfid: &::windows_core::PCWSTR, pcomponent: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn QueryNetCfgClass(&self, pguidclass: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfg {}
impl INetCfg_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>() -> INetCfg_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvreserved: *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&pvreserved)).into()
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Uninitialize().into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Apply().into()
        }
        unsafe extern "system" fn Cancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Cancel().into()
        }
        unsafe extern "system" fn EnumComponents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows_core::GUID, ppenumcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumComponents(::core::mem::transmute_copy(&pguidclass), ::core::mem::transmute_copy(&ppenumcomponent)).into()
        }
        unsafe extern "system" fn FindComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: ::windows_core::PCWSTR, pcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindComponent(::core::mem::transmute(&pszwinfid), ::core::mem::transmute_copy(&pcomponent)).into()
        }
        unsafe extern "system" fn QueryNetCfgClass<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfg_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows_core::GUID, riid: *const ::windows_core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryNetCfgClass(::core::mem::transmute_copy(&pguidclass), ::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppvobject)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            Cancel: Cancel::<Identity, Impl, OFFSET>,
            EnumComponents: EnumComponents::<Identity, Impl, OFFSET>,
            FindComponent: FindComponent::<Identity, Impl, OFFSET>,
            QueryNetCfgClass: QueryNetCfgClass::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfg as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgBindingInterface_Impl: Sized {
    fn GetName(&self, ppszwinterfacename: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetUpperComponent(&self, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn GetLowerComponent(&self, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgBindingInterface {}
impl INetCfgBindingInterface_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>() -> INetCfgBindingInterface_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwinterfacename: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetName(::core::mem::transmute_copy(&ppszwinterfacename)).into()
        }
        unsafe extern "system" fn GetUpperComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUpperComponent(::core::mem::transmute_copy(&ppnccitem)).into()
        }
        unsafe extern "system" fn GetLowerComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingInterface_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetLowerComponent(::core::mem::transmute_copy(&ppnccitem)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            GetUpperComponent: GetUpperComponent::<Identity, Impl, OFFSET>,
            GetLowerComponent: GetLowerComponent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgBindingInterface as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgBindingPath_Impl: Sized {
    fn IsSamePathAs(&self, ppath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn IsSubPathOf(&self, ppath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn IsEnabled(&self) -> ::windows_core::Result<()>;
    fn Enable(&self, fenable: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn GetPathToken(&self, ppszwpathtoken: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetOwner(&self, ppcomponent: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn GetDepth(&self) -> ::windows_core::Result<u32>;
    fn EnumBindingInterfaces(&self, ppenuminterface: *mut ::core::option::Option<IEnumNetCfgBindingInterface>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for INetCfgBindingPath {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgBindingPath_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>() -> INetCfgBindingPath_Vtbl {
        unsafe extern "system" fn IsSamePathAs<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSamePathAs(::windows_core::from_raw_borrowed(&ppath)).into()
        }
        unsafe extern "system" fn IsSubPathOf<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsSubPathOf(::windows_core::from_raw_borrowed(&ppath)).into()
        }
        unsafe extern "system" fn IsEnabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsEnabled().into()
        }
        unsafe extern "system" fn Enable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Enable(::core::mem::transmute_copy(&fenable)).into()
        }
        unsafe extern "system" fn GetPathToken<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwpathtoken: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPathToken(::core::mem::transmute_copy(&ppszwpathtoken)).into()
        }
        unsafe extern "system" fn GetOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetOwner(::core::mem::transmute_copy(&ppcomponent)).into()
        }
        unsafe extern "system" fn GetDepth<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinterfaces: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDepth() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcinterfaces, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumBindingInterfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgBindingPath_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenuminterface: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumBindingInterfaces(::core::mem::transmute_copy(&ppenuminterface)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgBindingPath as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgClass_Impl: Sized {
    fn FindComponent(&self, pszwinfid: &::windows_core::PCWSTR, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn EnumComponents(&self, ppenumcomponent: *mut ::core::option::Option<IEnumNetCfgComponent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgClass {}
impl INetCfgClass_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClass_Impl, const OFFSET: isize>() -> INetCfgClass_Vtbl {
        unsafe extern "system" fn FindComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: ::windows_core::PCWSTR, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.FindComponent(::core::mem::transmute(&pszwinfid), ::core::mem::transmute_copy(&ppnccitem)).into()
        }
        unsafe extern "system" fn EnumComponents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClass_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcomponent: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumComponents(::core::mem::transmute_copy(&ppenumcomponent)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindComponent: FindComponent::<Identity, Impl, OFFSET>,
            EnumComponents: EnumComponents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgClass as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetup_Impl: Sized {
    fn SelectAndInstall(&self, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn Install(&self, pszwinfid: &::windows_core::PCWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: &::windows_core::PCWSTR, pszwanswersections: &::windows_core::PCWSTR, ppnccitem: *mut ::core::option::Option<INetCfgComponent>) -> ::windows_core::Result<()>;
    fn DeInstall(&self, pcomponent: ::core::option::Option<&INetCfgComponent>, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for INetCfgClassSetup {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: isize>() -> INetCfgClassSetup_Vtbl {
        unsafe extern "system" fn SelectAndInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SelectAndInstall(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&ppnccitem)).into()
        }
        unsafe extern "system" fn Install<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: ::windows_core::PCWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: ::windows_core::PCWSTR, pszwanswersections: ::windows_core::PCWSTR, ppnccitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Install(::core::mem::transmute(&pszwinfid), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno), ::core::mem::transmute(&pszwanswerfile), ::core::mem::transmute(&pszwanswersections), ::core::mem::transmute_copy(&ppnccitem)).into()
        }
        unsafe extern "system" fn DeInstall<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomponent: *mut ::core::ffi::c_void, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeInstall(::windows_core::from_raw_borrowed(&pcomponent), ::core::mem::transmute_copy(&pobotoken), ::core::mem::transmute_copy(&pmszwrefs)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SelectAndInstall: SelectAndInstall::<Identity, Impl, OFFSET>,
            Install: Install::<Identity, Impl, OFFSET>,
            DeInstall: DeInstall::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgClassSetup as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetup2_Impl: Sized + INetCfgClassSetup_Impl {
    fn UpdateNonEnumeratedComponent(&self, picomp: ::core::option::Option<&INetCfgComponent>, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for INetCfgClassSetup2 {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetup2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup2_Impl, const OFFSET: isize>() -> INetCfgClassSetup2_Vtbl {
        unsafe extern "system" fn UpdateNonEnumeratedComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgClassSetup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateNonEnumeratedComponent(::windows_core::from_raw_borrowed(&picomp), ::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefrombuildno)).into()
        }
        Self {
            base__: INetCfgClassSetup_Vtbl::new::<Identity, Impl, OFFSET>(),
            UpdateNonEnumeratedComponent: UpdateNonEnumeratedComponent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgClassSetup2 as ::windows_core::ComInterface>::IID || iid == &<INetCfgClassSetup as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Registry\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait INetCfgComponent_Impl: Sized {
    fn GetDisplayName(&self, ppszwdisplayname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn SetDisplayName(&self, pszwdisplayname: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn GetHelpText(&self, pszwhelptext: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetId(&self, ppszwid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetCharacteristics(&self) -> ::windows_core::Result<u32>;
    fn GetInstanceGuid(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetPnpDevNodeId(&self, ppszwdevnodeid: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetClassGuid(&self, pguid: *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn GetBindName(&self, ppszwbindname: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn GetDeviceStatus(&self) -> ::windows_core::Result<u32>;
    fn OpenParamKey(&self, phkey: *mut super::super::System::Registry::HKEY) -> ::windows_core::Result<()>;
    fn RaisePropertyUi(&self, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl ::windows_core::RuntimeName for INetCfgComponent {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl INetCfgComponent_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>() -> INetCfgComponent_Vtbl {
        unsafe extern "system" fn GetDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdisplayname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayName(::core::mem::transmute_copy(&ppszwdisplayname)).into()
        }
        unsafe extern "system" fn SetDisplayName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdisplayname: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisplayName(::core::mem::transmute(&pszwdisplayname)).into()
        }
        unsafe extern "system" fn GetHelpText<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwhelptext: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetHelpText(::core::mem::transmute_copy(&pszwhelptext)).into()
        }
        unsafe extern "system" fn GetId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetId(::core::mem::transmute_copy(&ppszwid)).into()
        }
        unsafe extern "system" fn GetCharacteristics<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCharacteristics() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdwcharacteristics, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInstanceGuid(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetPnpDevNodeId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdevnodeid: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPnpDevNodeId(::core::mem::transmute_copy(&ppszwdevnodeid)).into()
        }
        unsafe extern "system" fn GetClassGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetClassGuid(::core::mem::transmute_copy(&pguid)).into()
        }
        unsafe extern "system" fn GetBindName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwbindname: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetBindName(::core::mem::transmute_copy(&ppszwbindname)).into()
        }
        unsafe extern "system" fn GetDeviceStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenParamKey<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenParamKey(::core::mem::transmute_copy(&phkey)).into()
        }
        unsafe extern "system" fn RaisePropertyUi<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RaisePropertyUi(::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::windows_core::from_raw_borrowed(&punkcontext)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgComponent as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgComponentBindings_Impl: Sized {
    fn BindTo(&self, pnccitem: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
    fn UnbindFrom(&self, pnccitem: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
    fn SupportsBindingInterface(&self, dwflags: u32, pszwinterfacename: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn IsBoundTo(&self, pnccitem: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
    fn IsBindableTo(&self, pnccitem: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
    fn EnumBindingPaths(&self, dwflags: u32, ppienum: *mut ::core::option::Option<IEnumNetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn MoveBefore(&self, pncbitemsrc: ::core::option::Option<&INetCfgBindingPath>, pncbitemdest: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn MoveAfter(&self, pncbitemsrc: ::core::option::Option<&INetCfgBindingPath>, pncbitemdest: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgComponentBindings {}
impl INetCfgComponentBindings_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>() -> INetCfgComponentBindings_Vtbl {
        unsafe extern "system" fn BindTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BindTo(::windows_core::from_raw_borrowed(&pnccitem)).into()
        }
        unsafe extern "system" fn UnbindFrom<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UnbindFrom(::windows_core::from_raw_borrowed(&pnccitem)).into()
        }
        unsafe extern "system" fn SupportsBindingInterface<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszwinterfacename: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SupportsBindingInterface(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&pszwinterfacename)).into()
        }
        unsafe extern "system" fn IsBoundTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsBoundTo(::windows_core::from_raw_borrowed(&pnccitem)).into()
        }
        unsafe extern "system" fn IsBindableTo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsBindableTo(::windows_core::from_raw_borrowed(&pnccitem)).into()
        }
        unsafe extern "system" fn EnumBindingPaths<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumBindingPaths(::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ppienum)).into()
        }
        unsafe extern "system" fn MoveBefore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: *mut ::core::ffi::c_void, pncbitemdest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveBefore(::windows_core::from_raw_borrowed(&pncbitemsrc), ::windows_core::from_raw_borrowed(&pncbitemdest)).into()
        }
        unsafe extern "system" fn MoveAfter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentBindings_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: *mut ::core::ffi::c_void, pncbitemdest: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MoveAfter(::windows_core::from_raw_borrowed(&pncbitemsrc), ::windows_core::from_raw_borrowed(&pncbitemdest)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgComponentBindings as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentControl_Impl: Sized {
    fn Initialize(&self, picomp: ::core::option::Option<&INetCfgComponent>, pinetcfg: ::core::option::Option<&INetCfg>, finstalling: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn ApplyRegistryChanges(&self) -> ::windows_core::Result<()>;
    fn ApplyPnpChanges(&self, picallback: ::core::option::Option<&INetCfgPnpReconfigCallback>) -> ::windows_core::Result<()>;
    fn CancelChanges(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for INetCfgComponentControl {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>() -> INetCfgComponentControl_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: *mut ::core::ffi::c_void, pinetcfg: *mut ::core::ffi::c_void, finstalling: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&picomp), ::windows_core::from_raw_borrowed(&pinetcfg), ::core::mem::transmute_copy(&finstalling)).into()
        }
        unsafe extern "system" fn ApplyRegistryChanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyRegistryChanges().into()
        }
        unsafe extern "system" fn ApplyPnpChanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyPnpChanges(::windows_core::from_raw_borrowed(&picallback)).into()
        }
        unsafe extern "system" fn CancelChanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelChanges().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            ApplyRegistryChanges: ApplyRegistryChanges::<Identity, Impl, OFFSET>,
            ApplyPnpChanges: ApplyPnpChanges::<Identity, Impl, OFFSET>,
            CancelChanges: CancelChanges::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgComponentControl as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgComponentNotifyBinding_Impl: Sized {
    fn QueryBindingPath(&self, dwchangeflag: u32, pipath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn NotifyBindingPath(&self, dwchangeflag: u32, pipath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgComponentNotifyBinding {}
impl INetCfgComponentNotifyBinding_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: isize>() -> INetCfgComponentNotifyBinding_Vtbl {
        unsafe extern "system" fn QueryBindingPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&pipath)).into()
        }
        unsafe extern "system" fn NotifyBindingPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyBinding_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&pipath)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryBindingPath: QueryBindingPath::<Identity, Impl, OFFSET>,
            NotifyBindingPath: NotifyBindingPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyBinding as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgComponentNotifyGlobal_Impl: Sized {
    fn GetSupportedNotifications(&self) -> ::windows_core::Result<u32>;
    fn SysQueryBindingPath(&self, dwchangeflag: u32, pipath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn SysNotifyBindingPath(&self, dwchangeflag: u32, pipath: ::core::option::Option<&INetCfgBindingPath>) -> ::windows_core::Result<()>;
    fn SysNotifyComponent(&self, dwchangeflag: u32, picomp: ::core::option::Option<&INetCfgComponent>) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgComponentNotifyGlobal {}
impl INetCfgComponentNotifyGlobal_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>() -> INetCfgComponentNotifyGlobal_Vtbl {
        unsafe extern "system" fn GetSupportedNotifications<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnotifications: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSupportedNotifications() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(dwnotifications, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysQueryBindingPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SysQueryBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyBindingPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SysNotifyBindingPath(::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&pipath)).into()
        }
        unsafe extern "system" fn SysNotifyComponent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentNotifyGlobal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, picomp: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SysNotifyComponent(::core::mem::transmute_copy(&dwchangeflag), ::windows_core::from_raw_borrowed(&picomp)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSupportedNotifications: GetSupportedNotifications::<Identity, Impl, OFFSET>,
            SysQueryBindingPath: SysQueryBindingPath::<Identity, Impl, OFFSET>,
            SysNotifyBindingPath: SysNotifyBindingPath::<Identity, Impl, OFFSET>,
            SysNotifyComponent: SysNotifyComponent::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyGlobal as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentPropertyUi_Impl: Sized {
    fn QueryPropertyUi(&self, punkreserved: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn SetContext(&self, punkreserved: ::core::option::Option<&::windows_core::IUnknown>) -> ::windows_core::Result<()>;
    fn MergePropPages(&self, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const ::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn ValidateProperties(&self, hwndsheet: super::super::Foundation::HWND) -> ::windows_core::Result<()>;
    fn ApplyProperties(&self) -> ::windows_core::Result<()>;
    fn CancelProperties(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for INetCfgComponentPropertyUi {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentPropertyUi_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>() -> INetCfgComponentPropertyUi_Vtbl {
        unsafe extern "system" fn QueryPropertyUi<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryPropertyUi(::windows_core::from_raw_borrowed(&punkreserved)).into()
        }
        unsafe extern "system" fn SetContext<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetContext(::windows_core::from_raw_borrowed(&punkreserved)).into()
        }
        unsafe extern "system" fn MergePropPages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.MergePropPages(::core::mem::transmute_copy(&pdwdefpages), ::core::mem::transmute_copy(&pahpspprivate), ::core::mem::transmute_copy(&pcpages), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&pszstartpage)).into()
        }
        unsafe extern "system" fn ValidateProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ValidateProperties(::core::mem::transmute_copy(&hwndsheet)).into()
        }
        unsafe extern "system" fn ApplyProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ApplyProperties().into()
        }
        unsafe extern "system" fn CancelProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentPropertyUi_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelProperties().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryPropertyUi: QueryPropertyUi::<Identity, Impl, OFFSET>,
            SetContext: SetContext::<Identity, Impl, OFFSET>,
            MergePropPages: MergePropPages::<Identity, Impl, OFFSET>,
            ValidateProperties: ValidateProperties::<Identity, Impl, OFFSET>,
            ApplyProperties: ApplyProperties::<Identity, Impl, OFFSET>,
            CancelProperties: CancelProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgComponentPropertyUi as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgComponentSetup_Impl: Sized {
    fn Install(&self, dwsetupflags: u32) -> ::windows_core::Result<()>;
    fn Upgrade(&self, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows_core::Result<()>;
    fn ReadAnswerFile(&self, pszwanswerfile: &::windows_core::PCWSTR, pszwanswersections: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Removing(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgComponentSetup {}
impl INetCfgComponentSetup_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>() -> INetCfgComponentSetup_Vtbl {
        unsafe extern "system" fn Install<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Install(::core::mem::transmute_copy(&dwsetupflags)).into()
        }
        unsafe extern "system" fn Upgrade<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Upgrade(::core::mem::transmute_copy(&dwsetupflags), ::core::mem::transmute_copy(&dwupgradefombuildno)).into()
        }
        unsafe extern "system" fn ReadAnswerFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: ::windows_core::PCWSTR, pszwanswersections: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadAnswerFile(::core::mem::transmute(&pszwanswerfile), ::core::mem::transmute(&pszwanswersections)).into()
        }
        unsafe extern "system" fn Removing<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSetup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Removing().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Install: Install::<Identity, Impl, OFFSET>,
            Upgrade: Upgrade::<Identity, Impl, OFFSET>,
            ReadAnswerFile: ReadAnswerFile::<Identity, Impl, OFFSET>,
            Removing: Removing::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgComponentSetup as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgComponentSysPrep_Impl: Sized {
    fn SaveAdapterParameters(&self, pncsp: ::core::option::Option<&INetCfgSysPrep>, pszwanswersections: &::windows_core::PCWSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn RestoreAdapterParameters(&self, pszwanswerfile: &::windows_core::PCWSTR, pszwanswersection: &::windows_core::PCWSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgComponentSysPrep {}
impl INetCfgComponentSysPrep_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSysPrep_Impl, const OFFSET: isize>() -> INetCfgComponentSysPrep_Vtbl {
        unsafe extern "system" fn SaveAdapterParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncsp: *mut ::core::ffi::c_void, pszwanswersections: ::windows_core::PCWSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveAdapterParameters(::windows_core::from_raw_borrowed(&pncsp), ::core::mem::transmute(&pszwanswersections), ::core::mem::transmute_copy(&padapterinstanceguid)).into()
        }
        unsafe extern "system" fn RestoreAdapterParameters<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: ::windows_core::PCWSTR, pszwanswersection: ::windows_core::PCWSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RestoreAdapterParameters(::core::mem::transmute(&pszwanswerfile), ::core::mem::transmute(&pszwanswersection), ::core::mem::transmute_copy(&padapterinstanceguid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SaveAdapterParameters: SaveAdapterParameters::<Identity, Impl, OFFSET>,
            RestoreAdapterParameters: RestoreAdapterParameters::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgComponentSysPrep as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgComponentUpperEdge_Impl: Sized {
    fn GetInterfaceIdsForAdapter(&self, padapter: ::core::option::Option<&INetCfgComponent>, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows_core::GUID) -> ::windows_core::Result<()>;
    fn AddInterfacesToAdapter(&self, padapter: ::core::option::Option<&INetCfgComponent>, dwnuminterfaces: u32) -> ::windows_core::Result<()>;
    fn RemoveInterfacesFromAdapter(&self, padapter: ::core::option::Option<&INetCfgComponent>, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgComponentUpperEdge {}
impl INetCfgComponentUpperEdge_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>() -> INetCfgComponentUpperEdge_Vtbl {
        unsafe extern "system" fn GetInterfaceIdsForAdapter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInterfaceIdsForAdapter(::windows_core::from_raw_borrowed(&padapter), ::core::mem::transmute_copy(&pdwnuminterfaces), ::core::mem::transmute_copy(&ppguidinterfaceids)).into()
        }
        unsafe extern "system" fn AddInterfacesToAdapter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, dwnuminterfaces: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddInterfacesToAdapter(::windows_core::from_raw_borrowed(&padapter), ::core::mem::transmute_copy(&dwnuminterfaces)).into()
        }
        unsafe extern "system" fn RemoveInterfacesFromAdapter<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgComponentUpperEdge_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: *mut ::core::ffi::c_void, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveInterfacesFromAdapter(::windows_core::from_raw_borrowed(&padapter), ::core::mem::transmute_copy(&dwnuminterfaces), ::core::mem::transmute_copy(&pguidinterfaceids)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInterfaceIdsForAdapter: GetInterfaceIdsForAdapter::<Identity, Impl, OFFSET>,
            AddInterfacesToAdapter: AddInterfacesToAdapter::<Identity, Impl, OFFSET>,
            RemoveInterfacesFromAdapter: RemoveInterfacesFromAdapter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgComponentUpperEdge as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgLock_Impl: Sized {
    fn AcquireWriteLock(&self, cmstimeout: u32, pszwclientdescription: &::windows_core::PCWSTR, ppszwclientdescription: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
    fn ReleaseWriteLock(&self) -> ::windows_core::Result<()>;
    fn IsWriteLocked(&self, ppszwclientdescription: *mut ::windows_core::PWSTR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgLock {}
impl INetCfgLock_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: isize>() -> INetCfgLock_Vtbl {
        unsafe extern "system" fn AcquireWriteLock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmstimeout: u32, pszwclientdescription: ::windows_core::PCWSTR, ppszwclientdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireWriteLock(::core::mem::transmute_copy(&cmstimeout), ::core::mem::transmute(&pszwclientdescription), ::core::mem::transmute_copy(&ppszwclientdescription)).into()
        }
        unsafe extern "system" fn ReleaseWriteLock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseWriteLock().into()
        }
        unsafe extern "system" fn IsWriteLocked<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgLock_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwclientdescription: *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.IsWriteLocked(::core::mem::transmute_copy(&ppszwclientdescription)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AcquireWriteLock: AcquireWriteLock::<Identity, Impl, OFFSET>,
            ReleaseWriteLock: ReleaseWriteLock::<Identity, Impl, OFFSET>,
            IsWriteLocked: IsWriteLocked::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgLock as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetCfgPnpReconfigCallback_Impl: Sized {
    fn SendPnpReconfig(&self, layer: NCPNP_RECONFIG_LAYER, pszwupper: &::windows_core::PCWSTR, pszwlower: &::windows_core::PCWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetCfgPnpReconfigCallback {}
impl INetCfgPnpReconfigCallback_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgPnpReconfigCallback_Impl, const OFFSET: isize>() -> INetCfgPnpReconfigCallback_Vtbl {
        unsafe extern "system" fn SendPnpReconfig<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgPnpReconfigCallback_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: ::windows_core::PCWSTR, pszwlower: ::windows_core::PCWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendPnpReconfig(::core::mem::transmute_copy(&layer), ::core::mem::transmute(&pszwupper), ::core::mem::transmute(&pszwlower), ::core::mem::transmute_copy(&pvdata), ::core::mem::transmute_copy(&dwsizeofdata)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SendPnpReconfig: SendPnpReconfig::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgPnpReconfigCallback as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgSysPrep_Impl: Sized {
    fn HrSetupSetFirstDword(&self, pwszsection: &::windows_core::PCWSTR, pwszkey: &::windows_core::PCWSTR, dwvalue: u32) -> ::windows_core::Result<()>;
    fn HrSetupSetFirstString(&self, pwszsection: &::windows_core::PCWSTR, pwszkey: &::windows_core::PCWSTR, pwszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn HrSetupSetFirstStringAsBool(&self, pwszsection: &::windows_core::PCWSTR, pwszkey: &::windows_core::PCWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows_core::Result<()>;
    fn HrSetupSetFirstMultiSzField(&self, pwszsection: &::windows_core::PCWSTR, pwszkey: &::windows_core::PCWSTR, pmszvalue: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for INetCfgSysPrep {}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgSysPrep_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>() -> INetCfgSysPrep_Vtbl {
        unsafe extern "system" fn HrSetupSetFirstDword<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows_core::PCWSTR, pwszkey: ::windows_core::PCWSTR, dwvalue: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HrSetupSetFirstDword(::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute_copy(&dwvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstString<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows_core::PCWSTR, pwszkey: ::windows_core::PCWSTR, pwszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HrSetupSetFirstString(::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute(&pwszvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstStringAsBool<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows_core::PCWSTR, pwszkey: ::windows_core::PCWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HrSetupSetFirstStringAsBool(::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute_copy(&fvalue)).into()
        }
        unsafe extern "system" fn HrSetupSetFirstMultiSzField<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetCfgSysPrep_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: ::windows_core::PCWSTR, pwszkey: ::windows_core::PCWSTR, pmszvalue: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.HrSetupSetFirstMultiSzField(::core::mem::transmute(&pwszsection), ::core::mem::transmute(&pwszkey), ::core::mem::transmute(&pmszvalue)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HrSetupSetFirstDword: HrSetupSetFirstDword::<Identity, Impl, OFFSET>,
            HrSetupSetFirstString: HrSetupSetFirstString::<Identity, Impl, OFFSET>,
            HrSetupSetFirstStringAsBool: HrSetupSetFirstStringAsBool::<Identity, Impl, OFFSET>,
            HrSetupSetFirstMultiSzField: HrSetupSetFirstMultiSzField::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetCfgSysPrep as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait INetLanConnectionUiInfo_Impl: Sized {
    fn GetDeviceGuid(&self) -> ::windows_core::Result<::windows_core::GUID>;
}
impl ::windows_core::RuntimeName for INetLanConnectionUiInfo {}
impl INetLanConnectionUiInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetLanConnectionUiInfo_Impl, const OFFSET: isize>() -> INetLanConnectionUiInfo_Vtbl {
        unsafe extern "system" fn GetDeviceGuid<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetLanConnectionUiInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDeviceGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pguid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetDeviceGuid: GetDeviceGuid::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetLanConnectionUiInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetRasConnectionIpUiInfo_Impl: Sized {
    fn GetUiInfo(&self, pinfo: *mut RASCON_IPUI) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for INetRasConnectionIpUiInfo {}
#[cfg(feature = "Win32_Foundation")]
impl INetRasConnectionIpUiInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetRasConnectionIpUiInfo_Impl, const OFFSET: isize>() -> INetRasConnectionIpUiInfo_Vtbl {
        unsafe extern "system" fn GetUiInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetRasConnectionIpUiInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetUiInfo(::core::mem::transmute_copy(&pinfo)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUiInfo: GetUiInfo::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetRasConnectionIpUiInfo as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IProvisioningDomain_Impl: Sized {
    fn Add(&self, pszwpathtofolder: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
    fn Query(&self, pszwdomain: &::windows_core::PCWSTR, pszwlanguage: &::windows_core::PCWSTR, pszwxpathquery: &::windows_core::PCWSTR) -> ::windows_core::Result<super::super::Data::Xml::MsXml::IXMLDOMNodeList>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IProvisioningDomain {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IProvisioningDomain_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningDomain_Impl, const OFFSET: isize>() -> IProvisioningDomain_Vtbl {
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwpathtofolder: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&pszwpathtofolder)).into()
        }
        unsafe extern "system" fn Query<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningDomain_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdomain: ::windows_core::PCWSTR, pszwlanguage: ::windows_core::PCWSTR, pszwxpathquery: ::windows_core::PCWSTR, nodes: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Query(::core::mem::transmute(&pszwdomain), ::core::mem::transmute(&pszwlanguage), ::core::mem::transmute(&pszwxpathquery)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(nodes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Add: Add::<Identity, Impl, OFFSET>, Query: Query::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProvisioningDomain as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetManagement\"`, `\"implement\"`*"]
pub trait IProvisioningProfileWireless_Impl: Sized {
    fn CreateProfile(&self, bstrxmlwirelessconfigprofile: &::windows_core::BSTR, bstrxmlconnectionconfigprofile: &::windows_core::BSTR, padapterinstanceguid: *const ::windows_core::GUID) -> ::windows_core::Result<u32>;
}
impl ::windows_core::RuntimeName for IProvisioningProfileWireless {}
impl IProvisioningProfileWireless_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningProfileWireless_Impl, const OFFSET: isize>() -> IProvisioningProfileWireless_Vtbl {
        unsafe extern "system" fn CreateProfile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProvisioningProfileWireless_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmlwirelessconfigprofile: ::std::mem::MaybeUninit<::windows_core::BSTR>, bstrxmlconnectionconfigprofile: ::std::mem::MaybeUninit<::windows_core::BSTR>, padapterinstanceguid: *const ::windows_core::GUID, pulstatus: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateProfile(::core::mem::transmute(&bstrxmlwirelessconfigprofile), ::core::mem::transmute(&bstrxmlconnectionconfigprofile), ::core::mem::transmute_copy(&padapterinstanceguid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pulstatus, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateProfile: CreateProfile::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProvisioningProfileWireless as ::windows_core::ComInterface>::IID
    }
}
