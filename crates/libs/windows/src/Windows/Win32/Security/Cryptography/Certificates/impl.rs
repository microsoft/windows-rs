#[cfg(feature = "Win32_System_Com")]
pub trait IAlternativeNameImpl: Sized + IDispatchImpl {
    fn InitializeFromString();
    fn InitializeFromRawData();
    fn InitializeFromOtherName();
    fn Type();
    fn StrValue();
    fn ObjectId();
    fn RawData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAlternativeNamesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBinaryConverterImpl: Sized + IDispatchImpl {
    fn StringToString();
    fn VariantByteArrayToString();
    fn StringToVariantByteArray();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBinaryConverter2Impl: Sized + IBinaryConverterImpl + IDispatchImpl {
    fn StringArrayToVariantArray();
    fn VariantArrayToStringArray();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICEnrollImpl: Sized + IDispatchImpl {
    fn createFilePKCS10();
    fn acceptFilePKCS7();
    fn createPKCS10();
    fn acceptPKCS7();
    fn getCertFromPKCS7();
    fn enumProviders();
    fn enumContainers();
    fn freeRequestInfo();
    fn MyStoreName();
    fn SetMyStoreName();
    fn MyStoreType();
    fn SetMyStoreType();
    fn MyStoreFlags();
    fn SetMyStoreFlags();
    fn CAStoreName();
    fn SetCAStoreName();
    fn CAStoreType();
    fn SetCAStoreType();
    fn CAStoreFlags();
    fn SetCAStoreFlags();
    fn RootStoreName();
    fn SetRootStoreName();
    fn RootStoreType();
    fn SetRootStoreType();
    fn RootStoreFlags();
    fn SetRootStoreFlags();
    fn RequestStoreName();
    fn SetRequestStoreName();
    fn RequestStoreType();
    fn SetRequestStoreType();
    fn RequestStoreFlags();
    fn SetRequestStoreFlags();
    fn ContainerName();
    fn SetContainerName();
    fn ProviderName();
    fn SetProviderName();
    fn ProviderType();
    fn SetProviderType();
    fn KeySpec();
    fn SetKeySpec();
    fn ProviderFlags();
    fn SetProviderFlags();
    fn UseExistingKeySet();
    fn SetUseExistingKeySet();
    fn GenKeyFlags();
    fn SetGenKeyFlags();
    fn DeleteRequestCert();
    fn SetDeleteRequestCert();
    fn WriteCertToCSP();
    fn SetWriteCertToCSP();
    fn SPCFileName();
    fn SetSPCFileName();
    fn PVKFileName();
    fn SetPVKFileName();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICEnroll2Impl: Sized + ICEnrollImpl + IDispatchImpl {
    fn addCertTypeToRequest();
    fn addNameValuePairToSignature();
    fn WriteCertToUserDS();
    fn SetWriteCertToUserDS();
    fn EnableT61DNEncoding();
    fn SetEnableT61DNEncoding();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICEnroll3Impl: Sized + ICEnroll2Impl + ICEnrollImpl + IDispatchImpl {
    fn InstallPKCS7();
    fn Reset();
    fn GetSupportedKeySpec();
    fn GetKeyLen();
    fn EnumAlgs();
    fn GetAlgName();
    fn SetReuseHardwareKeyIfUnableToGenNew();
    fn ReuseHardwareKeyIfUnableToGenNew();
    fn SetHashAlgID();
    fn HashAlgID();
    fn SetLimitExchangeKeyToEncipherment();
    fn LimitExchangeKeyToEncipherment();
    fn SetEnableSMIMECapabilities();
    fn EnableSMIMECapabilities();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICEnroll4Impl: Sized + ICEnroll3Impl + ICEnroll2Impl + ICEnrollImpl + IDispatchImpl {
    fn SetPrivateKeyArchiveCertificate();
    fn PrivateKeyArchiveCertificate();
    fn SetThumbPrint();
    fn ThumbPrint();
    fn binaryToString();
    fn stringToBinary();
    fn addExtensionToRequest();
    fn addAttributeToRequest();
    fn addNameValuePairToRequest();
    fn resetExtensions();
    fn resetAttributes();
    fn createRequest();
    fn createFileRequest();
    fn acceptResponse();
    fn acceptFileResponse();
    fn getCertFromResponse();
    fn getCertFromFileResponse();
    fn createPFX();
    fn createFilePFX();
    fn setPendingRequestInfo();
    fn enumPendingRequest();
    fn removePendingRequest();
    fn GetKeyLenEx();
    fn InstallPKCS7Ex();
    fn addCertTypeToRequestEx();
    fn getProviderType();
    fn SetSignerCertificate();
    fn SetClientId();
    fn ClientId();
    fn addBlobPropertyToCertificate();
    fn resetBlobProperties();
    fn SetIncludeSubjectKeyID();
    fn IncludeSubjectKeyID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertAdminImpl: Sized + IDispatchImpl {
    fn IsValidCertificate();
    fn GetRevocationReason();
    fn RevokeCertificate();
    fn SetRequestAttributes();
    fn SetCertificateExtension();
    fn DenyRequest();
    fn ResubmitRequest();
    fn PublishCRL();
    fn GetCRL();
    fn ImportCertificate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertAdmin2Impl: Sized + ICertAdminImpl + IDispatchImpl {
    fn PublishCRLs();
    fn GetCAProperty();
    fn SetCAProperty();
    fn GetCAPropertyFlags();
    fn GetCAPropertyDisplayName();
    fn GetArchivedKey();
    fn GetConfigEntry();
    fn SetConfigEntry();
    fn ImportKey();
    fn GetMyRoles();
    fn DeleteRow();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertConfigImpl: Sized + IDispatchImpl {
    fn Reset();
    fn Next();
    fn GetField();
    fn GetConfig();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertConfig2Impl: Sized + ICertConfigImpl + IDispatchImpl {
    fn SetSharedFolder();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeAltNameImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetNameCount();
    fn GetNameChoice();
    fn GetName();
    fn Reset();
    fn SetNameEntry();
    fn Encode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeAltName2Impl: Sized + ICertEncodeAltNameImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
    fn GetNameBlob();
    fn SetNameEntryBlob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeBitStringImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetBitCount();
    fn GetBitString();
    fn Encode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeBitString2Impl: Sized + ICertEncodeBitStringImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
    fn GetBitStringBlob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeCRLDistInfoImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetDistPointCount();
    fn GetNameCount();
    fn GetNameChoice();
    fn GetName();
    fn Reset();
    fn SetNameCount();
    fn SetNameEntry();
    fn Encode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeCRLDistInfo2Impl: Sized + ICertEncodeCRLDistInfoImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeDateArrayImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetCount();
    fn GetValue();
    fn Reset();
    fn SetValue();
    fn Encode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeDateArray2Impl: Sized + ICertEncodeDateArrayImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeLongArrayImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetCount();
    fn GetValue();
    fn Reset();
    fn SetValue();
    fn Encode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeLongArray2Impl: Sized + ICertEncodeLongArrayImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeStringArrayImpl: Sized + IDispatchImpl {
    fn Decode();
    fn GetStringType();
    fn GetCount();
    fn GetValue();
    fn Reset();
    fn SetValue();
    fn Encode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertEncodeStringArray2Impl: Sized + ICertEncodeStringArrayImpl + IDispatchImpl {
    fn DecodeBlob();
    fn EncodeBlob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertExitImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Notify();
    fn GetDescription();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertExit2Impl: Sized + ICertExitImpl + IDispatchImpl {
    fn GetManageModule();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertGetConfigImpl: Sized + IDispatchImpl {
    fn GetConfig();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertManageModuleImpl: Sized + IDispatchImpl {
    fn GetProperty();
    fn SetProperty();
    fn Configure();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPolicyImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn VerifyRequest();
    fn GetDescription();
    fn ShutDown();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPolicy2Impl: Sized + ICertPolicyImpl + IDispatchImpl {
    fn GetManageModule();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertiesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn InitializeFromCertificate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyImpl: Sized + IDispatchImpl {
    fn InitializeFromCertificate();
    fn InitializeDecode();
    fn PropertyId();
    fn SetPropertyId();
    fn RawData();
    fn RemoveFromCertificate();
    fn SetValueOnCertificate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyArchivedImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn Archived();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyArchivedKeyHashImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn ArchivedKeyHash();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyAutoEnrollImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn TemplateName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyBackedUpImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn InitializeFromCurrentTime();
    fn Initialize();
    fn BackedUpValue();
    fn BackedUpTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyDescriptionImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn Description();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyEnrollmentImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn RequestId();
    fn CADnsName();
    fn CAName();
    fn FriendlyName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyEnrollmentPolicyServerImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn GetPolicyServerUrl();
    fn GetPolicyServerId();
    fn GetEnrollmentServerUrl();
    fn GetRequestIdString();
    fn GetPropertyFlags();
    fn GetUrlFlags();
    fn GetAuthentication();
    fn GetEnrollmentServerAuthentication();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyFriendlyNameImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn FriendlyName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyKeyProvInfoImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn PrivateKey();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyRenewalImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn InitializeFromCertificateHash();
    fn Renewal();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertyRequestOriginatorImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn InitializeFromLocalRequestOriginator();
    fn RequestOriginator();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertPropertySHA1HashImpl: Sized + ICertPropertyImpl + IDispatchImpl {
    fn Initialize();
    fn SHA1Hash();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertRequestImpl: Sized + IDispatchImpl {
    fn Submit();
    fn RetrievePending();
    fn GetLastStatus();
    fn GetRequestId();
    fn GetDispositionMessage();
    fn GetCACertificate();
    fn GetCertificate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertRequest2Impl: Sized + ICertRequestImpl + IDispatchImpl {
    fn GetIssuedCertificate();
    fn GetErrorMessageText();
    fn GetCAProperty();
    fn GetCAPropertyFlags();
    fn GetCAPropertyDisplayName();
    fn GetFullResponseProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertRequest3Impl: Sized + ICertRequest2Impl + ICertRequestImpl + IDispatchImpl {
    fn SetCredential();
    fn GetRequestIdString();
    fn GetIssuedCertificate2();
    fn GetRefreshPolicy();
}
pub trait ICertRequestDImpl: Sized {
    fn Request();
    fn GetCACert();
    fn Ping();
}
pub trait ICertRequestD2Impl: Sized + ICertRequestDImpl {
    fn Request2();
    fn GetCAProperty();
    fn GetCAPropertyInfo();
    fn Ping2();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertServerExitImpl: Sized + IDispatchImpl {
    fn SetContext();
    fn GetRequestProperty();
    fn GetRequestAttribute();
    fn GetCertificateProperty();
    fn GetCertificateExtension();
    fn GetCertificateExtensionFlags();
    fn EnumerateExtensionsSetup();
    fn EnumerateExtensions();
    fn EnumerateExtensionsClose();
    fn EnumerateAttributesSetup();
    fn EnumerateAttributes();
    fn EnumerateAttributesClose();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertServerPolicyImpl: Sized + IDispatchImpl {
    fn SetContext();
    fn GetRequestProperty();
    fn GetRequestAttribute();
    fn GetCertificateProperty();
    fn SetCertificateProperty();
    fn GetCertificateExtension();
    fn GetCertificateExtensionFlags();
    fn SetCertificateExtension();
    fn EnumerateExtensionsSetup();
    fn EnumerateExtensions();
    fn EnumerateExtensionsClose();
    fn EnumerateAttributesSetup();
    fn EnumerateAttributes();
    fn EnumerateAttributesClose();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertViewImpl: Sized + IDispatchImpl {
    fn OpenConnection();
    fn EnumCertViewColumn();
    fn GetColumnCount();
    fn GetColumnIndex();
    fn SetResultColumnCount();
    fn SetResultColumn();
    fn SetRestriction();
    fn OpenView();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertView2Impl: Sized + ICertViewImpl + IDispatchImpl {
    fn SetTable();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificateAttestationChallengeImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn DecryptChallenge();
    fn RequestID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificateAttestationChallenge2Impl: Sized + ICertificateAttestationChallengeImpl + IDispatchImpl {
    fn SetKeyContainerName();
    fn SetKeyBlob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificatePoliciesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificatePolicyImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn ObjectId();
    fn PolicyQualifiers();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificationAuthoritiesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn ComputeSiteCosts();
    fn ItemByName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICertificationAuthorityImpl: Sized + IDispatchImpl {
    fn Property();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICryptAttributeImpl: Sized + IDispatchImpl {
    fn InitializeFromObjectId();
    fn InitializeFromValues();
    fn ObjectId();
    fn Values();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICryptAttributesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn IndexByObjectId();
    fn AddRange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICspAlgorithmImpl: Sized + IDispatchImpl {
    fn GetAlgorithmOid();
    fn DefaultLength();
    fn IncrementLength();
    fn LongName();
    fn Valid();
    fn MaxLength();
    fn MinLength();
    fn Name();
    fn Type();
    fn Operations();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICspAlgorithmsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn ItemByName();
    fn IndexByObjectId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICspInformationImpl: Sized + IDispatchImpl {
    fn InitializeFromName();
    fn InitializeFromType();
    fn CspAlgorithms();
    fn HasHardwareRandomNumberGenerator();
    fn IsHardwareDevice();
    fn IsRemovable();
    fn IsSoftwareDevice();
    fn Valid();
    fn MaxKeyContainerNameLength();
    fn Name();
    fn Type();
    fn Version();
    fn KeySpec();
    fn IsSmartCard();
    fn GetDefaultSecurityDescriptor();
    fn LegacyCsp();
    fn GetCspStatusFromOperations();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICspInformationsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddAvailableCsps();
    fn ItemByName();
    fn GetCspStatusFromProviderName();
    fn GetCspStatusesFromOperations();
    fn GetEncryptionCspAlgorithms();
    fn GetHashAlgorithms();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICspStatusImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Ordinal();
    fn SetOrdinal();
    fn CspAlgorithm();
    fn CspInformation();
    fn EnrollmentStatus();
    fn DisplayName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICspStatusesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn ItemByName();
    fn ItemByOrdinal();
    fn ItemByOperations();
    fn ItemByProvider();
}
pub trait IEnrollImpl: Sized {
    fn createFilePKCS10WStr();
    fn acceptFilePKCS7WStr();
    fn createPKCS10WStr();
    fn acceptPKCS7Blob();
    fn getCertContextFromPKCS7();
    fn getMyStore();
    fn getCAStore();
    fn getROOTHStore();
    fn enumProvidersWStr();
    fn enumContainersWStr();
    fn freeRequestInfoBlob();
    fn MyStoreNameWStr();
    fn SetMyStoreNameWStr();
    fn MyStoreTypeWStr();
    fn SetMyStoreTypeWStr();
    fn MyStoreFlags();
    fn SetMyStoreFlags();
    fn CAStoreNameWStr();
    fn SetCAStoreNameWStr();
    fn CAStoreTypeWStr();
    fn SetCAStoreTypeWStr();
    fn CAStoreFlags();
    fn SetCAStoreFlags();
    fn RootStoreNameWStr();
    fn SetRootStoreNameWStr();
    fn RootStoreTypeWStr();
    fn SetRootStoreTypeWStr();
    fn RootStoreFlags();
    fn SetRootStoreFlags();
    fn RequestStoreNameWStr();
    fn SetRequestStoreNameWStr();
    fn RequestStoreTypeWStr();
    fn SetRequestStoreTypeWStr();
    fn RequestStoreFlags();
    fn SetRequestStoreFlags();
    fn ContainerNameWStr();
    fn SetContainerNameWStr();
    fn ProviderNameWStr();
    fn SetProviderNameWStr();
    fn ProviderType();
    fn SetProviderType();
    fn KeySpec();
    fn SetKeySpec();
    fn ProviderFlags();
    fn SetProviderFlags();
    fn UseExistingKeySet();
    fn SetUseExistingKeySet();
    fn GenKeyFlags();
    fn SetGenKeyFlags();
    fn DeleteRequestCert();
    fn SetDeleteRequestCert();
    fn WriteCertToUserDS();
    fn SetWriteCertToUserDS();
    fn EnableT61DNEncoding();
    fn SetEnableT61DNEncoding();
    fn WriteCertToCSP();
    fn SetWriteCertToCSP();
    fn SPCFileNameWStr();
    fn SetSPCFileNameWStr();
    fn PVKFileNameWStr();
    fn SetPVKFileNameWStr();
    fn HashAlgorithmWStr();
    fn SetHashAlgorithmWStr();
    fn RenewalCertificate();
    fn SetRenewalCertificate();
    fn AddCertTypeToRequestWStr();
    fn AddNameValuePairToSignatureWStr();
    fn AddExtensionsToRequest();
    fn AddAuthenticatedAttributesToPKCS7Request();
    fn CreatePKCS7RequestFromRequest();
}
pub trait IEnroll2Impl: Sized + IEnrollImpl {
    fn InstallPKCS7Blob();
    fn Reset();
    fn GetSupportedKeySpec();
    fn GetKeyLen();
    fn EnumAlgs();
    fn GetAlgNameWStr();
    fn SetReuseHardwareKeyIfUnableToGenNew();
    fn ReuseHardwareKeyIfUnableToGenNew();
    fn SetHashAlgID();
    fn HashAlgID();
    fn SetHStoreMy();
    fn SetHStoreCA();
    fn SetHStoreROOT();
    fn SetHStoreRequest();
    fn SetLimitExchangeKeyToEncipherment();
    fn LimitExchangeKeyToEncipherment();
    fn SetEnableSMIMECapabilities();
    fn EnableSMIMECapabilities();
}
pub trait IEnroll4Impl: Sized + IEnroll2Impl + IEnrollImpl {
    fn SetThumbPrintWStr();
    fn ThumbPrintWStr();
    fn SetPrivateKeyArchiveCertificate();
    fn GetPrivateKeyArchiveCertificate();
    fn binaryBlobToString();
    fn stringToBinaryBlob();
    fn addExtensionToRequestWStr();
    fn addAttributeToRequestWStr();
    fn addNameValuePairToRequestWStr();
    fn resetExtensions();
    fn resetAttributes();
    fn createRequestWStr();
    fn createFileRequestWStr();
    fn acceptResponseBlob();
    fn acceptFileResponseWStr();
    fn getCertContextFromResponseBlob();
    fn getCertContextFromFileResponseWStr();
    fn createPFXWStr();
    fn createFilePFXWStr();
    fn setPendingRequestInfoWStr();
    fn enumPendingRequestWStr();
    fn removePendingRequestWStr();
    fn GetKeyLenEx();
    fn InstallPKCS7BlobEx();
    fn AddCertTypeToRequestWStrEx();
    fn getProviderTypeWStr();
    fn addBlobPropertyToCertificateWStr();
    fn SetSignerCertificate();
    fn SetClientId();
    fn ClientId();
    fn SetIncludeSubjectKeyID();
    fn IncludeSubjectKeyID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCERTVIEWATTRIBUTEImpl: Sized + IDispatchImpl {
    fn Next();
    fn GetName();
    fn GetValue();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCERTVIEWCOLUMNImpl: Sized + IDispatchImpl {
    fn Next();
    fn GetName();
    fn GetDisplayName();
    fn GetType();
    fn IsIndexed();
    fn GetMaxLength();
    fn GetValue();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCERTVIEWEXTENSIONImpl: Sized + IDispatchImpl {
    fn Next();
    fn GetName();
    fn GetFlags();
    fn GetValue();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumCERTVIEWROWImpl: Sized + IDispatchImpl {
    fn Next();
    fn EnumCertViewColumn();
    fn EnumCertViewAttribute();
    fn EnumCertViewExtension();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetMaxIndex();
}
pub trait INDESPolicyImpl: Sized {
    fn Initialize();
    fn Uninitialize();
    fn GenerateChallenge();
    fn VerifyRequest();
    fn Notify();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOCSPAdminImpl: Sized + IDispatchImpl {
    fn OCSPServiceProperties();
    fn OCSPCAConfigurationCollection();
    fn GetConfiguration();
    fn SetConfiguration();
    fn GetMyRoles();
    fn Ping();
    fn SetSecurity();
    fn GetSecurity();
    fn GetSigningCertificates();
    fn GetHashAlgorithms();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOCSPCAConfigurationImpl: Sized + IDispatchImpl {
    fn Identifier();
    fn CACertificate();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn SigningFlags();
    fn SetSigningFlags();
    fn SigningCertificate();
    fn SetSigningCertificate();
    fn ReminderDuration();
    fn SetReminderDuration();
    fn ErrorCode();
    fn CSPName();
    fn KeySpec();
    fn ProviderCLSID();
    fn SetProviderCLSID();
    fn ProviderProperties();
    fn SetProviderProperties();
    fn Modified();
    fn LocalRevocationInformation();
    fn SetLocalRevocationInformation();
    fn SigningCertificateTemplate();
    fn SetSigningCertificateTemplate();
    fn CAConfig();
    fn SetCAConfig();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOCSPCAConfigurationCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ItemByName();
    fn CreateCAConfiguration();
    fn DeleteCAConfiguration();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOCSPPropertyImpl: Sized + IDispatchImpl {
    fn Name();
    fn Value();
    fn SetValue();
    fn Modified();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOCSPPropertyCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ItemByName();
    fn CreateProperty();
    fn DeleteProperty();
    fn InitializeFromProperties();
    fn GetAllProperties();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectIdImpl: Sized + IDispatchImpl {
    fn InitializeFromName();
    fn InitializeFromValue();
    fn InitializeFromAlgorithmName();
    fn Name();
    fn FriendlyName();
    fn SetFriendlyName();
    fn Value();
    fn GetAlgorithmName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IObjectIdsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddRange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPolicyQualifierImpl: Sized + IDispatchImpl {
    fn InitializeEncode();
    fn ObjectId();
    fn Qualifier();
    fn Type();
    fn RawData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPolicyQualifiersImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISignerCertificateImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Certificate();
    fn PrivateKey();
    fn Silent();
    fn SetSilent();
    fn ParentWindow();
    fn SetParentWindow();
    fn UIContextMessage();
    fn SetUIContextMessage();
    fn SetPin();
    fn SignatureInformation();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISignerCertificatesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn Find();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISmimeCapabilitiesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn AddFromCsp();
    fn AddAvailableSmimeCapabilities();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISmimeCapabilityImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn ObjectId();
    fn BitCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX500DistinguishedNameImpl: Sized + IDispatchImpl {
    fn Decode();
    fn Encode();
    fn Name();
    fn EncodedName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509AttributeImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn ObjectId();
    fn RawData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509AttributeArchiveKeyImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn EncryptedKeyBlob();
    fn EncryptionAlgorithm();
    fn EncryptionStrength();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509AttributeArchiveKeyHashImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncodeFromEncryptedKeyBlob();
    fn InitializeDecode();
    fn EncryptedKeyHashBlob();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509AttributeClientIdImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn ClientId();
    fn MachineDnsName();
    fn UserSamName();
    fn ProcessName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509AttributeCspProviderImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn KeySpec();
    fn ProviderName();
    fn Signature();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509AttributeExtensionsImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn X509Extensions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509AttributeOSVersionImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn OSVersion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509AttributeRenewalCertificateImpl: Sized + IX509AttributeImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn RenewalCertificate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509AttributesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Encode();
    fn ResetForEncode();
    fn GetInnerRequest();
    fn Type();
    fn EnrollmentContext();
    fn Silent();
    fn SetSilent();
    fn ParentWindow();
    fn SetParentWindow();
    fn UIContextMessage();
    fn SetUIContextMessage();
    fn SuppressDefaults();
    fn SetSuppressDefaults();
    fn RenewalCertificate();
    fn SetRenewalCertificate();
    fn ClientId();
    fn SetClientId();
    fn CspInformations();
    fn SetCspInformations();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn AlternateSignatureAlgorithm();
    fn SetAlternateSignatureAlgorithm();
    fn RawData();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestCertificateImpl: Sized + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn CheckPublicKeySignature();
    fn Issuer();
    fn SetIssuer();
    fn NotBefore();
    fn SetNotBefore();
    fn NotAfter();
    fn SetNotAfter();
    fn SerialNumber();
    fn SetSerialNumber();
    fn SignerCertificate();
    fn SetSignerCertificate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestCertificate2Impl: Sized + IX509CertificateRequestCertificateImpl + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn InitializeFromPrivateKeyTemplate();
    fn PolicyServer();
    fn Template();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestCmcImpl: Sized + IX509CertificateRequestPkcs7Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromInnerRequestTemplateName();
    fn TemplateObjectId();
    fn NullSigned();
    fn CryptAttributes();
    fn NameValuePairs();
    fn X509Extensions();
    fn CriticalExtensions();
    fn SuppressOids();
    fn TransactionId();
    fn SetTransactionId();
    fn SenderNonce();
    fn SetSenderNonce();
    fn SignatureInformation();
    fn ArchivePrivateKey();
    fn SetArchivePrivateKey();
    fn KeyArchivalCertificate();
    fn SetKeyArchivalCertificate();
    fn EncryptionAlgorithm();
    fn SetEncryptionAlgorithm();
    fn EncryptionStrength();
    fn SetEncryptionStrength();
    fn EncryptedKeyHash();
    fn SignerCertificates();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestCmc2Impl: Sized + IX509CertificateRequestCmcImpl + IX509CertificateRequestPkcs7Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn InitializeFromInnerRequestTemplate();
    fn PolicyServer();
    fn Template();
    fn CheckSignature();
    fn CheckCertificateSignature();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestPkcs10Impl: Sized + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplateName();
    fn InitializeFromPrivateKey();
    fn InitializeFromPublicKey();
    fn InitializeFromCertificate();
    fn InitializeDecode();
    fn CheckSignature();
    fn IsSmartCard();
    fn TemplateObjectId();
    fn PublicKey();
    fn PrivateKey();
    fn NullSigned();
    fn ReuseKey();
    fn OldCertificate();
    fn Subject();
    fn SetSubject();
    fn CspStatuses();
    fn SmimeCapabilities();
    fn SetSmimeCapabilities();
    fn SignatureInformation();
    fn KeyContainerNamePrefix();
    fn SetKeyContainerNamePrefix();
    fn CryptAttributes();
    fn X509Extensions();
    fn CriticalExtensions();
    fn SuppressOids();
    fn RawDataToBeSigned();
    fn Signature();
    fn GetCspStatuses();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestPkcs10V2Impl: Sized + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn InitializeFromPrivateKeyTemplate();
    fn InitializeFromPublicKeyTemplate();
    fn PolicyServer();
    fn Template();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestPkcs10V3Impl: Sized + IX509CertificateRequestPkcs10V2Impl + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn AttestPrivateKey();
    fn SetAttestPrivateKey();
    fn AttestationEncryptionCertificate();
    fn SetAttestationEncryptionCertificate();
    fn EncryptionAlgorithm();
    fn SetEncryptionAlgorithm();
    fn EncryptionStrength();
    fn SetEncryptionStrength();
    fn ChallengePassword();
    fn SetChallengePassword();
    fn NameValuePairs();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestPkcs10V4Impl: Sized + IX509CertificateRequestPkcs10V3Impl + IX509CertificateRequestPkcs10V2Impl + IX509CertificateRequestPkcs10Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn ClaimType();
    fn SetClaimType();
    fn AttestPrivateKeyPreferred();
    fn SetAttestPrivateKeyPreferred();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestPkcs7Impl: Sized + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplateName();
    fn InitializeFromCertificate();
    fn InitializeFromInnerRequest();
    fn InitializeDecode();
    fn RequesterName();
    fn SetRequesterName();
    fn SignerCertificate();
    fn SetSignerCertificate();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRequestPkcs7V2Impl: Sized + IX509CertificateRequestPkcs7Impl + IX509CertificateRequestImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn PolicyServer();
    fn Template();
    fn CheckCertificateSignature();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRevocationListImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeDecode();
    fn Encode();
    fn ResetForEncode();
    fn CheckPublicKeySignature();
    fn CheckSignature();
    fn Issuer();
    fn SetIssuer();
    fn ThisUpdate();
    fn SetThisUpdate();
    fn NextUpdate();
    fn SetNextUpdate();
    fn X509CRLEntries();
    fn X509Extensions();
    fn CriticalExtensions();
    fn SignerCertificate();
    fn SetSignerCertificate();
    fn CRLNumber();
    fn SetCRLNumber();
    fn CAVersion();
    fn SetCAVersion();
    fn BaseCRL();
    fn NullSigned();
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn AlternateSignatureAlgorithm();
    fn SetAlternateSignatureAlgorithm();
    fn SignatureInformation();
    fn RawData();
    fn RawDataToBeSigned();
    fn Signature();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRevocationListEntriesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn IndexBySerialNumber();
    fn AddRange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateRevocationListEntryImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn SerialNumber();
    fn RevocationDate();
    fn RevocationReason();
    fn SetRevocationReason();
    fn X509Extensions();
    fn CriticalExtensions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateTemplateImpl: Sized + IDispatchImpl {
    fn Property();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateTemplateWritableImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Commit();
    fn Property();
    fn SetProperty();
    fn Template();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509CertificateTemplatesImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn ItemByName();
    fn ItemByOid();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509EndorsementKeyImpl: Sized + IDispatchImpl {
    fn ProviderName();
    fn SetProviderName();
    fn Length();
    fn Opened();
    fn AddCertificate();
    fn RemoveCertificate();
    fn GetCertificateByIndex();
    fn GetCertificateCount();
    fn ExportPublicKey();
    fn Open();
    fn Close();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509EnrollmentImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeFromTemplateName();
    fn InitializeFromRequest();
    fn CreateRequest();
    fn Enroll();
    fn InstallResponse();
    fn CreatePFX();
    fn Request();
    fn Silent();
    fn SetSilent();
    fn ParentWindow();
    fn SetParentWindow();
    fn NameValuePairs();
    fn EnrollmentContext();
    fn Status();
    fn Certificate();
    fn Response();
    fn CertificateFriendlyName();
    fn SetCertificateFriendlyName();
    fn CertificateDescription();
    fn SetCertificateDescription();
    fn RequestId();
    fn CAConfigString();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509Enrollment2Impl: Sized + IX509EnrollmentImpl + IDispatchImpl {
    fn InitializeFromTemplate();
    fn InstallResponse2();
    fn PolicyServer();
    fn Template();
    fn RequestIdString();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509EnrollmentHelperImpl: Sized + IDispatchImpl {
    fn AddPolicyServer();
    fn AddEnrollmentServer();
    fn Enroll();
    fn Initialize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509EnrollmentPolicyServerImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn LoadPolicy();
    fn GetTemplates();
    fn GetCAsForTemplate();
    fn GetCAs();
    fn Validate();
    fn GetCustomOids();
    fn GetNextUpdateTime();
    fn GetLastUpdateTime();
    fn GetPolicyServerUrl();
    fn GetPolicyServerId();
    fn GetFriendlyName();
    fn GetIsDefaultCEP();
    fn GetUseClientId();
    fn GetAllowUnTrustedCA();
    fn GetCachePath();
    fn GetCacheDir();
    fn GetAuthFlags();
    fn SetCredential();
    fn QueryChanges();
    fn InitializeImport();
    fn Export();
    fn Cost();
    fn SetCost();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509EnrollmentStatusImpl: Sized + IDispatchImpl {
    fn AppendText();
    fn Text();
    fn SetText();
    fn Selected();
    fn SetSelected();
    fn Display();
    fn SetDisplay();
    fn Status();
    fn SetStatus();
    fn Error();
    fn SetError();
    fn ErrorText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509EnrollmentWebClassFactoryImpl: Sized + IDispatchImpl {
    fn CreateObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn ObjectId();
    fn RawData();
    fn Critical();
    fn SetCritical();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionAlternativeNamesImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn AlternativeNames();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionAuthorityKeyIdentifierImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn AuthorityKeyIdentifier();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionBasicConstraintsImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn IsCA();
    fn PathLenConstraint();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionCertificatePoliciesImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn Policies();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionEnhancedKeyUsageImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn EnhancedKeyUsage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionKeyUsageImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn KeyUsage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionMSApplicationPoliciesImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn Policies();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionSmimeCapabilitiesImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn SmimeCapabilities();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionSubjectKeyIdentifierImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn SubjectKeyIdentifier();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionTemplateImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn TemplateOid();
    fn MajorVersion();
    fn MinorVersion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionTemplateNameImpl: Sized + IX509ExtensionImpl + IDispatchImpl {
    fn InitializeEncode();
    fn InitializeDecode();
    fn TemplateName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509ExtensionsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn IndexByObjectId();
    fn AddRange();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509MachineEnrollmentFactoryImpl: Sized + IDispatchImpl {
    fn CreateObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509NameValuePairImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Value();
    fn Name();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509NameValuePairsImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509PolicyServerListManagerImpl: Sized + IDispatchImpl {
    fn ItemByIndex();
    fn Count();
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn Clear();
    fn Initialize();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509PolicyServerUrlImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn Url();
    fn SetUrl();
    fn Default();
    fn SetDefault();
    fn Flags();
    fn SetFlags();
    fn AuthFlags();
    fn SetAuthFlags();
    fn Cost();
    fn SetCost();
    fn GetStringProperty();
    fn SetStringProperty();
    fn UpdateRegistry();
    fn RemoveFromRegistry();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509PrivateKeyImpl: Sized + IDispatchImpl {
    fn Open();
    fn Create();
    fn Close();
    fn Delete();
    fn Verify();
    fn Import();
    fn Export();
    fn ExportPublicKey();
    fn ContainerName();
    fn SetContainerName();
    fn ContainerNamePrefix();
    fn SetContainerNamePrefix();
    fn ReaderName();
    fn SetReaderName();
    fn CspInformations();
    fn SetCspInformations();
    fn CspStatus();
    fn SetCspStatus();
    fn ProviderName();
    fn SetProviderName();
    fn ProviderType();
    fn SetProviderType();
    fn LegacyCsp();
    fn SetLegacyCsp();
    fn Algorithm();
    fn SetAlgorithm();
    fn KeySpec();
    fn SetKeySpec();
    fn Length();
    fn SetLength();
    fn ExportPolicy();
    fn SetExportPolicy();
    fn KeyUsage();
    fn SetKeyUsage();
    fn KeyProtection();
    fn SetKeyProtection();
    fn MachineContext();
    fn SetMachineContext();
    fn SecurityDescriptor();
    fn SetSecurityDescriptor();
    fn Certificate();
    fn SetCertificate();
    fn UniqueContainerName();
    fn Opened();
    fn DefaultContainer();
    fn Existing();
    fn SetExisting();
    fn Silent();
    fn SetSilent();
    fn ParentWindow();
    fn SetParentWindow();
    fn UIContextMessage();
    fn SetUIContextMessage();
    fn SetPin();
    fn FriendlyName();
    fn SetFriendlyName();
    fn Description();
    fn SetDescription();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509PrivateKey2Impl: Sized + IX509PrivateKeyImpl + IDispatchImpl {
    fn HardwareKeyUsage();
    fn SetHardwareKeyUsage();
    fn AlternateStorageLocation();
    fn SetAlternateStorageLocation();
    fn AlgorithmName();
    fn SetAlgorithmName();
    fn AlgorithmParameters();
    fn SetAlgorithmParameters();
    fn ParametersExportType();
    fn SetParametersExportType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509PublicKeyImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeFromEncodedPublicKeyInfo();
    fn Algorithm();
    fn Length();
    fn EncodedKey();
    fn EncodedParameters();
    fn ComputeKeyIdentifier();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509SCEPEnrollmentImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeForPending();
    fn CreateRequestMessage();
    fn CreateRetrievePendingMessage();
    fn CreateRetrieveCertificateMessage();
    fn ProcessResponseMessage();
    fn SetServerCapabilities();
    fn FailInfo();
    fn SignerCertificate();
    fn SetSignerCertificate();
    fn OldCertificate();
    fn SetOldCertificate();
    fn TransactionId();
    fn SetTransactionId();
    fn Request();
    fn CertificateFriendlyName();
    fn SetCertificateFriendlyName();
    fn Status();
    fn Certificate();
    fn Silent();
    fn SetSilent();
    fn DeleteRequest();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509SCEPEnrollment2Impl: Sized + IX509SCEPEnrollmentImpl + IDispatchImpl {
    fn CreateChallengeAnswerMessage();
    fn ProcessResponseMessage2();
    fn ResultMessageText();
    fn DelayRetry();
    fn ActivityId();
    fn SetActivityId();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509SCEPEnrollmentHelperImpl: Sized + IDispatchImpl {
    fn Initialize();
    fn InitializeForPending();
    fn Enroll();
    fn FetchPending();
    fn X509SCEPEnrollment();
    fn ResultMessageText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IX509SignatureInformationImpl: Sized + IDispatchImpl {
    fn HashAlgorithm();
    fn SetHashAlgorithm();
    fn PublicKeyAlgorithm();
    fn SetPublicKeyAlgorithm();
    fn Parameters();
    fn SetParameters();
    fn AlternateSignatureAlgorithm();
    fn SetAlternateSignatureAlgorithm();
    fn AlternateSignatureAlgorithmSet();
    fn NullSigned();
    fn SetNullSigned();
    fn GetSignatureAlgorithm();
    fn SetDefaultValues();
}
