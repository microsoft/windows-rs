pub trait IEnumNetCfgBindingInterfaceImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumNetCfgBindingInterface {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.IEnumNetCfgBindingInterface";
}
impl IEnumNetCfgBindingInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>() -> IEnumNetCfgBindingInterfaceVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumNetCfgBindingInterface>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumNetCfgBindingPathImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumNetCfgBindingPath {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.IEnumNetCfgBindingPath";
}
impl IEnumNetCfgBindingPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>() -> IEnumNetCfgBindingPathVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumNetCfgBindingPath>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumNetCfgComponentImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumNetCfgComponent {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.IEnumNetCfgComponent";
}
impl IEnumNetCfgComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>() -> IEnumNetCfgComponentVtbl {
        unsafe extern "system" fn Next<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumNetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumNetCfgComponent>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait INetCfgImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn Apply();
    fn Cancel();
    fn EnumComponents();
    fn FindComponent();
    fn QueryNetCfgClass();
}
impl ::windows::core::RuntimeName for INetCfg {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfg";
}
impl INetCfgVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgImpl, const OFFSET: isize>() -> INetCfgVtbl {
        unsafe extern "system" fn Initialize<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pvreserved as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Uninitialize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Apply<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Apply() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Cancel<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Cancel() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumComponents<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumComponents(&*(&pguidclass as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppenumcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FindComponent<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindComponent(&*(&pszwinfid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryNetCfgClass<Impl: INetCfgImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguidclass: *const ::windows::core::GUID, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryNetCfgClass(&*(&pguidclass as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), &*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppvobject)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfg>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, Uninitialize::<Impl, OFFSET>, Apply::<Impl, OFFSET>, Cancel::<Impl, OFFSET>, EnumComponents::<Impl, OFFSET>, FindComponent::<Impl, OFFSET>, QueryNetCfgClass::<Impl, OFFSET>)
    }
}
pub trait INetCfgBindingInterfaceImpl: Sized {
    fn GetName();
    fn GetUpperComponent();
    fn GetLowerComponent();
}
impl ::windows::core::RuntimeName for INetCfgBindingInterface {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgBindingInterface";
}
impl INetCfgBindingInterfaceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>() -> INetCfgBindingInterfaceVtbl {
        unsafe extern "system" fn GetName<Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwinterfacename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetName(::core::mem::transmute_copy(&ppszwinterfacename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUpperComponent<Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUpperComponent(::core::mem::transmute_copy(&ppnccitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLowerComponent<Impl: INetCfgBindingInterfaceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLowerComponent(::core::mem::transmute_copy(&ppnccitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgBindingInterface>, ::windows::core::GetTrustLevel, GetName::<Impl, OFFSET>, GetUpperComponent::<Impl, OFFSET>, GetLowerComponent::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for INetCfgBindingPath {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgBindingPath";
}
impl INetCfgBindingPathVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgBindingPathImpl, const OFFSET: isize>() -> INetCfgBindingPathVtbl {
        unsafe extern "system" fn IsSamePathAs<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSamePathAs(&*(&ppath as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSubPathOf<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSubPathOf(&*(&ppath as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsEnabled<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enable<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fenable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enable(&*(&fenable as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPathToken<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwpathtoken: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPathToken(::core::mem::transmute_copy(&ppszwpathtoken)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOwner<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOwner(::core::mem::transmute_copy(&ppcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDepth<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcinterfaces: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDepth(::core::mem::transmute_copy(&pcinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumBindingInterfaces<Impl: INetCfgBindingPathImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenuminterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumBindingInterfaces(::core::mem::transmute_copy(&ppenuminterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INetCfgBindingPath>,
            ::windows::core::GetTrustLevel,
            IsSamePathAs::<Impl, OFFSET>,
            IsSubPathOf::<Impl, OFFSET>,
            IsEnabled::<Impl, OFFSET>,
            Enable::<Impl, OFFSET>,
            GetPathToken::<Impl, OFFSET>,
            GetOwner::<Impl, OFFSET>,
            GetDepth::<Impl, OFFSET>,
            EnumBindingInterfaces::<Impl, OFFSET>,
        )
    }
}
pub trait INetCfgClassImpl: Sized {
    fn FindComponent();
    fn EnumComponents();
}
impl ::windows::core::RuntimeName for INetCfgClass {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgClass";
}
impl INetCfgClassVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassImpl, const OFFSET: isize>() -> INetCfgClassVtbl {
        unsafe extern "system" fn FindComponent<Impl: INetCfgClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindComponent(&*(&pszwinfid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnccitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumComponents<Impl: INetCfgClassImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumcomponent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumComponents(::core::mem::transmute_copy(&ppenumcomponent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgClass>, ::windows::core::GetTrustLevel, FindComponent::<Impl, OFFSET>, EnumComponents::<Impl, OFFSET>)
    }
}
pub trait INetCfgClassSetupImpl: Sized {
    fn SelectAndInstall();
    fn Install();
    fn DeInstall();
}
impl ::windows::core::RuntimeName for INetCfgClassSetup {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgClassSetup";
}
impl INetCfgClassSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassSetupImpl, const OFFSET: isize>() -> INetCfgClassSetupVtbl {
        unsafe extern "system" fn SelectAndInstall<Impl: INetCfgClassSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, pobotoken: *const OBO_TOKEN, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SelectAndInstall(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&pobotoken as *const <OBO_TOKEN as ::windows::core::Abi>::Abi as *const <OBO_TOKEN as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnccitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Install<Impl: INetCfgClassSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwinfid: super::super::Foundation::PWSTR, pobotoken: *const OBO_TOKEN, dwsetupflags: u32, dwupgradefrombuildno: u32, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR, ppnccitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Install(
                &*(&pszwinfid as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pobotoken as *const <OBO_TOKEN as ::windows::core::Abi>::Abi as *const <OBO_TOKEN as ::windows::core::DefaultType>::DefaultType),
                dwsetupflags,
                dwupgradefrombuildno,
                &*(&pszwanswerfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszwanswersections as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppnccitem),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeInstall<Impl: INetCfgClassSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcomponent: ::windows::core::RawPtr, pobotoken: *const OBO_TOKEN, pmszwrefs: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeInstall(&*(&pcomponent as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType), &*(&pobotoken as *const <OBO_TOKEN as ::windows::core::Abi>::Abi as *const <OBO_TOKEN as ::windows::core::DefaultType>::DefaultType), &*(&pmszwrefs as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgClassSetup>, ::windows::core::GetTrustLevel, SelectAndInstall::<Impl, OFFSET>, Install::<Impl, OFFSET>, DeInstall::<Impl, OFFSET>)
    }
}
pub trait INetCfgClassSetup2Impl: Sized + INetCfgClassSetupImpl {
    fn UpdateNonEnumeratedComponent();
}
impl ::windows::core::RuntimeName for INetCfgClassSetup2 {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgClassSetup2";
}
impl INetCfgClassSetup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgClassSetup2Impl, const OFFSET: isize>() -> INetCfgClassSetup2Vtbl {
        unsafe extern "system" fn UpdateNonEnumeratedComponent<Impl: INetCfgClassSetup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, dwsetupflags: u32, dwupgradefrombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateNonEnumeratedComponent(&*(&picomp as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType), dwsetupflags, dwupgradefrombuildno) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgClassSetup2>, ::windows::core::GetTrustLevel, UpdateNonEnumeratedComponent::<Impl, OFFSET>)
    }
}
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
impl ::windows::core::RuntimeName for INetCfgComponent {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgComponent";
}
impl INetCfgComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentImpl, const OFFSET: isize>() -> INetCfgComponentVtbl {
        unsafe extern "system" fn GetDisplayName<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdisplayname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayName(::core::mem::transmute_copy(&ppszwdisplayname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisplayName<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdisplayname: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisplayName(&*(&pszwdisplayname as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetHelpText<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwhelptext: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetHelpText(::core::mem::transmute_copy(&pszwhelptext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetId<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetId(::core::mem::transmute_copy(&ppszwid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCharacteristics<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwcharacteristics: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCharacteristics(::core::mem::transmute_copy(&pdwcharacteristics)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInstanceGuid<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInstanceGuid(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPnpDevNodeId<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwdevnodeid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPnpDevNodeId(::core::mem::transmute_copy(&ppszwdevnodeid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetClassGuid<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetClassGuid(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBindName<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwbindname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindName(::core::mem::transmute_copy(&ppszwbindname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceStatus<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceStatus(::core::mem::transmute_copy(&pulstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenParamKey<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenParamKey(::core::mem::transmute_copy(&phkey)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RaisePropertyUi<Impl: INetCfgComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, dwflags: u32, punkcontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RaisePropertyUi(&*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags, &*(&punkcontext as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INetCfgComponent>,
            ::windows::core::GetTrustLevel,
            GetDisplayName::<Impl, OFFSET>,
            SetDisplayName::<Impl, OFFSET>,
            GetHelpText::<Impl, OFFSET>,
            GetId::<Impl, OFFSET>,
            GetCharacteristics::<Impl, OFFSET>,
            GetInstanceGuid::<Impl, OFFSET>,
            GetPnpDevNodeId::<Impl, OFFSET>,
            GetClassGuid::<Impl, OFFSET>,
            GetBindName::<Impl, OFFSET>,
            GetDeviceStatus::<Impl, OFFSET>,
            OpenParamKey::<Impl, OFFSET>,
            RaisePropertyUi::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for INetCfgComponentBindings {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgComponentBindings";
}
impl INetCfgComponentBindingsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>() -> INetCfgComponentBindingsVtbl {
        unsafe extern "system" fn BindTo<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BindTo(&*(&pnccitem as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnbindFrom<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnbindFrom(&*(&pnccitem as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportsBindingInterface<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, pszwinterfacename: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportsBindingInterface(dwflags, &*(&pszwinterfacename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBoundTo<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBoundTo(&*(&pnccitem as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBindableTo<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnccitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBindableTo(&*(&pnccitem as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumBindingPaths<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwflags: u32, ppienum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumBindingPaths(dwflags, ::core::mem::transmute_copy(&ppienum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveBefore<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveBefore(&*(&pncbitemsrc as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType), &*(&pncbitemdest as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MoveAfter<Impl: INetCfgComponentBindingsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncbitemsrc: ::windows::core::RawPtr, pncbitemdest: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MoveAfter(&*(&pncbitemsrc as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType), &*(&pncbitemdest as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<INetCfgComponentBindings>,
            ::windows::core::GetTrustLevel,
            BindTo::<Impl, OFFSET>,
            UnbindFrom::<Impl, OFFSET>,
            SupportsBindingInterface::<Impl, OFFSET>,
            IsBoundTo::<Impl, OFFSET>,
            IsBindableTo::<Impl, OFFSET>,
            EnumBindingPaths::<Impl, OFFSET>,
            MoveBefore::<Impl, OFFSET>,
            MoveAfter::<Impl, OFFSET>,
        )
    }
}
pub trait INetCfgComponentControlImpl: Sized {
    fn Initialize();
    fn ApplyRegistryChanges();
    fn ApplyPnpChanges();
    fn CancelChanges();
}
impl ::windows::core::RuntimeName for INetCfgComponentControl {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgComponentControl";
}
impl INetCfgComponentControlVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentControlImpl, const OFFSET: isize>() -> INetCfgComponentControlVtbl {
        unsafe extern "system" fn Initialize<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picomp: ::windows::core::RawPtr, pinetcfg: ::windows::core::RawPtr, finstalling: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&picomp as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType), &*(&pinetcfg as *const <INetCfg as ::windows::core::Abi>::Abi as *const <INetCfg as ::windows::core::DefaultType>::DefaultType), &*(&finstalling as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyRegistryChanges<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyRegistryChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyPnpChanges<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, picallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyPnpChanges(&*(&picallback as *const <INetCfgPnpReconfigCallback as ::windows::core::Abi>::Abi as *const <INetCfgPnpReconfigCallback as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelChanges<Impl: INetCfgComponentControlImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelChanges() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgComponentControl>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, ApplyRegistryChanges::<Impl, OFFSET>, ApplyPnpChanges::<Impl, OFFSET>, CancelChanges::<Impl, OFFSET>)
    }
}
pub trait INetCfgComponentNotifyBindingImpl: Sized {
    fn QueryBindingPath();
    fn NotifyBindingPath();
}
impl ::windows::core::RuntimeName for INetCfgComponentNotifyBinding {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgComponentNotifyBinding";
}
impl INetCfgComponentNotifyBindingVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentNotifyBindingImpl, const OFFSET: isize>() -> INetCfgComponentNotifyBindingVtbl {
        unsafe extern "system" fn QueryBindingPath<Impl: INetCfgComponentNotifyBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryBindingPath(dwchangeflag, &*(&pipath as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyBindingPath<Impl: INetCfgComponentNotifyBindingImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyBindingPath(dwchangeflag, &*(&pipath as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgComponentNotifyBinding>, ::windows::core::GetTrustLevel, QueryBindingPath::<Impl, OFFSET>, NotifyBindingPath::<Impl, OFFSET>)
    }
}
pub trait INetCfgComponentNotifyGlobalImpl: Sized {
    fn GetSupportedNotifications();
    fn SysQueryBindingPath();
    fn SysNotifyBindingPath();
    fn SysNotifyComponent();
}
impl ::windows::core::RuntimeName for INetCfgComponentNotifyGlobal {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgComponentNotifyGlobal";
}
impl INetCfgComponentNotifyGlobalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>() -> INetCfgComponentNotifyGlobalVtbl {
        unsafe extern "system" fn GetSupportedNotifications<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwnotifications: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedNotifications(::core::mem::transmute_copy(&dwnotifications)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysQueryBindingPath<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SysQueryBindingPath(dwchangeflag, &*(&pipath as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysNotifyBindingPath<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, pipath: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SysNotifyBindingPath(dwchangeflag, &*(&pipath as *const <INetCfgBindingPath as ::windows::core::Abi>::Abi as *const <INetCfgBindingPath as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SysNotifyComponent<Impl: INetCfgComponentNotifyGlobalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwchangeflag: u32, picomp: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SysNotifyComponent(dwchangeflag, &*(&picomp as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgComponentNotifyGlobal>, ::windows::core::GetTrustLevel, GetSupportedNotifications::<Impl, OFFSET>, SysQueryBindingPath::<Impl, OFFSET>, SysNotifyBindingPath::<Impl, OFFSET>, SysNotifyComponent::<Impl, OFFSET>)
    }
}
pub trait INetCfgComponentPropertyUiImpl: Sized {
    fn QueryPropertyUi();
    fn SetContext();
    fn MergePropPages();
    fn ValidateProperties();
    fn ApplyProperties();
    fn CancelProperties();
}
impl ::windows::core::RuntimeName for INetCfgComponentPropertyUi {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgComponentPropertyUi";
}
impl INetCfgComponentPropertyUiVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>() -> INetCfgComponentPropertyUiVtbl {
        unsafe extern "system" fn QueryPropertyUi<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryPropertyUi(&*(&punkreserved as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetContext<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, punkreserved: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetContext(&*(&punkreserved as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MergePropPages<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwdefpages: *mut u32, pahpspprivate: *mut *mut u8, pcpages: *mut u32, hwndparent: super::super::Foundation::HWND, pszstartpage: *const super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MergePropPages(pdwdefpages, ::core::mem::transmute_copy(&pahpspprivate), ::core::mem::transmute_copy(&pcpages), &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&pszstartpage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ValidateProperties<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hwndsheet: super::super::Foundation::HWND) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ValidateProperties(&*(&hwndsheet as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyProperties<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelProperties<Impl: INetCfgComponentPropertyUiImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgComponentPropertyUi>, ::windows::core::GetTrustLevel, QueryPropertyUi::<Impl, OFFSET>, SetContext::<Impl, OFFSET>, MergePropPages::<Impl, OFFSET>, ValidateProperties::<Impl, OFFSET>, ApplyProperties::<Impl, OFFSET>, CancelProperties::<Impl, OFFSET>)
    }
}
pub trait INetCfgComponentSetupImpl: Sized {
    fn Install();
    fn Upgrade();
    fn ReadAnswerFile();
    fn Removing();
}
impl ::windows::core::RuntimeName for INetCfgComponentSetup {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgComponentSetup";
}
impl INetCfgComponentSetupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentSetupImpl, const OFFSET: isize>() -> INetCfgComponentSetupVtbl {
        unsafe extern "system" fn Install<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Install(dwsetupflags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Upgrade<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dwsetupflags: u32, dwupgradefombuildno: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Upgrade(dwsetupflags, dwupgradefombuildno) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadAnswerFile<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersections: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadAnswerFile(&*(&pszwanswerfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pszwanswersections as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Removing<Impl: INetCfgComponentSetupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Removing() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgComponentSetup>, ::windows::core::GetTrustLevel, Install::<Impl, OFFSET>, Upgrade::<Impl, OFFSET>, ReadAnswerFile::<Impl, OFFSET>, Removing::<Impl, OFFSET>)
    }
}
pub trait INetCfgComponentSysPrepImpl: Sized {
    fn SaveAdapterParameters();
    fn RestoreAdapterParameters();
}
impl ::windows::core::RuntimeName for INetCfgComponentSysPrep {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgComponentSysPrep";
}
impl INetCfgComponentSysPrepVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentSysPrepImpl, const OFFSET: isize>() -> INetCfgComponentSysPrepVtbl {
        unsafe extern "system" fn SaveAdapterParameters<Impl: INetCfgComponentSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pncsp: ::windows::core::RawPtr, pszwanswersections: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveAdapterParameters(
                &*(&pncsp as *const <INetCfgSysPrep as ::windows::core::Abi>::Abi as *const <INetCfgSysPrep as ::windows::core::DefaultType>::DefaultType),
                &*(&pszwanswersections as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&padapterinstanceguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RestoreAdapterParameters<Impl: INetCfgComponentSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwanswerfile: super::super::Foundation::PWSTR, pszwanswersection: super::super::Foundation::PWSTR, padapterinstanceguid: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RestoreAdapterParameters(
                &*(&pszwanswerfile as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszwanswersection as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&padapterinstanceguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgComponentSysPrep>, ::windows::core::GetTrustLevel, SaveAdapterParameters::<Impl, OFFSET>, RestoreAdapterParameters::<Impl, OFFSET>)
    }
}
pub trait INetCfgComponentUpperEdgeImpl: Sized {
    fn GetInterfaceIdsForAdapter();
    fn AddInterfacesToAdapter();
    fn RemoveInterfacesFromAdapter();
}
impl ::windows::core::RuntimeName for INetCfgComponentUpperEdge {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgComponentUpperEdge";
}
impl INetCfgComponentUpperEdgeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>() -> INetCfgComponentUpperEdgeVtbl {
        unsafe extern "system" fn GetInterfaceIdsForAdapter<Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, pdwnuminterfaces: *mut u32, ppguidinterfaceids: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterfaceIdsForAdapter(&*(&padapter as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwnuminterfaces), ::core::mem::transmute_copy(&ppguidinterfaceids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddInterfacesToAdapter<Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddInterfacesToAdapter(&*(&padapter as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType), dwnuminterfaces) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveInterfacesFromAdapter<Impl: INetCfgComponentUpperEdgeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, padapter: ::windows::core::RawPtr, dwnuminterfaces: u32, pguidinterfaceids: *const ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveInterfacesFromAdapter(&*(&padapter as *const <INetCfgComponent as ::windows::core::Abi>::Abi as *const <INetCfgComponent as ::windows::core::DefaultType>::DefaultType), dwnuminterfaces, &*(&pguidinterfaceids as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgComponentUpperEdge>, ::windows::core::GetTrustLevel, GetInterfaceIdsForAdapter::<Impl, OFFSET>, AddInterfacesToAdapter::<Impl, OFFSET>, RemoveInterfacesFromAdapter::<Impl, OFFSET>)
    }
}
pub trait INetCfgLockImpl: Sized {
    fn AcquireWriteLock();
    fn ReleaseWriteLock();
    fn IsWriteLocked();
}
impl ::windows::core::RuntimeName for INetCfgLock {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgLock";
}
impl INetCfgLockVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgLockImpl, const OFFSET: isize>() -> INetCfgLockVtbl {
        unsafe extern "system" fn AcquireWriteLock<Impl: INetCfgLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cmstimeout: u32, pszwclientdescription: super::super::Foundation::PWSTR, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireWriteLock(cmstimeout, &*(&pszwclientdescription as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppszwclientdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseWriteLock<Impl: INetCfgLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseWriteLock() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsWriteLocked<Impl: INetCfgLockImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppszwclientdescription: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsWriteLocked(::core::mem::transmute_copy(&ppszwclientdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgLock>, ::windows::core::GetTrustLevel, AcquireWriteLock::<Impl, OFFSET>, ReleaseWriteLock::<Impl, OFFSET>, IsWriteLocked::<Impl, OFFSET>)
    }
}
pub trait INetCfgPnpReconfigCallbackImpl: Sized {
    fn SendPnpReconfig();
}
impl ::windows::core::RuntimeName for INetCfgPnpReconfigCallback {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgPnpReconfigCallback";
}
impl INetCfgPnpReconfigCallbackVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgPnpReconfigCallbackImpl, const OFFSET: isize>() -> INetCfgPnpReconfigCallbackVtbl {
        unsafe extern "system" fn SendPnpReconfig<Impl: INetCfgPnpReconfigCallbackImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, layer: NCPNP_RECONFIG_LAYER, pszwupper: super::super::Foundation::PWSTR, pszwlower: super::super::Foundation::PWSTR, pvdata: *const ::core::ffi::c_void, dwsizeofdata: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendPnpReconfig(
                layer,
                &*(&pszwupper as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszwlower as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pvdata as *const <::core::ffi::c_void as ::windows::core::Abi>::Abi as *const <::core::ffi::c_void as ::windows::core::DefaultType>::DefaultType),
                dwsizeofdata,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgPnpReconfigCallback>, ::windows::core::GetTrustLevel, SendPnpReconfig::<Impl, OFFSET>)
    }
}
pub trait INetCfgSysPrepImpl: Sized {
    fn HrSetupSetFirstDword();
    fn HrSetupSetFirstString();
    fn HrSetupSetFirstStringAsBool();
    fn HrSetupSetFirstMultiSzField();
}
impl ::windows::core::RuntimeName for INetCfgSysPrep {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetCfgSysPrep";
}
impl INetCfgSysPrepVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetCfgSysPrepImpl, const OFFSET: isize>() -> INetCfgSysPrepVtbl {
        unsafe extern "system" fn HrSetupSetFirstDword<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, dwvalue: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrSetupSetFirstDword(&*(&pwszsection as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrSetupSetFirstString<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pwszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrSetupSetFirstString(
                &*(&pwszsection as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrSetupSetFirstStringAsBool<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, fvalue: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrSetupSetFirstStringAsBool(
                &*(&pwszsection as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&fvalue as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HrSetupSetFirstMultiSzField<Impl: INetCfgSysPrepImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pwszsection: super::super::Foundation::PWSTR, pwszkey: super::super::Foundation::PWSTR, pmszvalue: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).HrSetupSetFirstMultiSzField(
                &*(&pwszsection as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszkey as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pmszvalue as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetCfgSysPrep>, ::windows::core::GetTrustLevel, HrSetupSetFirstDword::<Impl, OFFSET>, HrSetupSetFirstString::<Impl, OFFSET>, HrSetupSetFirstStringAsBool::<Impl, OFFSET>, HrSetupSetFirstMultiSzField::<Impl, OFFSET>)
    }
}
pub trait INetLanConnectionUiInfoImpl: Sized {
    fn GetDeviceGuid();
}
impl ::windows::core::RuntimeName for INetLanConnectionUiInfo {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetLanConnectionUiInfo";
}
impl INetLanConnectionUiInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetLanConnectionUiInfoImpl, const OFFSET: isize>() -> INetLanConnectionUiInfoVtbl {
        unsafe extern "system" fn GetDeviceGuid<Impl: INetLanConnectionUiInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceGuid(::core::mem::transmute_copy(&pguid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetLanConnectionUiInfo>, ::windows::core::GetTrustLevel, GetDeviceGuid::<Impl, OFFSET>)
    }
}
pub trait INetRasConnectionIpUiInfoImpl: Sized {
    fn GetUiInfo();
}
impl ::windows::core::RuntimeName for INetRasConnectionIpUiInfo {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.INetRasConnectionIpUiInfo";
}
impl INetRasConnectionIpUiInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: INetRasConnectionIpUiInfoImpl, const OFFSET: isize>() -> INetRasConnectionIpUiInfoVtbl {
        unsafe extern "system" fn GetUiInfo<Impl: INetRasConnectionIpUiInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pinfo: *mut RASCON_IPUI) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUiInfo(::core::mem::transmute_copy(&pinfo)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<INetRasConnectionIpUiInfo>, ::windows::core::GetTrustLevel, GetUiInfo::<Impl, OFFSET>)
    }
}
pub trait IProvisioningDomainImpl: Sized {
    fn Add();
    fn Query();
}
impl ::windows::core::RuntimeName for IProvisioningDomain {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.IProvisioningDomain";
}
impl IProvisioningDomainVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningDomainImpl, const OFFSET: isize>() -> IProvisioningDomainVtbl {
        unsafe extern "system" fn Add<Impl: IProvisioningDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwpathtofolder: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&pszwpathtofolder as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Query<Impl: IProvisioningDomainImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszwdomain: super::super::Foundation::PWSTR, pszwlanguage: super::super::Foundation::PWSTR, pszwxpathquery: super::super::Foundation::PWSTR, nodes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Query(
                &*(&pszwdomain as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszwlanguage as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pszwxpathquery as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&nodes),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvisioningDomain>, ::windows::core::GetTrustLevel, Add::<Impl, OFFSET>, Query::<Impl, OFFSET>)
    }
}
pub trait IProvisioningProfileWirelessImpl: Sized {
    fn CreateProfile();
}
impl ::windows::core::RuntimeName for IProvisioningProfileWireless {
    const NAME: &'static str = "Windows.Win32.NetworkManagement.NetManagement.IProvisioningProfileWireless";
}
impl IProvisioningProfileWirelessVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProvisioningProfileWirelessImpl, const OFFSET: isize>() -> IProvisioningProfileWirelessVtbl {
        unsafe extern "system" fn CreateProfile<Impl: IProvisioningProfileWirelessImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrxmlwirelessconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrxmlconnectionconfigprofile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, padapterinstanceguid: *const ::windows::core::GUID, pulstatus: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateProfile(
                &*(&bstrxmlwirelessconfigprofile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrxmlconnectionconfigprofile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&padapterinstanceguid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pulstatus),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProvisioningProfileWireless>, ::windows::core::GetTrustLevel, CreateProfile::<Impl, OFFSET>)
    }
}
