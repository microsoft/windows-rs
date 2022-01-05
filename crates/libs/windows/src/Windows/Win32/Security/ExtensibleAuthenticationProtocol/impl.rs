pub trait IAccountingProviderConfigImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn Configure();
    fn Activate();
    fn Deactivate();
}
pub trait IAuthenticationProviderConfigImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn Configure();
    fn Activate();
    fn Deactivate();
}
pub trait IEAPProviderConfigImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn ServerInvokeConfigUI();
    fn RouterInvokeConfigUI();
    fn RouterInvokeCredentialsUI();
}
pub trait IEAPProviderConfig2Impl: Sized + IEAPProviderConfigImpl {
    fn ServerInvokeConfigUI2();
    fn GetGlobalConfig();
}
pub trait IEAPProviderConfig3Impl: Sized + IEAPProviderConfig2Impl + IEAPProviderConfigImpl {
    fn ServerInvokeCertificateConfigUI();
}
pub trait IRouterProtocolConfigImpl: Sized {
    fn AddProtocol();
    fn RemoveProtocol();
}
