pub trait IAccountingProviderConfigImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn Configure();
    fn Activate();
    fn Deactivate();
}
impl ::windows::core::RuntimeName for IAccountingProviderConfig {
    const NAME: &'static str = "Windows.Win32.Security.ExtensibleAuthenticationProtocol.IAccountingProviderConfig";
}
impl IAccountingProviderConfigVtbl {
    pub const fn new<Impl: IAccountingProviderConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAccountingProviderConfigVtbl {
        unsafe extern "system" fn Initialize<Impl: IAccountingProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, puconnectionparam: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pszmachinename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&puconnectionparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IAccountingProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uninitialize(uconnectionparam) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IAccountingProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Configure(uconnectionparam, &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags, ureserved1, ureserved2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IAccountingProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Activate(uconnectionparam, ureserved1, ureserved2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: IAccountingProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Deactivate(uconnectionparam, ureserved1, ureserved2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAccountingProviderConfig>, base.5, Initialize::<Impl, OFFSET>, Uninitialize::<Impl, OFFSET>, Configure::<Impl, OFFSET>, Activate::<Impl, OFFSET>, Deactivate::<Impl, OFFSET>)
    }
}
pub trait IAuthenticationProviderConfigImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn Configure();
    fn Activate();
    fn Deactivate();
}
impl ::windows::core::RuntimeName for IAuthenticationProviderConfig {
    const NAME: &'static str = "Windows.Win32.Security.ExtensibleAuthenticationProtocol.IAuthenticationProviderConfig";
}
impl IAuthenticationProviderConfigVtbl {
    pub const fn new<Impl: IAuthenticationProviderConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IAuthenticationProviderConfigVtbl {
        unsafe extern "system" fn Initialize<Impl: IAuthenticationProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, puconnectionparam: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pszmachinename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&puconnectionparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IAuthenticationProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uninitialize(uconnectionparam) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Configure<Impl: IAuthenticationProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Configure(uconnectionparam, &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags, ureserved1, ureserved2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Activate<Impl: IAuthenticationProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Activate(uconnectionparam, ureserved1, ureserved2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deactivate<Impl: IAuthenticationProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Deactivate(uconnectionparam, ureserved1, ureserved2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IAuthenticationProviderConfig>, base.5, Initialize::<Impl, OFFSET>, Uninitialize::<Impl, OFFSET>, Configure::<Impl, OFFSET>, Activate::<Impl, OFFSET>, Deactivate::<Impl, OFFSET>)
    }
}
pub trait IEAPProviderConfigImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn ServerInvokeConfigUI();
    fn RouterInvokeConfigUI();
    fn RouterInvokeCredentialsUI();
}
impl ::windows::core::RuntimeName for IEAPProviderConfig {
    const NAME: &'static str = "Windows.Win32.Security.ExtensibleAuthenticationProtocol.IEAPProviderConfig";
}
impl IEAPProviderConfigVtbl {
    pub const fn new<Impl: IEAPProviderConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEAPProviderConfigVtbl {
        unsafe extern "system" fn Initialize<Impl: IEAPProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, dweaptypeid: u32, puconnectionparam: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&pszmachinename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dweaptypeid, ::core::mem::transmute_copy(&puconnectionparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IEAPProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Uninitialize(dweaptypeid, uconnectionparam) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ServerInvokeConfigUI<Impl: IEAPProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerInvokeConfigUI(dweaptypeid, uconnectionparam, &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), ureserved1, ureserved2) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RouterInvokeConfigUI<Impl: IEAPProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RouterInvokeConfigUI(dweaptypeid, uconnectionparam, &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags, pconnectiondatain, dwsizeofconnectiondatain, ::core::mem::transmute_copy(&ppconnectiondataout), ::core::mem::transmute_copy(&pdwsizeofconnectiondataout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RouterInvokeCredentialsUI<Impl: IEAPProviderConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RouterInvokeCredentialsUI(dweaptypeid, uconnectionparam, &*(&hwndparent as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), dwflags, pconnectiondatain, dwsizeofconnectiondatain, puserdatain, dwsizeofuserdatain, ::core::mem::transmute_copy(&ppuserdataout), ::core::mem::transmute_copy(&pdwsizeofuserdataout)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEAPProviderConfig>, base.5, Initialize::<Impl, OFFSET>, Uninitialize::<Impl, OFFSET>, ServerInvokeConfigUI::<Impl, OFFSET>, RouterInvokeConfigUI::<Impl, OFFSET>, RouterInvokeCredentialsUI::<Impl, OFFSET>)
    }
}
pub trait IEAPProviderConfig2Impl: Sized + IEAPProviderConfigImpl {
    fn ServerInvokeConfigUI2();
    fn GetGlobalConfig();
}
impl ::windows::core::RuntimeName for IEAPProviderConfig2 {
    const NAME: &'static str = "Windows.Win32.Security.ExtensibleAuthenticationProtocol.IEAPProviderConfig2";
}
impl IEAPProviderConfig2Vtbl {
    pub const fn new<Impl: IEAPProviderConfig2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEAPProviderConfig2Vtbl {
        unsafe extern "system" fn ServerInvokeConfigUI2<Impl: IEAPProviderConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerInvokeConfigUI2(dweaptypeid, uconnectionparam, &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), pconfigdatain, dwsizeofconfigdatain, ppconfigdataout, pdwsizeofconfigdataout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGlobalConfig<Impl: IEAPProviderConfig2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetGlobalConfig(dweaptypeid, ppconfigdataout, pdwsizeofconfigdataout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEAPProviderConfig2>, base.5, ServerInvokeConfigUI2::<Impl, OFFSET>, GetGlobalConfig::<Impl, OFFSET>)
    }
}
pub trait IEAPProviderConfig3Impl: Sized + IEAPProviderConfig2Impl + IEAPProviderConfigImpl {
    fn ServerInvokeCertificateConfigUI();
}
impl ::windows::core::RuntimeName for IEAPProviderConfig3 {
    const NAME: &'static str = "Windows.Win32.Security.ExtensibleAuthenticationProtocol.IEAPProviderConfig3";
}
impl IEAPProviderConfig3Vtbl {
    pub const fn new<Impl: IEAPProviderConfig3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IEAPProviderConfig3Vtbl {
        unsafe extern "system" fn ServerInvokeCertificateConfigUI<Impl: IEAPProviderConfig3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ServerInvokeCertificateConfigUI(dweaptypeid, uconnectionparam, &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), pconfigdatain, dwsizeofconfigdatain, ppconfigdataout, pdwsizeofconfigdataout, ureserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IEAPProviderConfig3>, base.5, ServerInvokeCertificateConfigUI::<Impl, OFFSET>)
    }
}
pub trait IRouterProtocolConfigImpl: Sized {
    fn AddProtocol();
    fn RemoveProtocol();
}
impl ::windows::core::RuntimeName for IRouterProtocolConfig {
    const NAME: &'static str = "Windows.Win32.Security.ExtensibleAuthenticationProtocol.IRouterProtocolConfig";
}
impl IRouterProtocolConfigVtbl {
    pub const fn new<Impl: IRouterProtocolConfigImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IRouterProtocolConfigVtbl {
        unsafe extern "system" fn AddProtocol<Impl: IRouterProtocolConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddProtocol(
                &*(&pszmachinename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwtransportid,
                dwprotocolid,
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                &*(&prouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ureserved1,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveProtocol<Impl: IRouterProtocolConfigImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).RemoveProtocol(
                &*(&pszmachinename as *const <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                dwtransportid,
                dwprotocolid,
                &*(&hwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType),
                dwflags,
                &*(&prouter as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                ureserved1,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IRouterProtocolConfig>, base.5, AddProtocol::<Impl, OFFSET>, RemoveProtocol::<Impl, OFFSET>)
    }
}
