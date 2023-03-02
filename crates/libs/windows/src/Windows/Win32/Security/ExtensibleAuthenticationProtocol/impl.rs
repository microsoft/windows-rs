#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAccountingProviderConfig_Impl: Sized {
    fn Initialize(&self, pszmachinename: &::windows::core::PCWSTR) -> ::windows::core::Result<usize>;
    fn Uninitialize(&self, uconnectionparam: usize) -> ::windows::core::Result<()>;
    fn Configure(&self, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn Activate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn Deactivate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAccountingProviderConfig {}
#[cfg(feature = "Win32_Foundation")]
impl IAccountingProviderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: isize>() -> IAccountingProviderConfig_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, puconnectionparam: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Initialize(::core::mem::transmute(&pszmachinename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puconnectionparam, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Uninitialize(::core::mem::transmute_copy(&uconnectionparam)).into()
        }
        unsafe extern "system" fn Configure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Configure(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAccountingProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            Configure: Configure::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAccountingProviderConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IAuthenticationProviderConfig_Impl: Sized {
    fn Initialize(&self, pszmachinename: &::windows::core::PCWSTR) -> ::windows::core::Result<usize>;
    fn Uninitialize(&self, uconnectionparam: usize) -> ::windows::core::Result<()>;
    fn Configure(&self, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn Activate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn Deactivate(&self, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IAuthenticationProviderConfig {}
#[cfg(feature = "Win32_Foundation")]
impl IAuthenticationProviderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: isize>() -> IAuthenticationProviderConfig_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, puconnectionparam: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Initialize(::core::mem::transmute(&pszmachinename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puconnectionparam, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Uninitialize(::core::mem::transmute_copy(&uconnectionparam)).into()
        }
        unsafe extern "system" fn Configure<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, dwflags: u32, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Configure(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Activate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Activate(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn Deactivate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAuthenticationProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uconnectionparam: usize, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Deactivate(::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            Configure: Configure::<Identity, Impl, OFFSET>,
            Activate: Activate::<Identity, Impl, OFFSET>,
            Deactivate: Deactivate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAuthenticationProviderConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEAPProviderConfig_Impl: Sized {
    fn Initialize(&self, pszmachinename: &::windows::core::PCWSTR, dweaptypeid: u32) -> ::windows::core::Result<usize>;
    fn Uninitialize(&self, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::Result<()>;
    fn ServerInvokeConfigUI(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> ::windows::core::Result<()>;
    fn RouterInvokeConfigUI(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::Result<()>;
    fn RouterInvokeCredentialsUI(&self, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IEAPProviderConfig {}
#[cfg(feature = "Win32_Foundation")]
impl IEAPProviderConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: isize>() -> IEAPProviderConfig_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, dweaptypeid: u32, puconnectionparam: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Initialize(::core::mem::transmute(&pszmachinename), ::core::mem::transmute_copy(&dweaptypeid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puconnectionparam, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Uninitialize(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam)).into()
        }
        unsafe extern "system" fn ServerInvokeConfigUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, ureserved1: usize, ureserved2: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ServerInvokeConfigUI(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&ureserved1), ::core::mem::transmute_copy(&ureserved2)).into()
        }
        unsafe extern "system" fn RouterInvokeConfigUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, ppconnectiondataout: *mut *mut u8, pdwsizeofconnectiondataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RouterInvokeConfigUI(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pconnectiondatain), ::core::mem::transmute_copy(&dwsizeofconnectiondatain), ::core::mem::transmute_copy(&ppconnectiondataout), ::core::mem::transmute_copy(&pdwsizeofconnectiondataout)).into()
        }
        unsafe extern "system" fn RouterInvokeCredentialsUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwndparent: super::super::Foundation::HWND, dwflags: u32, pconnectiondatain: *const u8, dwsizeofconnectiondatain: u32, puserdatain: *const u8, dwsizeofuserdatain: u32, ppuserdataout: *mut *mut u8, pdwsizeofuserdataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RouterInvokeCredentialsUI(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwndparent), ::core::mem::transmute_copy(&dwflags), ::core::mem::transmute_copy(&pconnectiondatain), ::core::mem::transmute_copy(&dwsizeofconnectiondatain), ::core::mem::transmute_copy(&puserdatain), ::core::mem::transmute_copy(&dwsizeofuserdatain), ::core::mem::transmute_copy(&ppuserdataout), ::core::mem::transmute_copy(&pdwsizeofuserdataout))
                .into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Uninitialize: Uninitialize::<Identity, Impl, OFFSET>,
            ServerInvokeConfigUI: ServerInvokeConfigUI::<Identity, Impl, OFFSET>,
            RouterInvokeConfigUI: RouterInvokeConfigUI::<Identity, Impl, OFFSET>,
            RouterInvokeCredentialsUI: RouterInvokeCredentialsUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEAPProviderConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEAPProviderConfig2_Impl: Sized + IEAPProviderConfig_Impl {
    fn ServerInvokeConfigUI2(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()>;
    fn GetGlobalConfig(&self, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IEAPProviderConfig2 {}
#[cfg(feature = "Win32_Foundation")]
impl IEAPProviderConfig2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig2_Impl, const OFFSET: isize>() -> IEAPProviderConfig2_Vtbl {
        unsafe extern "system" fn ServerInvokeConfigUI2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ServerInvokeConfigUI2(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pconfigdatain), ::core::mem::transmute_copy(&dwsizeofconfigdatain), ::core::mem::transmute_copy(&ppconfigdataout), ::core::mem::transmute_copy(&pdwsizeofconfigdataout)).into()
        }
        unsafe extern "system" fn GetGlobalConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetGlobalConfig(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&ppconfigdataout), ::core::mem::transmute_copy(&pdwsizeofconfigdataout)).into()
        }
        Self {
            base__: IEAPProviderConfig_Vtbl::new::<Identity, Impl, OFFSET>(),
            ServerInvokeConfigUI2: ServerInvokeConfigUI2::<Identity, Impl, OFFSET>,
            GetGlobalConfig: GetGlobalConfig::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEAPProviderConfig2 as ::windows::core::ComInterface>::IID || iid == &<IEAPProviderConfig as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IEAPProviderConfig3_Impl: Sized + IEAPProviderConfig2_Impl {
    fn ServerInvokeCertificateConfigUI(&self, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IEAPProviderConfig3 {}
#[cfg(feature = "Win32_Foundation")]
impl IEAPProviderConfig3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig3_Impl, const OFFSET: isize>() -> IEAPProviderConfig3_Vtbl {
        unsafe extern "system" fn ServerInvokeCertificateConfigUI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IEAPProviderConfig3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dweaptypeid: u32, uconnectionparam: usize, hwnd: super::super::Foundation::HWND, pconfigdatain: *const u8, dwsizeofconfigdatain: u32, ppconfigdataout: *mut *mut u8, pdwsizeofconfigdataout: *mut u32, ureserved: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ServerInvokeCertificateConfigUI(::core::mem::transmute_copy(&dweaptypeid), ::core::mem::transmute_copy(&uconnectionparam), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&pconfigdatain), ::core::mem::transmute_copy(&dwsizeofconfigdatain), ::core::mem::transmute_copy(&ppconfigdataout), ::core::mem::transmute_copy(&pdwsizeofconfigdataout), ::core::mem::transmute_copy(&ureserved)).into()
        }
        Self {
            base__: IEAPProviderConfig2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ServerInvokeCertificateConfigUI: ServerInvokeCertificateConfigUI::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEAPProviderConfig3 as ::windows::core::ComInterface>::IID || iid == &<IEAPProviderConfig as ::windows::core::ComInterface>::IID || iid == &<IEAPProviderConfig2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IRouterProtocolConfig_Impl: Sized {
    fn AddProtocol(&self, pszmachinename: &::windows::core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: ::core::option::Option<&::windows::core::IUnknown>, ureserved1: usize) -> ::windows::core::Result<()>;
    fn RemoveProtocol(&self, pszmachinename: &::windows::core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: ::core::option::Option<&::windows::core::IUnknown>, ureserved1: usize) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::RuntimeName for IRouterProtocolConfig {}
#[cfg(feature = "Win32_Foundation")]
impl IRouterProtocolConfig_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRouterProtocolConfig_Impl, const OFFSET: isize>() -> IRouterProtocolConfig_Vtbl {
        unsafe extern "system" fn AddProtocol<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRouterProtocolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddProtocol(::core::mem::transmute(&pszmachinename), ::core::mem::transmute_copy(&dwtransportid), ::core::mem::transmute_copy(&dwprotocolid), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&prouter), ::core::mem::transmute_copy(&ureserved1)).into()
        }
        unsafe extern "system" fn RemoveProtocol<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IRouterProtocolConfig_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszmachinename: ::windows::core::PCWSTR, dwtransportid: u32, dwprotocolid: u32, hwnd: super::super::Foundation::HWND, dwflags: u32, prouter: *mut ::core::ffi::c_void, ureserved1: usize) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveProtocol(::core::mem::transmute(&pszmachinename), ::core::mem::transmute_copy(&dwtransportid), ::core::mem::transmute_copy(&dwprotocolid), ::core::mem::transmute_copy(&hwnd), ::core::mem::transmute_copy(&dwflags), ::windows::core::from_raw_borrowed(&prouter), ::core::mem::transmute_copy(&ureserved1)).into()
        }
        Self {
            base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProtocol: AddProtocol::<Identity, Impl, OFFSET>,
            RemoveProtocol: RemoveProtocol::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRouterProtocolConfig as ::windows::core::ComInterface>::IID
    }
}
