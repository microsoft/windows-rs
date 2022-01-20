#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSMan_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateSession(&mut self, connection: &super::super::Foundation::BSTR, flags: i32, connectionoptions: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<super::Com::IDispatch>;
    fn CreateConnectionOptions(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn CommandLine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Error(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSMan_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSMan_Impl, const OFFSET: isize>() -> IWSMan_Vtbl {
        unsafe extern "system" fn CreateSession<Identity: ::windows::core::IUnknownImpl, Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, connectionoptions: ::windows::core::RawPtr, session: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateSession(::core::mem::transmute_copy(&connection), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&connectionoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *session = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConnectionOptions<Identity: ::windows::core::IUnknownImpl, Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateConnectionOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *connectionoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandLine<Identity: ::windows::core::IUnknownImpl, Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CommandLine() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateSession: CreateSession::<Identity, Impl, OFFSET>,
            CreateConnectionOptions: CreateConnectionOptions::<Identity, Impl, OFFSET>,
            CommandLine: CommandLine::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSMan as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManConnectionOptions_Impl: Sized + super::Com::IDispatch_Impl {
    fn UserName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetUserName(&mut self, name: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetPassword(&mut self, password: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManConnectionOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>() -> IWSManConnectionOptions_Vtbl {
        unsafe extern "system" fn UserName<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetUserName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn SetPassword<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&password)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            UserName: UserName::<Identity, Impl, OFFSET>,
            SetUserName: SetUserName::<Identity, Impl, OFFSET>,
            SetPassword: SetPassword::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManConnectionOptions as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManConnectionOptionsEx_Impl: Sized + super::Com::IDispatch_Impl + IWSManConnectionOptions_Impl {
    fn CertificateThumbprint(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetCertificateThumbprint(&mut self, thumbprint: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManConnectionOptionsEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>() -> IWSManConnectionOptionsEx_Vtbl {
        unsafe extern "system" fn CertificateThumbprint<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprint: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CertificateThumbprint() {
                ::core::result::Result::Ok(ok__) => {
                    *thumbprint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateThumbprint<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCertificateThumbprint(::core::mem::transmute_copy(&thumbprint)).into()
        }
        Self {
            base: IWSManConnectionOptions_Vtbl::new::<Identity, Impl, OFFSET>(),
            CertificateThumbprint: CertificateThumbprint::<Identity, Impl, OFFSET>,
            SetCertificateThumbprint: SetCertificateThumbprint::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManConnectionOptionsEx as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWSManConnectionOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManConnectionOptionsEx2_Impl: Sized + super::Com::IDispatch_Impl + IWSManConnectionOptions_Impl + IWSManConnectionOptionsEx_Impl {
    fn SetProxy(&mut self, accesstype: i32, authenticationmechanism: i32, username: &super::super::Foundation::BSTR, password: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ProxyIEConfig(&mut self) -> ::windows::core::Result<i32>;
    fn ProxyWinHttpConfig(&mut self) -> ::windows::core::Result<i32>;
    fn ProxyAutoDetect(&mut self) -> ::windows::core::Result<i32>;
    fn ProxyNoProxyServer(&mut self) -> ::windows::core::Result<i32>;
    fn ProxyAuthenticationUseNegotiate(&mut self) -> ::windows::core::Result<i32>;
    fn ProxyAuthenticationUseBasic(&mut self) -> ::windows::core::Result<i32>;
    fn ProxyAuthenticationUseDigest(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManConnectionOptionsEx2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>() -> IWSManConnectionOptionsEx2_Vtbl {
        unsafe extern "system" fn SetProxy<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: i32, authenticationmechanism: i32, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxy(::core::mem::transmute_copy(&accesstype), ::core::mem::transmute_copy(&authenticationmechanism), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn ProxyIEConfig<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyIEConfig() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyWinHttpConfig<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyWinHttpConfig() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAutoDetect<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyAutoDetect() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyNoProxyServer<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyNoProxyServer() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseNegotiate<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyAuthenticationUseNegotiate() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseBasic<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyAuthenticationUseBasic() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseDigest<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyAuthenticationUseDigest() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSManConnectionOptionsEx_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWSManConnectionOptionsEx2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWSManConnectionOptions as ::windows::core::Interface>::IID || iid == &<IWSManConnectionOptionsEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEnumerator_Impl: Sized + super::Com::IDispatch_Impl {
    fn ReadItem(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AtEndOfStream(&mut self) -> ::windows::core::Result<i16>;
    fn Error(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEnumerator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEnumerator_Impl, const OFFSET: isize>() -> IWSManEnumerator_Vtbl {
        unsafe extern "system" fn ReadItem<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ReadItem() {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AtEndOfStream<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eos: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AtEndOfStream() {
                ::core::result::Result::Ok(ok__) => {
                    *eos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ReadItem: ReadItem::<Identity, Impl, OFFSET>,
            AtEndOfStream: AtEndOfStream::<Identity, Impl, OFFSET>,
            Error: Error::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManEnumerator as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEx_Impl: Sized + super::Com::IDispatch_Impl + IWSMan_Impl {
    fn CreateResourceLocator(&mut self, strresourcelocator: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::Com::IDispatch>;
    fn SessionFlagUTF8(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagCredUsernamePassword(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagSkipCACheck(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagSkipCNCheck(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseDigest(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseNegotiate(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseBasic(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseKerberos(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagNoEncryption(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagEnableSPNServerPort(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseNoAuthentication(&mut self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagNonXmlText(&mut self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagReturnEPR(&mut self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagReturnObjectAndEPR(&mut self) -> ::windows::core::Result<i32>;
    fn GetErrorMessage(&mut self, errornumber: u32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EnumerationFlagHierarchyDeep(&mut self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagHierarchyShallow(&mut self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagHierarchyDeepBasePropsOnly(&mut self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagReturnObject(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEx_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>() -> IWSManEx_Vtbl {
        unsafe extern "system" fn CreateResourceLocator<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresourcelocator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newresourcelocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateResourceLocator(::core::mem::transmute_copy(&strresourcelocator)) {
                ::core::result::Result::Ok(ok__) => {
                    *newresourcelocator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUTF8<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUTF8() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagCredUsernamePassword<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagCredUsernamePassword() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipCACheck<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagSkipCACheck() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipCNCheck<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagSkipCNCheck() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseDigest<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUseDigest() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseNegotiate<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUseNegotiate() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseBasic<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUseBasic() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseKerberos<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUseKerberos() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagNoEncryption<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagNoEncryption() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagEnableSPNServerPort<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagEnableSPNServerPort() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseNoAuthentication<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUseNoAuthentication() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagNonXmlText<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationFlagNonXmlText() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnEPR<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationFlagReturnEPR() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObjectAndEPR<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationFlagReturnObjectAndEPR() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorMessage<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errornumber: u32, errormessage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetErrorMessage(::core::mem::transmute_copy(&errornumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *errormessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeep<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationFlagHierarchyDeep() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyShallow<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationFlagHierarchyShallow() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeepBasePropsOnly<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationFlagHierarchyDeepBasePropsOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObject<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationFlagReturnObject() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSMan_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWSManEx as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWSMan as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEx2_Impl: Sized + super::Com::IDispatch_Impl + IWSMan_Impl + IWSManEx_Impl {
    fn SessionFlagUseClientCertificate(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEx2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx2_Impl, const OFFSET: isize>() -> IWSManEx2_Vtbl {
        unsafe extern "system" fn SessionFlagUseClientCertificate<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUseClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IWSManEx_Vtbl::new::<Identity, Impl, OFFSET>(), SessionFlagUseClientCertificate: SessionFlagUseClientCertificate::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManEx2 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWSMan as ::windows::core::Interface>::IID || iid == &<IWSManEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEx3_Impl: Sized + super::Com::IDispatch_Impl + IWSMan_Impl + IWSManEx_Impl + IWSManEx2_Impl {
    fn SessionFlagUTF16(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseCredSsp(&mut self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagAssociationInstance(&mut self) -> ::windows::core::Result<i32>;
    fn EnumerationFlagAssociatedInstance(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagSkipRevocationCheck(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagAllowNegotiateImplicitCredentials(&mut self) -> ::windows::core::Result<i32>;
    fn SessionFlagUseSsl(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEx3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3_Impl, const OFFSET: isize>() -> IWSManEx3_Vtbl {
        unsafe extern "system" fn SessionFlagUTF16<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUTF16() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseCredSsp<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUseCredSsp() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociationInstance<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationFlagAssociationInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociatedInstance<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).EnumerationFlagAssociatedInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipRevocationCheck<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagSkipRevocationCheck() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagAllowNegotiateImplicitCredentials<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagAllowNegotiateImplicitCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseSsl<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).SessionFlagUseSsl() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSManEx2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWSManEx3 as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IWSMan as ::windows::core::Interface>::IID || iid == &<IWSManEx as ::windows::core::Interface>::IID || iid == &<IWSManEx2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManInternal_Impl: Sized + super::Com::IDispatch_Impl {
    fn ConfigSDDL(&mut self, session: &::core::option::Option<super::Com::IDispatch>, resourceuri: &super::Com::VARIANT, flags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManInternal_Impl, const OFFSET: isize>() -> IWSManInternal_Vtbl {
        unsafe extern "system" fn ConfigSDDL<Identity: ::windows::core::IUnknownImpl, Impl: IWSManInternal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ConfigSDDL(::core::mem::transmute(&session), ::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), ConfigSDDL: ConfigSDDL::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManInternal as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManResourceLocator_Impl: Sized + super::Com::IDispatch_Impl {
    fn SetResourceURI(&mut self, uri: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ResourceURI(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn AddSelector(&mut self, resourceselname: &super::super::Foundation::BSTR, selvalue: &super::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ClearSelectors(&mut self) -> ::windows::core::Result<()>;
    fn FragmentPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFragmentPath(&mut self, text: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn FragmentDialect(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetFragmentDialect(&mut self, text: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddOption(&mut self, optionname: &super::super::Foundation::BSTR, optionvalue: &super::Com::VARIANT, mustcomply: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetMustUnderstandOptions(&mut self, mustunderstand: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn MustUnderstandOptions(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn ClearOptions(&mut self) -> ::windows::core::Result<()>;
    fn Error(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManResourceLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>() -> IWSManResourceLocator_Vtbl {
        unsafe extern "system" fn SetResourceURI<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetResourceURI(::core::mem::transmute_copy(&uri)).into()
        }
        unsafe extern "system" fn ResourceURI<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ResourceURI() {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSelector<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceselname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, selvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddSelector(::core::mem::transmute_copy(&resourceselname), ::core::mem::transmute_copy(&selvalue)).into()
        }
        unsafe extern "system" fn ClearSelectors<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearSelectors().into()
        }
        unsafe extern "system" fn FragmentPath<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FragmentPath() {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFragmentPath<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFragmentPath(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn FragmentDialect<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FragmentDialect() {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFragmentDialect<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetFragmentDialect(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn AddOption<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>, mustcomply: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddOption(::core::mem::transmute_copy(&optionname), ::core::mem::transmute_copy(&optionvalue), ::core::mem::transmute_copy(&mustcomply)).into()
        }
        unsafe extern "system" fn SetMustUnderstandOptions<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mustunderstand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMustUnderstandOptions(::core::mem::transmute_copy(&mustunderstand)).into()
        }
        unsafe extern "system" fn MustUnderstandOptions<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mustunderstand: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MustUnderstandOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *mustunderstand = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearOptions<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ClearOptions().into()
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWSManResourceLocator as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IWSManResourceLocatorInternal_Impl: Sized {}
impl IWSManResourceLocatorInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocatorInternal_Impl, const OFFSET: isize>() -> IWSManResourceLocatorInternal_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManResourceLocatorInternal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManSession_Impl: Sized + super::Com::IDispatch_Impl {
    fn Get(&mut self, resourceuri: &super::Com::VARIANT, flags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Put(&mut self, resourceuri: &super::Com::VARIANT, resource: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Create(&mut self, resourceuri: &super::Com::VARIANT, resource: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Delete(&mut self, resourceuri: &super::Com::VARIANT, flags: i32) -> ::windows::core::Result<()>;
    fn Invoke2(&mut self, actionuri: &super::super::Foundation::BSTR, resourceuri: &super::Com::VARIANT, parameters: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Enumerate(&mut self, resourceuri: &super::Com::VARIANT, filter: &super::super::Foundation::BSTR, dialect: &super::super::Foundation::BSTR, flags: i32) -> ::windows::core::Result<super::Com::IDispatch>;
    fn Identify(&mut self, flags: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Error(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BatchItems(&mut self) -> ::windows::core::Result<i32>;
    fn SetBatchItems(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn Timeout(&mut self) -> ::windows::core::Result<i32>;
    fn SetTimeout(&mut self, value: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManSession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>() -> IWSManSession_Vtbl {
        unsafe extern "system" fn Get<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, resource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, resultresource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Put(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&resource), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, resource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, newuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&resource), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *newuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Invoke2<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actionuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, parameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, result: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Invoke2(::core::mem::transmute_copy(&actionuri), ::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&parameters), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enumerate<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dialect: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, resultset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Enumerate(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&dialect), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identify<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, result: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Identify(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatchItems<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BatchItems() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchItems<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBatchItems(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Timeout<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Timeout() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetTimeout(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
        iid == &<IWSManSession as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
