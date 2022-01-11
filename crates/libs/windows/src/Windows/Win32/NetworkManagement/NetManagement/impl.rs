pub trait IEnumNetCfgBindingInterfaceImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumNetCfgBindingInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgBindingInterfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetCfgBindingInterfaceVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingInterface as ::windows::core::Interface>::IID
    }
}
pub trait IEnumNetCfgBindingPathImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumNetCfgBindingPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgBindingPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetCfgBindingPathVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetCfgBindingPath as ::windows::core::Interface>::IID
    }
}
pub trait IEnumNetCfgComponentImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumNetCfgComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumNetCfgComponentVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Next::<Impl, IMPL_OFFSET>, Skip::<Impl, IMPL_OFFSET>, Reset::<Impl, IMPL_OFFSET>, Clone::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumNetCfgComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn Apply();
    fn Cancel();
    fn EnumComponents();
    fn FindComponent();
    fn QueryNetCfgClass();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgVtbl {
        unsafe extern "system" fn Initialize<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Uninitialize<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Apply<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Cancel<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumComponents<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FindComponent<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryNetCfgClass<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, Uninitialize::<Impl, IMPL_OFFSET>, Apply::<Impl, IMPL_OFFSET>, Cancel::<Impl, IMPL_OFFSET>, EnumComponents::<Impl, IMPL_OFFSET>, FindComponent::<Impl, IMPL_OFFSET>, QueryNetCfgClass::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfg as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgBindingInterfaceImpl: Sized {
    fn GetName();
    fn GetUpperComponent();
    fn GetLowerComponent();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgBindingInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgBindingInterfaceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgBindingInterfaceVtbl {
        unsafe extern "system" fn GetName<Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwinterfacename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUpperComponent<Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetLowerComponent<Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetName::<Impl, IMPL_OFFSET>, GetUpperComponent::<Impl, IMPL_OFFSET>, GetLowerComponent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgBindingInterface as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgBindingPathImpl: Sized {
    fn IsSamePathAs();
    fn IsSubPathOf();
    fn IsEnabled();
    fn Enable();
    fn GetPathToken();
    fn GetOwner();
    fn GetDepth();
    fn EnumBindingInterfaces();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgBindingPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgBindingPathImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgBindingPathVtbl {
        unsafe extern "system" fn IsSamePathAs<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSubPathOf<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsEnabled<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enable<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPathToken<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwpathtoken: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetOwner<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDepth<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinterfaces: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumBindingInterfaces<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenuminterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, IsSamePathAs::<Impl, IMPL_OFFSET>, IsSubPathOf::<Impl, IMPL_OFFSET>, IsEnabled::<Impl, IMPL_OFFSET>, Enable::<Impl, IMPL_OFFSET>, GetPathToken::<Impl, IMPL_OFFSET>, GetOwner::<Impl, IMPL_OFFSET>, GetDepth::<Impl, IMPL_OFFSET>, EnumBindingInterfaces::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgBindingPath as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassImpl: Sized {
    fn FindComponent();
    fn EnumComponents();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgClassVtbl {
        unsafe extern "system" fn FindComponent<Impl: INetCfgClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumComponents<Impl: INetCfgClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, FindComponent::<Impl, IMPL_OFFSET>, EnumComponents::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClass as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetupImpl: Sized {
    fn SelectAndInstall();
    fn Install();
    fn DeInstall();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgClassSetupVtbl {
        unsafe extern "system" fn SelectAndInstall<Impl: INetCfgClassSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Install<Impl: INetCfgClassSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeInstall<Impl: INetCfgClassSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomponent: ::windows::core::RawPtr, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SelectAndInstall::<Impl, IMPL_OFFSET>, Install::<Impl, IMPL_OFFSET>, DeInstall::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClassSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgClassSetup2Impl: Sized + INetCfgClassSetupImpl {
    fn UpdateNonEnumeratedComponent();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgClassSetup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassSetup2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgClassSetup2Vtbl {
        unsafe extern "system" fn UpdateNonEnumeratedComponent<Impl: INetCfgClassSetup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SelectAndInstall::<Impl, IMPL_OFFSET>, Install::<Impl, IMPL_OFFSET>, DeInstall::<Impl, IMPL_OFFSET>, UpdateNonEnumeratedComponent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgClassSetup2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
pub trait INetCfgComponentImpl: Sized {
    fn GetDisplayName();
    fn SetDisplayName();
    fn GetHelpText();
    fn GetId();
    fn GetCharacteristics();
    fn GetInstanceGuid();
    fn GetPnpDevNodeId();
    fn GetClassGuid();
    fn GetBindName();
    fn GetDeviceStatus();
    fn OpenParamKey();
    fn RaisePropertyUi();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
impl INetCfgComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentVtbl {
        unsafe extern "system" fn GetDisplayName<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisplayName<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdisplayname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetHelpText<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwhelptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetId<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetCharacteristics<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetInstanceGuid<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPnpDevNodeId<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdevnodeid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetClassGuid<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBindName<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwbindname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenParamKey<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RaisePropertyUi<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            GetDisplayName::<Impl, IMPL_OFFSET>,
            SetDisplayName::<Impl, IMPL_OFFSET>,
            GetHelpText::<Impl, IMPL_OFFSET>,
            GetId::<Impl, IMPL_OFFSET>,
            GetCharacteristics::<Impl, IMPL_OFFSET>,
            GetInstanceGuid::<Impl, IMPL_OFFSET>,
            GetPnpDevNodeId::<Impl, IMPL_OFFSET>,
            GetClassGuid::<Impl, IMPL_OFFSET>,
            GetBindName::<Impl, IMPL_OFFSET>,
            GetDeviceStatus::<Impl, IMPL_OFFSET>,
            OpenParamKey::<Impl, IMPL_OFFSET>,
            RaisePropertyUi::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentBindingsImpl: Sized {
    fn BindTo();
    fn UnbindFrom();
    fn SupportsBindingInterface();
    fn IsBoundTo();
    fn IsBindableTo();
    fn EnumBindingPaths();
    fn MoveBefore();
    fn MoveAfter();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentBindingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentBindingsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentBindingsVtbl {
        unsafe extern "system" fn BindTo<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UnbindFrom<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportsBindingInterface<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszwinterfacename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsBoundTo<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsBindableTo<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumBindingPaths<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveBefore<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MoveAfter<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, BindTo::<Impl, IMPL_OFFSET>, UnbindFrom::<Impl, IMPL_OFFSET>, SupportsBindingInterface::<Impl, IMPL_OFFSET>, IsBoundTo::<Impl, IMPL_OFFSET>, IsBindableTo::<Impl, IMPL_OFFSET>, EnumBindingPaths::<Impl, IMPL_OFFSET>, MoveBefore::<Impl, IMPL_OFFSET>, MoveAfter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentBindings as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentControlImpl: Sized {
    fn Initialize();
    fn ApplyRegistryChanges();
    fn ApplyPnpChanges();
    fn CancelChanges();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentControlImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentControlVtbl {
        unsafe extern "system" fn Initialize<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, pinetcfg: ::windows::core::RawPtr, finstalling: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyRegistryChanges<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyPnpChanges<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelChanges<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Initialize::<Impl, IMPL_OFFSET>, ApplyRegistryChanges::<Impl, IMPL_OFFSET>, ApplyPnpChanges::<Impl, IMPL_OFFSET>, CancelChanges::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentControl as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentNotifyBindingImpl: Sized {
    fn QueryBindingPath();
    fn NotifyBindingPath();
}
impl INetCfgComponentNotifyBindingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentNotifyBindingImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentNotifyBindingVtbl {
        unsafe extern "system" fn QueryBindingPath<Impl: INetCfgComponentNotifyBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyBindingPath<Impl: INetCfgComponentNotifyBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryBindingPath::<Impl, IMPL_OFFSET>, NotifyBindingPath::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyBinding as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentNotifyGlobalImpl: Sized {
    fn GetSupportedNotifications();
    fn SysQueryBindingPath();
    fn SysNotifyBindingPath();
    fn SysNotifyComponent();
}
impl INetCfgComponentNotifyGlobalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentNotifyGlobalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentNotifyGlobalVtbl {
        unsafe extern "system" fn GetSupportedNotifications<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnotifications: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SysQueryBindingPath<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SysNotifyBindingPath<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SysNotifyComponent<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, picomp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetSupportedNotifications::<Impl, IMPL_OFFSET>, SysQueryBindingPath::<Impl, IMPL_OFFSET>, SysNotifyBindingPath::<Impl, IMPL_OFFSET>, SysNotifyComponent::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentNotifyGlobal as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentPropertyUiImpl: Sized {
    fn QueryPropertyUi();
    fn SetContext();
    fn MergePropPages();
    fn ValidateProperties();
    fn ApplyProperties();
    fn CancelProperties();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentPropertyUiVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentPropertyUiImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentPropertyUiVtbl {
        unsafe extern "system" fn QueryPropertyUi<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetContext<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MergePropPages<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ValidateProperties<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ApplyProperties<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelProperties<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, QueryPropertyUi::<Impl, IMPL_OFFSET>, SetContext::<Impl, IMPL_OFFSET>, MergePropPages::<Impl, IMPL_OFFSET>, ValidateProperties::<Impl, IMPL_OFFSET>, ApplyProperties::<Impl, IMPL_OFFSET>, CancelProperties::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentPropertyUi as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentSetupImpl: Sized {
    fn Install();
    fn Upgrade();
    fn ReadAnswerFile();
    fn Removing();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentSetupImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentSetupVtbl {
        unsafe extern "system" fn Install<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Upgrade<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadAnswerFile<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Removing<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Install::<Impl, IMPL_OFFSET>, Upgrade::<Impl, IMPL_OFFSET>, ReadAnswerFile::<Impl, IMPL_OFFSET>, Removing::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentSetup as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgComponentSysPrepImpl: Sized {
    fn SaveAdapterParameters();
    fn RestoreAdapterParameters();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgComponentSysPrepVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentSysPrepImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentSysPrepVtbl {
        unsafe extern "system" fn SaveAdapterParameters<Impl: INetCfgComponentSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncsp: ::windows::core::RawPtr, pszwanswersections: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RestoreAdapterParameters<Impl: INetCfgComponentSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersection: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SaveAdapterParameters::<Impl, IMPL_OFFSET>, RestoreAdapterParameters::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentSysPrep as ::windows::core::Interface>::IID
    }
}
pub trait INetCfgComponentUpperEdgeImpl: Sized {
    fn GetInterfaceIdsForAdapter();
    fn AddInterfacesToAdapter();
    fn RemoveInterfacesFromAdapter();
}
impl INetCfgComponentUpperEdgeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentUpperEdgeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgComponentUpperEdgeVtbl {
        unsafe extern "system" fn GetInterfaceIdsForAdapter<Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddInterfacesToAdapter<Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveInterfacesFromAdapter<Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetInterfaceIdsForAdapter::<Impl, IMPL_OFFSET>, AddInterfacesToAdapter::<Impl, IMPL_OFFSET>, RemoveInterfacesFromAdapter::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgComponentUpperEdge as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgLockImpl: Sized {
    fn AcquireWriteLock();
    fn ReleaseWriteLock();
    fn IsWriteLocked();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgLockImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgLockVtbl {
        unsafe extern "system" fn AcquireWriteLock<Impl: INetCfgLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmstimeout: u32, pszwclientdescription: super::super::Foundation::PWSTR, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseWriteLock<Impl: INetCfgLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsWriteLocked<Impl: INetCfgLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, AcquireWriteLock::<Impl, IMPL_OFFSET>, ReleaseWriteLock::<Impl, IMPL_OFFSET>, IsWriteLocked::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgLock as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgPnpReconfigCallbackImpl: Sized {
    fn SendPnpReconfig();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgPnpReconfigCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgPnpReconfigCallbackImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgPnpReconfigCallbackVtbl {
        unsafe extern "system" fn SendPnpReconfig<Impl: INetCfgPnpReconfigCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: super::super::Foundation::PWSTR, pszwlower: super::super::Foundation::PWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, SendPnpReconfig::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgPnpReconfigCallback as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetCfgSysPrepImpl: Sized {
    fn HrSetupSetFirstDword();
    fn HrSetupSetFirstString();
    fn HrSetupSetFirstStringAsBool();
    fn HrSetupSetFirstMultiSzField();
}
#[cfg(feature = "Win32_Foundation")]
impl INetCfgSysPrepVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgSysPrepImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetCfgSysPrepVtbl {
        unsafe extern "system" fn HrSetupSetFirstDword<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HrSetupSetFirstString<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HrSetupSetFirstStringAsBool<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn HrSetupSetFirstMultiSzField<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pmszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, HrSetupSetFirstDword::<Impl, IMPL_OFFSET>, HrSetupSetFirstString::<Impl, IMPL_OFFSET>, HrSetupSetFirstStringAsBool::<Impl, IMPL_OFFSET>, HrSetupSetFirstMultiSzField::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetCfgSysPrep as ::windows::core::Interface>::IID
    }
}
pub trait INetLanConnectionUiInfoImpl: Sized {
    fn GetDeviceGuid();
}
impl INetLanConnectionUiInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetLanConnectionUiInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetLanConnectionUiInfoVtbl {
        unsafe extern "system" fn GetDeviceGuid<Impl: INetLanConnectionUiInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetDeviceGuid::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetLanConnectionUiInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait INetRasConnectionIpUiInfoImpl: Sized {
    fn GetUiInfo();
}
#[cfg(feature = "Win32_Foundation")]
impl INetRasConnectionIpUiInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetRasConnectionIpUiInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> INetRasConnectionIpUiInfoVtbl {
        unsafe extern "system" fn GetUiInfo<Impl: INetRasConnectionIpUiInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, GetUiInfo::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<INetRasConnectionIpUiInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
pub trait IProvisioningDomainImpl: Sized {
    fn Add();
    fn Query();
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
impl IProvisioningDomainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningDomainImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningDomainVtbl {
        unsafe extern "system" fn Add<Impl: IProvisioningDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwpathtofolder: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Query<Impl: IProvisioningDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdomain: super::super::Foundation::PWSTR, pszwlanguage: super::super::Foundation::PWSTR, pszwxpathquery: super::super::Foundation::PWSTR, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, Add::<Impl, IMPL_OFFSET>, Query::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningDomain as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IProvisioningProfileWirelessImpl: Sized {
    fn CreateProfile();
}
#[cfg(feature = "Win32_Foundation")]
impl IProvisioningProfileWirelessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningProfileWirelessImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProvisioningProfileWirelessVtbl {
        unsafe extern "system" fn CreateProfile<Impl: IProvisioningProfileWirelessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmlwirelessconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrxmlconnectionconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, padapterinstanceguid: *const ::windows::core::GUID, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, CreateProfile::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProvisioningProfileWireless as ::windows::core::Interface>::IID
    }
}
