#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSMan_Impl: Sized + super::Com::IDispatch_Impl {
    fn CreateSession(&mut self, connection: &super::super::Foundation::BSTR, flags: i32, connectionoptions: &::core::option::Option<super::Com::IDispatch>) -> ::windows::core::Result<super::Com::IDispatch>;
    fn CreateConnectionOptions(&mut self) -> ::windows::core::Result<super::Com::IDispatch>;
    fn CommandLine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Error(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSMan_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSMan_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSMan_Vtbl {
        unsafe extern "system" fn CreateSession<Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, connectionoptions: ::windows::core::RawPtr, session: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateSession(::core::mem::transmute_copy(&connection), ::core::mem::transmute_copy(&flags), ::core::mem::transmute(&connectionoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *session = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConnectionOptions<Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateConnectionOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *connectionoptions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandLine<Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CommandLine() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IWSMan_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateSession: CreateSession::<Impl, IMPL_OFFSET>,
            CreateConnectionOptions: CreateConnectionOptions::<Impl, IMPL_OFFSET>,
            CommandLine: CommandLine::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManConnectionOptions_Vtbl {
        unsafe extern "system" fn UserName<Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserName() {
                ::core::result::Result::Ok(ok__) => {
                    *name = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUserName(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn SetPassword<Impl: IWSManConnectionOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPassword(::core::mem::transmute_copy(&password)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UserName: UserName::<Impl, IMPL_OFFSET>,
            SetUserName: SetUserName::<Impl, IMPL_OFFSET>,
            SetPassword: SetPassword::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManConnectionOptionsEx_Vtbl {
        unsafe extern "system" fn CertificateThumbprint<Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprint: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CertificateThumbprint() {
                ::core::result::Result::Ok(ok__) => {
                    *thumbprint = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateThumbprint<Impl: IWSManConnectionOptionsEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCertificateThumbprint(::core::mem::transmute_copy(&thumbprint)).into()
        }
        Self {
            base: IWSManConnectionOptions_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CertificateThumbprint: CertificateThumbprint::<Impl, IMPL_OFFSET>,
            SetCertificateThumbprint: SetCertificateThumbprint::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManConnectionOptionsEx2_Vtbl {
        unsafe extern "system" fn SetProxy<Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: i32, authenticationmechanism: i32, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProxy(::core::mem::transmute_copy(&accesstype), ::core::mem::transmute_copy(&authenticationmechanism), ::core::mem::transmute_copy(&username), ::core::mem::transmute_copy(&password)).into()
        }
        unsafe extern "system" fn ProxyIEConfig<Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyIEConfig() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyWinHttpConfig<Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyWinHttpConfig() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAutoDetect<Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyAutoDetect() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyNoProxyServer<Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyNoProxyServer() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseNegotiate<Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyAuthenticationUseNegotiate() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseBasic<Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyAuthenticationUseBasic() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseDigest<Impl: IWSManConnectionOptionsEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProxyAuthenticationUseDigest() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSManConnectionOptionsEx_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetProxy: SetProxy::<Impl, IMPL_OFFSET>,
            ProxyIEConfig: ProxyIEConfig::<Impl, IMPL_OFFSET>,
            ProxyWinHttpConfig: ProxyWinHttpConfig::<Impl, IMPL_OFFSET>,
            ProxyAutoDetect: ProxyAutoDetect::<Impl, IMPL_OFFSET>,
            ProxyNoProxyServer: ProxyNoProxyServer::<Impl, IMPL_OFFSET>,
            ProxyAuthenticationUseNegotiate: ProxyAuthenticationUseNegotiate::<Impl, IMPL_OFFSET>,
            ProxyAuthenticationUseBasic: ProxyAuthenticationUseBasic::<Impl, IMPL_OFFSET>,
            ProxyAuthenticationUseDigest: ProxyAuthenticationUseDigest::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEnumerator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManEnumerator_Vtbl {
        unsafe extern "system" fn ReadItem<Impl: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadItem() {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AtEndOfStream<Impl: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eos: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AtEndOfStream() {
                ::core::result::Result::Ok(ok__) => {
                    *eos = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IWSManEnumerator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ReadItem: ReadItem::<Impl, IMPL_OFFSET>,
            AtEndOfStream: AtEndOfStream::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManEx_Vtbl {
        unsafe extern "system" fn CreateResourceLocator<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresourcelocator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newresourcelocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResourceLocator(::core::mem::transmute_copy(&strresourcelocator)) {
                ::core::result::Result::Ok(ok__) => {
                    *newresourcelocator = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUTF8<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUTF8() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagCredUsernamePassword<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagCredUsernamePassword() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipCACheck<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagSkipCACheck() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipCNCheck<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagSkipCNCheck() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseDigest<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseDigest() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseNegotiate<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseNegotiate() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseBasic<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseBasic() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseKerberos<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseKerberos() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagNoEncryption<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagNoEncryption() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagEnableSPNServerPort<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagEnableSPNServerPort() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseNoAuthentication<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseNoAuthentication() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagNonXmlText<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagNonXmlText() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnEPR<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagReturnEPR() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObjectAndEPR<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagReturnObjectAndEPR() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorMessage<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errornumber: u32, errormessage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetErrorMessage(::core::mem::transmute_copy(&errornumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *errormessage = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeep<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagHierarchyDeep() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyShallow<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagHierarchyShallow() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeepBasePropsOnly<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagHierarchyDeepBasePropsOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObject<Impl: IWSManEx_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagReturnObject() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSMan_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateResourceLocator: CreateResourceLocator::<Impl, IMPL_OFFSET>,
            SessionFlagUTF8: SessionFlagUTF8::<Impl, IMPL_OFFSET>,
            SessionFlagCredUsernamePassword: SessionFlagCredUsernamePassword::<Impl, IMPL_OFFSET>,
            SessionFlagSkipCACheck: SessionFlagSkipCACheck::<Impl, IMPL_OFFSET>,
            SessionFlagSkipCNCheck: SessionFlagSkipCNCheck::<Impl, IMPL_OFFSET>,
            SessionFlagUseDigest: SessionFlagUseDigest::<Impl, IMPL_OFFSET>,
            SessionFlagUseNegotiate: SessionFlagUseNegotiate::<Impl, IMPL_OFFSET>,
            SessionFlagUseBasic: SessionFlagUseBasic::<Impl, IMPL_OFFSET>,
            SessionFlagUseKerberos: SessionFlagUseKerberos::<Impl, IMPL_OFFSET>,
            SessionFlagNoEncryption: SessionFlagNoEncryption::<Impl, IMPL_OFFSET>,
            SessionFlagEnableSPNServerPort: SessionFlagEnableSPNServerPort::<Impl, IMPL_OFFSET>,
            SessionFlagUseNoAuthentication: SessionFlagUseNoAuthentication::<Impl, IMPL_OFFSET>,
            EnumerationFlagNonXmlText: EnumerationFlagNonXmlText::<Impl, IMPL_OFFSET>,
            EnumerationFlagReturnEPR: EnumerationFlagReturnEPR::<Impl, IMPL_OFFSET>,
            EnumerationFlagReturnObjectAndEPR: EnumerationFlagReturnObjectAndEPR::<Impl, IMPL_OFFSET>,
            GetErrorMessage: GetErrorMessage::<Impl, IMPL_OFFSET>,
            EnumerationFlagHierarchyDeep: EnumerationFlagHierarchyDeep::<Impl, IMPL_OFFSET>,
            EnumerationFlagHierarchyShallow: EnumerationFlagHierarchyShallow::<Impl, IMPL_OFFSET>,
            EnumerationFlagHierarchyDeepBasePropsOnly: EnumerationFlagHierarchyDeepBasePropsOnly::<Impl, IMPL_OFFSET>,
            EnumerationFlagReturnObject: EnumerationFlagReturnObject::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManEx2_Vtbl {
        unsafe extern "system" fn SessionFlagUseClientCertificate<Impl: IWSManEx2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseClientCertificate() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSManEx_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SessionFlagUseClientCertificate: SessionFlagUseClientCertificate::<Impl, IMPL_OFFSET>,
        }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManEx3_Vtbl {
        unsafe extern "system" fn SessionFlagUTF16<Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUTF16() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseCredSsp<Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseCredSsp() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociationInstance<Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagAssociationInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociatedInstance<Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagAssociatedInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipRevocationCheck<Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagSkipRevocationCheck() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagAllowNegotiateImplicitCredentials<Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagAllowNegotiateImplicitCredentials() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseSsl<Impl: IWSManEx3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseSsl() {
                ::core::result::Result::Ok(ok__) => {
                    *flags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWSManEx2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SessionFlagUTF16: SessionFlagUTF16::<Impl, IMPL_OFFSET>,
            SessionFlagUseCredSsp: SessionFlagUseCredSsp::<Impl, IMPL_OFFSET>,
            EnumerationFlagAssociationInstance: EnumerationFlagAssociationInstance::<Impl, IMPL_OFFSET>,
            EnumerationFlagAssociatedInstance: EnumerationFlagAssociatedInstance::<Impl, IMPL_OFFSET>,
            SessionFlagSkipRevocationCheck: SessionFlagSkipRevocationCheck::<Impl, IMPL_OFFSET>,
            SessionFlagAllowNegotiateImplicitCredentials: SessionFlagAllowNegotiateImplicitCredentials::<Impl, IMPL_OFFSET>,
            SessionFlagUseSsl: SessionFlagUseSsl::<Impl, IMPL_OFFSET>,
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManInternal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManInternal_Vtbl {
        unsafe extern "system" fn ConfigSDDL<Impl: IWSManInternal_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConfigSDDL(::core::mem::transmute(&session), ::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ConfigSDDL: ConfigSDDL::<Impl, IMPL_OFFSET> }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManResourceLocator_Vtbl {
        unsafe extern "system" fn SetResourceURI<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResourceURI(::core::mem::transmute_copy(&uri)).into()
        }
        unsafe extern "system" fn ResourceURI<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResourceURI() {
                ::core::result::Result::Ok(ok__) => {
                    *uri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSelector<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceselname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, selvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSelector(::core::mem::transmute_copy(&resourceselname), ::core::mem::transmute_copy(&selvalue)).into()
        }
        unsafe extern "system" fn ClearSelectors<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearSelectors().into()
        }
        unsafe extern "system" fn FragmentPath<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FragmentPath() {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFragmentPath<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFragmentPath(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn FragmentDialect<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FragmentDialect() {
                ::core::result::Result::Ok(ok__) => {
                    *text = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFragmentDialect<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFragmentDialect(::core::mem::transmute_copy(&text)).into()
        }
        unsafe extern "system" fn AddOption<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>, mustcomply: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOption(::core::mem::transmute_copy(&optionname), ::core::mem::transmute_copy(&optionvalue), ::core::mem::transmute_copy(&mustcomply)).into()
        }
        unsafe extern "system" fn SetMustUnderstandOptions<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mustunderstand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMustUnderstandOptions(::core::mem::transmute_copy(&mustunderstand)).into()
        }
        unsafe extern "system" fn MustUnderstandOptions<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mustunderstand: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MustUnderstandOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *mustunderstand = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearOptions<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearOptions().into()
        }
        unsafe extern "system" fn Error<Impl: IWSManResourceLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetResourceURI: SetResourceURI::<Impl, IMPL_OFFSET>,
            ResourceURI: ResourceURI::<Impl, IMPL_OFFSET>,
            AddSelector: AddSelector::<Impl, IMPL_OFFSET>,
            ClearSelectors: ClearSelectors::<Impl, IMPL_OFFSET>,
            FragmentPath: FragmentPath::<Impl, IMPL_OFFSET>,
            SetFragmentPath: SetFragmentPath::<Impl, IMPL_OFFSET>,
            FragmentDialect: FragmentDialect::<Impl, IMPL_OFFSET>,
            SetFragmentDialect: SetFragmentDialect::<Impl, IMPL_OFFSET>,
            AddOption: AddOption::<Impl, IMPL_OFFSET>,
            SetMustUnderstandOptions: SetMustUnderstandOptions::<Impl, IMPL_OFFSET>,
            MustUnderstandOptions: MustUnderstandOptions::<Impl, IMPL_OFFSET>,
            ClearOptions: ClearOptions::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManResourceLocator as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
pub trait IWSManResourceLocatorInternal_Impl: Sized {}
impl IWSManResourceLocatorInternal_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocatorInternal_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManResourceLocatorInternal_Vtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManSession_Vtbl {
        unsafe extern "system" fn Get<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Get(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, resource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, resultresource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Put(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&resource), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultresource = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, resource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, newuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&resource), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *newuri = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&flags)).into()
        }
        unsafe extern "system" fn Invoke2<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actionuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, parameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, result: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Invoke2(::core::mem::transmute_copy(&actionuri), ::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&parameters), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enumerate<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dialect: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, resultset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Enumerate(::core::mem::transmute_copy(&resourceuri), ::core::mem::transmute_copy(&filter), ::core::mem::transmute_copy(&dialect), ::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *resultset = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identify<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, result: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Identify(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Error() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatchItems<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BatchItems() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchItems<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBatchItems(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Timeout<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Timeout() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeout<Impl: IWSManSession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetTimeout(::core::mem::transmute_copy(&value)).into()
        }
        Self {
            base: super::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Get: Get::<Impl, IMPL_OFFSET>,
            Put: Put::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Invoke2: Invoke2::<Impl, IMPL_OFFSET>,
            Enumerate: Enumerate::<Impl, IMPL_OFFSET>,
            Identify: Identify::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
            BatchItems: BatchItems::<Impl, IMPL_OFFSET>,
            SetBatchItems: SetBatchItems::<Impl, IMPL_OFFSET>,
            Timeout: Timeout::<Impl, IMPL_OFFSET>,
            SetTimeout: SetTimeout::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManSession as ::windows::core::Interface>::IID || iid == &<super::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
