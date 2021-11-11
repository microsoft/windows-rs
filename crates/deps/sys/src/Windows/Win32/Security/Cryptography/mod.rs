#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Security_Cryptography_Catalog")]
pub mod Catalog;
#[cfg(feature = "Win32_Security_Cryptography_Certificates")]
pub mod Certificates;
#[cfg(feature = "Win32_Security_Cryptography_Sip")]
pub mod Sip;
#[cfg(feature = "Win32_Security_Cryptography_UI")]
pub mod UI;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptAddContextFunction();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCloseAlgorithmProvider();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptConfigureContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptConfigureContextFunction();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptCreateMultiHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDecrypt();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeleteContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKeyCapi();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDeriveKeyPBKDF2();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroyHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroyKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDestroySecret();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDuplicateHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptDuplicateKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEncrypt();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumAlgorithms();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContextFunctionProviders();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContextFunctions();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumContexts();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumProviders();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptEnumRegisteredProviders();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptExportKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptFinalizeKeyPair();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptFinishHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn BCryptFreeBuffer();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenRandom();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenerateKeyPair();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGenerateSymmetricKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGetFipsAlgorithmMode();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptGetProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptHashData();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptImportKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptImportKeyPair();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptKeyDerivation();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptOpenAlgorithmProvider();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptProcessMultiOperations();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextConfiguration();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextFunctionConfiguration();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryContextFunctionProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptQueryProviderRegistration();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptRegisterConfigChangeNotify();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptRemoveContextFunction();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptResolveProviders();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSecretAgreement();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSetContextFunctionProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSetProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptSignHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptUnregisterConfigChangeNotify();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BCryptVerifySignature();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCRLContextToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCRLLinkToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCTLContextToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCTLLinkToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCertificateContextToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddCertificateLinkToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCRLToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCTLToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToSystemStoreA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEncodedCertificateToSystemStoreW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddEnhancedKeyUsageIdentifier();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertAddRefServerOcspResponse();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertAddRefServerOcspResponseContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddSerializedElementToStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAddStoreToCollection();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertAlgIdToOID();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertCloseServerOcspResponse();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCloseStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareCertificate();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareCertificateName();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCompareIntegerBlob();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertComparePublicKeyInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertControlStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCRLContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCTLContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCTLEntryFromCertificateContextProperties();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCertificateChainEngine();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateCertificateContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertCreateSelfSignCertificate();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCRLFromStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCTLFromStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDeleteCertificateFromStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCRLContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCTLContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCertificateChain();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertDuplicateCertificateContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertDuplicateStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCRLContextProperties();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCRLsInStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCTLContextProperties();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCTLsInStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCertificateContextProperties();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumCertificatesInStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumPhysicalStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSubjectInSortedCTL();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSystemStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertEnumSystemStoreLocation();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindAttribute();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCRLInStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCTLInStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCertificateInCRL();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindCertificateInStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindChainInStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindExtension();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindRDNAttr();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindSubjectInCTL();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFindSubjectInSortedCTL();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCRLContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCTLContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateChain();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertFreeCertificateChainEngine();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateChainList();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertFreeCertificateContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertFreeServerOcspResponseContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCRLContextProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCRLFromStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCTLContextProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCertificateChain();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetCertificateContextProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetEnhancedKeyUsage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetIntendedKeyUsage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetIssuerCertificateFromStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetNameStringA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetNameStringW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetPublicKeyLength();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertGetServerOcspResponseContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetStoreProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetSubjectCertificateFromStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertGetValidUsages();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsRDNAttrsInCertificateName();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsStrongHashToSign();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsValidCRLForCertificate();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertIsWeakHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertNameToStrA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertNameToStrW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOIDToAlgId();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenServerOcspResponse();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenSystemStoreA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertOpenSystemStoreW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRDNValueToStrA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRDNValueToStrW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRegisterPhysicalStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRegisterSystemStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRemoveEnhancedKeyUsageIdentifier();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CertRemoveStoreFromCollection();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertResyncCertificateChainEngine();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertRetrieveLogoOrBiometricInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSaveStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSelectCertificateChains();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCRLStoreElement();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCTLStoreElement();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSerializeCertificateStoreElement();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCRLContextProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCTLContextProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCertificateContextPropertiesFromCTLEntry();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetCertificateContextProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetEnhancedKeyUsage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertSetStoreProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertStrToNameA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertStrToNameW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertUnregisterPhysicalStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertUnregisterSystemStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCRLRevocation();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCRLTimeValidity();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCTLUsage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyCertificateChainPolicy();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyRevocation();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifySubjectCertificateContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyTimeValidity();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CertVerifyValidityNesting();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CloseCryptoHandle();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireCertificatePrivateKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireContextA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptAcquireContextW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptBinaryToStringA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptBinaryToStringW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCloseAsyncHandle();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptContextAddRef();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateAsyncHandle();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptCreateKeyIdentifierFromCSP();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeMessage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeObject();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecodeObjectEx();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecrypt();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecryptAndVerifyMessageSignature();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDecryptMessage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDeriveKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDestroyHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDestroyKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDuplicateHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptDuplicateKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncodeObject();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncodeObjectEx();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncrypt();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEncryptMessage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumKeyIdentifierProperties();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumOIDFunction();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumOIDInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProviderTypesA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProviderTypesW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProvidersA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptEnumProvidersW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPKCS8();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfoEx();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptExportPublicKeyInfoFromBCryptKeyHandle();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindCertificateKeyProvInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindLocalizedName();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFindOIDInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFormatObject();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptFreeOIDFunctionAddress();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGenKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGenRandom();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetAsyncParam();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultOIDDllList();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultOIDFunctionAddress();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultProviderA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetDefaultProviderW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetHashParam();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetKeyIdentifierProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetKeyParam();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptGetMessageCertificates();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptGetMessageSignerCount();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetOIDFunctionAddress();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetOIDFunctionValue();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetObjectUrl();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetProvParam();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptGetUserKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashCertificate();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashCertificate2();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashData();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashMessage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashPublicKeyInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashSessionKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptHashToBeSigned();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPKCS8();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfoEx();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptImportPublicKeyInfoEx2();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInitOIDFunctionSet();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallCancelRetrieval();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallDefaultContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptInstallOIDFunctionAddress();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptMemAlloc();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptMemFree();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptMemRealloc();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCalculateEncodedLength();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgClose();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgControl();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCountersign();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgCountersignEncoded();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptMsgDuplicate();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgEncodeAndSignCTL();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgGetAndVerifySigner();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgGetParam();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgOpenToDecode();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgOpenToEncode();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgSignCTL();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgUpdate();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgVerifyCountersignatureEncoded();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptMsgVerifyCountersignatureEncodedEx();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptProtectData();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptProtectMemory();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptQueryObject();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterDefaultOIDFunction();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterOIDFunction();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRegisterOIDInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptReleaseContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveObjectByUrlA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveObjectByUrlW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptRetrieveTimeStamp();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetAsyncParam();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetHashParam();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetKeyIdentifierProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetKeyParam();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn CryptSetOIDFunctionValue();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProvParam();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderExA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderExW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSetProviderW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignAndEncodeCertificate();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignAndEncryptMessage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignCertificate();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignHashA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignHashW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignMessage();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptSignMessageWithKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptStringToBinaryA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptStringToBinaryW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUninstallCancelRetrieval();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUninstallDefaultContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnprotectData();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnprotectMemory();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterDefaultOIDFunction();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterOIDFunction();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUnregisterOIDInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptUpdateProtectedState();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyCertificateSignature();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyCertificateSignatureEx();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyDetachedMessageHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyDetachedMessageSignature();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageSignature();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyMessageSignatureWithKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifySignatureA();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifySignatureW();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptVerifyTimeStampSignature();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlAddObject();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlClose();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlCreateReference();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlDigestReference();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlEncode();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlEnumAlgorithmInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlFindAlgorithmInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetAlgorithmInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetDocContext();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetReference();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetSignature();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlGetStatus();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlGetTransforms();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlImportPublicKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlOpenToDecode();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlOpenToEncode();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlSetHMACSecret();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CryptXmlSign();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn CryptXmlVerifySignature();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Decrypt();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Encrypt();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindCertsByIssuer();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeToken();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GenerateDerivedKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn GetBrowserToken();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn GetCryptoTransform();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn GetKeyedHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetToken();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn HashCore();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn HashFinal();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportInformationCard();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn ManageCardSpace();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptCloseProtectionDescriptor();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptCreateClaim();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptCreatePersistedKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptCreateProtectionDescriptor();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptDecrypt();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptDeleteKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptDeriveKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptEncrypt();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptEnumAlgorithms();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptEnumKeys();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptEnumStorageProviders();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptExportKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptFinalizeKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptFreeBuffer();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptFreeObject();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptGetProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptGetProtectionDescriptorInfo();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptImportKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptIsAlgSupported();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptIsKeyHandle();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptKeyDerivation();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptNotifyChangeKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptOpenKey();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptOpenStorageProvider();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptProtectSecret();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptQueryProtectionDescriptorName();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptRegisterProtectionDescriptorName();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptSecretAgreement();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptSetProperty();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptSignHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptStreamClose();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToProtect();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToUnprotect();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamOpenToUnprotectEx();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptStreamUpdate();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptTranslateHandle();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NCryptUnprotectSecret();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptVerifyClaim();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn NCryptVerifySignature();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXExportCertStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXExportCertStoreEx();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXImportCertStore();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXIsPFXBlob();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PFXVerifyPassword();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SignHash();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn TransformBlock();
    #[doc = "*Required features: `Win32_Security_Cryptography`*"]
    pub fn TransformFinalBlock();
    #[doc = "*Required features: `Win32_Security_Cryptography`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyHash();
}
