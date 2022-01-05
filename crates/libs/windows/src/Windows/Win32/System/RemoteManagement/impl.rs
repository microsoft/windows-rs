#[cfg(feature = "Win32_System_Com")]
pub trait IWSManImpl: Sized + IDispatchImpl {
    fn CreateSession();
    fn CreateConnectionOptions();
    fn CommandLine();
    fn Error();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManConnectionOptionsImpl: Sized + IDispatchImpl {
    fn UserName();
    fn SetUserName();
    fn SetPassword();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWSManConnectionOptionsExImpl: Sized + IWSManConnectionOptionsImpl + IDispatchImpl {
    fn CertificateThumbprint();
    fn SetCertificateThumbprint();
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
pub trait IWSManEnumeratorImpl: Sized + IDispatchImpl {
    fn ReadItem();
    fn AtEndOfStream();
    fn Error();
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
pub trait IWSManEx2Impl: Sized + IWSManExImpl + IWSManImpl + IDispatchImpl {
    fn SessionFlagUseClientCertificate();
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
pub trait IWSManInternalImpl: Sized + IDispatchImpl {
    fn ConfigSDDL();
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
pub trait IWSManResourceLocatorInternalImpl: Sized {}
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
