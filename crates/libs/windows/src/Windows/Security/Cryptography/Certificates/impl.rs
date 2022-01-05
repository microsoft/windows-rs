#[cfg(feature = "implement_exclusive")]
pub trait ICertificateImpl: Sized {
    fn BuildChainAsync(&self, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>;
    fn BuildChainWithParametersAsync(&self, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>, parameters: &::core::option::Option<ChainBuildingParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<CertificateChain>>;
    fn SerialNumber(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn GetHashValue(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn GetHashValueWithAlgorithm(&self, hashalgorithmname: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn GetCertificateBlob(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Issuer(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HasPrivateKey(&self) -> ::windows::core::Result<bool>;
    fn IsStronglyProtected(&self) -> ::windows::core::Result<bool>;
    fn ValidFrom(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn ValidTo(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificate2Impl: Sized {
    fn IsSecurityDeviceBound(&self) -> ::windows::core::Result<bool>;
    fn KeyUsages(&self) -> ::windows::core::Result<CertificateKeyUsages>;
    fn KeyAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignatureAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SignatureHashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SubjectAlternativeName(&self) -> ::windows::core::Result<SubjectAlternativeNameInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificate3Impl: Sized {
    fn IsPerUser(&self) -> ::windows::core::Result<bool>;
    fn StoreName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateChainImpl: Sized {
    fn Validate(&self) -> ::windows::core::Result<ChainValidationResult>;
    fn ValidateWithParameters(&self, parameter: &::core::option::Option<ChainValidationParameters>) -> ::windows::core::Result<ChainValidationResult>;
    fn GetCertificates(&self, includeroot: bool) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateEnrollmentManagerStaticsImpl: Sized {
    fn CreateRequestAsync(&self, request: &::core::option::Option<CertificateRequestProperties>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn InstallCertificateAsync(&self, certificate: &::windows::core::HSTRING, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportPfxDataAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateEnrollmentManagerStatics2Impl: Sized {
    fn UserCertificateEnrollmentManager(&self) -> ::windows::core::Result<UserCertificateEnrollmentManager>;
    fn ImportPfxDataToKspAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING, keystorageprovider: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateEnrollmentManagerStatics3Impl: Sized {
    fn ImportPfxDataToKspWithParametersAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, pfximportparameters: &::core::option::Option<PfxImportParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateExtensionImpl: Sized {
    fn ObjectId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetObjectId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn IsCritical(&self) -> ::windows::core::Result<bool>;
    fn SetIsCritical(&self, value: bool) -> ::windows::core::Result<()>;
    fn EncodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetValue(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateFactoryImpl: Sized {
    fn CreateCertificate(&self, certblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<Certificate>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateKeyUsagesImpl: Sized {
    fn EncipherOnly(&self) -> ::windows::core::Result<bool>;
    fn SetEncipherOnly(&self, value: bool) -> ::windows::core::Result<()>;
    fn CrlSign(&self) -> ::windows::core::Result<bool>;
    fn SetCrlSign(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyCertificateSign(&self) -> ::windows::core::Result<bool>;
    fn SetKeyCertificateSign(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyAgreement(&self) -> ::windows::core::Result<bool>;
    fn SetKeyAgreement(&self, value: bool) -> ::windows::core::Result<()>;
    fn DataEncipherment(&self) -> ::windows::core::Result<bool>;
    fn SetDataEncipherment(&self, value: bool) -> ::windows::core::Result<()>;
    fn KeyEncipherment(&self) -> ::windows::core::Result<bool>;
    fn SetKeyEncipherment(&self, value: bool) -> ::windows::core::Result<()>;
    fn NonRepudiation(&self) -> ::windows::core::Result<bool>;
    fn SetNonRepudiation(&self, value: bool) -> ::windows::core::Result<()>;
    fn DigitalSignature(&self) -> ::windows::core::Result<bool>;
    fn SetDigitalSignature(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateQueryImpl: Sized {
    fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IssuerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetIssuerName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Thumbprint(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetThumbprint(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn HardwareOnly(&self) -> ::windows::core::Result<bool>;
    fn SetHardwareOnly(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateQuery2Impl: Sized {
    fn IncludeDuplicates(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeDuplicates(&self, value: bool) -> ::windows::core::Result<()>;
    fn IncludeExpiredCertificates(&self) -> ::windows::core::Result<bool>;
    fn SetIncludeExpiredCertificates(&self, value: bool) -> ::windows::core::Result<()>;
    fn StoreName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetStoreName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestPropertiesImpl: Sized {
    fn Subject(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSubject(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn KeyAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyAlgorithmName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn KeySize(&self) -> ::windows::core::Result<u32>;
    fn SetKeySize(&self, value: u32) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn HashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHashAlgorithmName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Exportable(&self) -> ::windows::core::Result<ExportOption>;
    fn SetExportable(&self, value: ExportOption) -> ::windows::core::Result<()>;
    fn KeyUsages(&self) -> ::windows::core::Result<EnrollKeyUsages>;
    fn SetKeyUsages(&self, value: EnrollKeyUsages) -> ::windows::core::Result<()>;
    fn KeyProtectionLevel(&self) -> ::windows::core::Result<KeyProtectionLevel>;
    fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> ::windows::core::Result<()>;
    fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyStorageProviderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestProperties2Impl: Sized {
    fn SmartcardReaderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetSmartcardReaderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SigningCertificate(&self) -> ::windows::core::Result<Certificate>;
    fn SetSigningCertificate(&self, value: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
    fn AttestationCredentialCertificate(&self) -> ::windows::core::Result<Certificate>;
    fn SetAttestationCredentialCertificate(&self, value: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestProperties3Impl: Sized {
    fn CurveName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCurveName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CurveParameters(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn SetCurveParameters(&self, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn ContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContainerNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainerName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContainerName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn UseExistingKey(&self) -> ::windows::core::Result<bool>;
    fn SetUseExistingKey(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateRequestProperties4Impl: Sized {
    fn SuppressedDefaults(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn SubjectAlternativeName(&self) -> ::windows::core::Result<SubjectAlternativeNameInfo>;
    fn Extensions(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<CertificateExtension>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStoreImpl: Sized {
    fn Add(&self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
    fn Delete(&self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStore2Impl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStoresStaticsImpl: Sized {
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>;
    fn FindAllWithQueryAsync(&self, query: &::core::option::Option<CertificateQuery>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<Certificate>>>;
    fn TrustedRootCertificationAuthorities(&self) -> ::windows::core::Result<CertificateStore>;
    fn IntermediateCertificationAuthorities(&self) -> ::windows::core::Result<CertificateStore>;
    fn GetStoreByName(&self, storename: &::windows::core::HSTRING) -> ::windows::core::Result<CertificateStore>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICertificateStoresStatics2Impl: Sized {
    fn GetUserStoreByName(&self, storename: &::windows::core::HSTRING) -> ::windows::core::Result<UserCertificateStore>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChainBuildingParametersImpl: Sized {
    fn EnhancedKeyUsages(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn ValidationTimestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn SetValidationTimestamp(&self, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn RevocationCheckEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetRevocationCheckEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn NetworkRetrievalEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetNetworkRetrievalEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn AuthorityInformationAccessEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetAuthorityInformationAccessEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn CurrentTimeValidationEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetCurrentTimeValidationEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn ExclusiveTrustRoots(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<Certificate>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IChainValidationParametersImpl: Sized {
    fn CertificateChainPolicy(&self) -> ::windows::core::Result<CertificateChainPolicy>;
    fn SetCertificateChainPolicy(&self, value: CertificateChainPolicy) -> ::windows::core::Result<()>;
    fn ServerDnsName(&self) -> ::windows::core::Result<super::super::super::Networking::HostName>;
    fn SetServerDnsName(&self, value: &::core::option::Option<super::super::super::Networking::HostName>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsAttachedSignatureImpl: Sized {
    fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
    fn Content(&self) -> ::windows::core::Result<::windows::core::Array<u8>>;
    fn Signers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>;
    fn VerifySignature(&self) -> ::windows::core::Result<SignatureValidationResult>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsAttachedSignatureFactoryImpl: Sized {
    fn CreateCmsAttachedSignature(&self, inputblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CmsAttachedSignature>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsAttachedSignatureStaticsImpl: Sized {
    fn GenerateSignatureAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>, signers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsDetachedSignatureImpl: Sized {
    fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
    fn Signers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<CmsSignerInfo>>;
    fn VerifySignatureAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<SignatureValidationResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsDetachedSignatureFactoryImpl: Sized {
    fn CreateCmsDetachedSignature(&self, inputblob: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<CmsDetachedSignature>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsDetachedSignatureStaticsImpl: Sized {
    fn GenerateSignatureAsync(&self, data: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, signers: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<CmsSignerInfo>>, certificates: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<Certificate>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IBuffer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsSignerInfoImpl: Sized {
    fn Certificate(&self) -> ::windows::core::Result<Certificate>;
    fn SetCertificate(&self, value: &::core::option::Option<Certificate>) -> ::windows::core::Result<()>;
    fn HashAlgorithmName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetHashAlgorithmName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn TimestampInfo(&self) -> ::windows::core::Result<CmsTimestampInfo>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICmsTimestampInfoImpl: Sized {
    fn SigningCertificate(&self) -> ::windows::core::Result<Certificate>;
    fn Certificates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<Certificate>>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAlgorithmNamesStaticsImpl: Sized {
    fn Rsa(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Dsa(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh521(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdsa256(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdsa384(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdsa521(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAlgorithmNamesStatics2Impl: Sized {
    fn Ecdsa(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Ecdh(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAttestationHelperStaticsImpl: Sized {
    fn DecryptTpmAttestationCredentialAsync(&self, credential: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn GetTpmAttestationCredentialId(&self, credential: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyAttestationHelperStatics2Impl: Sized {
    fn DecryptTpmAttestationCredentialWithContainerNameAsync(&self, credential: &::windows::core::HSTRING, containername: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyStorageProviderNamesStaticsImpl: Sized {
    fn SoftwareKeyStorageProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SmartcardKeyStorageProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PlatformKeyStorageProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IKeyStorageProviderNamesStatics2Impl: Sized {
    fn PassportKeyStorageProvider(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPfxImportParametersImpl: Sized {
    fn Exportable(&self) -> ::windows::core::Result<ExportOption>;
    fn SetExportable(&self, value: ExportOption) -> ::windows::core::Result<()>;
    fn KeyProtectionLevel(&self) -> ::windows::core::Result<KeyProtectionLevel>;
    fn SetKeyProtectionLevel(&self, value: KeyProtectionLevel) -> ::windows::core::Result<()>;
    fn InstallOptions(&self) -> ::windows::core::Result<InstallOptions>;
    fn SetInstallOptions(&self, value: InstallOptions) -> ::windows::core::Result<()>;
    fn FriendlyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFriendlyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn KeyStorageProviderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetKeyStorageProviderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ContainerNamePrefix(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContainerNamePrefix(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ReaderName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetReaderName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStandardCertificateStoreNamesStaticsImpl: Sized {
    fn Personal(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TrustedRootCertificationAuthorities(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IntermediateCertificationAuthorities(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISubjectAlternativeNameInfoImpl: Sized {
    fn EmailName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn IPAddress(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn Url(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DnsName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn DistinguishedName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn PrincipalName(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISubjectAlternativeNameInfo2Impl: Sized {
    fn EmailNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn IPAddresses(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Urls(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DnsNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn DistinguishedNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn PrincipalNames(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>;
    fn Extension(&self) -> ::windows::core::Result<CertificateExtension>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserCertificateEnrollmentManagerImpl: Sized {
    fn CreateRequestAsync(&self, request: &::core::option::Option<CertificateRequestProperties>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>;
    fn InstallCertificateAsync(&self, certificate: &::windows::core::HSTRING, installoption: InstallOptions) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportPfxDataAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ImportPfxDataToKspAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, exportable: ExportOption, keyprotectionlevel: KeyProtectionLevel, installoption: InstallOptions, friendlyname: &::windows::core::HSTRING, keystorageprovider: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserCertificateEnrollmentManager2Impl: Sized {
    fn ImportPfxDataToKspWithParametersAsync(&self, pfxdata: &::windows::core::HSTRING, password: &::windows::core::HSTRING, pfximportparameters: &::core::option::Option<PfxImportParameters>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUserCertificateStoreImpl: Sized {
    fn RequestAddAsync(&self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestDeleteAsync(&self, certificate: &::core::option::Option<Certificate>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
