pub trait IChannelCredentialsImpl: Sized + IDispatchImpl {
    fn SetWindowsCredential();
    fn SetUserNameCredential();
    fn SetClientCertificateFromStore();
    fn SetClientCertificateFromStoreByName();
    fn SetClientCertificateFromFile();
    fn SetDefaultServiceCertificateFromStore();
    fn SetDefaultServiceCertificateFromStoreByName();
    fn SetDefaultServiceCertificateFromFile();
    fn SetServiceCertificateAuthentication();
    fn SetIssuedToken();
}
