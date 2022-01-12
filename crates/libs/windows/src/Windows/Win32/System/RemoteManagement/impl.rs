#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManImpl: Sized + IDispatchImpl {
    fn CreateSession();
    fn CreateConnectionOptions();
    fn CommandLine();
    fn Error();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManVtbl {
        unsafe extern "system" fn CreateSession<Impl: IWSManImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, connectionoptions: ::windows::core::RawPtr, session: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateConnectionOptions<Impl: IWSManImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CommandLine<Impl: IWSManImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: IWSManImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateSession: CreateSession::<Impl, IMPL_OFFSET>,
            CreateConnectionOptions: CreateConnectionOptions::<Impl, IMPL_OFFSET>,
            CommandLine: CommandLine::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSMan as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManConnectionOptionsImpl: Sized + IDispatchImpl {
    fn UserName();
    fn SetUserName();
    fn SetPassword();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManConnectionOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManConnectionOptionsVtbl {
        unsafe extern "system" fn UserName<Impl: IWSManConnectionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUserName<Impl: IWSManConnectionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPassword<Impl: IWSManConnectionOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UserName: UserName::<Impl, IMPL_OFFSET>,
            SetUserName: SetUserName::<Impl, IMPL_OFFSET>,
            SetPassword: SetPassword::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManConnectionOptions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManConnectionOptionsExImpl: Sized + IDispatchImpl + IWSManConnectionOptionsImpl {
    fn CertificateThumbprint();
    fn SetCertificateThumbprint();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManConnectionOptionsExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManConnectionOptionsExVtbl {
        unsafe extern "system" fn CertificateThumbprint<Impl: IWSManConnectionOptionsExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprint: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCertificateThumbprint<Impl: IWSManConnectionOptionsExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, thumbprint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWSManConnectionOptionsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CertificateThumbprint: CertificateThumbprint::<Impl, IMPL_OFFSET>,
            SetCertificateThumbprint: SetCertificateThumbprint::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManConnectionOptionsEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManConnectionOptionsEx2Impl: Sized + IDispatchImpl + IWSManConnectionOptionsImpl + IWSManConnectionOptionsExImpl {
    fn SetProxy();
    fn ProxyIEConfig();
    fn ProxyWinHttpConfig();
    fn ProxyAutoDetect();
    fn ProxyNoProxyServer();
    fn ProxyAuthenticationUseNegotiate();
    fn ProxyAuthenticationUseBasic();
    fn ProxyAuthenticationUseDigest();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManConnectionOptionsEx2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManConnectionOptionsEx2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManConnectionOptionsEx2Vtbl {
        unsafe extern "system" fn SetProxy<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, accesstype: i32, authenticationmechanism: i32, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProxyIEConfig<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProxyWinHttpConfig<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProxyAutoDetect<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProxyNoProxyServer<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProxyAuthenticationUseNegotiate<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProxyAuthenticationUseBasic<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProxyAuthenticationUseDigest<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWSManConnectionOptionsExVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<IWSManConnectionOptionsEx2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEnumeratorImpl: Sized + IDispatchImpl {
    fn ReadItem();
    fn AtEndOfStream();
    fn Error();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEnumeratorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEnumeratorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManEnumeratorVtbl {
        unsafe extern "system" fn ReadItem<Impl: IWSManEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AtEndOfStream<Impl: IWSManEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eos: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: IWSManEnumeratorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ReadItem: ReadItem::<Impl, IMPL_OFFSET>,
            AtEndOfStream: AtEndOfStream::<Impl, IMPL_OFFSET>,
            Error: Error::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManEnumerator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManExImpl: Sized + IDispatchImpl + IWSManImpl {
    fn CreateResourceLocator();
    fn SessionFlagUTF8();
    fn SessionFlagCredUsernamePassword();
    fn SessionFlagSkipCACheck();
    fn SessionFlagSkipCNCheck();
    fn SessionFlagUseDigest();
    fn SessionFlagUseNegotiate();
    fn SessionFlagUseBasic();
    fn SessionFlagUseKerberos();
    fn SessionFlagNoEncryption();
    fn SessionFlagEnableSPNServerPort();
    fn SessionFlagUseNoAuthentication();
    fn EnumerationFlagNonXmlText();
    fn EnumerationFlagReturnEPR();
    fn EnumerationFlagReturnObjectAndEPR();
    fn GetErrorMessage();
    fn EnumerationFlagHierarchyDeep();
    fn EnumerationFlagHierarchyShallow();
    fn EnumerationFlagHierarchyDeepBasePropsOnly();
    fn EnumerationFlagReturnObject();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManExVtbl {
        unsafe extern "system" fn CreateResourceLocator<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, strresourcelocator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newresourcelocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagUTF8<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagCredUsernamePassword<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagSkipCACheck<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagSkipCNCheck<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagUseDigest<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagUseNegotiate<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagUseBasic<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagUseKerberos<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagNoEncryption<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagEnableSPNServerPort<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagUseNoAuthentication<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerationFlagNonXmlText<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerationFlagReturnEPR<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerationFlagReturnObjectAndEPR<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetErrorMessage<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, errornumber: u32, errormessage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeep<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerationFlagHierarchyShallow<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeepBasePropsOnly<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerationFlagReturnObject<Impl: IWSManExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWSManVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<IWSManEx as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEx2Impl: Sized + IDispatchImpl + IWSManImpl + IWSManExImpl {
    fn SessionFlagUseClientCertificate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEx2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManEx2Vtbl {
        unsafe extern "system" fn SessionFlagUseClientCertificate<Impl: IWSManEx2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWSManExVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SessionFlagUseClientCertificate: SessionFlagUseClientCertificate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManEx2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManEx3Impl: Sized + IDispatchImpl + IWSManImpl + IWSManExImpl + IWSManEx2Impl {
    fn SessionFlagUTF16();
    fn SessionFlagUseCredSsp();
    fn EnumerationFlagAssociationInstance();
    fn EnumerationFlagAssociatedInstance();
    fn SessionFlagSkipRevocationCheck();
    fn SessionFlagAllowNegotiateImplicitCredentials();
    fn SessionFlagUseSsl();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManEx3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManEx3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManEx3Vtbl {
        unsafe extern "system" fn SessionFlagUTF16<Impl: IWSManEx3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagUseCredSsp<Impl: IWSManEx3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerationFlagAssociationInstance<Impl: IWSManEx3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumerationFlagAssociatedInstance<Impl: IWSManEx3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagSkipRevocationCheck<Impl: IWSManEx3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagAllowNegotiateImplicitCredentials<Impl: IWSManEx3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionFlagUseSsl<Impl: IWSManEx3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWSManEx2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<IWSManEx3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManInternalImpl: Sized + IDispatchImpl {
    fn ConfigSDDL();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManInternalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManInternalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManInternalVtbl {
        unsafe extern "system" fn ConfigSDDL<Impl: IWSManInternalImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ConfigSDDL: ConfigSDDL::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManInternal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManResourceLocatorImpl: Sized + IDispatchImpl {
    fn SetResourceURI();
    fn ResourceURI();
    fn AddSelector();
    fn ClearSelectors();
    fn FragmentPath();
    fn SetFragmentPath();
    fn FragmentDialect();
    fn SetFragmentDialect();
    fn AddOption();
    fn SetMustUnderstandOptions();
    fn MustUnderstandOptions();
    fn ClearOptions();
    fn Error();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManResourceLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManResourceLocatorVtbl {
        unsafe extern "system" fn SetResourceURI<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResourceURI<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSelector<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceselname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, selvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearSelectors<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FragmentPath<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFragmentPath<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FragmentDialect<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFragmentDialect<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddOption<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, optionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>, mustcomply: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMustUnderstandOptions<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mustunderstand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MustUnderstandOptions<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, mustunderstand: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearOptions<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: IWSManResourceLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
        iid == &<IWSManResourceLocator as ::windows::core::Interface>::IID
    }
}
pub trait IWSManResourceLocatorInternalImpl: Sized {}
impl IWSManResourceLocatorInternalVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManResourceLocatorInternalImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManResourceLocatorInternalVtbl {
        Self { base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWSManResourceLocatorInternal as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWSManSessionImpl: Sized + IDispatchImpl {
    fn Get();
    fn Put();
    fn Create();
    fn Delete();
    fn Invoke();
    fn Enumerate();
    fn Identify();
    fn Error();
    fn BatchItems();
    fn SetBatchItems();
    fn Timeout();
    fn SetTimeout();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWSManSessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWSManSessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWSManSessionVtbl {
        unsafe extern "system" fn Get<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Put<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, resource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, resultresource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Create<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, resource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, newuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Delete<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Invoke<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, actionuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, parameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, result: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Enumerate<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dialect: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, resultset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Identify<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: i32, result: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Error<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BatchItems<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBatchItems<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Timeout<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetTimeout<Impl: IWSManSessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Get: Get::<Impl, IMPL_OFFSET>,
            Put: Put::<Impl, IMPL_OFFSET>,
            Create: Create::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Invoke: Invoke::<Impl, IMPL_OFFSET>,
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
        iid == &<IWSManSession as ::windows::core::Interface>::IID
    }
}
