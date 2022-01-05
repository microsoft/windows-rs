#[cfg(feature = "Win32_System_Com")]
pub trait ICertSrvSetupImpl: Sized + IDispatchImpl {
    fn CAErrorId();
    fn CAErrorString();
    fn InitializeDefaults();
    fn GetCASetupProperty();
    fn SetCASetupProperty();
    fn IsPropertyEditable();
    fn GetSupportedCATypes();
    fn GetProviderNameList();
    fn GetKeyLengthList();
    fn GetHashAlgorithmList();
    fn GetPrivateKeyContainerList();
    fn GetExistingCACertificates();
    fn CAImportPFX();
    fn SetCADistinguishedName();
    fn SetDatabaseInformation();
    fn SetParentCAInformation();
    fn SetWebCAInformation();
    fn Install();
    fn PreUnInstall();
    fn PostUnInstall();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertSrvSetupKeyInformationImpl: Sized + IDispatchImpl {
    fn ProviderName();
    fn SetProviderName();
    fn Length();
    fn SetLength();
    fn Existing();
    fn SetExisting();
    fn ContainerName();
    fn SetContainerName();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn ExistingCACertificate();
    fn SetExistingCACertificate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertSrvSetupKeyInformationCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn Add();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificateEnrollmentPolicyServerSetupImpl: Sized + IDispatchImpl {
    fn ErrorString();
    fn InitializeInstallDefaults();
    fn GetProperty();
    fn SetProperty();
    fn Install();
    fn UnInstall();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificateEnrollmentServerSetupImpl: Sized + IDispatchImpl {
    fn ErrorString();
    fn InitializeInstallDefaults();
    fn GetProperty();
    fn SetProperty();
    fn SetApplicationPoolCredentials();
    fn Install();
    fn UnInstall();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMSCEPSetupImpl: Sized + IDispatchImpl {
    fn MSCEPErrorId();
    fn MSCEPErrorString();
    fn InitializeDefaults();
    fn GetMSCEPSetupProperty();
    fn SetMSCEPSetupProperty();
    fn SetAccountInformation();
    fn IsMSCEPStoreEmpty();
    fn GetProviderNameList();
    fn GetKeyLengthList();
    fn Install();
    fn PreUnInstall();
    fn PostUnInstall();
}
