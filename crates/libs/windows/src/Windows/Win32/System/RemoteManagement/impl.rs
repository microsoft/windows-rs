#[cfg(feature = "Win32_System_Com")]
pub trait IWSManImpl: Sized + IDispatchImpl {
    fn CreateSession();
    fn CreateConnectionOptions();
    fn CommandLine();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSMan {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSMan";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManVtbl {
    pub const fn new<Impl: IWSManImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManVtbl {
        unsafe extern "system" fn CreateSession<Impl: IWSManImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connection: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, connectionoptions: ::windows::core::RawPtr, session: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateSession(&*(&connection as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), flags, &*(&connectionoptions as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&session)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateConnectionOptions<Impl: IWSManImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, connectionoptions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateConnectionOptions(::core::mem::transmute_copy(&connectionoptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CommandLine<Impl: IWSManImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CommandLine(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IWSManImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSMan>, base.5, CreateSession::<Impl, OFFSET>, CreateConnectionOptions::<Impl, OFFSET>, CommandLine::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManConnectionOptionsImpl: Sized + IDispatchImpl {
    fn UserName();
    fn SetUserName();
    fn SetPassword();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManConnectionOptions {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManConnectionOptions";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptionsVtbl {
    pub const fn new<Impl: IWSManConnectionOptionsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManConnectionOptionsVtbl {
        unsafe extern "system" fn UserName<Impl: IWSManConnectionOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).UserName(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUserName<Impl: IWSManConnectionOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetUserName(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPassword<Impl: IWSManConnectionOptionsImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetPassword(&*(&password as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSManConnectionOptions>, base.5, UserName::<Impl, OFFSET>, SetUserName::<Impl, OFFSET>, SetPassword::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManConnectionOptionsExImpl: Sized + IWSManConnectionOptionsImpl + IDispatchImpl {
    fn CertificateThumbprint();
    fn SetCertificateThumbprint();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManConnectionOptionsEx {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManConnectionOptionsEx";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptionsExVtbl {
    pub const fn new<Impl: IWSManConnectionOptionsExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManConnectionOptionsExVtbl {
        unsafe extern "system" fn CertificateThumbprint<Impl: IWSManConnectionOptionsExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thumbprint: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CertificateThumbprint(::core::mem::transmute_copy(&thumbprint)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCertificateThumbprint<Impl: IWSManConnectionOptionsExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, thumbprint: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCertificateThumbprint(&*(&thumbprint as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSManConnectionOptionsEx>, base.5, CertificateThumbprint::<Impl, OFFSET>, SetCertificateThumbprint::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManConnectionOptionsEx2Impl: Sized + IWSManConnectionOptionsExImpl + IWSManConnectionOptionsImpl + IDispatchImpl {
    fn SetProxy();
    fn ProxyIEConfig();
    fn ProxyWinHttpConfig();
    fn ProxyAutoDetect();
    fn ProxyNoProxyServer();
    fn ProxyAuthenticationUseNegotiate();
    fn ProxyAuthenticationUseBasic();
    fn ProxyAuthenticationUseDigest();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManConnectionOptionsEx2 {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManConnectionOptionsEx2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManConnectionOptionsEx2Vtbl {
    pub const fn new<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManConnectionOptionsEx2Vtbl {
        unsafe extern "system" fn SetProxy<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, accesstype: i32, authenticationmechanism: i32, username: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, password: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProxy(accesstype, authenticationmechanism, &*(&username as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&password as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyIEConfig<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyIEConfig(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyWinHttpConfig<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyWinHttpConfig(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAutoDetect<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyAutoDetect(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyNoProxyServer<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyNoProxyServer(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseNegotiate<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyAuthenticationUseNegotiate(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseBasic<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyAuthenticationUseBasic(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProxyAuthenticationUseDigest<Impl: IWSManConnectionOptionsEx2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ProxyAuthenticationUseDigest(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSManConnectionOptionsEx2>, base.5, SetProxy::<Impl, OFFSET>, ProxyIEConfig::<Impl, OFFSET>, ProxyWinHttpConfig::<Impl, OFFSET>, ProxyAutoDetect::<Impl, OFFSET>, ProxyNoProxyServer::<Impl, OFFSET>, ProxyAuthenticationUseNegotiate::<Impl, OFFSET>, ProxyAuthenticationUseBasic::<Impl, OFFSET>, ProxyAuthenticationUseDigest::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManEnumeratorImpl: Sized + IDispatchImpl {
    fn ReadItem();
    fn AtEndOfStream();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManEnumerator {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManEnumerator";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManEnumeratorVtbl {
    pub const fn new<Impl: IWSManEnumeratorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManEnumeratorVtbl {
        unsafe extern "system" fn ReadItem<Impl: IWSManEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ReadItem(::core::mem::transmute_copy(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AtEndOfStream<Impl: IWSManEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eos: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AtEndOfStream(::core::mem::transmute_copy(&eos)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IWSManEnumeratorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSManEnumerator>, base.5, ReadItem::<Impl, OFFSET>, AtEndOfStream::<Impl, OFFSET>, Error::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManExImpl: Sized + IWSManImpl + IDispatchImpl {
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManEx {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManEx";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManExVtbl {
    pub const fn new<Impl: IWSManExImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManExVtbl {
        unsafe extern "system" fn CreateResourceLocator<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strresourcelocator: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newresourcelocator: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).CreateResourceLocator(&*(&strresourcelocator as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newresourcelocator)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUTF8<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUTF8(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagCredUsernamePassword<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagCredUsernamePassword(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipCACheck<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagSkipCACheck(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipCNCheck<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagSkipCNCheck(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseDigest<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseDigest(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseNegotiate<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseNegotiate(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseBasic<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseBasic(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseKerberos<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseKerberos(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagNoEncryption<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagNoEncryption(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagEnableSPNServerPort<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagEnableSPNServerPort(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseNoAuthentication<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseNoAuthentication(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagNonXmlText<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagNonXmlText(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnEPR<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagReturnEPR(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObjectAndEPR<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagReturnObjectAndEPR(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetErrorMessage<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errornumber: u32, errormessage: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorMessage(errornumber, ::core::mem::transmute_copy(&errormessage)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeep<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagHierarchyDeep(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyShallow<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagHierarchyShallow(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagHierarchyDeepBasePropsOnly<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagHierarchyDeepBasePropsOnly(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagReturnObject<Impl: IWSManExImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagReturnObject(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWSManEx>,
            base.5,
            CreateResourceLocator::<Impl, OFFSET>,
            SessionFlagUTF8::<Impl, OFFSET>,
            SessionFlagCredUsernamePassword::<Impl, OFFSET>,
            SessionFlagSkipCACheck::<Impl, OFFSET>,
            SessionFlagSkipCNCheck::<Impl, OFFSET>,
            SessionFlagUseDigest::<Impl, OFFSET>,
            SessionFlagUseNegotiate::<Impl, OFFSET>,
            SessionFlagUseBasic::<Impl, OFFSET>,
            SessionFlagUseKerberos::<Impl, OFFSET>,
            SessionFlagNoEncryption::<Impl, OFFSET>,
            SessionFlagEnableSPNServerPort::<Impl, OFFSET>,
            SessionFlagUseNoAuthentication::<Impl, OFFSET>,
            EnumerationFlagNonXmlText::<Impl, OFFSET>,
            EnumerationFlagReturnEPR::<Impl, OFFSET>,
            EnumerationFlagReturnObjectAndEPR::<Impl, OFFSET>,
            GetErrorMessage::<Impl, OFFSET>,
            EnumerationFlagHierarchyDeep::<Impl, OFFSET>,
            EnumerationFlagHierarchyShallow::<Impl, OFFSET>,
            EnumerationFlagHierarchyDeepBasePropsOnly::<Impl, OFFSET>,
            EnumerationFlagReturnObject::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManEx2Impl: Sized + IWSManExImpl + IWSManImpl + IDispatchImpl {
    fn SessionFlagUseClientCertificate();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManEx2 {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManEx2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx2Vtbl {
    pub const fn new<Impl: IWSManEx2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManEx2Vtbl {
        unsafe extern "system" fn SessionFlagUseClientCertificate<Impl: IWSManEx2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseClientCertificate(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSManEx2>, base.5, SessionFlagUseClientCertificate::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManEx3Impl: Sized + IWSManEx2Impl + IWSManExImpl + IWSManImpl + IDispatchImpl {
    fn SessionFlagUTF16();
    fn SessionFlagUseCredSsp();
    fn EnumerationFlagAssociationInstance();
    fn EnumerationFlagAssociatedInstance();
    fn SessionFlagSkipRevocationCheck();
    fn SessionFlagAllowNegotiateImplicitCredentials();
    fn SessionFlagUseSsl();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManEx3 {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManEx3";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManEx3Vtbl {
    pub const fn new<Impl: IWSManEx3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManEx3Vtbl {
        unsafe extern "system" fn SessionFlagUTF16<Impl: IWSManEx3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUTF16(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseCredSsp<Impl: IWSManEx3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseCredSsp(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociationInstance<Impl: IWSManEx3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagAssociationInstance(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumerationFlagAssociatedInstance<Impl: IWSManEx3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).EnumerationFlagAssociatedInstance(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagSkipRevocationCheck<Impl: IWSManEx3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagSkipRevocationCheck(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagAllowNegotiateImplicitCredentials<Impl: IWSManEx3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagAllowNegotiateImplicitCredentials(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionFlagUseSsl<Impl: IWSManEx3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SessionFlagUseSsl(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSManEx3>, base.5, SessionFlagUTF16::<Impl, OFFSET>, SessionFlagUseCredSsp::<Impl, OFFSET>, EnumerationFlagAssociationInstance::<Impl, OFFSET>, EnumerationFlagAssociatedInstance::<Impl, OFFSET>, SessionFlagSkipRevocationCheck::<Impl, OFFSET>, SessionFlagAllowNegotiateImplicitCredentials::<Impl, OFFSET>, SessionFlagUseSsl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManInternalImpl: Sized + IDispatchImpl {
    fn ConfigSDDL();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManInternal {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManInternal";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManInternalVtbl {
    pub const fn new<Impl: IWSManInternalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManInternalVtbl {
        unsafe extern "system" fn ConfigSDDL<Impl: IWSManInternalImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, session: ::windows::core::RawPtr, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ConfigSDDL(&*(&session as *const <super::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&resourceuri as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSManInternal>, base.5, ConfigSDDL::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManResourceLocator {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManResourceLocator";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManResourceLocatorVtbl {
    pub const fn new<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManResourceLocatorVtbl {
        unsafe extern "system" fn SetResourceURI<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetResourceURI(&*(&uri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResourceURI<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ResourceURI(::core::mem::transmute_copy(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSelector<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceselname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, selvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddSelector(&*(&resourceselname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&selvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearSelectors<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearSelectors() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FragmentPath<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FragmentPath(::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFragmentPath<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFragmentPath(&*(&text as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FragmentDialect<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).FragmentDialect(::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFragmentDialect<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetFragmentDialect(&*(&text as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOption<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, optionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, optionvalue: ::core::mem::ManuallyDrop<super::Com::VARIANT>, mustcomply: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).AddOption(
                &*(&optionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&optionvalue as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&mustcomply as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMustUnderstandOptions<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mustunderstand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetMustUnderstandOptions(&*(&mustunderstand as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MustUnderstandOptions<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mustunderstand: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).MustUnderstandOptions(::core::mem::transmute_copy(&mustunderstand)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearOptions<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ClearOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IWSManResourceLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IWSManResourceLocator>,
            base.5,
            SetResourceURI::<Impl, OFFSET>,
            ResourceURI::<Impl, OFFSET>,
            AddSelector::<Impl, OFFSET>,
            ClearSelectors::<Impl, OFFSET>,
            FragmentPath::<Impl, OFFSET>,
            SetFragmentPath::<Impl, OFFSET>,
            FragmentDialect::<Impl, OFFSET>,
            SetFragmentDialect::<Impl, OFFSET>,
            AddOption::<Impl, OFFSET>,
            SetMustUnderstandOptions::<Impl, OFFSET>,
            MustUnderstandOptions::<Impl, OFFSET>,
            ClearOptions::<Impl, OFFSET>,
            Error::<Impl, OFFSET>,
        )
    }
}
pub trait IWSManResourceLocatorInternalImpl: Sized {}
impl ::windows::core::RuntimeName for IWSManResourceLocatorInternal {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManResourceLocatorInternal";
}
impl IWSManResourceLocatorInternalVtbl {
    pub const fn new<Impl: IWSManResourceLocatorInternalImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManResourceLocatorInternalVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSManResourceLocatorInternal>, base.5)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWSManSession {
    const NAME: &'static str = "Windows.Win32.System.RemoteManagement.IWSManSession";
}
#[cfg(feature = "Win32_System_Com")]
impl IWSManSessionVtbl {
    pub const fn new<Impl: IWSManSessionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IWSManSessionVtbl {
        unsafe extern "system" fn Get<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32, resource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Get(&*(&resourceuri as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&resource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Put<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, resource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, resultresource: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Put(&*(&resourceuri as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&resource as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&resultresource)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Create<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, resource: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, newuri: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Create(&*(&resourceuri as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&resource as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), flags, ::core::mem::transmute_copy(&newuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, flags: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&resourceuri as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), flags) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Invoke<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, actionuri: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, parameters: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, result: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Invoke(
                &*(&actionuri as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&resourceuri as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&parameters as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                flags,
                ::core::mem::transmute_copy(&result),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Enumerate<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resourceuri: ::core::mem::ManuallyDrop<super::Com::VARIANT>, filter: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, dialect: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, flags: i32, resultset: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Enumerate(
                &*(&resourceuri as *const <super::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&filter as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&dialect as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                flags,
                ::core::mem::transmute_copy(&resultset),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Identify<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, flags: i32, result: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Identify(flags, ::core::mem::transmute_copy(&result)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Error<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Error(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BatchItems<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).BatchItems(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBatchItems<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetBatchItems(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Timeout<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Timeout(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetTimeout<Impl: IWSManSessionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetTimeout(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IWSManSession>, base.5, Get::<Impl, OFFSET>, Put::<Impl, OFFSET>, Create::<Impl, OFFSET>, Delete::<Impl, OFFSET>, Invoke::<Impl, OFFSET>, Enumerate::<Impl, OFFSET>, Identify::<Impl, OFFSET>, Error::<Impl, OFFSET>, BatchItems::<Impl, OFFSET>, SetBatchItems::<Impl, OFFSET>, Timeout::<Impl, OFFSET>, SetTimeout::<Impl, OFFSET>)
    }
}
