#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSMan_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateSession(&self, connection: &::windows::core::BSTR, flags: i32, connectionoptions: ::core::option::Option<&super::Com::IDispatch>) -> ::windows::core::Result<super::Com::IDispatch>;
    fn CreateConnectionOptions(&self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn CommandLine(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Error(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSMan {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSMan_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: isize>() -> IWSMan_Vtbl {
        unsafe extern "system" fn CreateSession<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connection: ::std::mem::MaybeUninit<::windows::core::BSTR>, flags: i32, connectionoptions: *mut ::core::ffi::c_void, session: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateSession(::core::mem::transmute(&connection), ::core::mem::transmute_copy(&flags), ::windows::core::from_raw_borrowed(&connectionoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(session, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConnectionOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionoptions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateConnectionOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(connectionoptions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandLine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CommandLine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Error() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateSession: CreateSession::<Identity, Impl, OFFSET>,
            CreateConnectionOptions: CreateConnectionOptions::<Identity, Impl, OFFSET>,
            CommandLine: CommandLine::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSMan as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManConnectionOptions_Impl: Sized + super::Com::IDispatch_Impl {
    fn UserName(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetUserName(&self, name: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn SetPassword(&self, password: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManConnectionOptions {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManConnectionOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>() -> IWSManConnectionOptions_Vtbl {
        unsafe extern "system" fn UserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(name, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUserName(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPassword(::core::mem::transmute(&password)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserName: UserName::<Identity, Impl, OFFSET>,
            SetUserName: SetUserName::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManConnectionOptions as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManConnectionOptionsEx_Impl: Sized + IWSManConnectionOptions_Impl {
    fn CertificateThumbprint(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetCertificateThumbprint(&self, thumbprint: &::windows::core::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManConnectionOptionsEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManConnectionOptionsEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>() -> IWSManConnectionOptionsEx_Vtbl {
        unsafe extern "system" fn CertificateThumbprint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprint: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CertificateThumbprint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(thumbprint, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateThumbprint<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprint: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCertificateThumbprint(::core::mem::transmute(&thumbprint)).into()
        }
        Self {
            base__: IWSManConnectionOptions_Vtbl::new::<Identity, Impl, OFFSET>(),
            CertificateThumbprint: CertificateThumbprint::<Identity, Impl, OFFSET>,
            SetCertificateThumbprint: SetCertificateThumbprint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManConnectionOptionsEx as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWSManConnectionOptions as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManConnectionOptionsEx2_Impl: Sized + IWSManConnectionOptionsEx_Impl {
    fn SetProxy(&self, accesstype: i32, authenticationmechanism: i32, username: &::windows::core::BSTR, password: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ProxyIEConfig(&self) -> ::windows::core::Result<i32>;
    fn ProxyWinHttpConfig(&self) -> ::windows::core::Result<i32>;
    fn ProxyAutoDetect(&self) -> ::windows::core::Result<i32>;
    fn ProxyNoProxyServer(&self) -> ::windows::core::Result<i32>;
    fn ProxyAuthenticationUseNegotiate(&self) -> ::windows::core::Result<i32>;
    fn ProxyAuthenticationUseBasic(&self) -> ::windows::core::Result<i32>;
    fn ProxyAuthenticationUseDigest(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManConnectionOptionsEx2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManConnectionOptionsEx2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>() -> IWSManConnectionOptionsEx2_Vtbl {
        unsafe extern "system" fn SetProxy<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: i32, authenticationmechanism: i32, username: ::std::mem::MaybeUninit<::windows::core::BSTR>, password: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProxy(::core::mem::transmute_copy(&accesstype), ::core::mem::transmute_copy(&authenticationmechanism), ::core::mem::transmute(&username), ::core::mem::transmute(&password)).into()
        }
        unsafe extern "system" fn ProxyIEConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyIEConfig() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyWinHttpConfig<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyWinHttpConfig() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAutoDetect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyAutoDetect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyNoProxyServer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyNoProxyServer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseNegotiate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyAuthenticationUseNegotiate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseBasic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyAuthenticationUseBasic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseDigest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProxyAuthenticationUseDigest() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWSManConnectionOptionsEx_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetProxy: SetProxy::<Identity, Impl, OFFSET>,
            ProxyIEConfig: ProxyIEConfig::<Identity, Impl, OFFSET>,
            ProxyWinHttpConfig: ProxyWinHttpConfig::<Identity, Impl, OFFSET>,
            ProxyAutoDetect: ProxyAutoDetect::<Identity, Impl, OFFSET>,
            ProxyNoProxyServer: ProxyNoProxyServer::<Identity, Impl, OFFSET>,
            ProxyAuthenticationUseNegotiate: ProxyAuthenticationUseNegotiate::<Identity, Impl, OFFSET>,
            ProxyAuthenticationUseBasic: ProxyAuthenticationUseBasic::<Identity, Impl, OFFSET>,
            ProxyAuthenticationUseDigest: ProxyAuthenticationUseDigest::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManConnectionOptionsEx2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWSManConnectionOptions as ::windows::core::ComInterface>::IID || iid == &<IWSManConnectionOptionsEx as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEnumerator_Impl: Sized + super::Com::IDispatch_Impl {
    fn ReadItem(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn AtEndOfStream(&self) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Error(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManEnumerator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEnumerator_Impl, const OFFSET: isize>() -> IWSManEnumerator_Vtbl {
        unsafe extern "system" fn ReadItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ReadItem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AtEndOfStream<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eos: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AtEndOfStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eos, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Error() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReadItem: ReadItem::<Identity, Impl, OFFSET>,
            AtEndOfStream: AtEndOfStream::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManEnumerator as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEx_Impl: Sized + IWSMan_Impl {
    fn CreateResourceLocator(&self, strresourcelocator: &::windows::core::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn SessionFlagUTF8(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagCredUsernamePassword(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagSkipCACheck(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagSkipCNCheck(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseDigest(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseNegotiate(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseBasic(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseKerberos(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagNoEncryption(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagEnableSPNServerPort(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseNoAuthentication(&self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagNonXmlText(&self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagReturnEPR(&self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagReturnObjectAndEPR(&self) -> ::windows::core::Result<i32>;
    fn GetErrorMessage(&self, errornumber: u32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn EnumerationFlagHierarchyDeep(&self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagHierarchyShallow(&self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagHierarchyDeepBasePropsOnly(&self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagReturnObject(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManEx {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>() -> IWSManEx_Vtbl {
        unsafe extern "system" fn CreateResourceLocator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresourcelocator: ::std::mem::MaybeUninit<::windows::core::BSTR>, newresourcelocator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateResourceLocator(::core::mem::transmute(&strresourcelocator)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newresourcelocator, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUTF8<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUTF8() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagCredUsernamePassword<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagCredUsernamePassword() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipCACheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagSkipCACheck() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipCNCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagSkipCNCheck() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseDigest<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUseDigest() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseNegotiate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUseNegotiate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseBasic<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUseBasic() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseKerberos<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUseKerberos() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagNoEncryption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagNoEncryption() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagEnableSPNServerPort<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagEnableSPNServerPort() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseNoAuthentication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUseNoAuthentication() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagNonXmlText<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationFlagNonXmlText() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnEPR<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationFlagReturnEPR() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObjectAndEPR<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationFlagReturnObjectAndEPR() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorMessage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errornumber: u32, errormessage: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetErrorMessage(::core::mem::transmute_copy(&errornumber)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(errormessage, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeep<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationFlagHierarchyDeep() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyShallow<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationFlagHierarchyShallow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeepBasePropsOnly<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationFlagHierarchyDeepBasePropsOnly() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObject<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationFlagReturnObject() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWSMan_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateResourceLocator: CreateResourceLocator::<Identity, Impl, OFFSET>,
            SessionFlagUTF8: SessionFlagUTF8::<Identity, Impl, OFFSET>,
            SessionFlagCredUsernamePassword: SessionFlagCredUsernamePassword::<Identity, Impl, OFFSET>,
            SessionFlagSkipCACheck: SessionFlagSkipCACheck::<Identity, Impl, OFFSET>,
            SessionFlagSkipCNCheck: SessionFlagSkipCNCheck::<Identity, Impl, OFFSET>,
            SessionFlagUseDigest: SessionFlagUseDigest::<Identity, Impl, OFFSET>,
            SessionFlagUseNegotiate: SessionFlagUseNegotiate::<Identity, Impl, OFFSET>,
            SessionFlagUseBasic: SessionFlagUseBasic::<Identity, Impl, OFFSET>,
            SessionFlagUseKerberos: SessionFlagUseKerberos::<Identity, Impl, OFFSET>,
            SessionFlagNoEncryption: SessionFlagNoEncryption::<Identity, Impl, OFFSET>,
            SessionFlagEnableSPNServerPort: SessionFlagEnableSPNServerPort::<Identity, Impl, OFFSET>,
            SessionFlagUseNoAuthentication: SessionFlagUseNoAuthentication::<Identity, Impl, OFFSET>,
            EnumerationFlagNonXmlText: EnumerationFlagNonXmlText::<Identity, Impl, OFFSET>,
            EnumerationFlagReturnEPR: EnumerationFlagReturnEPR::<Identity, Impl, OFFSET>,
            EnumerationFlagReturnObjectAndEPR: EnumerationFlagReturnObjectAndEPR::<Identity, Impl, OFFSET>,
            GetErrorMessage: GetErrorMessage::<Identity, Impl, OFFSET>,
            EnumerationFlagHierarchyDeep: EnumerationFlagHierarchyDeep::<Identity, Impl, OFFSET>,
            EnumerationFlagHierarchyShallow: EnumerationFlagHierarchyShallow::<Identity, Impl, OFFSET>,
            EnumerationFlagHierarchyDeepBasePropsOnly: EnumerationFlagHierarchyDeepBasePropsOnly::<Identity, Impl, OFFSET>,
            EnumerationFlagReturnObject: EnumerationFlagReturnObject::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManEx as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWSMan as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEx2_Impl: Sized + IWSManEx_Impl {
    fn SessionFlagUseClientCertificate(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManEx2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEx2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx2_Impl, const OFFSET: isize>() -> IWSManEx2_Vtbl {
        unsafe extern "system" fn SessionFlagUseClientCertificate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUseClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWSManEx_Vtbl::new::<Identity, Impl, OFFSET>(),
            SessionFlagUseClientCertificate: SessionFlagUseClientCertificate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManEx2 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWSMan as ::windows::core::ComInterface>::IID || iid == &<IWSManEx as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEx3_Impl: Sized + IWSManEx2_Impl {
    fn SessionFlagUTF16(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseCredSsp(&self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagAssociationInstance(&self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagAssociatedInstance(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagSkipRevocationCheck(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagAllowNegotiateImplicitCredentials(&self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseSsl(&self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManEx3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEx3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: isize>() -> IWSManEx3_Vtbl {
        unsafe extern "system" fn SessionFlagUTF16<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUTF16() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseCredSsp<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUseCredSsp() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociationInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationFlagAssociationInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociatedInstance<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumerationFlagAssociatedInstance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipRevocationCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagSkipRevocationCheck() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagAllowNegotiateImplicitCredentials<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagAllowNegotiateImplicitCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseSsl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionFlagUseSsl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(flags, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWSManEx2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SessionFlagUTF16: SessionFlagUTF16::<Identity, Impl, OFFSET>,
            SessionFlagUseCredSsp: SessionFlagUseCredSsp::<Identity, Impl, OFFSET>,
            EnumerationFlagAssociationInstance: EnumerationFlagAssociationInstance::<Identity, Impl, OFFSET>,
            EnumerationFlagAssociatedInstance: EnumerationFlagAssociatedInstance::<Identity, Impl, OFFSET>,
            SessionFlagSkipRevocationCheck: SessionFlagSkipRevocationCheck::<Identity, Impl, OFFSET>,
            SessionFlagAllowNegotiateImplicitCredentials: SessionFlagAllowNegotiateImplicitCredentials::<Identity, Impl, OFFSET>,
            SessionFlagUseSsl: SessionFlagUseSsl::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManEx3 as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID || iid == &<IWSMan as ::windows::core::ComInterface>::IID || iid == &<IWSManEx as ::windows::core::ComInterface>::IID || iid == &<IWSManEx2 as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManInternal_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConfigSDDL(&self, session: ::core::option::Option<&super::Com::IDispatch>, resourceuri: &super::Com::VARIANT, flags: i32) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManInternal {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManInternal_Impl, const OFFSET: isize>() -> IWSManInternal_Vtbl {
        unsafe extern "system" fn ConfigSDDL<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManInternal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, session: *mut ::core::ffi::c_void, resourceuri: super::Com::VARIANT, flags: i32, resource: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ConfigSDDL(::windows::core::from_raw_borrowed(&session), ::core::mem::transmute(&resourceuri), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ConfigSDDL: ConfigSDDL::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManInternal as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManResourceLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn SetResourceURI(&self, uri: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn ResourceURI(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn AddSelector(&self, resourceselname: &::windows::core::BSTR, selvalue: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ClearSelectors(&self) -> ::windows::core::Result<()>;
    fn FragmentPath(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFragmentPath(&self, text: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn FragmentDialect(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn SetFragmentDialect(&self, text: &::windows::core::BSTR) -> ::windows::core::Result<()>;
    fn AddOption(&self, optionname: &::windows::core::BSTR, optionvalue: &super::Com::VARIANT, mustcomply: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetMustUnderstandOptions(&self, mustunderstand: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MustUnderstandOptions(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ClearOptions(&self) -> ::windows::core::Result<()>;
    fn Error(&self) -> ::windows::core::Result<::windows::core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManResourceLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManResourceLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>() -> IWSManResourceLocator_Vtbl {
        unsafe extern "system" fn SetResourceURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetResourceURI(::core::mem::transmute(&uri)).into()
        }
        unsafe extern "system" fn ResourceURI<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResourceURI() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(uri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSelector<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceselname: ::std::mem::MaybeUninit<::windows::core::BSTR>, selvalue: super::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSelector(::core::mem::transmute(&resourceselname), ::core::mem::transmute(&selvalue)).into()
        }
        unsafe extern "system" fn ClearSelectors<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearSelectors().into()
        }
        unsafe extern "system" fn FragmentPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FragmentPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFragmentPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFragmentPath(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn FragmentDialect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FragmentDialect() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(text, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFragmentDialect<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFragmentDialect(::core::mem::transmute(&text)).into()
        }
        unsafe extern "system" fn AddOption<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionname: ::std::mem::MaybeUninit<::windows::core::BSTR>, optionvalue: super::Com::VARIANT, mustcomply: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOption(::core::mem::transmute(&optionname), ::core::mem::transmute(&optionvalue), ::core::mem::transmute_copy(&mustcomply)).into()
        }
        unsafe extern "system" fn SetMustUnderstandOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mustunderstand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMustUnderstandOptions(::core::mem::transmute_copy(&mustunderstand)).into()
        }
        unsafe extern "system" fn MustUnderstandOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mustunderstand: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MustUnderstandOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(mustunderstand, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearOptions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearOptions().into()
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Error() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetResourceURI: SetResourceURI::<Identity, Impl, OFFSET>,
            ResourceURI: ResourceURI::<Identity, Impl, OFFSET>,
            AddSelector: AddSelector::<Identity, Impl, OFFSET>,
            ClearSelectors: ClearSelectors::<Identity, Impl, OFFSET>,
            FragmentPath: FragmentPath::<Identity, Impl, OFFSET>,
            SetFragmentPath: SetFragmentPath::<Identity, Impl, OFFSET>,
            FragmentDialect: FragmentDialect::<Identity, Impl, OFFSET>,
            SetFragmentDialect: SetFragmentDialect::<Identity, Impl, OFFSET>,
            AddOption: AddOption::<Identity, Impl, OFFSET>,
            SetMustUnderstandOptions: SetMustUnderstandOptions::<Identity, Impl, OFFSET>,
            MustUnderstandOptions: MustUnderstandOptions::<Identity, Impl, OFFSET>,
            ClearOptions: ClearOptions::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManResourceLocator as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"implement\"`*"]
pub trait IWSManResourceLocatorInternal_Impl: Sized {}
impl ::windows::core::RuntimeName for IWSManResourceLocatorInternal {}
impl IWSManResourceLocatorInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManResourceLocatorInternal_Impl, const OFFSET: isize>() -> IWSManResourceLocatorInternal_Vtbl {
        Self { base__: ::windows::core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManResourceLocatorInternal as ::windows::core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_System_RemoteManagement\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn Get(&self, resourceuri: &super::Com::VARIANT, flags: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Put(&self, resourceuri: &super::Com::VARIANT, resource: &::windows::core::BSTR, flags: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Create(&self, resourceuri: &super::Com::VARIANT, resource: &::windows::core::BSTR, flags: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Delete(&self, resourceuri: &super::Com::VARIANT, flags: i32) -> ::windows::core::Result<()>;
    fn Invoke2(&self, actionuri: &::windows::core::BSTR, resourceuri: &super::Com::VARIANT, parameters: &::windows::core::BSTR, flags: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Enumerate(&self, resourceuri: &super::Com::VARIANT, filter: &::windows::core::BSTR, dialect: &::windows::core::BSTR, flags: i32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Identify(&self, flags: i32) -> ::windows::core::Result<::windows::core::BSTR>;
    fn Error(&self) -> ::windows::core::Result<::windows::core::BSTR>;
    fn BatchItems(&self) -> ::windows::core::Result<i32>;
    fn SetBatchItems(&self, value: i32) -> ::windows::core::Result<()>;
    fn Timeout(&self) -> ::windows::core::Result<i32>;
    fn SetTimeout(&self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IWSManSession {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>() -> IWSManSession_Vtbl {
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: super::Com::VARIANT, flags: i32, resource: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Get(::core::mem::transmute(&resourceuri), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: super::Com::VARIANT, resource: ::std::mem::MaybeUninit<::windows::core::BSTR>, flags: i32, resultresource: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Put(::core::mem::transmute(&resourceuri), ::core::mem::transmute(&resource), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultresource, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: super::Com::VARIANT, resource: ::std::mem::MaybeUninit<::windows::core::BSTR>, flags: i32, newuri: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Create(::core::mem::transmute(&resourceuri), ::core::mem::transmute(&resource), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newuri, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: super::Com::VARIANT, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&resourceuri), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Invoke2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actionuri: ::std::mem::MaybeUninit<::windows::core::BSTR>, resourceuri: super::Com::VARIANT, parameters: ::std::mem::MaybeUninit<::windows::core::BSTR>, flags: i32, result: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Invoke2(::core::mem::transmute(&actionuri), ::core::mem::transmute(&resourceuri), ::core::mem::transmute(&parameters), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enumerate<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: super::Com::VARIANT, filter: ::std::mem::MaybeUninit<::windows::core::BSTR>, dialect: ::std::mem::MaybeUninit<::windows::core::BSTR>, flags: i32, resultset: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Enumerate(::core::mem::transmute(&resourceuri), ::core::mem::transmute(&filter), ::core::mem::transmute(&dialect), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultset, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identify<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, result: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Identify(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(result, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Error() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatchItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BatchItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchItems<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBatchItems(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Timeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Timeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetTimeout(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base__: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Get: Get::<Identity, Impl, OFFSET>,
            Put: Put::<Identity, Impl, OFFSET>,
            Create: Create::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Invoke2: Invoke2::<Identity, Impl, OFFSET>,
            Enumerate: Enumerate::<Identity, Impl, OFFSET>,
            Identify: Identify::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
            BatchItems: BatchItems::<Identity, Impl, OFFSET>,
            SetBatchItems: SetBatchItems::<Identity, Impl, OFFSET>,
            Timeout: Timeout::<Identity, Impl, OFFSET>,
            SetTimeout: SetTimeout::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManSession as ::windows::core::ComInterface>::IID || iid == &<super::Com::IDispatch as ::windows::core::ComInterface>::IID
    }
}
