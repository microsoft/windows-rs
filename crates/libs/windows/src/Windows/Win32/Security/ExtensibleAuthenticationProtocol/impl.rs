#[cfg(feature = "Win32_Foundation")]
pub trait IAccountingProviderConfigImpl: Sized {
    fn Initialize(&mut self, pszmachinename: super::super::Foundation::PWSTR) -> ::windows::core::Result<usize>;
    fn Uninitialize(&mut self, uconnectionparam: usize) -> ::windows::core::Result<()>;
    fn Configure(&mut self, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn Activate(&mut self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAccountingProviderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAccountingProviderConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAccountingProviderConfigVtbl {
        unsafe extern "system" fn Initialize<Impl: IAccountingProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, puconnectionparam: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(::core::mem::transmute_copy(&pszmachinename)) {
                ::core::result::Result::Ok(ok__) => {
                    *puconnectionparam = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IAccountingProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize(::core::mem::transmute_copy(&uconnectionparam)).into()
        }
        unsafe extern "system" fn Configure<Impl: IAccountingProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Configure(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Activate<Impl: IAccountingProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Deactivate<Impl: IAccountingProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Uninitialize: Uninitialize::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountingProviderConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IAuthenticationProviderConfigImpl: Sized {
    fn Initialize(&mut self, pszmachinename: super::super::Foundation::PWSTR) -> ::windows::core::Result<usize>;
    fn Uninitialize(&mut self, uconnectionparam: usize) -> ::windows::core::Result<()>;
    fn Configure(&mut self, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn Activate(&mut self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn Deactivate(&mut self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IAuthenticationProviderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAuthenticationProviderConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAuthenticationProviderConfigVtbl {
        unsafe extern "system" fn Initialize<Impl: IAuthenticationProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, puconnectionparam: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(::core::mem::transmute_copy(&pszmachinename)) {
                ::core::result::Result::Ok(ok__) => {
                    *puconnectionparam = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IAuthenticationProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize(::core::mem::transmute_copy(&uconnectionparam)).into()
        }
        unsafe extern "system" fn Configure<Impl: IAuthenticationProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Configure(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Activate<Impl: IAuthenticationProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Activate(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Deactivate<Impl: IAuthenticationProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Deactivate(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Uninitialize: Uninitialize::<Impl, IMPL_OFFSET>,
            Configure: Configure::<Impl, IMPL_OFFSET>,
            Activate: Activate::<Impl, IMPL_OFFSET>,
            Deactivate: Deactivate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAuthenticationProviderConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEAPProviderConfigImpl: Sized {
    fn Initialize(&mut self, pszmachinename: super::super::Foundation::PWSTR, dweaptypeid: u32) -> ::windows::core::Result<usize>;
    fn Uninitialize(&mut self, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::Result<()>;
    fn ServerInvokeConfigUI(&mut self, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn RouterInvokeConfigUI(&mut self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::Result<()>;
    fn RouterInvokeCredentialsUI(&mut self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEAPProviderConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEAPProviderConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEAPProviderConfigVtbl {
        unsafe extern "system" fn Initialize<Impl: IEAPProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, dweaptypeid: u32, puconnectionparam: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(::core::mem::transmute_copy(&pszmachinename), ::core::mem::transmute_copy(&dweaptypeid)) {
                ::core::result::Result::Ok(ok__) => {
                    *puconnectionparam = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Impl: IEAPProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Uninitialize(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam)).into()
        }
        unsafe extern "system" fn ServerInvokeConfigUI<Impl: IEAPProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ServerInvokeConfigUI(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn RouterInvokeConfigUI<Impl: IEAPProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RouterInvokeConfigUI(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pconnectiondatain), ::core::mem::transmute_copy(&dwsizeofconnectiondatain), ::core::mem::transmute_copy(&ppconnectiondataout), ::core::mem::transmute_copy(&pdwsizeofconnectiondataout)).into()
        }
        unsafe extern "system" fn RouterInvokeCredentialsUI<Impl: IEAPProviderConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .RouterInvokeCredentialsUI(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pconnectiondatain), ::core::mem::transmute_copy(&dwsizeofconnectiondatain), ::core::mem::transmute_copy(&puserdatain), ::core::mem::transmute_copy(&dwsizeofuserdatain), ::core::mem::transmute_copy(&ppuserdataout), ::core::mem::transmute_copy(&pdwsizeofuserdataout))
                .into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Uninitialize: Uninitialize::<Impl, IMPL_OFFSET>,
            ServerInvokeConfigUI: ServerInvokeConfigUI::<Impl, IMPL_OFFSET>,
            RouterInvokeConfigUI: RouterInvokeConfigUI::<Impl, IMPL_OFFSET>,
            RouterInvokeCredentialsUI: RouterInvokeCredentialsUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEAPProviderConfig as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEAPProviderConfig2Impl: Sized + IEAPProviderConfigImpl {
    fn ServerInvokeConfigUI2(&mut self, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()>;
    fn GetGlobalConfig(&mut self, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEAPProviderConfig2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEAPProviderConfig2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEAPProviderConfig2Vtbl {
        unsafe extern "system" fn ServerInvokeConfigUI2<Impl: IEAPProviderConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ServerInvokeConfigUI2(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pconfigdatain), ::core::mem::transmute_copy(&dwsizeofconfigdatain), ::core::mem::transmute_copy(&ppconfigdataout), ::core::mem::transmute_copy(&pdwsizeofconfigdataout)).into()
        }
        unsafe extern "system" fn GetGlobalConfig<Impl: IEAPProviderConfig2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetGlobalConfig(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&ppconfigdataout), ::core::mem::transmute_copy(&pdwsizeofconfigdataout)).into()
        }
        Self {
            base: IEAPProviderConfigVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ServerInvokeConfigUI2: ServerInvokeConfigUI2::<Impl, IMPL_OFFSET>,
            GetGlobalConfig: GetGlobalConfig::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEAPProviderConfig2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IEAPProviderConfig3Impl: Sized + IEAPProviderConfigImpl + IEAPProviderConfig2Impl {
    fn ServerInvokeCertificateConfigUI(&mut self, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IEAPProviderConfig3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEAPProviderConfig3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEAPProviderConfig3Vtbl {
        unsafe extern "system" fn ServerInvokeCertificateConfigUI<Impl: IEAPProviderConfig3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ServerInvokeCertificateConfigUI(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pconfigdatain), ::core::mem::transmute_copy(&dwsizeofconfigdatain), ::core::mem::transmute_copy(&ppconfigdataout), ::core::mem::transmute_copy(&pdwsizeofconfigdataout), ::core::mem::transmute_copy(&ureserved)).into()
        }
        Self {
            base: IEAPProviderConfig2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ServerInvokeCertificateConfigUI: ServerInvokeCertificateConfigUI::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEAPProviderConfig3 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IRouterProtocolConfigImpl: Sized {
    fn AddProtocol(&mut self, pszmachinename: super::super::Foundation::PWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: ::core::option::Option<::windows::core::IUnknown>, ureserved1: usize) -> ::windows::core::Result<()>;
    fn RemoveProtocol(&mut self, pszmachinename: super::super::Foundation::PWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: ::core::option::Option<::windows::core::IUnknown>, ureserved1: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl IRouterProtocolConfigVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRouterProtocolConfigImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRouterProtocolConfigVtbl {
        unsafe extern "system" fn AddProtocol<Impl: IRouterProtocolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddProtocol(::core::mem::transmute_copy(&pszmachinename), ::core::mem::transmute_copy(&dwtransportid), ::core::mem::transmute_copy(&dwprotocolid), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&prouter), ::core::mem::transmute_copy(&ureserved1)).into()
        }
        unsafe extern "system" fn RemoveProtocol<Impl: IRouterProtocolConfigImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: super::super::Foundation::PWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveProtocol(::core::mem::transmute_copy(&pszmachinename), ::core::mem::transmute_copy(&dwtransportid), ::core::mem::transmute_copy(&dwprotocolid), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute(&prouter), ::core::mem::transmute_copy(&ureserved1)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            AddProtocol: AddProtocol::<Impl, IMPL_OFFSET>,
            RemoveProtocol: RemoveProtocol::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRouterProtocolConfig as ::windows::core::Interface>::IID
    }
}
